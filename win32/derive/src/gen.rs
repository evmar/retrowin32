use super::parse;
use crate::parse::DllExport;
use proc_macro2::TokenStream;
use quote::quote;

/// Generate the wrapper function that calls a win32api function by taking arguments using from_x86.
///
/// The caller of winapi functions is responsible for pushing/popping the
/// return address, because some callers actually 'jmp' directly.
///
/// This macro generates shim wrappers of functions, taking their
/// input args off the stack and forwarding their return values via eax.
pub fn fn_wrapper(module: TokenStream, dllexport: &DllExport) -> (TokenStream, TokenStream) {
    // Example: IDirectDraw::QueryInterface
    let base_name = &dllexport.func.sig.ident; // QueryInterface
    let impl_name = match dllexport.vtable {
        Some(vtable) => quote!(#module::#vtable::#base_name), // winapi::ddraw::IDirectDraw::QueryInterface
        None => quote!(#module::#base_name),                  // winapi::kernel32::LoadLibrary
    };
    let sym_name = &dllexport.sym_name; // IDirectDraw_QueryInterface

    let mut fetch_args = TokenStream::new();
    fetch_args.extend(quote!(let mem = machine.mem().detach();));
    let mut stack_offset = 4u32; // return address
    for parse::Argument { name, ty, stack } in dllexport.args.iter() {
        // We expect all the stack_offset math to be inlined by the compiler into plain constants.
        // TODO: reading the args in reverse would produce fewer bounds checks...
        fetch_args.extend(quote! {
            let #name = <#ty>::from_stack(mem, esp + #stack_offset);
        });
        stack_offset += stack.consumed();
    }

    let stack_consumed = dllexport.stack_consumed();

    // If the function is async, we need to handle the return value a bit differently.
    let is_async = dllexport.func.sig.asyncness.is_some();
    let args = dllexport
        .args
        .iter()
        .map(|arg| arg.name)
        .collect::<Vec<_>>();
    let body = if dllexport.func.sig.asyncness.is_some() {
        quote! {
        #fetch_args
        #[cfg(feature = "x86-emu")]
        {
            // Yuck: we know Machine will outlive the future, but Rust doesn't.
            // At least we managed to isolate the yuck to this point.
            let m: *mut Machine = machine;
            let result = async move {
                use memory::Extensions;
                let machine = unsafe { &mut *m };
                let result = #impl_name(machine, #(#args),*).await;
                let cpu = &mut machine.emu.x86.cpu_mut();
                cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                cpu.regs.set32(x86::Register::EAX, result.to_raw());
            };
            machine.emu.x86.cpu_mut().call_async(Box::pin(result));
            // async block will set up the stack and eip.
            0
        }
        #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
        {
            // In the non-emulated case, we synchronously evaluate the future.
            let pin = std::pin::pin!(#impl_name(machine, #(#args),*));
            crate::shims::call_sync(pin).to_raw()
        }
        }
    } else {
        quote! {
            #fetch_args
            #impl_name(machine, #(#args),*).to_raw()
        }
    };

    let name_str = match dllexport.vtable {
        Some(vtable) => format!("{}::{}", vtable, base_name), // "IDirectDraw::QueryInterface"
        None => format!("{}", base_name),                     // "LoadLibrary"
    };

    (
        quote!(pub unsafe fn #sym_name(machine: &mut Machine, esp: u32) -> u32 { #body }),
        quote!(pub const #sym_name: Shim = Shim {
            name: #name_str,
            func: impls::#sym_name,
            stack_consumed: #stack_consumed,
            is_async: #is_async,
        };),
    )
}
