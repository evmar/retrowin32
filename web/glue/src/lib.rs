mod debugger;
mod host;
mod log;

use crate::host::JsHost;
use wasm_bindgen::prelude::*;

pub type JsResult<T> = Result<T, JsError>;
fn err_from_anyhow(err: anyhow::Error) -> JsError {
    JsError::new(&err.to_string())
}

#[wasm_bindgen]
pub struct Emulator {
    machine: win32::Machine,
}

#[wasm_bindgen]
pub enum CPUState {
    Running,
    Blocked,
    Error,
    Exit,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen]
    pub fn load_exe(&mut self, name: &str, buf: &[u8], relocate: bool) -> JsResult<()> {
        self.machine
            .load_exe(buf, name, relocate)
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
        let ofs = self.machine.mem().as_slice_todo().as_ptr() as usize;
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
        self.machine.unblock();
    }

    /// Run code until at least count instructions have run.
    /// This exists to avoid many round-trips from JS to Rust in the execution loop.
    pub fn run(&mut self, count: usize) -> JsResult<CPUState> {
        if count == 1 {
            self.machine.single_step_next_block();
            self.machine.run();
        } else {
            // Note that instr_count overflows at 4b, but we don't expect to run
            // 4b instructions in a single run() invocation.
            let start = self.machine.emu.x86.instr_count;
            while self.machine.emu.x86.instr_count.wrapping_sub(start) < count {
                if !self.machine.run() {
                    break;
                }
            }
        }

        Ok(match &self.machine.emu.x86.cpu().state {
            x86::CPUState::Running => CPUState::Running,
            x86::CPUState::Blocked(_) => CPUState::Blocked,
            x86::CPUState::Error(msg) => return Err(JsError::new(msg)),
            x86::CPUState::Exit(_) => CPUState::Exit,
        })
    }

    pub fn breakpoint_add(&mut self, addr: u32) {
        self.machine
            .emu
            .x86
            .add_breakpoint(self.machine.emu.memory.mem(), addr)
    }
    pub fn breakpoint_clear(&mut self, addr: u32) {
        self.machine
            .emu
            .x86
            .clear_breakpoint(self.machine.emu.memory.mem(), addr)
    }

    pub fn mappings_json(&self) -> String {
        serde_json::to_string(&self.machine.state.kernel32.mappings.vec()).unwrap_throw()
    }

    pub fn poke(&mut self, addr: u32, value: u8) {
        *self.machine.mem().view_mut::<u8>(addr) = value;
    }

    pub fn snapshot(&self) -> Box<[u8]> {
        self.machine.snapshot()
    }
    pub fn load_snapshot(&mut self, bytes: &[u8]) {
        self.machine.load_snapshot(bytes)
    }

    pub fn set_tracing_scheme(&self, scheme: &str) {
        win32::trace::set_scheme(scheme);
    }
}

#[wasm_bindgen]
pub fn new_emulator(host: JsHost, cmdline: String) -> Emulator {
    log::init(log::JsLogger::unchecked_from_js(host.clone()));
    let machine = win32::Machine::new(Box::new(host), cmdline);
    Emulator { machine }
}
