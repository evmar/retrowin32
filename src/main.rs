use anyhow::bail;

mod pe;
mod reader;

use pe::parse;

fn run() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let exe = match args.as_slice() {
        [_, exe] => exe,
        _ => bail!("specify path"),
    };

    let buf = std::fs::read(exe)?;
    let file = parse(&buf)?;
    println!("{file:#?}");
    Ok(())
}

fn main() {
    run().unwrap();
}
