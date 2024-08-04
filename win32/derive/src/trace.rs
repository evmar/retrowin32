use quote::{format_ident, quote};

/// Insert a call to trace::trace() at the front of a function that logs its name and arguments.
// TODO: this maybe belongs at the "impls" mod, so it can do its own traversal of the stack (?).
pub fn add_trace(mut func: syn::ItemFn) -> proc_macro2::TokenStream {
    let name = &func.sig.ident;
    let name_string = name.to_string();
    let mut arg_names: Vec<&syn::Ident> = Vec::new();
    for arg in &func.sig.inputs {
        match arg {
            syn::FnArg::Typed(arg) => match &*arg.pat {
                syn::Pat::Ident(pat) => {
                    arg_names.push(&pat.ident);
                }
                _ => {}
            },
            _ => {}
        };
    }
    let trace_arg_values = arg_names
        .iter()
        .skip(1)
        .map(|arg| {
            let name = arg.to_string();
            quote!((#name, &#arg))
        })
        .collect::<Vec<_>>();
    let trace_arg_count = trace_arg_values.len();
    let prolog = quote! {
        let __trace_context = if crate::trace::enabled(const_format::concatcp!(TRACE_CONTEXT, "/", #name_string)) {
            let args: &[(&str, &dyn std::fmt::Debug); #trace_arg_count] = &[#(#trace_arg_values),*];
            Some(crate::trace::trace_begin(TRACE_CONTEXT, #name_string, args))
        } else {
            None
        };
    };
    let epilog = quote! {
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(&__trace_context, std::file!(), std::line!(), &__ret);
        }
    };

    let trace_vis = func.vis.clone();
    let mut trace_sig = func.sig.clone();
    // Remove mutability and references from arguments.
    for arg in &mut trace_sig.inputs {
        match arg {
            syn::FnArg::Typed(arg) => match &mut *arg.pat {
                syn::Pat::Ident(pat) => {
                    pat.by_ref = None;
                    pat.mutability = None;
                }
                _ => {}
            },
            _ => {}
        };
    }

    let impl_name = format_ident!("{}_impl", name);
    func.sig.ident = impl_name.clone();
    let await_t = if func.sig.asyncness.is_some() {
        quote!(.await)
    } else {
        quote!()
    };
    // Previously the trace macro caused Rust to believe that all function args were used,
    // so to preserve that behavior after an unrelated change, we added the allow here.
    // TODO: remove these.
    quote! {
        #[allow(unused_variables)]
        #trace_vis #trace_sig {
            #func
            #prolog
            let __ret = #impl_name(#(#arg_names),*)#await_t;
            #epilog
            __ret
        }
    }
}
