mod host;
mod log;

use wasm_bindgen::prelude::*;

use crate::host::JsHost;

#[wasm_bindgen(
    inline_js = "export function mem(memory, offset) { return new DataView(memory.buffer, offset); }"
)]
extern "C" {
    fn mem(mem: JsValue, offset: u32) -> JsValue;
}

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
    pub fn load_exe(&mut self, name: String, buf: &[u8]) -> JsResult<()> {
        self.runner.load_exe(buf, name).map_err(err_from_anyhow)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn labels(&self) -> JsResult<String> {
        let str = serde_json::to_string(&self.runner.machine.labels)?;
        Ok(str)
    }

    pub fn memory(&self) -> js_sys::DataView {
        js_sys::DataView::from(mem(
            wasm_bindgen::memory(),
            self.runner.machine.x86.mem.as_ptr() as u32,
        ))
    }

    #[wasm_bindgen(getter)]
    pub fn eax(&self) -> u32 {
        self.runner.machine.x86.regs.eax
    }
    #[wasm_bindgen(getter)]
    pub fn ebx(&self) -> u32 {
        self.runner.machine.x86.regs.ebx
    }
    #[wasm_bindgen(getter)]
    pub fn ecx(&self) -> u32 {
        self.runner.machine.x86.regs.ecx
    }
    #[wasm_bindgen(getter)]
    pub fn edx(&self) -> u32 {
        self.runner.machine.x86.regs.edx
    }

    #[wasm_bindgen(getter)]
    pub fn esp(&self) -> u32 {
        self.runner.machine.x86.regs.esp
    }
    #[wasm_bindgen(getter)]
    pub fn ebp(&self) -> u32 {
        self.runner.machine.x86.regs.ebp
    }
    #[wasm_bindgen(getter)]
    pub fn esi(&self) -> u32 {
        self.runner.machine.x86.regs.esi
    }
    #[wasm_bindgen(getter)]
    pub fn edi(&self) -> u32 {
        self.runner.machine.x86.regs.edi
    }

    #[wasm_bindgen(getter)]
    pub fn eip(&self) -> u32 {
        self.runner.machine.x86.regs.eip
    }

    #[wasm_bindgen(getter)]
    pub fn cs(&self) -> u16 {
        self.runner.machine.x86.regs.cs
    }
    #[wasm_bindgen(getter)]
    pub fn ds(&self) -> u16 {
        self.runner.machine.x86.regs.ds
    }
    #[wasm_bindgen(getter)]
    pub fn es(&self) -> u16 {
        self.runner.machine.x86.regs.es
    }
    #[wasm_bindgen(getter)]
    pub fn fs(&self) -> u16 {
        self.runner.machine.x86.regs.fs
    }
    #[wasm_bindgen(getter)]
    pub fn gs(&self) -> u16 {
        self.runner.machine.x86.regs.gs
    }
    #[wasm_bindgen(getter)]
    pub fn ss(&self) -> u16 {
        self.runner.machine.x86.regs.ss
    }
    #[wasm_bindgen(getter)]
    pub fn flags(&self) -> u32 {
        self.runner.machine.x86.flags.bits()
    }
    pub fn flags_str(&self) -> String {
        format!("{:?}", self.runner.machine.x86.flags)
    }

    pub fn st(&self) -> Box<[f64]> {
        let s = &self.runner.machine.x86.regs.st[self.runner.machine.x86.regs.st_top..];
        s.into()
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
        self.runner.machine.x86.mem[addr as usize] = value;
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
