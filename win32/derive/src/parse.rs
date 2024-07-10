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

#[derive(Default)]
pub struct DllExports<'a> {
    pub fns: Vec<DllExport<'a>>,
    pub vtables: Vec<()>,
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

fn parse_args(func: &syn::ItemFn) -> Vec<Argument> {
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
    args
}

/// Gather all the dllexports in a list of syn::Items (module contents).
pub fn gather_dllexports<'a>(items: &'a [syn::Item], out: &mut DllExports<'a>) -> syn::Result<()> {
    for item in items {
        let func = match item {
            syn::Item::Fn(func) => func,
            _ => continue,
        };
        for attr in func.attrs.iter() {
            if let Some(meta) = parse_dllexport(attr)? {
                let args = parse_args(func);
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
                out.fns.push(DllExport { meta, args, func });
                break;
            }
        }
    }
    Ok(())
}
