use proc_macro2::TokenStream;
use quote::quote;

pub struct DllExport {
    pub ordinal: Option<usize>,
}

/// Parse a #[attr] looking for #[win32_derive::dllexport].
fn parse_dllexport(attr: &syn::Attribute) -> anyhow::Result<Option<DllExport>> {
    let (path, nested) = match attr.parse_meta() {
        Ok(syn::Meta::Path(path)) => (path, None),
        Ok(syn::Meta::List(list)) => (list.path, Some(list.nested)),
        _ => return Ok(None), // ignore unexpected attrs
    };
    if path.leading_colon.is_some()
        || path.segments.len() != 2
        || path.segments[0].ident != "win32_derive"
    {
        return Ok(None);
    }
    let seg = &attr.path.segments[1];
    if seg.ident != "dllexport" {
        anyhow::bail!("bad win32_derive attribute")
    }

    let ordinal = match nested {
        None => None,
        Some(n) => {
            if n.len() != 1 {
                anyhow::bail!("bad dllexport");
            }
            match n.first().unwrap() {
                syn::NestedMeta::Lit(syn::Lit::Int(i)) => Some(i.base10_parse::<usize>()?),
                _ => anyhow::bail!("bad dllexport"),
            }
        }
    };
    Ok(Some(DllExport { ordinal }))
}

/// Gather all the dllexport fns in a list of syn::Items (module contents).
pub fn gather_shims(items: &[syn::Item]) -> anyhow::Result<Vec<(&syn::ItemFn, DllExport)>> {
    let mut fns = Vec::new();
    for item in items {
        match item {
            syn::Item::Fn(func) => {
                let mut dllexport = None;
                for attr in func.attrs.iter() {
                    dllexport = parse_dllexport(attr)?;
                    if dllexport.is_some() {
                        break;
                    }
                }
                if let Some(dllexport) = dllexport {
                    fns.push((func, dllexport));
                }
            }
            _ => {}
        }
    }
    Ok(fns)
}

/// Return the amount of stack a given stdcall argument uses.
/// (All of them except the array+size type are 4 bytes.)
fn stack_consumed(ty: &syn::Type) -> u32 {
    let ty = match ty {
        syn::Type::Path(ty) => ty,
        _ => panic!("unhandled type {ty:?}"),
    };
    if ty.path.segments.len() != 1 {
        panic!("unhandled type {ty:?}");
    }

    if ty.path.segments[0].ident == "ArrayWithSize"
        || ty.path.segments[0].ident == "ArrayWithSizeMut"
    {
        8
    } else {
        4
    }
}

/// Generate the wrapper function that calls a win32api function by taking arguments using from_x86.
///
/// winapi is stdcall, which means args are pushed right to left.
/// They are callee-cleaned which means they are popped left to right, as done here.
/// The caller of winapi functions is responsible for pushing/popping the
/// return address, because some callers actually 'jmp' directly.
///
/// This macro generates shim wrappers of functions, taking their
/// input args off the stack and forwarding their return values via eax.
pub fn fn_wrapper(module: TokenStream, func: &syn::ItemFn) -> (TokenStream, TokenStream) {
    let mut args = Vec::new();
    let mut tys = Vec::new();

    // Skip first arg, the &Machine.
    for arg in func.sig.inputs.iter().skip(1) {
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

    let name = &func.sig.ident;
    let name_str = name.to_string();

    let mut fetch_args = TokenStream::new();
    let mut stack_offset = 4u32; // return address
    for (arg, ty) in args.iter().zip(tys.iter()) {
        // We expect all the stack_offset math to be inlined by the compiler into plain constants.
        // TODO: reading the args in reverse would produce fewer bounds checks...
        fetch_args.extend(quote! {
            let #arg = <#ty>::from_stack(machine.mem(), esp + #stack_offset);
        });
        stack_offset += stack_consumed(ty);
    }
    let ret = quote! {
        machine.x86.cpu.regs.eax = result.to_raw();
    };

    let mut stack_consumed = quote!(Some(#stack_offset));

    // If the function is async, we need to handle the return value a bit differently.
    let body = if func.sig.asyncness.is_some() {
        stack_consumed = quote!(None);
        quote! {
            #fetch_args
            // Yuck: we know Machine will outlive the future, but Rust doesn't.
            // At least we managed to isolate the yuck to this point.
            let m: *mut Machine = machine;
            let result = async move {
                let machine = unsafe { &mut *m };
                let result = #module::#name(machine, #(#args),*).await;
                machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
                machine.x86.cpu.regs.esp += #stack_offset;
                #ret
            };
            crate::shims::become_async(machine, Box::pin(result));
            // push_async will set up the stack and eip.
        }
    } else {
        quote! {
            #fetch_args
            let result = #module::#name(machine, #(#args),*);
            #ret
        }
    };

    (
        quote!(pub unsafe fn #name(machine: &mut Machine, esp: u32) { #body }),
        quote!(pub const #name: Shim = Shim {name: #name_str, func: impls::#name, stack_consumed: #stack_consumed };),
    )
}

/// Insert a `log::info!` at the front of a function that logs its name and arguments.
// TODO: this fn is used by lib.rs, but not main.rs.
#[allow(dead_code)]
pub fn add_trace(func: &mut syn::ItemFn) {
    let mut fmt: String = "{}/".into(); // TRACE_CONTEXT prefix
    fmt.push_str(&func.sig.ident.to_string());
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
    fmt.push_str("(");
    fmt.push_str(
        &args
            .iter()
            .map(|arg| format!("{arg}:{{:x?}}"))
            .collect::<Vec<_>>()
            .join(", "),
    );
    fmt.push_str(")");
    let stmt = syn::parse_quote! {
        if crate::trace::enabled(TRACE_CONTEXT) { log::info!(#fmt, TRACE_CONTEXT, #(#args),*); }
    };
    func.block.stmts.insert(0, stmt);
}
