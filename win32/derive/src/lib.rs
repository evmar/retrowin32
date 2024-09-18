mod trace;

use quote::{quote, ToTokens};

#[proc_macro_attribute]
pub fn dllexport(
    _attr: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // dllexport only triggers tracing for functions.
    // The rest of the dllexport logic happens in a separate a code generator found in main.rs.
    let item: syn::Item = syn::parse_macro_input!(tokens);
    match item {
        syn::Item::Fn(func) => trace::add_trace(func).into(),
        syn::Item::Mod(_) => item.into_token_stream().into(), // ignore
        _ => syn::Error::new_spanned(&item, "expected fn or mod")
            .to_compile_error()
            .into(),
    }
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
