//! Parsing of dllexport attributes and functions that use them.

use syn::parse::Parser;

#[derive(Clone, Copy)]
pub enum CallConv {
    Stdcall,
    Cdecl,
}

pub struct DllExportMeta {
    pub ordinal: Option<usize>,
    pub callconv: CallConv,
}

/// A dllexport function.
pub struct DllExport<'a> {
    /// Module name as used in tracing, e.g. "kernel32/file".
    pub trace_module: &'a str,
    pub meta: DllExportMeta,
    pub args: Vec<Argument<'a>>,
    /// If this function is part of a vtable, this is the name of the vtable.
    pub vtable: Option<&'a syn::Ident>,
    // E.g. IDirectDraw_QueryInterface for symbols within a module.
    pub sym_name: syn::Ident,
    pub func: &'a syn::ItemFn,
}

impl<'a> DllExport<'a> {
    pub fn stack_consumed(&self) -> u32 {
        match self.meta.callconv {
            CallConv::Stdcall => self.args.iter().map(|arg| arg.stack_consumed).sum(),
            CallConv::Cdecl => 0, // caller cleaned
        }
    }
}

pub struct DllExportData<'a> {
    pub name: &'a syn::Ident,
}

pub struct Vtable {
    pub name: syn::Ident,
    pub fns: Vec<(syn::Ident, Option<String>)>,
}

#[derive(Default)]
pub struct DllExports<'a> {
    pub fns: Vec<DllExport<'a>>,
    pub data: Vec<DllExportData<'a>>,
    pub vtables: Vec<Vtable>,
}

/// Parse a #[attr] looking for #[win32_derive::dllexport].
fn parse_dllexport(attr: &syn::Attribute) -> syn::Result<Option<DllExportMeta>> {
    let path = attr.path();
    if path.segments.len() != 2 || path.segments[0].ident != "win32_derive" {
        return Ok(None);
    }
    let seg = &path.segments[1];
    if seg.ident != "dllexport" {
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
    pub stack_consumed: u32,
}

/// Parse a function argument type, returning how much stack it uses.
/// (We cannot rely on some const fn on the type because we need this info at compile time
/// when generating the wrapper function that pops the stack.)
pub fn parse_argument_stack(ty: &syn::Type) -> u32 {
    let ty = match ty {
        syn::Type::Path(ty) => ty,
        syn::Type::Reference(_) => return 4,
        _ => panic!("unhandled type {ty:?}"),
    };
    if ty.path.segments.len() != 1 {
        panic!("unhandled type {ty:?}");
    }
    let seg = &ty.path.segments[0];

    let name = &seg.ident;
    // cannot use "match" here because we rely on == impl on Ident :(
    if name == "Option" {
        let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
            args, ..
        }) = &seg.arguments
        else {
            panic!("unhandled type {ty:?}");
        };
        let syn::GenericArgument::Type(ty) = &args[0] else {
            panic!("unhandled type {ty:?}");
        };
        parse_argument_stack(&ty)
    } else if name == "Array" || name == "ArrayOut" || name == "POINT" || name == "f64" {
        8
    } else if name == "VarArgs" {
        0
    } else {
        4
    }
}

fn parse_fn<'a>(
    trace_module: &'a str,
    func: &'a syn::ItemFn,
) -> syn::Result<Option<DllExport<'a>>> {
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
        let stack_consumed = parse_argument_stack(ty);
        if stack_consumed == 0 {
            if !matches!(meta.callconv, CallConv::Cdecl) {
                return Err(syn::Error::new_spanned(
                    func,
                    "VarArgs only works for cdecl functions",
                ));
            }
        }
        args.push(Argument {
            name,
            ty,
            stack_consumed,
        });
    }

    Ok(Some(DllExport {
        trace_module,
        meta,
        args,
        vtable: None,
        sym_name: func.sig.ident.clone(),
        func,
    }))
}

fn parse_mod<'a>(
    trace_module: &'a str,
    item: &'a syn::ItemMod,
) -> syn::Result<Option<DllExports<'a>>> {
    if find_dllexport(&item.attrs)?.is_none() {
        return Ok(None);
    }

    let name = &item.ident;
    let mut dllexports = DllExports::default();
    let body = &item.content.as_ref().unwrap().1;
    gather_dllexports(trace_module, body, &mut dllexports)?;
    for dllexport in &mut dllexports.fns {
        dllexport.vtable = Some(name);
        dllexport.sym_name = quote::format_ident!("{}_{}", name, dllexport.sym_name);
    }

    // Look for a call to the vtable! macro.
    for item in body {
        match item {
            syn::Item::Macro(item) => {
                if let Some(vtable) = parse_vtable(name, item)? {
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
fn parse_vtable(name: &syn::Ident, item: &syn::ItemMacro) -> syn::Result<Option<Vtable>> {
    let mac = &item.mac;
    if !mac.path.is_ident("vtable") {
        return Ok(None);
    }

    let parser = syn::punctuated::Punctuated::<syn::FieldValue, syn::Token![,]>::parse_terminated;
    let fields = parser.parse2(mac.tokens.clone())?;

    let mut fns = Vec::new();
    for field in fields {
        let field_name = match field.member {
            syn::Member::Named(name) => name,
            syn::Member::Unnamed(_) => todo!(),
        };
        let imp = match field.expr {
            syn::Expr::Path(expr) => {
                if expr.path.is_ident("ok") {
                    Some(format!("{name}_{field_name}"))
                } else if expr.path.is_ident("todo") {
                    None
                } else {
                    return Err(syn::Error::new_spanned(expr, "bad vtable value"));
                }
            }
            syn::Expr::Paren(expr) => match &*expr.expr {
                syn::Expr::Path(expr) => {
                    let parts = expr
                        .path
                        .segments
                        .iter()
                        .map(|s| s.ident.to_string())
                        .collect::<Vec<_>>();
                    let name = parts.join("_");
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
        fns.push((field_name, imp));
    }

    Ok(Some(Vtable {
        name: name.clone(),
        fns,
    }))
}

fn parse_const(item: &syn::ItemConst) -> syn::Result<Option<DllExportData>> {
    if find_dllexport(&item.attrs)?.is_none() {
        return Ok(None);
    }

    Ok(Some(DllExportData { name: &item.ident }))
}

/// Gather all the dllexports in a list of syn::Items (module contents).
pub fn gather_dllexports<'a>(
    trace_module: &'a str,
    items: &'a [syn::Item],
    out: &mut DllExports<'a>,
) -> syn::Result<()> {
    for item in items {
        match item {
            syn::Item::Fn(func) => {
                if let Some(func) = parse_fn(trace_module, func)? {
                    out.fns.push(func);
                }
            }
            syn::Item::Mod(item) => {
                if let Some(exports) = parse_mod(trace_module, item)? {
                    out.fns.extend(exports.fns);
                    out.vtables.extend(exports.vtables);
                }
            }
            syn::Item::Const(item) => {
                if let Some(item) = parse_const(item)? {
                    out.data.push(item);
                }
            }
            _ => continue,
        }
    }
    Ok(())
}
