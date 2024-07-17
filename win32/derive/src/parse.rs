//! Parsing of dllexport attributes and functions that use them.

#[derive(Clone, Copy)]
pub enum CallConv {
    Stdcall,
    Cdecl,
}

pub struct DllExportMeta {
    pub ordinal: Option<usize>,
    pub callconv: CallConv,
}

pub struct DllExport<'a> {
    pub meta: DllExportMeta,
    pub args: Vec<Argument<'a>>,
    /// If this function is part of a vtable, this is the name of the vtable.
    pub vtable: Option<&'a syn::Ident>,
    pub func: &'a syn::ItemFn,
}

impl<'a> DllExport<'a> {
    pub fn stack_consumed(&self) -> u32 {
        match self.meta.callconv {
            CallConv::Stdcall => self
                .args
                .iter()
                .map(|arg| arg.stack.consumed())
                .sum::<u32>(),
            CallConv::Cdecl => 0, // caller cleaned
        }
    }
}

pub struct Vtable {
    pub name: syn::Ident,
    pub fns: Vec<(syn::Ident, Option<String>)>,
}

#[derive(Default)]
pub struct DllExports<'a> {
    pub fns: Vec<DllExport<'a>>,
    pub vtables: Vec<Vtable>,
}

/// Parse a #[attr] looking for #[win32_derive::dllexport].
fn parse_dllexport(attr: &syn::Attribute) -> syn::Result<Option<DllExportMeta>> {
    let path = attr.path();
    if path.segments.len() != 2 || path.segments[0].ident != "win32_derive" {
        return Ok(None);
    }
    let seg = &path.segments[1];
    // TODO: remove shims_from_x86
    if seg.ident != "dllexport" && seg.ident != "shims_from_x86" {
        return Ok(None);
    }

    let mut ordinal = None;
    let mut callconv = CallConv::Stdcall;

    if matches!(attr.meta, syn::Meta::List(_)) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("ordinal") {
                let value: syn::LitInt = meta.value()?.parse()?;
                ordinal = Some(value.base10_parse::<usize>()?);
                Ok(())
            } else if meta.path.is_ident("cdecl") {
                callconv = CallConv::Cdecl;
                Ok(())
            } else {
                Err(meta.error("bad path {path:?}"))
            }
        })?;
    }

    Ok(Some(DllExportMeta { ordinal, callconv }))
}

fn find_dllexport(attrs: &[syn::Attribute]) -> syn::Result<Option<DllExportMeta>> {
    for attr in attrs {
        if let Some(meta) = parse_dllexport(attr)? {
            return Ok(Some(meta));
        }
    }
    Ok(None)
}

pub struct Argument<'a> {
    pub name: &'a syn::Ident,
    pub ty: &'a syn::Type,
    pub stack: ArgumentStack,
}

pub enum ArgumentStack {
    /// Value is amount of stack the argument uses in stdcall.
    Ordinary(u32),
    VarArgs,
}

impl ArgumentStack {
    pub fn consumed(&self) -> u32 {
        match self {
            ArgumentStack::Ordinary(ofs) => *ofs,
            ArgumentStack::VarArgs => 0,
        }
    }
}

/// Parse a function argument type into the metadata we care about.
pub fn parse_argument_stack(ty: &syn::Type) -> ArgumentStack {
    let ty = match ty {
        syn::Type::Path(ty) => ty,
        _ => panic!("unhandled type {ty:?}"),
    };
    if ty.path.segments.len() != 1 {
        panic!("unhandled type {ty:?}");
    }

    let name = &ty.path.segments[0].ident;
    if name == "ArrayWithSize" || name == "ArrayWithSizeMut" || name == "POINT" {
        ArgumentStack::Ordinary(8)
    } else if name == "VarArgs" {
        ArgumentStack::VarArgs
    } else {
        ArgumentStack::Ordinary(4)
    }
}

