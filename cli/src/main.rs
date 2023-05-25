extern crate argh;
extern crate win32;
mod logging;
use anyhow::anyhow;
use std::{
    cell::RefCell,
    collections::HashSet,
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

#[cfg(feature = "sdl")]
mod sdl;
#[cfg(feature = "sdl")]
use sdl::GUI;
#[cfg(not(feature = "sdl"))]
mod headless;
#[cfg(not(feature = "sdl"))]
use headless::GUI;

fn dump_asm(machine: &win32::Machine) {
    let instrs = win32::disassemble(&machine.x86.mem, machine.x86.cpu.regs.eip);

    for instr in &instrs[..std::cmp::min(instrs.len(), 5)] {
        print!("{:08x} {:10} ", instr.addr, instr.bytes);
        for part in &instr.code {
            print!("{}", part.text);
        }
        println!();
    }
}

struct File {
    f: std::fs::File,
}
impl File {
    fn open(path: &Path) -> Self {
        let f = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(err) => {
                log::error!("opening {:?}: {}", path, err);
                std::fs::File::open("/dev/null").unwrap()
            }
        };
        File { f }
    }
}
impl win32::File for File {
    fn seek(&mut self, ofs: u32) -> bool {
        self.f.seek(std::io::SeekFrom::Start(ofs as u64)).unwrap();
        true
    }

    fn read(&mut self, buf: &mut [u8], len: &mut u32) -> bool {
        let n = self.f.read(buf).unwrap();
        *len = n as u32;
        true
    }
}

struct Env {
    gui: Option<GUI>,
    exit_code: Option<u32>,
    cwd: PathBuf,
}

impl Env {
    pub fn new(cwd: PathBuf) -> Self {
        Env {
            gui: None,
            exit_code: None,
            cwd,
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

    fn open(&self, path: &str) -> Box<dyn win32::File> {
        let env = self.0.borrow();
        Box::new(File::open(&env.cwd.join(path)))
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

fn hex_arg(arg: &str) -> Result<u32, String> {
    u32::from_str_radix(arg, 16).map_err(|err| err.to_string())
}

#[derive(argh::FromArgs)]
/// win32 emulator.
struct Args {
    /// win32 modules to trace calls into
    #[argh(option)]
    win32_trace: Option<String>,

    #[argh(option, from_str_fn(hex_arg))]
    /// addresses to dump emulator state
    trace_points: Vec<u32>,

    /// exe to run
    #[argh(positional)]
    exe: String,

    /// cmdline to pass to exe
    #[argh(positional)]
    cmdline: Option<String>,
}

fn main() -> anyhow::Result<()> {
    logging::init()?;
    let args: Args = argh::from_env();
    win32::trace::set_scheme(args.win32_trace.as_deref().unwrap_or("-"));
    let cmdline = args.cmdline.as_ref().unwrap_or(&args.exe);

    let buf = std::fs::read(&args.exe).map_err(|err| anyhow!("{}: {}", args.exe, err))?;
    let cwd = Path::parent(Path::new(&args.exe)).unwrap();
    let host = EnvRef(Rc::new(RefCell::new(Env::new(cwd.to_owned()))));
    let mut machine = win32::Machine::new(Box::new(host.clone()));
    machine.load_exe(&buf, cmdline.clone(), false)?;

    let mut trace_points = HashSet::new();
    for &tp in &args.trace_points {
        trace_points.insert(tp);
        machine.x86.add_breakpoint(tp);
    }

    let start = std::time::Instant::now();
    loop {
        if let Some(gui) = &mut host.0.borrow_mut().gui {
            if !gui.pump_messages() {
                break;
            }
        }
        match machine.execute_block() {
            Err(err) => {
                dump_asm(&machine);
                log::error!("{:?}", err);
                break;
            }
            Ok(done) => {
                if host.0.borrow().exit_code.is_some() {
                    break;
                }

                let ip = machine.x86.cpu.regs.eip;
                if !done && trace_points.contains(&ip) {
                    let regs = &machine.x86.cpu.regs;
                    eprintln!(
                        "trace ip:{:x} eax:{:x} ebx:{:x} ecx:{:x} edx:{:x} esi:{:x} edi:{:x}",
                        regs.eip, regs.eax, regs.ebx, regs.ecx, regs.edx, regs.esi, regs.edi
                    );
                    machine.x86.clear_breakpoint(ip);
                    machine.single_step().unwrap();
                    machine.x86.add_breakpoint(ip);
                }
            }
        }
    }
    let millis = start.elapsed().as_millis() as usize;
    if millis > 0 {
        eprintln!(
            "{} instrs in {} ms: {}m/s",
            machine.x86.instr_count,
            millis,
            (machine.x86.instr_count / millis) / 1000
        );
    }

    Ok(())
}
