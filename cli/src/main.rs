extern crate argh;
extern crate win32;
mod logging;
use anyhow::anyhow;
use std::{
    cell::RefCell,
    io::{Seek, Write},
    path::Path,
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

#[cfg(feature = "x86-64")]
mod resv32;

#[cfg(feature = "x86-emu")]
static mut SNAPSHOT_REQUESTED: bool = false;

#[cfg(feature = "x86-emu")]
fn dump_asm(machine: &win32::Machine, count: usize) {
    let instrs = win32::disassemble(machine.mem(), machine.emu.x86.cpu().regs.eip, count);

    for instr in instrs {
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
    fn info(&self) -> u32 {
        let meta = self.f.metadata().unwrap();
        meta.len() as u32
    }

    fn seek(&mut self, ofs: u32) -> bool {
        self.f.seek(std::io::SeekFrom::Start(ofs as u64)).unwrap();
        true
    }
}

impl std::io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.f.read(buf)
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
    fn exit(&self, code: u32) {
        self.0.borrow_mut().exit_code = Some(code);
    }

    fn time(&self) -> u32 {
        let mut env = self.0.borrow_mut();
        let gui = env.ensure_gui().unwrap();
        gui.time()
    }

    fn get_message(&self) -> Option<win32::Message> {
        let mut env = self.0.borrow_mut();
        let gui = env.gui.as_mut().unwrap();
        gui.get_message()
    }

    fn block(&self, wait: Option<u32>) -> bool {
        let mut env = self.0.borrow_mut();
        let gui = env.gui.as_mut().unwrap();
        gui.block(wait)
    }

    fn open(&self, path: &str) -> Box<dyn win32::File> {
        Box::new(File::open(Path::new(path)))
    }

    fn write(&self, buf: &[u8]) -> usize {
        std::io::stdout().lock().write(buf).unwrap()
    }

    fn create_window(&mut self, hwnd: u32) -> Box<dyn win32::Window> {
        let mut env = self.0.borrow_mut();
        let gui = env.ensure_gui().unwrap();
        gui.create_window(hwnd)
    }

    fn create_surface(
        &mut self,
        _hwnd: u32,
        opts: &win32::SurfaceOptions,
    ) -> Box<dyn win32::Surface> {
        let mut env = self.0.borrow_mut();
        let gui = env.ensure_gui().unwrap();
        gui.create_surface(opts)
    }
}

#[derive(argh::FromArgs)]
/// win32 emulator.
struct Args {
    /// change working directory before running
    #[argh(option)]
    chdir: Option<String>,

    /// winapi systems to trace; see trace.rs for docs
    #[argh(option)]
    win32_trace: Option<String>,

    /// log CPU state upon each new basic block
    #[argh(switch)]
    #[cfg(feature = "x86-emu")]
    trace_blocks: bool,

    /// log CPU state first time each point reached
    #[argh(option)]
    trace_points: Option<String>,

    /// exe to run
    #[argh(positional)]
    exe: String,

    /// cmdline to pass to exe
    #[argh(positional)]
    cmdline: Option<String>,

    /// load snapshot from file
    #[cfg(feature = "x86-emu")]
    #[argh(option)]
    snapshot: Option<String>,
}

/// Transfer control to the executable's entry point.
/// Needs to switch code segments to enter compatibility mode, stacks, etc.
#[cfg(feature = "x86-64")]
#[inline(never)] // aid in debugging
fn jump_to_entry_point(machine: &mut win32::Machine, entry_point: u32) {
    // Assert that our code was loaded in the 3-4gb memory range, which means
    // that calls from/to it can be managed with 32-bit pointers.
    // (This arrangement is set up by the linker flags.)
    let mem_3gb_range = 0xc000_0000u64..0x1_0000_0000u64;
    let fn_addr = &jump_to_entry_point as *const _ as u64;
    assert!(mem_3gb_range.contains(&fn_addr));

    println!("entry point at {:x}, about to jump", entry_point);
    std::io::stdin().read_line(&mut String::new()).unwrap();

    let pin = std::pin::pin!(machine.call_x86(entry_point, vec![]));
    win32::shims::call_sync(pin);
}

#[cfg(any(feature = "x86-emu", feature = "x86-unicorn"))]
fn print_trace(machine: &win32::Machine) {
    #[cfg(feature = "x86-emu")]
    let (eip, eax, ebx, ecx, edx, esi, edi, esp, st_top) = {
        let regs = &machine.emu.x86.cpu().regs;
        (
            regs.eip,
            regs.get32(x86::Register::EAX),
            regs.get32(x86::Register::EBX),
            regs.get32(x86::Register::ECX),
            regs.get32(x86::Register::EDX),
            regs.get32(x86::Register::ESI),
            regs.get32(x86::Register::EDI),
            regs.get32(x86::Register::ESP),
            machine.emu.x86.cpu().fpu.st_top,
        )
    };

    #[cfg(feature = "x86-unicorn")]
    let (eip, eax, ebx, ecx, edx, esi, edi, esp, st_top) = {
        let unicorn = &machine.emu.unicorn;
        (
            unicorn.reg_read(unicorn_engine::RegisterX86::EIP).unwrap(),
            unicorn.reg_read(unicorn_engine::RegisterX86::EAX).unwrap(),
            unicorn.reg_read(unicorn_engine::RegisterX86::EBX).unwrap(),
            unicorn.reg_read(unicorn_engine::RegisterX86::ECX).unwrap(),
            unicorn.reg_read(unicorn_engine::RegisterX86::EDX).unwrap(),
            unicorn.reg_read(unicorn_engine::RegisterX86::ESI).unwrap(),
            unicorn.reg_read(unicorn_engine::RegisterX86::EDI).unwrap(),
            unicorn.reg_read(unicorn_engine::RegisterX86::ESP).unwrap(),
            -1,
        )
    };

    println!("@{eip:x}\n  eax:{eax:x} ebx:{ebx:x} ecx:{ecx:x} edx:{edx:x} esi:{esi:x} edi:{edi:x} esp:{esp:x} st_top:{st_top}");
}

fn main() -> anyhow::Result<()> {
    logging::init();

    #[cfg(feature = "x86-64")]
    unsafe {
        crate::resv32::init_resv32();
    }

    let args: Args = argh::from_env();

    if let Some(dir) = args.chdir {
        std::env::set_current_dir(dir).unwrap();
    }

    let mut trace_points = std::collections::VecDeque::new();
    if let Some(arg) = args.trace_points {
        for addr in arg.split(",") {
            if addr.is_empty() {
                continue;
            }
            let addr = u32::from_str_radix(addr, 16)
                .map_err(|_| anyhow!("bad addr {addr:?}"))
                .unwrap();
            trace_points.push_back(addr);
        }
    }

    win32::trace::set_scheme(args.win32_trace.as_deref().unwrap_or("-"));
    let cmdline = args.cmdline.as_ref().unwrap_or(&args.exe);

    let buf = std::fs::read(&args.exe).map_err(|err| anyhow!("{}: {}", args.exe, err))?;
    let host = EnvRef(Rc::new(RefCell::new(Env::new())));
    let mut machine = win32::Machine::new(Box::new(host.clone()), cmdline.clone());

    let addrs = machine
        .load_exe(&buf, &args.exe, false)
        .map_err(|err| anyhow!("loading {}: {}", args.exe, err))?;

    #[cfg(feature = "x86-64")]
    unsafe {
        let ptr: *mut win32::Machine = &mut machine;
        machine.emu.shims.set_machine_hack(ptr, addrs.stack_pointer);
        jump_to_entry_point(&mut machine, addrs.entry_point);
    }

    #[cfg(feature = "x86-emu")]
    {
        _ = addrs;

        unsafe {
            unsafe extern "C" fn sigusr1(_sig: usize) {
                SNAPSHOT_REQUESTED = true;
            }
            if libc::signal(libc::SIGUSR1, sigusr1 as *const fn(usize) as usize) != 0 {
                log::error!("failed to install signal handler for snapshot");
            }
        }

        if let Some(snap) = args.snapshot {
            let bytes = std::fs::read(snap).unwrap();
            machine.load_snapshot(&bytes);
        }

        let start = std::time::Instant::now();
        if args.trace_blocks {
            let mut seen_blocks = std::collections::HashSet::new();
            while machine.run() {
                let regs = &machine.emu.x86.cpu().regs;
                if regs.eip & 0xFFFF_0000 == 0xF1A7_0000 {
                    continue;
                }
                if seen_blocks.contains(&regs.eip) {
                    continue;
                }
                print_trace(&machine);
                seen_blocks.insert(regs.eip);
            }
        } else if !trace_points.is_empty() {
            while let Some(next_trace) = trace_points.pop_front() {
                machine
                    .emu
                    .x86
                    .add_breakpoint(machine.emu.memory.mem(), next_trace);
                loop {
                    // Ignore errors here because we will hit breakpoints.
                    machine.run();
                    if machine.emu.x86.cpu().regs.eip == next_trace {
                        break;
                    }
                }
                machine
                    .emu
                    .x86
                    .clear_breakpoint(machine.emu.memory.mem(), next_trace);

                print_trace(&machine);
            }
        } else {
            while machine.run() {
                unsafe {
                    if SNAPSHOT_REQUESTED {
                        let buf = machine.snapshot();
                        let path = "snapshot";
                        std::fs::write(path, buf).unwrap();
                        log::info!("wrote snapshot to {path:?}");
                        SNAPSHOT_REQUESTED = false;
                    }
                }
            }
        }

        match &machine.emu.x86.cpu().state {
            x86::CPUState::Error(error) => {
                log::error!("{:?}", error);
                dump_asm(&machine, 5);
            }
            x86::CPUState::Exit(_) => {}
            x86::CPUState::Blocked(_) => unreachable!(),
            x86::CPUState::Running => unreachable!(),
        }

        let millis = start.elapsed().as_millis() as usize;
        if millis > 0 {
            eprintln!(
                "{} instrs in {} ms: {}m/s",
                machine.emu.x86.instr_count,
                millis,
                (machine.emu.x86.instr_count / millis) / 1000
            );
            eprintln!("icache: {}", machine.emu.x86.icache.stats());
        }
    }

    #[cfg(feature = "x86-unicorn")]
    {
        let mut eip = addrs.entry_point;
        loop {
            let end = trace_points.pop_front().unwrap_or(0);
            crate::win32::shims::unicorn_loop(&mut machine, eip, end);
            if end == 0 {
                break;
            } else {
                print_trace(&machine);
                eip = end;
            }
        }
    }

    Ok(())
}
