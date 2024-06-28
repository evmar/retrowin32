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
    pub func: &'a syn::ItemFn,
}

/// Parse a #[attr] looking for #[win32_derive::dllexport].
fn parse_dllexport(attr: &syn::Attribute) -> anyhow::Result<Option<DllExportMeta>> {
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

    let mut ordinal = None;
    let mut callconv = CallConv::Stdcall;
    if let Some(nested) = nested {
        for n in nested.iter() {
            match n {
                syn::NestedMeta::Lit(syn::Lit::Int(i)) => {
                    ordinal = Some(i.base10_parse::<usize>()?);
                }
                syn::NestedMeta::Meta(meta) => match meta {
                    syn::Meta::Path(path) => {
                        if path.is_ident("cdecl") {
                            callconv = CallConv::Cdecl;
                        } else {
                            anyhow::bail!("bad path {path:?}");
                        }
                    }
                    _ => anyhow::bail!("bad meta {meta:?}"),
                },
                n => anyhow::bail!("bad dllexport {n:?}"),
            }
        }
    };
    Ok(Some(DllExportMeta { ordinal, callconv }))
}

/// Gather all the dllexport fns in a list of syn::Items (module contents).
pub fn gather_dllexports<'a>(
    items: &'a [syn::Item],
    out: &mut Vec<DllExport<'a>>,
) -> anyhow::Result<()> {
    for item in items {
        match item {
            syn::Item::Fn(func) => {
                let mut meta = None;
                for attr in func.attrs.iter() {
                    meta = parse_dllexport(attr)?;
                    if meta.is_some() {
                        break;
                    }
                }
                if let Some(meta) = meta {
                    out.push(DllExport { func, meta });
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub enum Argument {
    /// Value is amount of stack the argument uses in stdcall.
    /// (All of them except the array+size type are 4 bytes.)
    Ordinary(u32),
    VarArgs,
}

/// Parse a function argument type into the metadata we care about.
pub fn parse_argument_type(ty: &syn::Type) -> Argument {
    let ty = match ty {
        syn::Type::Path(ty) => ty,
        _ => panic!("unhandled type {ty:?}"),
    };
    if ty.path.segments.len() != 1 {
        panic!("unhandled type {ty:?}");
    }

    let name = &ty.path.segments[0].ident;
    if name == "ArrayWithSize" || name == "ArrayWithSizeMut" || name == "POINT" {
        Argument::Ordinary(8)
    } else if name == "VarArgs" {
        Argument::VarArgs
    } else {
        Argument::Ordinary(4)
    }
}
