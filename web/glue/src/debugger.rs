//! API used specifically for debugging the emulator.

use tsify::{JsValueSerdeExt, Tsify};
use wasm_bindgen::prelude::*;

use crate::host::WebSurface;

/// Registers are serialized as a JSON blob.
#[derive(Tsify, serde::Serialize)]
#[tsify(into_wasm_abi)]
pub struct Registers {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    esp: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
    eip: u32,
    cs: u16,
    ds: u16,
    es: u16,
    fs: u16,
    gs: u16,
    ss: u16,
    flags: u32,
    flags_str: String,
    st: Box<[f64]>,
}

impl Registers {
    pub fn from_x86(x86: &x86::CPU) -> Registers {
        Registers {
            eax: x86.regs.get32(x86::Register::EAX),
            ebx: x86.regs.get32(x86::Register::EBX),
            ecx: x86.regs.get32(x86::Register::ECX),
            edx: x86.regs.get32(x86::Register::EDX),
            esp: x86.regs.get32(x86::Register::ESP),
            ebp: x86.regs.get32(x86::Register::EBP),
            esi: x86.regs.get32(x86::Register::ESI),
            edi: x86.regs.get32(x86::Register::EDI),
            eip: x86.regs.eip,
            cs: x86.regs.get16(x86::Register::CS),
            ds: x86.regs.get16(x86::Register::DS),
            es: x86.regs.get16(x86::Register::ES),
            fs: x86.regs.get16(x86::Register::FS),
            gs: x86.regs.get16(x86::Register::GS),
            ss: x86.regs.get16(x86::Register::SS),
            flags: x86.flags.bits(),
            flags_str: format!("{:?}", x86.flags),
            st: x86.fpu.st[x86.fpu.st_top..].into(),
        }
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
