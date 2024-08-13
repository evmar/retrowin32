mod debugger;
mod host;
mod log;

use crate::host::JsHost;
use std::task::{Context, Poll};
use wasm_bindgen::prelude::*;

pub type JsResult<T> = Result<T, JsError>;
fn err_from_anyhow(err: anyhow::Error) -> JsError {
    JsError::new(&err.to_string())
}

struct AsyncShimCall {
    shim: &'static win32::shims::Shim,
    future: win32::shims::BoxFuture<u32>,
}

#[wasm_bindgen]
pub struct Emulator {
    machine: win32::Machine,
    shim_calls: Vec<AsyncShimCall>,
    ctx: Context<'static>,
}

#[wasm_bindgen]
pub enum CPUState {
    Running,
    Blocked,
    Error,
    DebugBreak,
    Exit,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen]
    pub fn load_exe(&mut self, name: &str, buf: &[u8], relocate: bool) -> JsResult<()> {
        self.machine
            .load_exe(buf, name, if relocate { Some(None) } else { None })
            .map_err(err_from_anyhow)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn labels(&self) -> JsResult<String> {
        let str = serde_json::to_string(&self.machine.labels)?;
        Ok(str)
    }

    pub fn memory(&self) -> js_sys::DataView {
        let mem = js_sys::WebAssembly::Memory::from(wasm_bindgen::memory());
        let buf = js_sys::ArrayBuffer::from(mem.buffer());
        let ofs = self.machine.emu.memory.as_ptr() as usize;
        js_sys::DataView::new(&buf, ofs, buf.byte_length() as usize - ofs)
    }

    #[wasm_bindgen(getter)]
    pub fn esp(&self) -> u32 {
        self.machine.emu.x86.cpu().regs.get32(x86::Register::ESP)
    }

    #[wasm_bindgen(getter)]
    pub fn eip(&self) -> u32 {
        self.machine.emu.x86.cpu().regs.eip
    }

    pub fn regs(&self) -> debugger::Registers {
        debugger::Registers::from_x86(&self.machine.emu.x86.cpu())
    }

    #[wasm_bindgen(getter)]
    pub fn instr_count(&self) -> usize {
        self.machine.emu.x86.instr_count
    }

    pub fn disassemble_json(&self, addr: u32, limit: usize) -> String {
        serde_json::to_string(&win32::disassemble(self.machine.mem(), addr, limit)).unwrap_throw()
    }

    pub fn unblock(&mut self) {
        self.machine.unblock_all();
    }

    /// Run code until at least count instructions have run.
    /// This exists to avoid many round-trips from JS to Rust in the execution loop.
    pub fn run(&mut self, count: usize) -> JsResult<CPUState> {
        if count == 1 {
            return match self.machine.run(1) {
                win32::StopReason::None => Ok(CPUState::Running),
                win32::StopReason::Blocked => {
                    // Poll the last future.
                    let shim_call = self.shim_calls.last_mut().unwrap();
                    match shim_call.future.as_mut().poll(&mut self.ctx) {
                        Poll::Ready(ret) => {
                            self.machine.finish_shim_call(shim_call.shim, ret);
                            self.shim_calls.pop();
                        }
                        Poll::Pending => {}
                    }
                    Ok(CPUState::Running)
                }
                win32::StopReason::Breakpoint { .. } => Ok(CPUState::DebugBreak),
                win32::StopReason::ShimCall(shim) => {
                    if let Some(future) = self.machine.call_shim(shim) {
                        self.shim_calls.push(AsyncShimCall { shim, future });
                    }
                    Ok(CPUState::Running)
                }
                win32::StopReason::Error { message, .. } => Err(JsError::new(&message)),
                win32::StopReason::Exit { .. } => Ok(CPUState::Exit),
            };
        } else {
            // Note that instr_count overflows at 4b, but we don't expect to run
            // 4b instructions in a single run() invocation.
            let start = self.machine.emu.x86.instr_count;
            while self.machine.emu.x86.instr_count.wrapping_sub(start) < count {
                match self.machine.run(0) {
                    win32::StopReason::None => {}
                    win32::StopReason::Blocked => {
                        // Poll the last future.
                        let shim_call = self.shim_calls.last_mut().unwrap();
                        match shim_call.future.as_mut().poll(&mut self.ctx) {
                            Poll::Ready(ret) => {
                                self.machine.finish_shim_call(shim_call.shim, ret);
                                self.shim_calls.pop();
                                // Continue running after the shim call completes.
                            }
                            Poll::Pending => {
                                // Return control to the event loop to allow the future to progress.
                                break;
                            }
                        }
                    }
                    win32::StopReason::Breakpoint { .. } => return Ok(CPUState::DebugBreak),
                    win32::StopReason::ShimCall(shim) => {
                        if let Some(future) = self.machine.call_shim(shim) {
                            self.shim_calls.push(AsyncShimCall { shim, future });
                            // Return control to the event loop to allow the future to progress.
                            break;
                        }
                    }
                    win32::StopReason::Error { message, .. } => return Err(JsError::new(&message)),
                    win32::StopReason::Exit { .. } => return Ok(CPUState::Exit),
                }
            }
            Ok(match &self.machine.emu.x86.cpu().state {
                x86::CPUState::Running => CPUState::Running,
                x86::CPUState::Blocked(_) => CPUState::Blocked,
                x86::CPUState::Error(msg) => return Err(JsError::new(msg)),
                x86::CPUState::DebugBreak => CPUState::DebugBreak,
                x86::CPUState::Exit(_) => CPUState::Exit,
            })
        }
    }

    pub fn breakpoint_add(&mut self, addr: u32) -> bool {
        self.machine.add_breakpoint(addr)
    }
    pub fn breakpoint_clear(&mut self, addr: u32) -> bool {
        self.machine.clear_breakpoint(addr)
    }

    pub fn mappings_json(&self) -> String {
        serde_json::to_string(&self.machine.state.kernel32.mappings.vec()).unwrap_throw()
    }

    pub fn poke(&mut self, addr: u32, value: u8) {
        *self.machine.mem().view_mut::<u8>(addr) = value;
    }

    pub fn set_tracing_scheme(&self, scheme: &str) {
        win32::trace::set_scheme(scheme);
    }
}

#[wasm_bindgen]
pub fn new_emulator(host: JsHost, cmdline: String) -> Emulator {
    log::init(log::JsLogger::unchecked_from_js(host.clone()));
    let machine = win32::Machine::new(Box::new(host), cmdline);
    Emulator {
        machine,
        shim_calls: Default::default(),
        ctx: Context::from_waker(futures::task::noop_waker_ref()),
    }
}
