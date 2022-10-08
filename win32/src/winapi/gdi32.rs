#![allow(non_snake_case)]

use crate::{winapi, winapi::user32, X86};

/// GDI Object, as identified by HANDLEs.
#[derive(Debug)]
pub enum Object {
    Bitmap(user32::Bitmap),
}

#[derive(Debug)]
struct DC {
    bitmap: u32, // HANDLE
}
impl DC {
    fn new() -> Self {
        DC { bitmap: 0 }
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
    x86.state.gdi32.dcs.push(DC::new());
    x86.state.gdi32.dcs.len() as u32
}

fn DeleteDC(_x86: &mut X86, hdc: u32) -> u32 {
    log::warn!("DeleteDC({hdc:x})");
    0 // fail
}

winapi!(
    fn GetStockObject(i: u32);
    fn SelectObject(hdc: u32, hGdiObj: u32);
    fn GetObjectA(handle: u32, bytes: u32, out: u32);

    fn CreateCompatibleDC(hdc: u32);
    fn DeleteDC(hdc: u32);
);
