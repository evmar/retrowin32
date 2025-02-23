//! Generates the 'builtins.rs' module with metadata/wrappers for builtin DLLs.

use crate::parse;
use proc_macro2::TokenStream;
use quote::quote;

/// Generate the handler function that calls a win32api function by taking arguments using from_x86.
///
/// The caller of winapi functions is responsible for pushing/popping the
/// return address, because some callers actually 'jmp' directly.
///
/// This macro generates handler wrappers of functions, taking their
/// input args off the stack and returning their return values that belong in eax.
fn fn_wrapper(module: TokenStream, dllexport: &parse::DllExport) -> (TokenStream, TokenStream) {
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

    let pos_name = syn::Ident::new(&format!("{}_pos", base_name), base_name.span());

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
            let __trace_record = if crate::winapi::trace::enabled(#trace_module_name) {
                crate::winapi::trace::Record::new(#impls_mod::#pos_name, #trace_module_name, #name_str, &[#(#trace_args),*]).enter()
            } else {
                None
            };
        });
    }

    let return_result = quote! {
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    };

    let (func, defn) = if dllexport.func.sig.asyncness.is_some() {
        (
            quote!(Handler::Async(wrappers::#sym_name)),
            quote! {
                pub unsafe fn #sym_name(machine: &mut Machine, stack_args: u32) -> std::pin::Pin<Box<dyn std::future::Future<Output = u64>>> {
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
                pub unsafe fn #sym_name(machine: &mut Machine, stack_args: u32) -> u64 {
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

/// Generate one module (e.g. kernel32) of shim functions.
pub fn shims_module(module_name: &str, dllexports: parse::DllExports) -> TokenStream {
    let module = quote::format_ident!("{}", module_name);
    let dll_name = format!("{}.dll", module_name);

    let mut wrappers = Vec::new();
    let mut shims = Vec::new();
    for dllexport in &dllexports.fns {
        let (wrapper, shim) = fn_wrapper(quote!(winapi::#module), dllexport);
        wrappers.push(wrapper);
        shims.push(shim);
    }

    let shims_count = shims.len();
    let raw_dll_path = format!("../../../dll/{}", dll_name);
    quote! {
        //! Generated code, do not edit.  See winapi/builtin.rs for an overview.

        #![allow(unused_imports)]
        #![allow(unused_variables)]

        use crate::{shims::{Shim, Handler}, winapi::builtin::BuiltinDLL};

        mod wrappers {
            use ::memory::Extensions;
            use crate::{machine::Machine, winapi::{self, *, calling_convention::*}};
            use winapi::#module::*;  // for types
            #(#wrappers)*
        }

        const SHIMS: [Shim; #shims_count] = [
            #(#shims),*
        ];

        pub const DLL: BuiltinDLL = BuiltinDLL {
            file_name: #dll_name,
            shims: &SHIMS,
            raw: std::include_bytes!(#raw_dll_path),
        };
    }
}
