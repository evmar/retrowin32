mod fs;
mod host;
mod logging;
mod time;

#[cfg(feature = "x86-emu")]
mod unpack;

#[cfg(not(feature = "sdl"))]
mod headless;
#[cfg(feature = "sdl")]
mod sdl;

#[cfg(feature = "x86-64")]
mod resv32;

use anyhow::anyhow;
use std::process::ExitCode;
use win32::Machine;

#[derive(argh::FromArgs)]
/// win32 emulator.
struct Args {
    /// change working directory before running
    #[argh(option, short = 'C')]
    chdir: Option<String>,

    /// dlls to load from disk rather than retrowin32's builtin
    #[argh(option)]
    external_dll: Vec<String>,

    /// winapi systems to trace; see HACKING.md for docs
    #[argh(option)]
    win32_trace: Option<String>,

    /// log CPU state upon each new basic block
    #[cfg(feature = "x86-emu")]
    #[argh(switch)]
    trace_blocks: bool,

    /// log CPU state first time each point reached
    #[cfg(any(feature = "x86-emu", feature = "x86-unicorn"))]
    #[argh(option, from_str_fn(parse_trace_points))]
    trace_points: Option<std::collections::VecDeque<u32>>,

    /// exit after executing this many instructions
    #[cfg(feature = "x86-emu")]
    #[argh(option)]
    exit_after: Option<usize>,

    /// dump exe file at state when jumping from this instruction
    #[cfg(feature = "x86-emu")]
    #[argh(option, from_str_fn(parse_hex))]
    unpack_at: Option<u32>,

    /// enable debug logging
    #[argh(switch)]
    debug: bool,

    /// debugger break right when we call exe entry point
    #[argh(switch)]
    break_on_startup: bool,

    /// enable audio output
    #[argh(switch)]
    audio: bool,

    /// command line to run
    #[argh(positional, greedy)]
    cmdline: Vec<String>,
}

#[cfg(any(feature = "x86-emu", feature = "x86-unicorn"))]
fn parse_trace_points(param: &str) -> Result<std::collections::VecDeque<u32>, String> {
    let mut trace_points = std::collections::VecDeque::new();
    for mut addr in param.split(&[',', ' ']) {
        if addr.is_empty() {
            continue;
        }
        if addr.starts_with("@") {
            addr = &addr[1..];
        }
        let addr = u32::from_str_radix(addr, 16).map_err(|_| format!("bad addr {addr:?}"))?;
        trace_points.push_back(addr);
    }
    Ok(trace_points)
}

#[cfg(feature = "x86-emu")]
fn parse_hex(param: &str) -> Result<u32, String> {
    u32::from_str_radix(param, 16).map_err(|_| format!("bad hex {param:?}"))
}

/// Convert a unix command line to a Windows form.
fn command_line_to_windows(
    host: &dyn win32::host::Host,
    cmdline: Vec<String>,
) -> anyhow::Result<String> {
    let mut cmdline = cmdline;
    if cmdline.is_empty() {
        anyhow::bail!("specify command line");
    }

    // Convert first argument to full path to exe.
    let cwd = host
        .current_dir()
        .map_err(|e| anyhow!("failed to get current dir: {e:?}"))?;
    cmdline[0] = cwd
        .join(&cmdline[0])
        .normalize()
        .to_str()
        .ok_or_else(|| anyhow!("invalid command line"))?
        .to_owned();

    // Quote any arguments with spaces etc.
    for arg in &mut cmdline {
        escape_arg(arg);
    }

    Ok(cmdline.join(" "))
}

fn escape_arg(arg: &mut String) {
    // TODO: correct escaping here:
    // https://learn.microsoft.com/en-us/archive/blogs/twistylittlepassagesallalike/everyone-quotes-command-line-arguments-the-wrong-way

    if !arg.contains(&['"', ' ', '\t', '\n']) {
        return;
    }

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
    *arg = escaped;
}

fn main() -> anyhow::Result<ExitCode> {
    #[cfg(feature = "x86-64")]
    unsafe {
        // Initialize memory as early as possible, ideally before before any heap allocations.
        resv32::init_resv32();
    }

    let mut args: Args = argh::from_env();
    logging::init(if args.debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    });

    if let Some(dir) = &args.chdir {
        std::env::set_current_dir(dir).unwrap();
    }

    win32::trace::set_scheme(args.win32_trace.as_deref().unwrap_or("-"));

    let host = host::new_host();
    let cmdline = command_line_to_windows(&host, std::mem::take(&mut args.cmdline))?;
    let mut machine = win32::Machine::new(Box::new(host));

    if args.break_on_startup {
        machine.break_on_startup();
    }
    machine.set_external_dlls(std::mem::take(&mut args.external_dll));
    machine.set_audio(args.audio);
    machine.start_exe(cmdline, None);

    let status = run_emu(args, &mut machine)?;
    let exit_code = match status {
        win32::Status::Exit(code) => code,
        win32::Status::Error { message } => {
            log::error!("{}", message);
            machine.dump_state(0);
            1
        }
        _ => unreachable!("{status:?}"),
    };
    Ok(ExitCode::from(exit_code as u8))
}

#[cfg(feature = "x86-emu")]
fn run_emu(args: Args, machine: &mut Machine) -> anyhow::Result<win32::Status> {
    let start = std::time::Instant::now();
    if args.trace_blocks {
        let mut seen_blocks = std::collections::HashSet::new();
        while machine.run() {
            let regs = &machine.emu.x86.cpu().regs;
            if seen_blocks.contains(&regs.eip) {
                continue;
            }
            machine.print_trace();
            seen_blocks.insert(regs.eip);
        }
    } else if let Some(mut trace_points) = args.trace_points {
        // Break once the exe is loaded, so we can set breakpoints within it.
        machine.break_on_startup();
        while machine.run() {}
        assert_eq!(machine.emu.status, win32::Status::DebugBreak);
        machine.unblock();

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
            machine.unblock();

            machine.print_trace();
        }
    } else if let Some(unpack_at) = args.unpack_at {
        unpack::unpack(machine, unpack_at)?;
        return Ok(win32::Status::Exit(0));
    } else {
        while machine.run() {
            if let Some(exit_after) = args.exit_after {
                if machine.emu.x86.instr_count >= exit_after {
                    machine.emu.status = win32::Status::Exit(0);
                    break;
                }
            }
        }
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

    Ok(std::mem::take(&mut machine.emu.status))
}

#[cfg(feature = "x86-64")]
fn run_emu(_args: Args, _machine: &mut Machine) -> anyhow::Result<win32::Status> {
    // start_exe synchronously runs the process to completion;
    // we shouldn't get here.
    unreachable!();
}

#[cfg(feature = "x86-unicorn")]
fn run_emu(args: Args, machine: &mut Machine) -> anyhow::Result<win32::Status> {
    if let Some(_trace_points) = args.trace_points {
        machine.print_trace();
        todo!();
    } else {
        machine.run();
    }
    Ok(std::mem::take(&mut machine.emu.status))
}
