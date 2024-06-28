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
    let mut args = Vec::new();
    let mut tys = Vec::new();

    // Skip first arg, the &Machine.
    for arg in dllexport.func.sig.inputs.iter().skip(1) {
        let arg = match arg {
            syn::FnArg::Typed(arg) => arg,
            _ => unimplemented!(),
        };
        let name = match arg.pat.as_ref() {
            syn::Pat::Ident(ident) => &ident.ident,
            _ => unimplemented!(),
        };
        args.push(name);
        tys.push(&arg.ty);
    }

    let name = &dllexport.func.sig.ident;
    let name_str = name.to_string();

    let mut fetch_args = TokenStream::new();
    fetch_args.extend(quote!(let mem = machine.mem().detach();));
    let mut stack_offset = 4u32; // return address
    for (arg, ty) in args.iter().zip(tys.iter()) {
        // We expect all the stack_offset math to be inlined by the compiler into plain constants.
        // TODO: reading the args in reverse would produce fewer bounds checks...
        fetch_args.extend(quote! {
            let #arg = <#ty>::from_stack(mem, esp + #stack_offset);
        });
        match parse::parse_argument_type(ty) {
            parse::Argument::Ordinary(ofs) => stack_offset += ofs,
            parse::Argument::VarArgs => {
                // VarArgs only works for cdecl functions
                assert!(matches!(dllexport.meta.callconv, parse::CallConv::Cdecl));
            }
        }
    }

    let stack_consumed = match dllexport.meta.callconv {
        parse::CallConv::Stdcall => stack_offset - 4, // don't include return address
        parse::CallConv::Cdecl => 0,                  // caller cleaned
    };

    // If the function is async, we need to handle the return value a bit differently.
    let is_async = dllexport.func.sig.asyncness.is_some();
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
                let result = #module::#name(machine, #(#args),*).await;
                let regs = &mut machine.emu.x86.cpu_mut().regs;
                regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                *regs.get32_mut(x86::Register::ESP) += #stack_consumed + 4;
                regs.set32(x86::Register::EAX, result.to_raw());
            };
            machine.emu.x86.cpu_mut().call_async(Box::pin(result));
            // async block will set up the stack and eip.
            0
        }
        #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
        {
            // In the non-emulated case, we synchronously evaluate the future.
            let pin = std::pin::pin!(#module::#name(machine, #(#args),*));
            crate::shims::call_sync(pin).to_raw()
        }
        }
    } else {
        quote! {
            #fetch_args
            #module::#name(machine, #(#args),*).to_raw()
        }
    };

    (
        quote!(pub unsafe fn #name(machine: &mut Machine, esp: u32) -> u32 { #body }),
        quote!(pub const #name: Shim = Shim {
            name: #name_str,
            func: impls::#name,
            stack_consumed: #stack_consumed,
            is_async: #is_async,
        };),
    )
}
