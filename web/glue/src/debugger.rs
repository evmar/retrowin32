//! API used specifically for debugging the emulator.

use tsify::{JsValueSerdeExt, Tsify};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Register {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
    CS,
    DS,
    ES,
    FS,
    GS,
    SS,
}

#[wasm_bindgen]
pub struct CPU {
    // It's gross to use a raw pointer here, but in practice CPUs are Pin<>ed and
    // you only access one of these off an Emulator that is keeping things alive.
    cpu: *mut x86::CPU,
}

impl From<&mut x86::CPU> for CPU {
    fn from(cpu: &mut x86::CPU) -> Self {
        Self { cpu }
    }
}

#[wasm_bindgen]
impl CPU {
    pub fn state(&self) -> String {
        let cpu = unsafe { &mut *self.cpu };
        format!("{:?}", cpu.state)
    }

    #[wasm_bindgen(getter)]
    pub fn eip(&self) -> u32 {
        let cpu = unsafe { &mut *self.cpu };
        cpu.regs.eip
    }
    pub fn jmp(&mut self, _eip: u32) {
        let _cpu = unsafe { &mut *self.cpu };
        todo!(); //cpu.jmp(eip);
    }

    pub fn get(&self, reg: Register) -> u32 {
        let cpu = unsafe { &mut *self.cpu };
        match reg {
            Register::EAX => cpu.regs.get32(x86::Register::EAX),
            Register::ECX => cpu.regs.get32(x86::Register::ECX),
            Register::EDX => cpu.regs.get32(x86::Register::EDX),
            Register::EBX => cpu.regs.get32(x86::Register::EBX),
            Register::ESP => cpu.regs.get32(x86::Register::ESP),
            Register::EBP => cpu.regs.get32(x86::Register::EBP),
            Register::ESI => cpu.regs.get32(x86::Register::ESI),
            Register::EDI => cpu.regs.get32(x86::Register::EDI),
            Register::CS => cpu.regs.get16(x86::Register::CS) as u32,
            Register::DS => cpu.regs.get16(x86::Register::DS) as u32,
            Register::ES => cpu.regs.get16(x86::Register::ES) as u32,
            Register::FS => cpu.regs.get16(x86::Register::FS) as u32,
            Register::GS => cpu.regs.get16(x86::Register::GS) as u32,
            Register::SS => cpu.regs.get16(x86::Register::SS) as u32,
        }
    }

    pub fn set(&self, reg: Register, value: u32) {
        let cpu = unsafe { &mut *self.cpu };
        match reg {
            Register::EAX => cpu.regs.set32(x86::Register::EAX, value),
            Register::ECX => cpu.regs.set32(x86::Register::ECX, value),
            Register::EDX => cpu.regs.set32(x86::Register::EDX, value),
            Register::EBX => cpu.regs.set32(x86::Register::EBX, value),
            Register::ESP => cpu.regs.set32(x86::Register::ESP, value),
            Register::EBP => cpu.regs.set32(x86::Register::EBP, value),
            Register::ESI => cpu.regs.set32(x86::Register::ESI, value),
            Register::EDI => cpu.regs.set32(x86::Register::EDI, value),
            Register::CS => cpu.regs.set16(x86::Register::CS, value as u16),
            Register::DS => cpu.regs.set16(x86::Register::DS, value as u16),
            Register::ES => cpu.regs.set16(x86::Register::ES, value as u16),
            Register::FS => cpu.regs.set16(x86::Register::FS, value as u16),
            Register::GS => cpu.regs.set16(x86::Register::GS, value as u16),
            Register::SS => cpu.regs.set16(x86::Register::SS, value as u16),
        }
    }

    pub fn flags(&self) -> u32 {
        let cpu = unsafe { &mut *self.cpu };
        cpu.flags.bits()
    }
    pub fn flags_str(&self) -> String {
        let cpu = unsafe { &mut *self.cpu };
        format!("{:?}", cpu.flags)
    }

    pub fn st(&self) -> Box<[f64]> {
        let cpu = unsafe { &mut *self.cpu };
        cpu.fpu.st[cpu.fpu.st_top..].iter().copied().collect()
    }
}

#[wasm_bindgen(typescript_custom_section)]
const SURFACE_META_TS: &'static str = r#"
export type SurfaceDebug = DirectDrawSurfaceMeta & { canvas: HTMLCanvasElement };
"#;

#[derive(Tsify, serde::Serialize)]
pub struct DirectDrawSurfaceMeta {
    pub ptr: u32,
    pub width: u32,
    pub height: u32,
    pub bytes_per_pixel: u32,
    pub primary: bool,
    pub pixels: Option<u32>,
    pub palette: Option<u32>,
    // TODO:
    // pub attached: u32,
}

pub fn surfaces_from_machine(machine: &win32::Machine) -> Vec<JsValue> {
    machine
        .state
        .ddraw
        .surfaces
        .iter()
        .map(|(&ptr, s)| {
            let meta = DirectDrawSurfaceMeta {
                ptr,
                width: s.width,
                height: s.height,
                bytes_per_pixel: s.bytes_per_pixel,
                primary: s.primary,
                pixels: if s.pixels != 0 { Some(s.pixels) } else { None },
                palette: s.palette.as_ref().map(|p| p.ptr),
            };

            // Attach canvas property to JS object we create from meta.
            let val = JsValue::from_serde(&meta).unwrap();
            let web_surface =
                unsafe { &*(s.host.as_ref() as *const dyn win32::Surface as *const WebSurface) };
            js_sys::Reflect::set(&val, &"canvas".into(), &web_surface.canvas.clone().into())
                .unwrap();

            val
        })
        .collect()
}
