use super::parse;
use crate::parse::DllExport;
use proc_macro2::TokenStream;
use quote::quote;

/// Generate the handler function that calls a win32api function by taking arguments using from_x86.
///
/// The caller of winapi functions is responsible for pushing/popping the
/// return address, because some callers actually 'jmp' directly.
///
/// This macro generates handler wrappers of functions, taking their
/// input args off the stack and returning their return values that belong in eax.
pub fn fn_wrapper(module: TokenStream, dllexport: &DllExport) -> (TokenStream, TokenStream) {
    let base_name = &dllexport.func.sig.ident; // QueryInterface
    let name_str = match dllexport.vtable {
        Some(vtable) => format!("{}::{}", vtable, base_name), // "IDirectDraw::QueryInterface"
        None => format!("{}", base_name),                     // "LoadLibrary"
    };
    let impls_mod = match dllexport.vtable {
        Some(vtable) => quote!(#module::#vtable), // winapi::ddraw::IDirectDraw
        None => quote!(#module),                  // winapi::kernel32
    };
    let sym_name = &dllexport.sym_name; // IDirectDraw_QueryInterface

    let mut fetch_args = quote! {
        let mem = machine.mem().detach();
    };
    let mut stack_offset = 0u32;
    for parse::Argument { name, ty, stack } in dllexport.args.iter() {
        // We expect all the stack_offset math to be inlined by the compiler into plain constants.
        // TODO: reading the args in reverse would produce fewer bounds checks...
        fetch_args.extend(quote! {
            let #name = <#ty>::from_stack(mem, stack_args + #stack_offset);
        });
        stack_offset += stack.consumed();
    }

    let args = dllexport
        .args
        .iter()
        .map(|arg| arg.name)
        .collect::<Vec<_>>();

    {
        let trace_module_name = dllexport.trace_module;
        let trace_args = args
            .iter()
            .map(|arg| {
                let mut name = arg.to_string();
                if name.starts_with("_") {
                    name.remove(0);
                }
                quote!((#name, &#arg))
            })
            .collect::<Vec<_>>();
        fetch_args.extend(quote! {
            let __trace_context = if crate::trace::enabled(#trace_module_name) {
                Some(crate::trace::trace_begin(#trace_module_name, #name_str, &[#(#trace_args),*]))
            } else {
                None
            };
        });
    }

    let pos_name = syn::Ident::new(&format!("{}_pos", base_name), base_name.span());
    let return_result = quote! {
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(&__trace_context, #impls_mod::#pos_name.0, #impls_mod::#pos_name.1, &result);
        }
        result.to_raw()
    };

    let (func, defn) = if dllexport.func.sig.asyncness.is_some() {
        (
            quote!(Handler::Async(wrappers::#sym_name)),
            quote! {
                pub unsafe fn #sym_name(machine: &mut Machine, stack_args: u32) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
                    #fetch_args
                    let machine: *mut Machine = machine;
                    Box::pin(async move {
                        let machine = unsafe { &mut *machine };
                        let result = #impls_mod::#base_name(machine, #(#args),*).await;
                        #return_result
                    })
                }
            },
        )
    } else {
        (
            quote!(Handler::Sync(wrappers::#sym_name)),
            quote! {
                pub unsafe fn #sym_name(machine: &mut Machine, stack_args: u32) -> u32 {
                    #fetch_args
                    let result = #impls_mod::#base_name(machine, #(#args),*);
                    #return_result
                }
            },
        )
    };

    (
        defn,
        quote!(Shim {
            name: #name_str,
            func: #func,
        }),
    )
}
