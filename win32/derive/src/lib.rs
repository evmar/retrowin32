use quote::quote;

#[proc_macro_attribute]
pub fn dllexport(
    _attr: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // dllexport triggers external code generation, and has no effect otherwise.

    let input = tokens.clone();
    let item: syn::Item = syn::parse_macro_input!(input);
    match item {
        syn::Item::Fn(func) => {
            let pos = syn::Ident::new(&format!("{}_pos", &func.sig.ident), func.sig.ident.span());

            // Previously the trace macro caused Rust to believe that all function args were used,
            // so to preserve that behavior after an unrelated change, we added the allow here.
            // TODO: remove these.
            proc_macro::TokenStream::from_iter(vec![
                quote! {
                    #[allow(non_upper_case_globals)]
                    pub const #pos: (&'static str, u32) = (file!(), line!()+1);
                    #[allow(unused_variables)]
                }
                .into(),
                tokens,
            ])
        }
        _ => tokens,
    }
}

#[proc_macro_derive(TryFromEnum)]
pub fn try_from_enum(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let enum_: syn::ItemEnum = syn::parse_macro_input!(item);

    let name = &enum_.ident;
    // If one of the values is negative, match using i32 instead of u32.
    let has_negative = enum_.variants.iter().any(|variant| {
        let num = &variant.discriminant.as_ref().unwrap().1;
        match num {
            syn::Expr::Unary(syn::ExprUnary {
                op: syn::UnOp::Neg(_),
                ..
            }) => true,
            _ => false,
        }
    });

    let matches = enum_.variants.iter().map(|variant| {
        let num = &variant.discriminant.as_ref().unwrap().1;
        let sym = &variant.ident;
        quote! {
            #num => #name::#sym,
        }
    });

    let value_match = if has_negative {
        quote!(value as i32)
    } else {
        quote!(value)
    };

    quote! {
        impl TryFrom<u32> for #name {
            type Error = u32;

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                Ok(match #value_match {
                    #(#matches)*
                    _ => return Err(value),
                })
            }
        }
    }
    .into()
}
