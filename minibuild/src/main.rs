use minibuild::*;

fn build_dll(b: &B, dll: &str) -> anyhow::Result<()> {
    let dll_dir = format!("win32/dll/{dll}");
    let asm_path = format!("{dll_dir}/{dll}.s");
    let def_path = format!("{dll_dir}/{dll}.def");
    let dll_path = format!("{dll_dir}/{dll}.dll");

    b.task("generate source", |b| {
        let mut ins = Vec::new();

        // win32-derive program sources
        // TODO: get this from cargo .d files?
        for f in glob("win32/derive/src/*.rs")? {
            ins.push(f?);
        }

        // processed source files
        for f in glob(&format!("{dll_dir}/src/**/*.rs"))? {
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
                &dll_dir,
            ])
        })
    })?;

    b.task("compile+link", |b| {
        b.files(&[&asm_path, &def_path], &[&dll_path], |b| {
            b.cmd(&[
                "clang-cl",
                "-fuse-ld=lld",
                "-target",
                "i686-pc-windows-msvc",
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
        "comctl32",
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

    let b = B::default();
    b.task("dlls", |b| {
        for dll in dlls {
            b.task(format!("{dll}.dll"), |b| build_dll(b, dll))?;
        }
        Ok(())
    })?;

    Ok(())
}
