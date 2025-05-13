use minibuild::*;

fn build_dll(b: &mut B, dll: &str, is_split: bool) -> anyhow::Result<()> {
    let out_dir = if is_split {
        format!("win32/dll/{dll}")
    } else {
        format!("win32/dll")
    };
    let asm_path = format!("{out_dir}/{dll}.s");
    let def_path = format!("{out_dir}/{dll}.def");
    let dll_path = format!("{out_dir}/{dll}.dll");

    b.task("generate source", |b| {
        let mut ins = Vec::new();

        // win32-derive program sources
        // TODO: get this from cargo .d files?
        for f in glob("win32/derive/src/*.rs")? {
            ins.push(f?);
        }

        let src_dir = if is_split {
            format!("win32/dll/{dll}/src")
        } else {
            format!("win32/src/winapi/{dll}")
        };
        // processed source files
        for f in glob(&format!("{src_dir}/**/*.rs"))? {
            let f = f?;
            if f.file_name().unwrap() == "builtin.rs" {
                continue;
            }
            ins.push(f);
        }

        b.files(&ins, &[&asm_path, &def_path], |b| {
            b.cmd(&[
                "cargo",
                "run",
                "--quiet",
                "--release",
                "-p",
                "win32-derive",
                "--",
                "--out-dir",
                &out_dir,
                &src_dir,
            ])
        })
    })?;

    b.task("compile+link", |b| {
        b.files(&[&asm_path, &def_path], &[&dll_path], |b| {
            b.cmd(&[
                "clang-cl",
                "-fuse-ld=lld",
                "-target",
                "i586-pc-windows-msvc",
                &asm_path,
                "/link",
                "/dll",
                &format!("/def:{def_path}"),
                &format!("/out:{dll_path}"),
                "/Brepro",
                "/safeseh:no",
                "/noentry",
                "/nodefaultlib",
                "/subsystem:console",
                "win32/lib/retrowin32.lib",
            ])
        })
    })?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let dlls = [
        ("advapi32", true),
        ("bass", true),
        ("comctl32", true),
        ("ddraw", false),
        ("dinput", true),
        ("dsound", true),
        ("gdi32", true),
        ("kernel32", false),
        ("ntdll", false),
        ("ole32", true),
        ("oleaut32", true),
        ("retrowin32_test", true),
        ("shlwapi", true),
        ("ucrtbase", false),
        ("vcruntime140", true),
        ("version", true),
        ("user32", true),
        ("wininet", true),
        ("winmm", false),
    ];

    let mut b = B::default();
    b.task("dlls", |b| {
        for (dll, is_split) in dlls {
            b.task(format!("{dll}.dll"), |b| build_dll(b, dll, is_split))?;
        }
        Ok(())
    })?;

    Ok(())
}
