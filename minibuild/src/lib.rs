use std::{
    fs::File,
    io::ErrorKind,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub fn overprint(msg: &str) {
    use std::io::Write;
    print!("\r\x1b[K{}", msg);
    std::io::stdout().flush().unwrap();
}

fn is_not_found(err: &std::io::Error) -> bool {
    matches!(err.kind(), ErrorKind::NotFound)
}

enum OutOfDate<'a> {
    MissingOutput(&'a Path),
    OldInput(&'a Path),
}

impl<'a> std::fmt::Display for OutOfDate<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutOfDate::MissingOutput(path) => write!(f, "missing output {:?}", path),
            OutOfDate::OldInput(path) => write!(f, "input {:?} is newer than output", path),
        }
    }
}

fn out_of_date<'a>(
    ins: &'a [PathBuf],
    outs: &'a [PathBuf],
) -> anyhow::Result<Option<OutOfDate<'a>>> {
    let mut oldest_out: Option<SystemTime> = None;
    for out in outs {
        let mtime = match out.metadata() {
            Err(err) if is_not_found(&err) => return Ok(Some(OutOfDate::MissingOutput(out))),
            m => m?.modified()?,
        };
        oldest_out = Some(match oldest_out {
            None => mtime,
            Some(t) => t.min(mtime),
        });
    }
    let oldest_out = oldest_out.unwrap();

    for in_ in ins {
        let mtime = in_.metadata()?.modified()?;
        if mtime > oldest_out {
            return Ok(Some(OutOfDate::OldInput(in_)));
        }
    }

    Ok(None)
}

fn mark_up_to_date(outs: &[PathBuf]) -> anyhow::Result<()> {
    let now = SystemTime::now();
    for out in outs {
        let f = match File::open(out) {
            Err(err) if is_not_found(&err) => {
                anyhow::bail!("didn't write output {}", out.display());
            }
            f => f?,
        };
        f.set_modified(now)?;
    }
    Ok(())
}

#[derive(Default)]
pub struct B {
    desc: String,
    indent: usize,
    cwd: PathBuf,
}

impl B {
    pub fn chdir(&mut self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        self.cwd = path.as_ref().to_path_buf();
        Ok(())
    }

    pub fn task(
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
        f(&mut b)?;
        if self.indent == 0 {
            overprint("up to date\n");
        }
        Ok(())
    }

    pub fn files(
        &mut self,
        ins: &[&Path],
        outs: &[&Path],
        f: impl FnOnce(&mut B) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        let ins = ins.iter().map(|p| self.cwd.join(p)).collect::<Vec<_>>();
        let outs = outs.iter().map(|p| self.cwd.join(p)).collect::<Vec<_>>();
        if let Some(reason) = out_of_date(&ins, &outs)? {
            overprint(&format!("{}\n", reason));
            f(self)?;
            mark_up_to_date(&outs)?;
        }
        Ok(())
    }

    pub fn cmd(&mut self, argv: &[&str]) -> anyhow::Result<()> {
        self.task(argv[0], |b| {
            overprint(&format!("$ {}\n", argv.join(" ")));
            let mut cmd = std::process::Command::new(argv[0]);
            cmd.args(&argv[1..]);
            if !b.cwd.as_os_str().is_empty() {
                cmd.current_dir(&b.cwd);
            }
            let output = cmd.output()?;
            if !output.status.success() {
                println!();
                println!("{}", std::str::from_utf8(&output.stdout).unwrap());
                anyhow::bail!("command failed");
            }
            Ok(())
        })
    }
}
