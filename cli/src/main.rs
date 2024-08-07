mod debugger;
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
use std::task::{Context, Poll};
use win32::shims::BoxFuture;
use win32::winapi::types::win32_error_str;
use win32::Host;

#[derive(argh::FromArgs)]
/// win32 emulator.
struct Args {
    /// change working directory before running
    #[argh(option, short = 'C')]
    chdir: Option<String>,

    /// winapi systems to trace; see trace.rs for docs
    #[argh(option)]
    win32_trace: Option<String>,

    /// enable GDB stub
    #[argh(switch)]
    #[cfg(any(feature = "x86-emu", feature = "x86-unicorn"))]
    gdb_stub: bool,

    /// enable debug logging
    #[argh(switch)]
    debug: bool,

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
        .map_err(|e| anyhow!("failed to get current dir: {}", win32_error_str(e)))?;
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
    let mut machine = win32::Machine::new(Box::new(host.clone()), cmdline);

    let addrs = machine
        .load_exe(&buf, &exe, None)
        .map_err(|err| anyhow!("loading {}: {}", exe.display(), err))?;
    _ = addrs;

    #[cfg(feature = "x86-64")]
    let exit_code = {
        assert!(args.trace_points.is_none());
        unsafe {
            let ptr: *mut win32::Machine = &mut machine;
            machine.emu.shims.set_machine_hack(ptr, addrs.stack_pointer);
            machine.jump_to_entry_point(addrs.entry_point);
        }
        ExitCode::SUCCESS
    };

    #[cfg(feature = "x86-emu")]
    let start = std::time::Instant::now();

    #[cfg(any(feature = "x86-emu", feature = "x86-unicorn"))]
    let exit_code = {
        use debugger::{DebuggerExt, MachineTarget, MachineTargetAction};
        let mut target = MachineTarget::new(machine);
        let mut debugger = if args.gdb_stub {
            Some(debugger::run(&mut target)?)
        } else {
            None
        };
        enum MachineState {
            Running,
            Stopped,
        }
        let mut state = if debugger.is_some() {
            MachineState::Stopped
        } else {
            MachineState::Running
        };
        struct AsyncShimCall {
            shim: &'static win32::shims::Shim,
            future: BoxFuture<u32>,
        }
        let mut ctx = Context::from_waker(futures::task::noop_waker_ref());
        let mut shim_calls = Vec::<AsyncShimCall>::new();
        let exit_code = loop {
            let mut instruction_count = 0; // used to step a single instruction or range
            match debugger.poll(&mut target)? {
                Some(MachineTargetAction::Stop) => {
                    state = MachineState::Stopped;
                }
                Some(MachineTargetAction::Resume) => {
                    state = MachineState::Running;
                }
                Some(MachineTargetAction::SingleStep) => {
                    instruction_count = 1;
                    state = MachineState::Running;
                }
                Some(MachineTargetAction::Exit) => {
                    break ExitCode::SUCCESS;
                }
                None => {}
            }
            state = match state {
                MachineState::Running => {
                    match target.machine.run(instruction_count) {
                        win32::StopReason::None => {
                            if instruction_count > 0 {
                                debugger.done_step(&mut target)?;
                                MachineState::Stopped
                            } else {
                                MachineState::Running
                            }
                        }
                        win32::StopReason::Blocked => {
                            // Poll the last future.
                            let shim_call = shim_calls.last_mut().unwrap();
                            match shim_call.future.as_mut().poll(&mut ctx) {
                                Poll::Ready(ret) => {
                                    target.machine.finish_shim_call(shim_call.shim, ret);
                                    shim_calls.pop();
                                }
                                Poll::Pending => {}
                            }
                            // TODO: handle single stepping
                            MachineState::Running
                        }
                        win32::StopReason::Error {
                            message,
                            signal,
                            eip,
                        } => {
                            log::error!("Machine error @ {eip:x}: {message} (signal {signal})");
                            print_trace(&target.machine);
                            debugger.machine_error(&mut target, signal)?;
                            MachineState::Stopped
                        }
                        win32::StopReason::Exit { code } => {
                            break ExitCode::from(code.try_into().unwrap_or(u8::MAX));
                        }
                        win32::StopReason::Breakpoint { eip } => {
                            log::info!("Breakpoint hit @ {eip:x}");
                            debugger.breakpoint(&mut target)?;
                            MachineState::Stopped
                        }
                        win32::StopReason::ShimCall(shim) => {
                            // log::info!("Shim call {}", shim.name);
                            if let Some(future) = target.machine.call_shim(shim) {
                                shim_calls.push(AsyncShimCall { shim, future });
                            }
                            // TODO: handle single stepping
                            MachineState::Running
                        }
                    }
                }
                MachineState::Stopped => {
                    debugger.stopped(&mut target)?;
                    MachineState::Stopped
                }
            };
        };

        #[cfg(feature = "x86-emu")]
        {
            let millis = start.elapsed().as_millis() as u64;
            if millis > 0 {
                eprintln!(
                    "{} instrs in {} ms: {}m/s",
                    target.machine.emu.x86.instr_count,
                    millis,
                    (target.machine.emu.x86.instr_count / millis) / 1000
                );
                eprintln!("icache: {}", target.machine.emu.x86.icache.stats());
            }
        }

        exit_code
    };

    Ok(exit_code)
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
