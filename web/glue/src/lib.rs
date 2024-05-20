mod debugger;
mod host;
mod log;

use crate::host::{FileHost, Host, JsHost, State};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use win32::Host as _;

pub type JsResult<T> = Result<T, JsError>;
fn err_from_anyhow(err: anyhow::Error) -> JsError {
    JsError::new(&err.to_string())
}

#[wasm_bindgen]
pub struct Emulator {
    machine: win32::Machine,
    state: Rc<RefCell<State>>,
    waker: web_sys::MessagePort,
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
    fn new(
        machine: win32::Machine,
        state: Rc<RefCell<State>>,
        waker: web_sys::MessagePort,
    ) -> Self {
        Self {
            machine,
            state,
            waker,
        }
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

    pub fn post_win32_message(&mut self, msg: win32::Message) {
        {
            let mut state = self.state.borrow_mut();
            ::log::info!("posted {:?}", msg);
            state.messages.push_back(msg);
            ::log::info!("msgs {}", state.messages.len());
        }
        self.unblock();
        self.waker.post_message(&JsValue::null()).unwrap();
    }

    /// Declared here so it shows up in TS type; actually implemented in worker/main.ts
    pub fn start(&mut self) {}

    /// Run code until at least count instructions have run.
    /// This exists to avoid many round-trips from JS to Rust in the execution loop.
    pub fn run(&mut self, count: usize) -> JsResult<CPUState> {
        if count == 1 {
            self.machine.single_step_next_block();
            self.machine.run();
        } else {
            let start = self.machine.emu.x86.instr_count;
            while self.machine.emu.x86.instr_count < start + count {
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
}

#[wasm_bindgen]
pub fn set_tracing_scheme(scheme: &str) {
    win32::trace::set_scheme(scheme);
}

#[wasm_bindgen]
pub fn new_emulator(
    js_host: JsHost,
    file_host: FileHost,
    cmdline: String,
    filename: &str,
    waker: web_sys::MessagePort,
) -> JsResult<Emulator> {
    win32::trace::set_scheme("*");
    log::init(js_host.clone().into());

    let host = Box::new(Host::new(js_host, file_host));
    let state = host.state.clone();
    let mut buf = Vec::new();
    host.open(filename).read_to_end(&mut buf).unwrap();
    let mut machine = win32::Machine::new(host, cmdline.clone());

    machine
        .load_exe(&buf, filename, false)
        .map_err(err_from_anyhow)?;

    Ok(Emulator::new(machine, state, waker))
}
