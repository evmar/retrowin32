mod debugger;
mod host;
mod log;

use crate::host::JsHost;
use wasm_bindgen::prelude::*;
use x86::Memory;

pub type JsResult<T> = Result<T, JsError>;
fn err_from_anyhow(err: anyhow::Error) -> JsError {
    JsError::new(&err.to_string())
}

#[wasm_bindgen]
pub struct Emulator {
    runner: win32::Runner,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen]
    pub fn load_exe(&mut self, name: String, buf: &[u8], relocate: bool) -> JsResult<()> {
        self.runner
            .load_exe(buf, name, relocate)
            .map_err(err_from_anyhow)
    }

    #[wasm_bindgen]
    pub fn labels(&self) -> JsResult<String> {
        let str = serde_json::to_string(&self.runner.machine.labels)?;
        Ok(str)
    }

    pub fn memory(&self) -> js_sys::DataView {
        let mem = js_sys::WebAssembly::Memory::from(wasm_bindgen::memory());
        let buf = js_sys::ArrayBuffer::from(mem.buffer());
        let ofs = self.runner.machine.x86.mem.as_slice_todo().as_ptr() as usize;
        js_sys::DataView::new(&buf, ofs, buf.byte_length() as usize - ofs)
    }

    #[wasm_bindgen(getter)]
    pub fn esp(&self) -> u32 {
        self.runner.machine.x86.regs.esp
    }

    #[wasm_bindgen(getter)]
    pub fn eip(&self) -> u32 {
        self.runner.machine.x86.regs.eip
    }

    pub fn regs(&self) -> debugger::Registers {
        debugger::Registers::from_x86(&self.runner.machine.x86)
    }

    #[wasm_bindgen(getter)]
    pub fn instr_count(&self) -> usize {
        self.runner.instr_count
    }

    pub fn disassemble_json(&self, addr: u32) -> String {
        serde_json::to_string(&win32::disassemble(&self.runner.machine.x86.mem, addr))
            .unwrap_throw()
    }

    pub fn single_step(&mut self) -> JsResult<()> {
        self.runner.single_step().map_err(err_from_anyhow)?;
        Ok(())
    }

    /// Execute multiple basic blocks until at least count instructions have run.
    /// This exists to avoid many round-trips from JS to Rust in the execution loop.
    pub fn execute_many(&mut self, count: usize) -> JsResult<usize> {
        let start = self.runner.instr_count;
        while self.runner.instr_count < start + count {
            if !self.runner.execute_block().map_err(err_from_anyhow)? {
                break;
            }
        }
        Ok(self.runner.instr_count - start)
    }

    pub fn breakpoint_add(&mut self, addr: u32) {
        self.runner.add_breakpoint(addr)
    }
    pub fn breakpoint_clear(&mut self, addr: u32) {
        self.runner.clear_breakpoint(addr)
    }

    pub fn mappings_json(&self) -> String {
        serde_json::to_string(&self.runner.machine.state.kernel32.mappings.vec()).unwrap_throw()
    }

    pub fn poke(&mut self, addr: u32, value: u8) {
        *self.runner.machine.x86.mem.view_mut::<u8>(addr) = value;
    }

    pub fn snapshot(&self) -> Box<[u8]> {
        bincode::serialize(&self.runner.machine.x86).unwrap().into()
    }
    pub fn load_snapshot(&mut self, bytes: &[u8]) {
        let snap = bincode::deserialize(bytes).unwrap();
        self.runner.load_snapshot(snap);
    }
}

#[wasm_bindgen]
pub fn new_emulator(host: JsHost) -> JsResult<Emulator> {
    log::init(log::JsLogger::unchecked_from_js(host.clone()))?;
    win32::trace::set_scheme("-kernel32");
    let runner = win32::Runner::new(Box::new(host));
    Ok(Emulator { runner })
}
