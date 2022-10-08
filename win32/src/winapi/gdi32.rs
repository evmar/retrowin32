#![allow(non_snake_case)]

use crate::{winapi, winapi::user32, X86};

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

    fn get_dc(&mut self, handle: u32) -> Option<&mut DC> {
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

fn GetStockObject(_x86: &mut X86, _i: u32) -> u32 {
    0
}

fn SelectObject(x86: &mut X86, hdc: u32, hGdiObj: u32) -> u32 {
    let obj = match x86.state.gdi32.get_object(hGdiObj) {
        None => return 0, // TODO: HGDI_ERROR
        Some(obj) => obj,
    };

    let op = match obj {
        Object::Bitmap(_) => |dc: &mut DC| std::mem::replace(&mut dc.bitmap, hGdiObj),
    };

    let dc = match x86.state.gdi32.get_dc(hdc) {
        None => return 0, // TODO: HGDI_ERROR
        Some(dc) => dc,
    };

    op(dc) // returns previous value
}

fn GetObjectA(x86: &mut X86, handle: u32, _bytes: u32, _out: u32) -> u32 {
    let obj = match x86.state.gdi32.get_object(handle) {
        None => return 0, // fail
        Some(obj) => obj,
    };
    log::warn!("unimp GetObjectA: got {:?}, unimplemented return", obj);
    // TODO: it turns out BasicDD.exe doesn't depend on this working anyway.
    0 // fail
}

fn CreateCompatibleDC(x86: &mut X86, hdc: u32) -> u32 {
    assert!(hdc == 0); // null means "compatible with current screen"
    let (handle, _) = x86.state.gdi32.new_dc();
    handle
}

fn DeleteDC(_x86: &mut X86, hdc: u32) -> u32 {
    log::warn!("DeleteDC({hdc:x})");
    0 // fail
}

fn StretchBlt(
    _x86: &mut X86,
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
    log::warn!("StretchBlt({hdcDest:x}, {xDest:x}, {yDest:x}, {wDest:x}, {hDest:x}, {hdcSrc:x}, {xSrc:x}, {ySrc:x}, {wSrc:x}, {hSrc:x}, {rop:x})");
    1 // fail
}

winapi!(
    fn GetStockObject(i: u32);
    fn SelectObject(hdc: u32, hGdiObj: u32);
    fn GetObjectA(handle: u32, bytes: u32, out: u32);

    fn CreateCompatibleDC(hdc: u32);
    fn DeleteDC(hdc: u32);

    fn StretchBlt(
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
    );
);
