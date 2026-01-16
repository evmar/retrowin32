use std::{fs::File, io::ErrorKind, path::Path, time::SystemTime};

pub use glob::glob;

const EXPLAIN: bool = false;
const VERBOSE: bool = false;

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
    OldInput(&'a Path, &'a Path),
}

impl<'a> std::fmt::Display for OutOfDate<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutOfDate::MissingOutput(path) => write!(f, "missing output {:?}", path),
            OutOfDate::OldInput(in_, out) => {
                write!(f, "input {:?} is newer than output {:?}", in_, out)
            }
        }
    }
}

fn out_of_date<'a>(ins: &'a [&Path], outs: &'a [&Path]) -> Option<OutOfDate<'a>> {
    let mut oldest_out: Option<(&Path, SystemTime)> = None;
    for out in outs {
        let mtime = match out.metadata() {
            Err(err) if is_not_found(&err) => return Some(OutOfDate::MissingOutput(out)),
            m => m.unwrap().modified().unwrap(),
        };
        oldest_out = Some(match oldest_out {
            None => (out, mtime),
            Some((_, t)) if mtime < t => (out, mtime),
            Some(oldest) => oldest,
        });
    }
    let (oldest_out_name, oldest_out) = oldest_out.unwrap();

    for in_ in ins {
        let mtime = in_.metadata().unwrap().modified().unwrap();
        if mtime > oldest_out {
            return Some(OutOfDate::OldInput(in_, oldest_out_name));
        }
    }

    None
}

#[allow(unused)]
fn mark_up_to_date(outs: &[&Path]) {
    let now = SystemTime::now();
    for out in outs {
        let f = match File::open(out) {
            Err(err) if is_not_found(&err) => {
                panic!("failed to write declared output {}", out.display());
            }
            f => f.unwrap(),
        };
        f.set_modified(now).unwrap();
    }
}

#[derive(Default)]
pub struct B {
    desc: String,
    indent: usize,
}

impl B {
    pub fn run(f: impl FnOnce(B)) {
        f(B::default());
        overprint("up to date\n");
    }

    fn new_task(&self, desc: String) -> B {
        let desc = if self.desc.is_empty() {
            desc.into()
        } else {
            format!("{} > {}", self.desc, desc)
        };

        let b = B {
            desc,
            indent: self.indent + 1,
        };
        overprint(&b.desc);
        b
    }

    pub fn task(&self, desc: impl Into<String>, f: impl FnOnce(B)) {
        let b = self.new_task(desc.into());
        f(b);
    }

    pub fn spawn<'scope, 'env>(
        &self,
        scope: &'scope std::thread::Scope<'scope, 'env>,
        desc: impl Into<String>,
        f: impl FnOnce(B) + Send + 'scope,
    ) {
        let b = self.new_task(desc.into());
        scope.spawn(move || f(b));
    }

    pub fn out_of_date<I: AsRef<Path>, O: AsRef<Path>>(&self, ins: &[I], outs: &[O]) -> bool {
        let ins = ins.iter().map(|p| p.as_ref()).collect::<Vec<_>>();
        let outs = outs.iter().map(|p| p.as_ref()).collect::<Vec<_>>();
        if let Some(reason) = out_of_date(&ins, &outs) {
            if EXPLAIN {
                overprint(&format!("{}\n", reason));
            }
            true
        } else {
            false
        }
    }

    pub fn cmd(&self, argv: &[&str]) {
        self.task(argv[0], |_| {
            if VERBOSE {
                overprint(&format!("$ {}\n", argv.join(" ")));
            } else {
                //println!(); // Show task
            }
            let mut cmd = std::process::Command::new(argv[0]);
            cmd.args(&argv[1..]);
            let output = cmd.output().unwrap();
            if !output.stdout.is_empty() {
                println!("{}", std::str::from_utf8(&output.stdout).unwrap());
            }
            if !output.stderr.is_empty() {
                println!("{}", std::str::from_utf8(&output.stderr).unwrap());
            }
            if !output.status.success() {
                println!();
                panic!("command failed");
            }
        });
    }
}
