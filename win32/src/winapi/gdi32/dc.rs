use super::*;
use crate::{
    machine::Machine,
    winapi::bitmap::{Bitmap, PixelData, PixelFormat},
};

const TRACE_CONTEXT: &'static str = "gdi32/dc";

pub type HDC = HANDLE<DC>;

/// Target device for a DC.
#[derive(Debug)]
pub enum DCTarget {
    Memory(HGDIOBJ), // aka Bitmap
    Window(HWND),
    DirectDrawSurface(u32),
}

#[derive(Debug)]
pub struct DC {
    // TODO: it's unclear to me what the representation of a DC ought to be.
    // DirectDraw can also create a DC, and DirectDraw (as a DLL that came
    // later) can't retrofit the DC type with a DirectDraw field.
    // Wine appears to use a vtable (for generic behavior).
    pub target: DCTarget,

    pub r2: R2,
    pub x: u32,
    pub y: u32,

    // The SelectObject() API sets a drawing-related field on the DC and returns the
    // previously selected object of a given type, which means we need a storage field
    // per object type.
    pub brush: HGDIOBJ,
    pub pen: HGDIOBJ,
}

impl DC {
    pub fn new(target: DCTarget) -> Self {
        DC {
            target,
            r2: R2::default(),
            x: 0,
            y: 0,
            brush: Default::default(),
            pen: Default::default(),
        }
    }

    pub fn new_memory(machine: &mut Machine) -> Self {
        // MSDN says: "When a memory device context is created, it initially has a 1-by-1 monochrome bitmap selected into it."
        // SkiFree depends on this!
        let bitmap = Bitmap {
            width: 1,
            height: 1,
            format: PixelFormat::Mono,
            pixels: PixelData::Ptr(0),
        };
        let hobj = machine.state.gdi32.objects.add(Object::Bitmap(bitmap));
        Self::new(DCTarget::Memory(hobj))
    }
}

#[win32_derive::dllexport]
pub fn CreateCompatibleDC(machine: &mut Machine, hdc: HDC) -> HDC {
    let dc = DC::new_memory(machine);
    let handle = machine.state.gdi32.dcs.add(dc);
    handle
}

#[win32_derive::dllexport]
pub fn DeleteDC(_machine: &mut Machine, hdc: u32) -> u32 {
    log::warn!("todo: DeleteDC({hdc:x})");
    0 // fail
}
