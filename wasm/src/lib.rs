use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(
    inline_js = "export function mem(memory, offset) { return new DataView(memory.buffer, offset); }"
)]
extern "C" {
    fn mem(mem: JsValue, offset: u32) -> JsValue;
}

#[wasm_bindgen]
pub struct X86 {
    x86: win32::X86,
}

#[wasm_bindgen]
impl X86 {
    pub fn memory(&self) -> js_sys::DataView {
        js_sys::DataView::from(mem(wasm_bindgen::memory(), self.x86.mem.as_ptr() as u32))
    }

    pub fn regs_json(&self) -> String {
        serde_json::to_string(&self.x86.regs).unwrap_throw()
    }

    pub fn disassemble_json(&self, addr: u32) -> String {
        serde_json::to_string(&win32::disassemble(&self.x86.mem, addr)).unwrap_throw()
    }

    pub fn step(&mut self) -> Result<(), String> {
        self.x86.step().map_err(|err| err.to_string())
    }
}

#[wasm_bindgen]
pub fn load_exe(buf: &[u8]) -> Result<X86, String> {
    let mut x86 = win32::X86::new();
    win32::load_exe(&mut x86, buf).map_err(|err| err.to_string())?;
    Ok(X86 { x86 })
}
