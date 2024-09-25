//! Subcommands exercise various ways a program can fail.

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("usage: {} <mode>", args[0]);
        std::process::exit(1);
    }
    let mode = &args[1];

    match mode.as_str() {
        "exit" => std::process::exit(2),
        "write-null" => {
            // Note: Rust appears to optimize this out if we don't print, eek.
            println!("writing mem[0]");
            unsafe {
                let ptr = 0 as *mut u32;
                *ptr = 1;
            }
        }
        "write-high" => {
            println!("writing mem[0xFFFF_F000]");
            unsafe {
                let ptr = 0xFFFF_F000 as *mut u32;
                *ptr = 1;
            }
        }
        _ => eprintln!("unknown mode: {}", mode),
    }
}
