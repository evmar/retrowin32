extern crate win32;

use std::io::Write;

use win32::X86;

use anyhow::bail;

fn dump_asm(x86: &X86) {
    let mut decoder = iced_x86::Decoder::with_ip(
        32,
        &x86.mem[x86.regs.eip as usize..],
        x86.regs.eip as u64,
        iced_x86::DecoderOptions::NONE,
    );

    for instruction in decoder.iter().take(10) {
        print!("{:08X} ", instruction.ip());
        let start_index = instruction.ip() as usize;
        let instr_bytes = &x86.mem[start_index..start_index + instruction.len()];
        for b in instr_bytes.iter() {
            print!("{:02x}", b);
        }
        if instr_bytes.len() < 10 {
            for _ in 0..10 - instr_bytes.len() {
                print!("  ");
            }
        }
        println!(" {}", instruction);
    }
}

struct Host {
    exit_code: std::cell::Cell<Option<u32>>,
}
impl win32::Host for Host {
    fn exit(&self, code: u32) {
        self.exit_code.set(Some(code));
    }

    fn write(&self, buf: &[u8]) -> usize {
        std::io::stdout().lock().write(buf).unwrap()
    }
}

fn run() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let exe = match args.as_slice() {
        [_, exe] => exe,
        _ => bail!("specify path"),
    };

    let buf = std::fs::read(exe)?;
    let host = Host {
        exit_code: std::cell::Cell::new(None),
    };
    let mut x86 = X86::new(&host);
    win32::load_exe(&mut x86, &buf)?;

    while host.exit_code.get().is_none() {
        if let Err(err) = x86.step() {
            dump_asm(&x86);
            println!("err: {:?}", err);
            break;
        }
    }

    Ok(())
}

static LOGGER: Logger = Logger {};
struct Logger {}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        println!("{} {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    run().unwrap();
}
