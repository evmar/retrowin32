#![allow(non_snake_case)]

use crate::{winapi::user32, x86::X86};

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
    pub fn new() -> Self {
        State {
            dcs: Vec::new(),
            objects: Vec::new(),
        }
    }

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

pub fn GetStockObject(_x86: &mut X86, _i: u32) -> u32 {
    0
}

pub fn SelectObject(x86: &mut X86, hdc: u32, hGdiObj: u32) -> u32 {
    let obj = match x86.state.gdi32.get_object(hGdiObj) {
        None => return 0, // TODO: HGDI_ERROR
        Some(obj) => obj,
    };

    let op = match obj {
        Object::Bitmap(_) => |dc: &mut DC| std::mem::replace(&mut dc.bitmap, hGdiObj),
    };

    let dc = match x86.state.gdi32.get_dc_mut(hdc) {
        None => return 0, // TODO: HGDI_ERROR
        Some(dc) => dc,
    };

    op(dc) // returns previous value
}

pub fn GetObjectA(x86: &mut X86, handle: u32, _bytes: u32, _out: u32) -> u32 {
    let obj = match x86.state.gdi32.get_object(handle) {
        None => return 0, // fail
        Some(obj) => obj,
    };
    log::warn!("unimp GetObjectA: got {:?}, unimplemented return", obj);
    // TODO: it turns out BasicDD.exe doesn't depend on this working anyway.
    0 // fail
}

pub fn CreateCompatibleDC(x86: &mut X86, hdc: u32) -> u32 {
    assert!(hdc == 0); // null means "compatible with current screen"
    let (handle, _) = x86.state.gdi32.new_dc();
    handle
}

pub fn DeleteDC(_x86: &mut X86, hdc: u32) -> u32 {
    log::warn!("todo: DeleteDC({hdc:x})");
    0 // fail
}

const SRCCOPY: u32 = 0xcc0020;

pub fn BitBlt(
    x86: &mut X86,
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
    let hdc = x86.state.gdi32.get_dc(hdc).unwrap();
    let surface = x86
        .state
        .ddraw
        .surfaces
        .get_mut(&hdc.ddraw_surface)
        .unwrap();
    let hdcSrc = x86.state.gdi32.get_dc(hdcSrc).unwrap();
    let obj = x86.state.gdi32.get_object(hdcSrc.bitmap).unwrap();
    let bitmap = match obj {
        Object::Bitmap(bmp) => bmp,
    };
    assert!(x == 0 && y == 0 && x1 == 0 && y1 == 0);
    assert!(cx == surface.width && cy == surface.height);
    assert!(surface.width == bitmap.width && surface.height == bitmap.height);

    surface.host.write_pixels(&bitmap.pixels);
    1 // success
}

pub fn StretchBlt(
    x86: &mut X86,
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
        x86, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, rop,
    )
}
