mod host;
mod logging;

#[cfg(not(feature = "sdl"))]
mod headless;
#[cfg(feature = "sdl")]
mod sdl;

#[cfg(feature = "x86-64")]
mod resv32;

use anyhow::anyhow;
use std::borrow::Cow;
use std::process::ExitCode;
use win32::FileSystem;

#[derive(argh::FromArgs)]
/// win32 emulator.
struct Args {
    /// change working directory before running
    #[argh(option, short = 'C')]
    chdir: Option<String>,

    /// dlls to load from disk rather than builtin
    #[argh(option)]
    external_dll: Vec<String>,

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

    /// exit after executing this many instructions
    #[argh(option)]
    #[cfg(feature = "x86-emu")]
    exit_after: Option<usize>,

    /// enable debug logging
    #[argh(switch)]
    debug: bool,

    /// enable audio output
    #[argh(switch)]
    audio: bool,

    /// command line to run
    #[argh(positional, greedy)]
    cmdline: Vec<String>,
}

#[cfg(any(feature = "x86-emu", feature = "x86-unicorn"))]
fn print_trace(machine: &win32::Machine) {
    #[cfg(feature = "x86-emu")]
    let (eip, eax, ebx, ecx, edx, esi, edi, esp, ebp, st_top) = {
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
            regs.get32(x86::Register::EBP),
            machine.emu.x86.cpu().fpu.st_top,
        )
    };

    #[cfg(feature = "x86-unicorn")]
    let (eip, eax, ebx, ecx, edx, esi, edi, esp, ebp, st_top) = {
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
            unicorn.reg_read(unicorn_engine::RegisterX86::EBP).unwrap(),
            -1,
        )
    };

    println!("@{eip:x}\n  eax:{eax:x} ebx:{ebx:x} ecx:{ecx:x} edx:{edx:x} esi:{esi:x} edi:{edi:x} esp:{esp:x} ebp:{ebp:x} st_top:{st_top}");
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

fn main() -> anyhow::Result<ExitCode> {
    #[cfg(feature = "x86-64")]
    unsafe {
        crate::resv32::init_resv32();
    }

    let args: Args = argh::from_env();
    logging::init(if args.debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    });

    if let Some(dir) = args.chdir {
        std::env::set_current_dir(dir).unwrap();
    }

    win32::trace::set_scheme(args.win32_trace.as_deref().unwrap_or("-"));

    let exe = args
        .cmdline
        .first()
        .ok_or_else(|| anyhow!("missing command line"))?;
    let exe = std::fs::canonicalize(exe).map_err(|err| anyhow!("{}: {}", exe, err))?;
    let buf = std::fs::read(&exe).map_err(|err| anyhow!("{}: {}", exe.display(), err))?;
    let host = host::new_host();

    let mut cmdline = args.cmdline.clone();
    let cwd = host
        .current_dir()
        .map_err(|e| anyhow!("failed to get current dir: {e:?}"))?;
    cmdline[0] = cwd
        .join(&cmdline[0])
        .normalize()
        .to_string_lossy()
        .into_owned();
    let cmdline = cmdline
        .iter()
        .map(|s| escape_arg(s))
        .collect::<Vec<_>>()
        .join(" ");
    let mut machine = win32::Machine::new(Box::new(host.clone()));
    machine.set_external_dlls(&args.external_dll);
    machine.state.winmm.audio_enabled = args.audio;

    let addrs = machine
        .load_exe(&buf, cmdline, None)
        .map_err(|err| anyhow!("loading {}: {}", exe.display(), err))?;
    _ = addrs;

    let exit_code: u32;

    #[cfg(feature = "x86-64")]
    {
        assert!(args.trace_points.is_none());
        unsafe {
            let ptr: *mut win32::Machine = &mut machine;
            machine.emu.shims.set_machine_hack(ptr, addrs.stack_pointer);
            machine.jump_to_entry_point(addrs.entry_point);
        }
        exit_code = 0; // TODO: exit code path never hit
    }

    #[cfg(feature = "x86-emu")]
    {
        _ = addrs;

        let start = std::time::Instant::now();
        if args.trace_blocks {
            let mut seen_blocks = std::collections::HashSet::new();
            while machine.run() {
                let regs = &machine.emu.x86.cpu().regs;
                if seen_blocks.contains(&regs.eip) {
                    continue;
                }
                print_trace(&machine);
                seen_blocks.insert(regs.eip);
            }
        } else if let Some(mut trace_points) = args.trace_points {
            while let Some(next_trace) = trace_points.pop_front() {
                machine.add_breakpoint(next_trace);
                loop {
                    // Ignore errors here because we will hit breakpoints.
                    machine.run();
                    if machine.emu.x86.cpu().regs.eip == next_trace {
                        break;
                    }
                }
                machine.clear_breakpoint(next_trace);

                print_trace(&machine);
            }
        } else {
            while machine.run() {
                if let Some(exit_after) = args.exit_after {
                    if machine.emu.x86.instr_count >= exit_after {
                        machine.status = win32::Status::Exit(0);
                        break;
                    }
                }
            }
        }

        match &machine.status {
            win32::Status::Exit(code) => {
                exit_code = *code;
            }
            win32::Status::Error { message } => {
                log::error!("{}", message);
                machine.dump_state(0);
                exit_code = 1;
            }
            _ => unreachable!(),
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
        if let Some(_trace_points) = args.trace_points {
            print_trace(&machine);
            todo!();
        } else {
            match machine.main(addrs.entry_point) {
                win32::Status::Exit(code) => {
                    exit_code = *code;
                }
                win32::Status::Error { message } => {
                    log::error!("{}", message);
                    machine.dump_state();
                    exit_code = 1;
                }
                _ => unreachable!(),
            }
        }
    }

    Ok(ExitCode::from(exit_code as u8))
}

fn escape_arg(arg: &str) -> Cow<str> {
    if arg.contains(['"', ' ', '\t', '\n'].as_ref()) {
        let mut escaped = String::with_capacity(arg.len() + 2);
        escaped.push('"');
        for c in arg.chars() {
            match c {
                '"' => {
                    escaped.push('\\');
                    escaped.push(c);
                }
                _ => escaped.push(c),
            }
        }
        escaped.push('"');
        Cow::Owned(escaped)
    } else {
        Cow::Borrowed(arg)
    }
}
