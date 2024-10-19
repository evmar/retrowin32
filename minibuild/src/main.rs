use minibuild::B;
use std::path::PathBuf;

fn build_dll(b: &mut B, dll: &str) -> anyhow::Result<()> {
    let asm_path = PathBuf::from(format!("dll/{}.s", dll));
    let def_path = PathBuf::from(format!("dll/{}.def", dll));
    let dll_path = PathBuf::from(format!("dll/{}.dll", dll));

    b.task("generate source", |b| {
        b.files(&[], &[&asm_path, &def_path], |b| {
            b.cmd(&[
                "cargo",
                "run",
                "--quiet",
                "--release",
                "-p",
                "win32-derive",
                "--",
                "--dll-dir",
                "dll",
                &format!("src/winapi/{}", dll),
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
                &format!("dll/{dll}.s"),
                "/link",
                "/dll",
                &format!("/def:dll/{dll}.def"),
                &format!("/out:dll/{dll}.dll"),
                "/Brepro",
                "/safeseh:no",
                "/noentry",
                "/nodefaultlib",
                "/subsystem:console",
                "../lib/retrowin32.lib",
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
        "ucrtbase",
        "vcruntime140",
        "version",
        "user32",
        "wininet",
        "winmm",
    ];

    let mut b = B::default();
    b.task("dlls", |b| {
        b.chdir("win32")?;
        for dll in dlls {
            b.task(format!("{dll}.dll"), |b| build_dll(b, dll))?;
        }
        Ok(())
    })?;

    Ok(())
}
