use wasm_bindgen::prelude::*;

#[wasm_bindgen(
    inline_js = "export function mem(memory, offset) { return new DataView(memory.buffer, offset); }"
)]
extern "C" {
    fn mem(mem: JsValue, offset: u32) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    pub type JsWindow;
    #[wasm_bindgen(method, getter)]
    fn id(this: &JsWindow) -> u32;
    #[wasm_bindgen(method, setter)]
    fn set_title(this: &JsWindow, title: &str);
}

impl win32::Window for JsWindow {
    fn id(&self) -> u32 {
        self.id()
    }
    fn set_title(&mut self, title: &str) {
        JsWindow::set_title(self, title);
    }
}

#[wasm_bindgen]
extern "C" {
    pub type JsHost;
    #[wasm_bindgen(method)]
    fn exit(this: &JsHost, exit_code: u32);
    #[wasm_bindgen(method)]
    fn write(this: &JsHost, buf: &[u8]) -> usize;
    #[wasm_bindgen(method)]
    fn time(this: &JsHost) -> u32;
    #[wasm_bindgen(method)]
    fn create_window(this: &JsHost) -> JsWindow;
    #[wasm_bindgen(method)]
    fn get_window(this: &JsHost, id: u32) -> JsWindow;
}

impl win32::Host for JsHost {
    fn exit(&self, exit_code: u32) {
        JsHost::exit(self, exit_code)
    }
    fn write(&self, buf: &[u8]) -> usize {
        JsHost::write(self, buf)
    }
    fn time(&self) -> u32 {
        JsHost::time(self)
    }
    fn create_window(&self) -> Box<dyn win32::Window> {
        let window = JsHost::create_window(self);
        window.set_title("test");
        Box::new(window)
    }
}

#[wasm_bindgen]
pub struct X86 {
    // Hack: x86 expects a host to outlive it, but I cannot figure out the lifetimes.
    // This member keeps host alive, and x86 holds a ref into it.
    #[allow(dead_code)]
    host: std::pin::Pin<Box<JsHost>>,
    x86: win32::X86<'static>,
}

#[wasm_bindgen]
impl X86 {
    #[wasm_bindgen]
    pub fn load_exe(&mut self, buf: &[u8]) -> Result<String, String> {
        let imports = win32::load_exe(&mut self.x86, buf).map_err(|err| err.to_string())?;
        serde_json::to_string(&imports).map_err(|err| err.to_string())
    }

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
    #[wasm_bindgen(getter)]
    pub fn flags(&self) -> u32 {
        self.x86.regs.flags.bits()
    }
    pub fn flags_str(&self) -> String {
        format!("{:?}", self.x86.regs.flags)
    }
    pub fn disassemble_json(&self, addr: u32) -> String {
        serde_json::to_string(&win32::disassemble(&self.x86.mem, addr)).unwrap_throw()
    }

    pub fn step(&mut self) -> Result<(), String> {
        self.x86.step().map_err(|err| err.to_string())
    }

    pub fn mappings_json(&self) -> String {
        serde_json::to_string(&self.x86.state.kernel32.mappings).unwrap_throw()
    }

    pub fn poke(&mut self, addr: u32, value: u8) {
        self.x86.mem[addr as usize] = value;
    }
}

#[wasm_bindgen]
pub fn new_x86(host: JsHost) -> Result<X86, String> {
    let host = Box::pin(host);
    let r = host.as_ref().get_ref();
    let static_host: &'static JsHost = unsafe { std::mem::transmute(r) };
    let x86 = win32::X86::new(static_host);
    Ok(X86 { host, x86 })
}

fn panic_hook(info: &std::panic::PanicInfo) {
    log::error!("{}", info);
}

#[wasm_bindgen(start)]
pub fn init_logging() -> Result<(), JsValue> {
    console_log::init().map_err(|err| err.to_string())?;
    std::panic::set_hook(Box::new(panic_hook));
    Ok(())
}
