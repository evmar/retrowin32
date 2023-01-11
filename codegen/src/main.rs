use anyhow;

fn run() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("specify exe");
    }
    let buf = std::fs::read(&args[1])?;
    codegen::load(&buf)?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("ERROR: {err}");
    }
}
