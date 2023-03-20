extern crate win32;
mod logging;
use anyhow::bail;
use std::{cell::RefCell, io::Write, rc::Rc};

#[cfg(feature = "sdl")]
mod sdl;
#[cfg(feature = "sdl")]
use sdl::GUI;
#[cfg(not(feature = "sdl"))]
mod headless;
#[cfg(not(feature = "sdl"))]
use headless::GUI;

fn dump_asm(runner: &win32::Runner) {
    let instrs = win32::disassemble(&runner.machine.x86.mem, runner.machine.x86.regs.eip);

    for instr in &instrs[..std::cmp::min(instrs.len(), 5)] {
        print!("{:08x} {:10} ", instr.addr, instr.bytes);
        for part in &instr.code {
            print!("{}", part.text);
        }
        println!();
    }
}

struct Env {
    gui: Option<GUI>,
    exit_code: Option<u32>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            gui: None,
            exit_code: None,
        }
    }

    pub fn ensure_gui(&mut self) -> anyhow::Result<&mut GUI> {
        if self.gui.is_none() {
            self.gui = Some(GUI::new()?);
        }
        Ok(self.gui.as_mut().unwrap())
    }
}

#[derive(Clone)]
struct EnvRef(Rc<RefCell<Env>>);

impl win32::Host for EnvRef {
    fn exit(&mut self, code: u32) {
        self.0.borrow_mut().exit_code = Some(code);
    }

    fn time(&self) -> u32 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32
    }

    fn open(&self, _path: &str) -> Box<dyn win32::File> {
        unimplemented!();
    }

    fn write(&self, buf: &[u8]) -> usize {
        std::io::stdout().lock().write(buf).unwrap()
    }

    fn create_window(&mut self) -> Box<dyn win32::Window> {
        let mut env = self.0.borrow_mut();
        let gui = env.ensure_gui().unwrap();
        gui.create_window()
    }

    fn create_surface(&mut self, opts: &win32::SurfaceOptions) -> Box<dyn win32::Surface> {
        let mut env = self.0.borrow_mut();
        let gui = env.ensure_gui().unwrap();
        gui.create_surface(opts)
    }
}

fn main() -> anyhow::Result<()> {
    logging::init()?;
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        bail!("specify path to exe");
    }
    let exe = &args[1];
    let cmdline = args[1..].join(" ");

    let buf = std::fs::read(exe)?;
    let host = EnvRef(Rc::new(RefCell::new(Env::new())));
    let mut runner = win32::Runner::new(Box::new(host.clone()));
    runner.load_exe(&buf, cmdline)?;

    let start = std::time::Instant::now();
    loop {
        if let Some(gui) = &mut host.0.borrow_mut().gui {
            if !gui.pump_messages() {
                break;
            }
        }
        match runner.execute_block() {
            Err(err) => {
                dump_asm(&runner);
                log::error!("{:?}", err);
                break;
            }
            Ok(_) => {
                if host.0.borrow().exit_code.is_some() {
                    break;
                }
            }
        }
    }
    let millis = start.elapsed().as_millis() as usize;
    if millis > 0 {
        eprintln!(
            "{} instrs in {} ms: {}m/s",
            runner.instr_count,
            millis,
            (runner.instr_count / millis) / 1000
        );
    }

    Ok(())
}
