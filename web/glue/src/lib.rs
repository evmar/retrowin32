use wasm_bindgen::prelude::*;

struct HostLogger {
    host: JsHost,
}
unsafe impl Send for HostLogger {}
unsafe impl Sync for HostLogger {}
impl log::Log for HostLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let level = match record.level() {
            log::Level::Error => 5,
            log::Level::Warn => 4,
            log::Level::Info => 3,
            log::Level::Debug => 2,
            log::Level::Trace => 1,
        };
        self.host.log(
            level,
            format!(
                "{}:{}: {}",
                record.file().unwrap_or(""),
                record.line().unwrap_or(0),
                record.args()
            ),
        );
    }

    fn flush(&self) {}
}

#[wasm_bindgen(
    inline_js = "export function mem(memory, offset) { return new DataView(memory.buffer, offset); }"
)]
extern "C" {
    fn mem(mem: JsValue, offset: u32) -> JsValue;
}

type JsResult<T> = Result<T, JsError>;
fn err_from_anyhow(err: anyhow::Error) -> JsError {
    JsError::new(&err.to_string())
}

#[wasm_bindgen]
extern "C" {
    pub type JsSurface;
    #[wasm_bindgen(method)]
    fn write_pixels(this: &JsSurface, pixels: &[u8]) -> JsSurface;
    #[wasm_bindgen(method)]
    fn get_attached(this: &JsSurface) -> JsSurface;
    #[wasm_bindgen(method)]
    fn flip(this: &JsSurface);
    #[wasm_bindgen(method)]
    fn bit_blt(
        this: &JsSurface,
        dx: u32,
        dy: u32,
        src: &JsSurface,
        sx: u32,
        sy: u32,
        w: u32,
        h: u32,
    );
}

impl win32::Surface for JsSurface {
    fn write_pixels(&mut self, pixels: &[[u8; 4]]) {
        let slice = unsafe {
            let p = pixels.as_ptr() as *const u8;
            std::slice::from_raw_parts(p, pixels.len() * 4)
        };
        JsSurface::write_pixels(self, slice);
    }

    fn get_attached(&self) -> Box<dyn win32::Surface> {
        Box::new(JsSurface::get_attached(self))
    }

    fn flip(&mut self) {
        JsSurface::flip(self);
    }

    fn bit_blt(
        &mut self,
        dx: u32,
        dy: u32,
        src: &dyn win32::Surface,
        sx: u32,
        sy: u32,
        w: u32,
        h: u32,
    ) {
        // Hack: we know all surfaces are JsSurface.
        // I think to fix this properly I might need to make every X86 generic across all the
        // host types, eek.
        let src = unsafe { &*(src as *const dyn win32::Surface as *const JsSurface) };
        JsSurface::bit_blt(self, dx, dy, src, sx, sy, w, h);
    }
}

#[wasm_bindgen]
extern "C" {
    pub type JsWindow;
    #[wasm_bindgen(method, setter)]
    fn set_title(this: &JsWindow, title: &str);
    #[wasm_bindgen(method)]
    fn set_size(this: &JsWindow, width: u32, height: u32);
}

impl win32::Window for JsWindow {
    fn set_title(&mut self, title: &str) {
        JsWindow::set_title(self, title);
    }
    fn set_size(&mut self, width: u32, height: u32) {
        JsWindow::set_size(self, width, height);
    }
}

#[wasm_bindgen]
extern "C" {
    pub type JsHost;

    #[wasm_bindgen(method)]
    fn log(this: &JsHost, level: u8, msg: String);

    #[wasm_bindgen(method)]
    fn exit(this: &JsHost, exit_code: u32);
    #[wasm_bindgen(method)]
    fn write(this: &JsHost, buf: &[u8]) -> usize;
    #[wasm_bindgen(method)]
    fn time(this: &JsHost) -> u32;
    #[wasm_bindgen(method)]
    fn create_window(this: &JsHost) -> JsWindow;
    #[wasm_bindgen(method)]
    fn create_surface(this: &JsHost, opts: win32::SurfaceOptions) -> JsSurface;
}

impl win32::Host for JsHost {
    fn exit(&mut self, exit_code: u32) {
        JsHost::exit(self, exit_code)
    }
    fn write(&self, buf: &[u8]) -> usize {
        JsHost::write(self, buf)
    }
    fn time(&self) -> u32 {
        JsHost::time(self)
    }
    fn create_window(&mut self) -> Box<dyn win32::Window> {
        let window = JsHost::create_window(self);
        window.set_title("test");
        Box::new(window)
    }
    fn create_surface(&mut self, opts: &win32::SurfaceOptions) -> Box<dyn win32::Surface> {
        Box::new(JsHost::create_surface(self, opts.clone()))
    }
}

#[wasm_bindgen]
pub struct Emulator {
    runner: win32::Runner,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen]
    pub fn load_exe(&mut self, buf: &[u8]) -> JsResult<String> {
        let imports = self
            .runner
            .load_exe(buf, "".into())
            .map_err(err_from_anyhow)?;
        let str = serde_json::to_string(&imports)?;
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

    pub fn step(&mut self) -> JsResult<()> {
        self.runner.step().map_err(err_from_anyhow)?;
        Ok(())
    }
    pub fn step_many(&mut self, count: usize) -> JsResult<usize> {
        self.runner.step_many(count).map_err(err_from_anyhow)
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
    init_logging(JsHost::from(host.clone()))?;
    let runner = win32::Runner::new(Box::new(host));
    Ok(Emulator { runner })
}

fn panic_hook(info: &std::panic::PanicInfo) {
    log::error!("{}", info);
}

pub fn init_logging(host: JsHost) -> JsResult<()> {
    let logger: &'static mut HostLogger = Box::leak(Box::new(HostLogger { host }));
    log::set_logger(logger).map_err(|err| JsError::new(&err.to_string()))?;
    log::set_max_level(log::LevelFilter::Debug);
    std::panic::set_hook(Box::new(panic_hook));
    Ok(())
}
