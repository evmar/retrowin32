use quote::quote;

/// Insert a call to trace::trace() at the front of a function that logs its name and arguments.
// TODO: this maybe belongs at the "impls" mod, so it can do its own traversal of the stack (?).
pub fn add_trace(func: &mut syn::ItemFn) {
    let name = func.sig.ident.to_string();
    let mut args: Vec<&syn::Ident> = Vec::new();
    for arg in func.sig.inputs.iter().skip(1) {
        match arg {
            syn::FnArg::Typed(arg) => match &*arg.pat {
                syn::Pat::Ident(pat) => {
                    args.push(&pat.ident);
                }
                _ => {}
            },
            _ => {}
        };
    }
    let synargs = args
        .iter()
        .map(|arg| {
            let name = arg.to_string();
            quote!((#name, &#arg))
        })
        .collect::<Vec<_>>();
    let arg_count = args.len();
    let stmt: syn::Stmt = syn::parse_quote! {
        if crate::trace::enabled(TRACE_CONTEXT) {
            let args: &[(&str, &dyn std::fmt::Debug); #arg_count] = &[#(#synargs),*];
            crate::trace::trace(TRACE_CONTEXT, std::file!(), std::line!(), #name, args);
        }
    };
    func.block.stmts.insert(0, stmt);
}
