mod host;
mod logging;

#[cfg(not(feature = "sdl"))]
mod headless;
#[cfg(feature = "sdl")]
mod sdl;

#[cfg(feature = "x86-64")]
mod resv32;

use anyhow::anyhow;

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
    #[argh(option, from_str_fn(parse_trace_points))]
    trace_points: Option<std::collections::VecDeque<u32>>,

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

fn parse_trace_points(param: &str) -> Result<std::collections::VecDeque<u32>, String> {
    let mut trace_points = std::collections::VecDeque::new();
    for addr in param.split(",") {
        if addr.is_empty() {
            continue;
        }
        let addr = u32::from_str_radix(addr, 16).map_err(|_| format!("bad addr {addr:?}"))?;
        trace_points.push_back(addr);
    }
    Ok(trace_points)
}

static mut SNAPSHOT_REQUESTED: bool = false;

#[cfg(target_family = "unix")]
fn install_sigusr1_handler() {
    unsafe extern "C" fn sigusr1(_sig: usize) {
        SNAPSHOT_REQUESTED = true;
    }
    let ret = unsafe { libc::signal(libc::SIGUSR1, sigusr1 as *const fn(usize) as usize) };
    if ret != 0 {
        log::error!("failed to install signal handler for snapshot");
    }
}
#[cfg(not(target_family = "unix"))]
fn install_sigusr1_handler() {}

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

    win32::trace::set_scheme(args.win32_trace.as_deref().unwrap_or("-"));
    let cmdline = args.cmdline.as_ref().unwrap_or(&args.exe);

    let buf = std::fs::read(&args.exe).map_err(|err| anyhow!("{}: {}", args.exe, err))?;
    let host = host::new_host();
    let mut machine = win32::Machine::new(Box::new(host.clone()), cmdline.clone());

    let addrs = machine
        .load_exe(&buf, &args.exe, false)
        .map_err(|err| anyhow!("loading {}: {}", args.exe, err))?;

    #[cfg(target_family = "unix")]
    install_sigusr1_handler();

    #[cfg(feature = "x86-64")]
    {
        assert!(args.trace_points.is_none());
        unsafe {
            let ptr: *mut win32::Machine = &mut machine;
            machine.emu.shims.set_machine_hack(ptr, addrs.stack_pointer);
            machine.jump_to_entry_point(addrs.entry_point);
        }
    }

    #[cfg(feature = "x86-emu")]
    {
        _ = addrs;

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
        } else if let Some(mut trace_points) = args.trace_points {
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
        let mut trace_points = args.trace_points.unwrap();
        let mut eip = addrs.entry_point;
        loop {
            let end = trace_points.pop_front().unwrap_or(0);
            win32::shims::unicorn_loop(&mut machine, eip, end);
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
