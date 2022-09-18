use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct X86 {
    x86: win32::X86,
}

#[wasm_bindgen]
impl X86 {
    pub fn mem(&self, addr: u32, len: u32) -> Box<[u8]> {
        let addr = addr as usize;
        let len = len as usize;
        self.x86.mem[addr..addr + len].into()
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
