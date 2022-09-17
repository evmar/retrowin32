
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
        self.x86.mem[addr..addr+len].into()
    }
}

#[wasm_bindgen]
pub fn load_exe(buf: &[u8]) -> Result<X86, String> {
    let mut x86 = win32::X86::new();
    win32::load_exe(&mut x86, buf).map_err(|err| err.to_string())?;
    Ok(X86 { x86 } )
}