fn parse_fn(func: &syn::ItemFn) -> syn::Result<Option<DllExport>> {
    let meta = match find_dllexport(&func.attrs)? {
        Some(meta) => meta,
        None => return Ok(None),
    };

    let mut args = Vec::new();

    // Skip first arg, the &Machine.
    let iter = func.sig.inputs.iter().skip(1);
    for arg in iter {
        let arg = match arg {
            syn::FnArg::Typed(arg) => arg,
            _ => unimplemented!(),
        };
        let name = match arg.pat.as_ref() {
            syn::Pat::Ident(ident) => &ident.ident,
            _ => unimplemented!(),
        };
        let ty = &*arg.ty;
        let stack = parse_argument_stack(ty);
        args.push(Argument { name, ty, stack });
    }

    if args
        .iter()
        .any(|arg| matches!(arg.stack, ArgumentStack::VarArgs))
    {
        if !matches!(meta.callconv, CallConv::Cdecl) {
            return Err(syn::Error::new_spanned(
                func,
                "VarArgs only works for cdecl functions",
            ));
        }
    }

    Ok(Some(DllExport {
        meta,
        args,
        vtable: None,
        func,
    }))
}

fn parse_mod(item: &syn::ItemMod) -> syn::Result<Option<DllExports>> {
    if find_dllexport(&item.attrs)?.is_none() {
        return Ok(None);
    }

    let name = &item.ident;
    let mut dllexports = DllExports::default();
    let body = &item.content.as_ref().unwrap().1;
    gather_dllexports(body, &mut dllexports)?;
    for dllexport in &mut dllexports.fns {
        dllexport.vtable = Some(name);
    }

    // Look for a call to the vtable! macro.
    for item in body {
        match item {
            syn::Item::Macro(item) => {
                if let Some(vtable) = parse_vtable(item)? {
                    dllexports.vtables.push(vtable);
                    break;
                }
            }
            _ => {}
        }
    }

    Ok(Some(dllexports))
}

/// Parse a call to the vtable! macro.
fn parse_vtable(item: &syn::ItemMacro) -> syn::Result<Option<Vtable>> {
    let mac = &item.mac;
    if !mac.path.is_ident("vtable") {
        return Ok(None);
    }

    struct VtableMacro {
        name: syn::Ident,
        fields: syn::punctuated::Punctuated<syn::FieldValue, syn::Token![,]>,
    }
    impl syn::parse::Parse for VtableMacro {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            /*
               vtable![IDirectDrawClipper shims
               QueryInterface: todo,
               AddRef: todo,
            */
            let name = input.parse::<syn::Ident>()?;
            input.parse::<syn::Ident>()?; // "shims"
            let fields = input.parse_terminated(syn::FieldValue::parse, syn::Token![,])?;
            Ok(VtableMacro { name, fields })
        }
    }

    let vt = syn::parse2::<VtableMacro>(mac.tokens.clone())?;

    let mut fns = Vec::new();
    for field in vt.fields {
        let name = match field.member {
            syn::Member::Named(name) => name,
            syn::Member::Unnamed(_) => todo!(),
        };
        let imp = match field.expr {
            syn::Expr::Path(expr) => {
                if expr.path.is_ident("ok") {
                    Some(format!("{}_{}", vt.name, name))
                } else if expr.path.is_ident("todo") {
                    None
                } else {
                    return Err(syn::Error::new_spanned(expr, "bad vtable value"));
                }
            }
            syn::Expr::Paren(expr) => match &*expr.expr {
                syn::Expr::Path(expr) => {
                    // Gross: reference is like (IDirectDrawClipper::shims::QueryInterface), need
                    // to extract the the part around the shims bit.
                    let parts = expr
                        .path
                        .segments
                        .iter()
                        .map(|s| s.ident.to_string())
                        .collect::<Vec<_>>();
                    let name = match parts.len() {
                        4 => [parts[1].as_str(), parts[3].as_str()].join("_"),
                        3 => [parts[0].as_str(), parts[2].as_str()].join("_"),
                        _ => unimplemented!("{}", parts.len()),
                    };
                    Some(name)
                }
                expr => {
                    return Err(syn::Error::new_spanned(expr, "bad vtable value"));
                }
            },
            e => {
                return Err(syn::Error::new_spanned(e, "bad input"));
            }
        };
        fns.push((name, imp));
    }

    Ok(Some(Vtable { name: vt.name, fns }))
}

/// Gather all the dllexports in a list of syn::Items (module contents).
pub fn gather_dllexports<'a>(items: &'a [syn::Item], out: &mut DllExports<'a>) -> syn::Result<()> {
    for item in items {
        match item {
            syn::Item::Fn(func) => {
                if let Some(func) = parse_fn(func)? {
                    out.fns.push(func);
                }
            }
            syn::Item::Mod(item) => {
                if let Some(exports) = parse_mod(item)? {
                    out.fns.extend(exports.fns);
                    out.vtables.extend(exports.vtables);
                }
            }
            _ => continue,
        }
    }
    Ok(())
}
