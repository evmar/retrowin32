mod gen;
mod parse;
mod trace;

use proc_macro2::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn dllexport(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let func: syn::ItemFn = syn::parse_macro_input!(item);
    trace::add_trace(func).into()
}

/// Generate a `shims` module that contains a wrapper for each function in this module
/// that transports arguments/return via the Machine's x86 stack.
#[proc_macro_attribute]
pub fn shims_from_x86(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut module: syn::ItemMod = syn::parse_macro_input!(item);

    // Generate one wrapper function per function found in the input module.
    let items = &module.content.as_ref().unwrap().1;
    let mut dllexports = parse::DllExports::default();
    parse::gather_dllexports(items, &mut dllexports).unwrap();

    let mut impls: Vec<TokenStream> = Vec::new();
    let mut shims: Vec<TokenStream> = Vec::new();
    for dllexport in &dllexports.fns {
        if dllexport.vtable.is_some() {
            continue;
        }
        let (wrapper, shim) = gen::fn_wrapper(quote!(super), dllexport);
        impls.push(wrapper);
        shims.push(shim);
    }

    let impls_module = syn::parse2::<syn::ItemMod>(quote! {
        mod impls {
            use crate::{machine::Machine, winapi::{self, stack_args::*, types::*}};
            use super::*;
            #(#impls)*
        }
    })
    .unwrap();

    let shims_module: syn::ItemMod = syn::parse2::<syn::ItemMod>(quote! {
        pub mod shims {
            use crate::shims::Shim;
            use super::impls;
            #(#shims)*
        }
    })
    .unwrap();

    // Add modules into the outer module.
    module
        .content
        .as_mut()
        .unwrap()
        .1
        .extend([syn::Item::Mod(shims_module), syn::Item::Mod(impls_module)]);
    let out = quote! {
        #module
    };
    // eprintln!("out {}", out);
    out.into()
}

#[proc_macro_derive(TryFromEnum)]
pub fn try_from_enum(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let enum_: syn::ItemEnum = syn::parse_macro_input!(item);

    let name = &enum_.ident;
    let matches = enum_.variants.iter().map(|variant| {
        let num = &variant.discriminant.as_ref().unwrap().1;
        let sym = &variant.ident;
        quote! {
            #num => #name::#sym,
        }
    });
    quote! {
        impl TryFrom<u32> for #name {
            type Error = u32;

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                Ok(match value {
                    #(#matches)*
                    _ => return Err(value),
                })
            }
        }
    }
    .into()
}
