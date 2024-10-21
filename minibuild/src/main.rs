use minibuild::*;

fn build_dll(b: &mut B, dll: &str) -> anyhow::Result<()> {
    let asm_path = format!("win32/dll/{}.s", dll);
    let def_path = format!("win32/dll/{}.def", dll);
    let dll_path = format!("win32/dll/{}.dll", dll);

    b.task("generate source", |b| {
        let mut ins = Vec::new();

        // win32-derive program sources
        // TODO: get this from cargo .d files?
        for f in glob("win32/derive/src/*.rs")? {
            ins.push(f?);
        }
        // processed source files
        for f in glob(&format!("win32/src/winapi/{dll}/**/*.rs"))? {
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
                "--dll-dir",
                "win32/dll",
                &format!("win32/src/winapi/{}", dll),
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
        "advapi32",
        "bass",
        "ddraw",
        "dinput",
        "dsound",
        "gdi32",
        "kernel32",
        "ntdll",
        "ole32",
        "oleaut32",
        "retrowin32_test",
        "shlwapi",
        "ucrtbase",
        "vcruntime140",
        "version",
        "user32",
        "wininet",
        "winmm",
    ];

    let mut b = B::default();
    b.task("dlls", |b| {
        for dll in dlls {
            b.task(format!("{dll}.dll"), |b| build_dll(b, dll))?;
        }
        Ok(())
    })?;

    Ok(())
}
