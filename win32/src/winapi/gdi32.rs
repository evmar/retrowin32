#![allow(non_snake_case)]

use super::types::*;
use crate::{machine::Machine, winapi::user32};

const TRACE_CONTEXT: &'static str = "gdi32";

/// GDI Object, as identified by HANDLEs.
#[derive(Debug)]
pub enum Object {
    Bitmap(user32::Bitmap),
}

#[derive(Debug)]
pub struct DC {
    // TODO: it's unclear to me what the representation of a DC ought to be.
    // The SelectObject() API returns the previously selected object of a given
    // type, which implies that we need a storage field per object type.
    // But then DirectDraw can also create a DC, and DirectDraw (as a DLL that came
    // later can't retrofit the DC type with a DirectDraw field.
    // Wine appears to use a vtable (for generic behavior) *and* per-object-type fields.
    bitmap: u32, // HANDLE
    pub ddraw_surface: u32,
}
impl DC {
    fn new() -> Self {
        DC {
            bitmap: 0,
            ddraw_surface: 0,
        }
    }
}

pub struct State {
    dcs: Vec<DC>,
    pub objects: Vec<Object>,
}

impl State {
    pub fn new_dc(&mut self) -> (u32, &mut DC) {
        self.dcs.push(DC::new());
        let handle = self.dcs.len() as u32;
        (handle, &mut self.dcs[handle as usize - 1])
    }

    fn get_dc(&self, handle: u32) -> Option<&DC> {
        if handle > 0 {
            self.dcs.get((handle - 1) as usize)
        } else {
            None
        }
    }
    fn get_dc_mut(&mut self, handle: u32) -> Option<&mut DC> {
        if handle > 0 {
            self.dcs.get_mut((handle - 1) as usize)
        } else {
            None
        }
    }

    fn get_object(&self, handle: u32) -> Option<&Object> {
        if handle > 0 {
            self.objects.get((handle - 1) as usize)
        } else {
            None
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            dcs: Vec::new(),
            objects: Vec::new(),
        }
    }
}

#[win32_derive::dllexport]
pub fn GetStockObject(_machine: &mut Machine, _i: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn SelectObject(machine: &mut Machine, hdc: u32, hGdiObj: u32) -> u32 {
    let obj = match machine.state.gdi32.get_object(hGdiObj) {
        None => return 0, // TODO: HGDI_ERROR
        Some(obj) => obj,
    };

    let op = match obj {
        Object::Bitmap(_) => |dc: &mut DC| std::mem::replace(&mut dc.bitmap, hGdiObj),
    };

    let dc = match machine.state.gdi32.get_dc_mut(hdc) {
        None => return 0, // TODO: HGDI_ERROR
        Some(dc) => dc,
    };

    op(dc) // returns previous value
}

#[win32_derive::dllexport]
pub fn GetObjectA(machine: &mut Machine, handle: u32, _bytes: u32, _out: u32) -> u32 {
    let obj = match machine.state.gdi32.get_object(handle) {
        None => return 0, // fail
        Some(obj) => obj,
    };
    log::warn!("unimp GetObjectA: got {:?}, unimplemented return", obj);
    // TODO: it turns out BasicDD.exe doesn't depend on this working anyway.
    0 // fail
}

#[win32_derive::dllexport]
pub fn CreateCompatibleDC(machine: &mut Machine, hdc: u32) -> u32 {
    assert!(hdc == 0); // null means "compatible with current screen"
    let (handle, _) = machine.state.gdi32.new_dc();
    handle
}

#[win32_derive::dllexport]
pub fn DeleteDC(_machine: &mut Machine, hdc: u32) -> u32 {
    log::warn!("todo: DeleteDC({hdc:x})");
    0 // fail
}

const SRCCOPY: u32 = 0xcc0020;

#[win32_derive::dllexport]
pub fn BitBlt(
    machine: &mut Machine,
    hdc: u32,
    x: u32,
    y: u32,
    cx: u32,
    cy: u32,
    hdcSrc: u32,
    x1: u32,
    y1: u32,
    rop: u32,
) -> u32 {
    if rop != SRCCOPY {
        log::error!("unimp: raster op {rop:x}");
        return 0;
    }

    // TODO: we special case exactly one BitBlt, from a GDI bitmap to a DirectDraw surface,
    // where the surface sizes match as well.
    let hdc = machine.state.gdi32.get_dc(hdc).unwrap();
    let surface = machine
        .state
        .ddraw
        .surfaces
        .get_mut(&hdc.ddraw_surface)
        .unwrap();
    let hdcSrc = machine.state.gdi32.get_dc(hdcSrc).unwrap();
    let obj = machine.state.gdi32.get_object(hdcSrc.bitmap).unwrap();
    let bitmap = match obj {
        Object::Bitmap(bmp) => bmp,
    };
    assert!(x == 0 && y == 0 && x1 == 0 && y1 == 0);
    assert!(cx == surface.width && cy == surface.height);
    assert!(surface.width == bitmap.width && surface.height == bitmap.height);

    surface.host.write_pixels(&bitmap.pixels);
    1 // success
}

#[win32_derive::dllexport]
pub fn StretchBlt(
    machine: &mut Machine,
    hdcDest: u32,
    xDest: u32,
    yDest: u32,
    wDest: u32,
    hDest: u32,
    hdcSrc: u32,
    xSrc: u32,
    ySrc: u32,
    wSrc: u32,
    hSrc: u32,
    rop: u32,
) -> u32 {
    if wDest != wSrc || hDest != hSrc {
        log::error!("unimp: StretchBlt with actual stretching");
        return 0;
    }
    BitBlt(
        machine, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, rop,
    )
}

#[repr(C)]
#[derive(Debug)]
pub struct BITMAPINFO {
    bmiHeader: user32::BITMAPINFOHEADER,
    bmiColors: [u32; 0], // TODO
}
unsafe impl memory::Pod for BITMAPINFO {}

#[win32_derive::dllexport]
pub fn CreateDIBSection(
    _machine: &mut Machine,
    hdc: HDC,
    pbmi: Option<&BITMAPINFO>,
    usage: u32,
    ppvBits: Option<&mut u32>, // **void
    hSection: u32,
    offset: u32,
) -> u32 {
    // TODO: HBITMAP
    if hSection != 0 || offset != 0 {
        todo!()
    }
    0 // fail
}
