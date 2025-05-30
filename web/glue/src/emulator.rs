use crate::{debugger, host::JsHost};
use wasm_bindgen::prelude::*;

type JsResult<T> = Result<T, JsError>;

#[wasm_bindgen]
pub struct Emulator {
    machine: win32::Machine,
}

#[wasm_bindgen]
pub enum Status {
    Running,
    Blocked,
    Error,
    DebugBreak,
    Exit,
}

#[wasm_bindgen]
impl Emulator {
    pub fn set_external_dlls(&mut self, dlls: Vec<String>) {
        self.machine.set_external_dlls(dlls);
    }

    pub fn start_exe(&mut self, cmdline: String, relocate: bool) {
        self.machine
            .start_exe(cmdline, if relocate { Some(None) } else { None });
    }

    pub fn labels(&self) -> JsResult<String> {
        let str = serde_json::to_string(&self.machine.memory.labels)?;
        Ok(str)
    }

    pub fn memory(&self) -> js_sys::DataView {
        let mem = js_sys::WebAssembly::Memory::from(wasm_bindgen::memory());
        let buf = js_sys::ArrayBuffer::from(mem.buffer());
        let ofs = self.machine.memory.imp.as_ptr() as usize;
        js_sys::DataView::new(&buf, ofs, buf.byte_length() as usize - ofs)
    }

    pub fn cpus(&mut self) -> Box<[debugger::CPU]> {
        self.machine
            .emu
            .x86
            .cpus
            .iter_mut()
            .map(|cpu| debugger::CPU::from(unsafe { cpu.as_mut().get_unchecked_mut() }))
            .collect()
    }

    pub fn cpu(&mut self) -> debugger::CPU {
        debugger::CPU::from(self.machine.emu.x86.cpu_mut())
    }

    #[wasm_bindgen(getter)]
    pub fn exit_code(&self) -> u32 {
        match self.machine.status {
            win32::Status::Exit(code) => code,
            _ => 0,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn instr_count(&self) -> usize {
        self.machine.emu.x86.instr_count
    }

    pub fn disassemble_json(&self, addr: u32, limit: usize) -> String {
        serde_json::to_string(&x86::debug::disassemble(self.machine.mem(), addr, limit))
            .unwrap_throw()
    }

    pub fn unblock(&mut self) {
        self.machine.unblock_all();
    }

    /// Run code until at least count instructions have run.
    /// This exists to avoid many round-trips from JS to Rust in the execution loop.
    pub fn run(&mut self, count: usize) -> JsResult<Status> {
        if count == 1 {
            self.machine.single_step();
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

        Ok(match &self.machine.status {
            win32::Status::Running => Status::Running,
            win32::Status::Blocked => Status::Blocked,
            win32::Status::Error { message } => return Err(JsError::new(message)),
            win32::Status::DebugBreak => Status::DebugBreak,
            win32::Status::Exit(_code) => {
                // TODO: use exit code
                Status::Exit
            }
        })
    }

    pub fn breakpoint_add(&mut self, addr: u32) {
        self.machine.add_breakpoint(addr);
    }
    pub fn breakpoint_clear(&mut self, addr: u32) {
        self.machine.clear_breakpoint(addr);
    }

    pub fn mappings_json(&self) -> String {
        serde_json::to_string(&self.machine.memory.mappings.vec()).unwrap_throw()
    }

    pub fn set_tracing_scheme(&self, scheme: &str) {
        win32::trace::set_scheme(scheme);
    }

    pub fn direct_draw_surfaces(&self) -> Vec<JsValue> {
        debugger::surfaces_from_machine(&self.machine)
    }
}

static mut JS_HOST: Option<JsHost> = None;

#[allow(static_mut_refs)]
pub fn js_host() -> &'static JsHost {
    unsafe { JS_HOST.as_ref().unwrap() }
}

fn win32_trace(r: &win32::trace::Record) {
    js_host().win32_trace(r.context, &r.msg);
}

#[wasm_bindgen]
pub fn new_emulator(host: JsHost) -> Emulator {
    unsafe { JS_HOST = Some(host.clone().unchecked_into()) };
    crate::log::init();
    win32::trace::set_output(win32_trace);
    let mut machine = win32::Machine::new(Box::new(host));
    machine.set_audio(true);
    Emulator { machine }
}
