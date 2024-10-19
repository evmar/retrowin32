pub fn overprint(msg: &str) {
    use std::io::Write;
    print!("\r\x1b[K{}", msg);
    std::io::stdout().flush().unwrap();
}

#[derive(Default)]
struct B {
    desc: String,
    indent: usize,
    cwd: Option<std::path::PathBuf>,
}

impl B {
    fn chdir(&mut self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        self.cwd = Some(path.as_ref().to_path_buf());
        Ok(())
    }

    fn task(
        &mut self,
        desc: impl Into<String>,
        f: impl FnOnce(&mut B) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        let desc = if self.desc.is_empty() {
            desc.into()
        } else {
            format!("{} > {}", self.desc, desc.into())
        };

        let mut b = B {
            desc,
            indent: self.indent + 1,
            cwd: self.cwd.clone(),
        };
        overprint(&b.desc);
        f(&mut b)
    }

    fn cmd(&mut self, argv: &[&str]) -> anyhow::Result<()> {
        //self.print(format!("$ {}", argv.join(" ")));
        self.task(argv[0], |b| {
            let mut cmd = std::process::Command::new(argv[0]);
            cmd.args(&argv[1..]);
            if let Some(cwd) = &b.cwd {
                cmd.current_dir(cwd);
            }
            let output = cmd.output()?;
            if !output.status.success() {
                anyhow::bail!("command failed");
            }
            Ok(())
        })
    }
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
            b.task(format!("{dll}.dll"), |b| {
                b.task("generate source", |b| {
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
                })?;

                b.task("compile+link", |b| {
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
                })?;

                Ok(())
            })?;
        }
        Ok(())
    })?;
    println!();

    Ok(())
}
