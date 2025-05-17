/*

#[win32_derive::dllexport]
pub fn GetDiskFreeSpaceA(
    sys: &dyn System,
    lpRootPathName: Option<&str>,
    lpSectorsPerCluster: Option<&mut u32>,
    lpBytesPerSector: Option<&mut u32>,
    lpNumberOfFreeClusters: Option<&mut u32>,
    lpTotalNumberOfClusters: Option<&mut u32>,
) -> i32 {
    todo!();
}
*/

fn convert_typename(
    name: &windows_metadata::TypeName,
    ext: Option<windows_metadata::TypeName>,
) -> String {
    if *name == windows_metadata::TypeName::PSTR {
        "Option<&str>".into()
    } else if *name == windows_metadata::TypeName::PWSTR {
        "Option<&Str16>".into()
    } else if *name == windows_metadata::TypeName::BOOL {
        "bool".into()
    } else {
        if let Some(ext) = ext {
            if ext == windows_metadata::TypeName::Struct {
                format!("{}", name.name())
            } else if ext == windows_metadata::TypeName::Enum {
                format!("u32 /* {} */", name.name())
            } else if ext == windows_metadata::TypeName::Delegate {
                // e.g. WNDPROC
                format!("u32 /* {} */", name.name())
            } else {
                todo!("{} ({})", name.name(), ext.name())
            }
        } else {
            format!("{}", name.name())
        }
    }
}

fn convert_type(ty: &windows_metadata::Type) -> String {
    use windows_metadata::Type::*;
    match ty {
        Void => format!("()"), // found in e.g void* params
        Bool => todo!(),
        Char => todo!(),
        I8 => format!("i8"),
        U8 => format!("u8"),
        I16 => todo!(),
        U16 => format!("u16"),
        I32 => format!("i32"),
        U32 => format!("u32"),
        I64 => todo!(),
        U64 => todo!(),
        F32 => todo!(),
        F64 => todo!(),
        ISize => todo!(),
        USize => format!("usize"),
        String => todo!(),
        Object => todo!(),
        Name(name) => convert_typename(name, None),
        Const(name) => convert_typename(name, None),
        GenericParam(_generic_param) => todo!(),
        TypeDef(type_def, vec) => {
            assert!(vec.is_empty()); // ?
            convert_typename(&type_def.type_name(), type_def.extends())
        }
        MutPtr(ty, x) => {
            if *x != 1 {
                // it appears this is for like lplpFoo types
            }
            format!("Option<&mut {}>", convert_type(ty))
        }
        ConstPtr(_, _) => todo!(),
        Win32Array(ty, len) => format!("[{}; {}]", convert_type(ty), len),
        WinrtArray(_) => todo!(),
        WinrtArrayRef(_) => todo!(),
        ConstRef(_) => todo!(),
        PrimitiveOrEnum(_, _) => todo!(),
    }
}

fn convert_method(method: &windows_metadata::MethodDef) {
    println!("// module: {}", method.module_name());

    let sig = method.signature(&[]);
    //println!("{:?}", sig);

    println!("#[win32_derive::dllexport]");
    println!("pub fn {name}(", name = method.name());
    println!("    sys: &dyn System,");
    if sig.call_flags.0 != 0 {
        todo!("call_flags {:#x}", sig.call_flags.0);
    }

    // First param is possibly this pointer(?)
    let mut params = method.params().collect::<Vec<_>>();
    let types = &sig.params;

    if params.len() == types.len() + 1 && params[0].name() == "" {
        // ??
        params.remove(0);
    }

    if params.len() != types.len() {
        todo!(
            "params.len() {} != types.len() {}",
            params.len(),
            types.len()
        );
    }

    for (param, ty) in params.iter().zip(types) {
        println!(
            "    {name}: {ty},",
            name = param.name(),
            ty = convert_type(ty)
        );
    }

    match sig.return_type {
        windows_metadata::Type::Void => println!(") {{ todo!() }}"),
        ty => println!(") -> {} {{ todo!() }}", convert_type(&ty)),
    }
}

fn convert_typedef(type_def: &windows_metadata::TypeDef) {
    println!("struct {name} {{", name = type_def.name());
    for f in type_def.fields() {
        println!(
            "    {name}: {ty},",
            name = f.name(),
            ty = convert_type(&f.ty(None))
        );
    }
    println!("}}");
}

fn main() {
    let bytes = std::fs::read(r#"Windows.Win32.winmd"#).expect("File not found");

    let file = windows_metadata::File::new(bytes).expect("Invalid metadata");
    let reader = windows_metadata::Reader::new(vec![file]);

    for search in std::env::args().skip(1) {
        for item in reader.items() {
            match item {
                windows_metadata::Item::Type(type_def) => {
                    if type_def.name() != search {
                        continue;
                    }
                    convert_typedef(&type_def);
                }
                windows_metadata::Item::Const(_field) => {
                    //println!("f {n}", n = field.name());
                }
                windows_metadata::Item::Fn(method_def, _) => {
                    if method_def.name() != search {
                        continue;
                    }
                    convert_method(&method_def);
                }
            }
        }
    }
}
