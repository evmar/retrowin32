use wasm_bindgen::prelude::*;

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

    #[wasm_bindgen(getter)]
    pub fn eax(&self) -> u32 {
        self.x86.regs.eax
    }
    #[wasm_bindgen(getter)]
    pub fn ebx(&self) -> u32 {
        self.x86.regs.ebx
    }
    #[wasm_bindgen(getter)]
    pub fn ecx(&self) -> u32 {
        self.x86.regs.ecx
    }
    #[wasm_bindgen(getter)]
    pub fn edx(&self) -> u32 {
        self.x86.regs.edx
    }

    #[wasm_bindgen(getter)]
    pub fn esp(&self) -> u32 {
        self.x86.regs.esp
    }
    #[wasm_bindgen(getter)]
    pub fn ebp(&self) -> u32 {
        self.x86.regs.ebp
    }
    #[wasm_bindgen(getter)]
    pub fn esi(&self) -> u32 {
        self.x86.regs.esi
    }
    #[wasm_bindgen(getter)]
    pub fn edi(&self) -> u32 {
        self.x86.regs.edi
    }

    #[wasm_bindgen(getter)]
    pub fn eip(&self) -> u32 {
        self.x86.regs.eip
    }

    #[wasm_bindgen(getter)]
    pub fn cs(&self) -> u16 {
        self.x86.regs.cs
    }
    #[wasm_bindgen(getter)]
    pub fn ds(&self) -> u16 {
        self.x86.regs.ds
    }
    #[wasm_bindgen(getter)]
    pub fn es(&self) -> u16 {
        self.x86.regs.es
    }
    #[wasm_bindgen(getter)]
    pub fn fs(&self) -> u16 {
        self.x86.regs.fs
    }
    #[wasm_bindgen(getter)]
    pub fn gs(&self) -> u16 {
        self.x86.regs.gs
    }
    #[wasm_bindgen(getter)]
    pub fn ss(&self) -> u16 {
        self.x86.regs.ss
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

fn panic_hook(info: &std::panic::PanicInfo) {
    log::error!("{}", info);
}

#[wasm_bindgen]
pub fn init_logging() {
    console_log::init().unwrap();
    std::panic::set_hook(Box::new(panic_hook));
}
