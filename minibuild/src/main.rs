use std::path::{Path, PathBuf};

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

fn build_dlls(b: &B) -> anyhow::Result<()> {
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
    for dll in dlls {
        b.task(format!("{dll}.dll"), |b| build_dll(b, dll))?;
    }
    Ok(())
}

fn build_exe_cpp(b: &B) -> anyhow::Result<()> {
    let xwin = {
        let xwin = std::env::var("XWIN");
        match xwin {
            Ok(xwin) => xwin,
            Err(_) => {
                let home = std::env::var("HOME")?;
                home + "/.xwin-cache/splat"
            }
        }
    };
    let sdk_flags = ["/winsysroot", &xwin];

    let clang_flags = [
        "-fuse-ld=lld",
        "-flto",
        "-target",
        "i686-pc-windows-msvc",
        "-mno-sse",
        "-mno-sse2",
    ];

    let cflags = [
        "/std:c++20",
        // reproducible builds
        "/Brepro",
        // optimize for size
        "/Os",
        // no security cookies
        "/GS-",
        "/MT",
        // note: /Zi for debug info (useful for ghidra) but it breaks build reproducibility
    ];

    // https://devblogs.microsoft.com/cppblog/introducing-the-universal-crt/
    let link_flags = [
        "/subsystem:console",
        "ddraw.lib",
        "gdi32.lib",
        "kernel32.lib",
        "user32.lib",
        "libcmt.lib",
        "libvcruntime.lib",
        "libucrt.lib",
    ];

    let srcs = [
        "cmdline.cc",
        "ddraw.cc",
        "dib.cc",
        "errors.cc",
        "gdi.cc",
        "metrics.cc",
        "thread.cc",
    ];
    for src in srcs {
        b.task(src, |b| {
            let src_path = PathBuf::from(format!("exe/cpp/{src}"));
            let util_path = Path::new("exe/cpp/util.h");
            let exe_path = src_path.with_extension("exe");
            b.files(&[src_path.as_ref(), util_path], &[&exe_path], |b| {
                b.cmd(
                    &[
                        ["clang-cl"].as_slice(),
                        &clang_flags,
                        &cflags,
                        &sdk_flags,
                        &[src_path.to_str().unwrap(), "/Fe:exe/cpp/"],
                        &["/link"],
                        &link_flags,
                    ]
                    .concat(),
                )
            })
        })?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    B::run(|b| {
        b.task("dlls", build_dlls)?;
        b.task("exe/cpp test programs", build_exe_cpp)?;
        Ok(())
    })
}
