extern crate win32;

use std::io::Write;

use anyhow::bail;

fn dump_asm(runner: &win32::Runner) {
    let instrs = win32::disassemble(&runner.x86.mem, runner.x86.regs.eip);

    for instr in instrs {
        print!("{:08X} ", instr.addr);
        for b in instr.bytes.bytes() {
            print!("{:02x}", b);
        }
        if instr.bytes.len() < 10 {
            for _ in 0..10 - instr.bytes.len() {
                print!("  ");
            }
        }
        for part in instr.code {
            print!("{}", part.text);
        }
        println!();
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

    fn time(&self) -> u32 {
        todo!()
    }

    fn create_window(&self) -> Box<dyn win32::Window> {
        todo!()
    }

    fn create_surface(&self, _opts: &win32::SurfaceOptions) -> Box<dyn win32::Surface> {
        todo!()
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
    let mut runner = win32::Runner::new(&host);
    win32::load_exe(&mut runner.x86, &buf)?;

    while host.exit_code.get().is_none() {
        if let Err(err) = runner.step() {
            dump_asm(&runner);
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
