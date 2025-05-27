use crate::{
    HDC,
    bitmap::Bitmap,
    bitmap_api::BITMAP,
    draw::{Brush, COLORREF, Pen},
    state::get_state,
};
use memory::ExtensionsMut;
use std::{cell::RefCell, rc::Rc};
use win32_system::System;
use win32_winapi::HANDLE;

/// GDI Object, as identified by HANDLEs.
// We need to be generic across these because any handle can be passed
// to SelectObject.
#[derive(Debug)]
pub enum Object {
    Brush(Brush),
    Bitmap(Rc<RefCell<Bitmap>>),
    Pen(Pen),
}

pub type HGDIOBJ = HANDLE<Object>;

/// Some Windows APIs use low values of GDI objects as known system constants,
/// so start the handles higher.
pub const LOWEST_HGDIOBJ: u32 = 0x100;

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum GetStockObjectArg {
    WHITE_BRUSH = 0,
    LTGRAY_BRUSH = 1,
    GRAY_BRUSH = 2,
    DKGRAY_BRUSH = 3,
    BLACK_BRUSH = 4,
    NULL_BRUSH = 5,
    WHITE_PEN = 6,
    BLACK_PEN = 7,
    NULL_PEN = 8,
    OEM_FIXED_FONT = 10,
    ANSI_FIXED_FONT = 11,
    ANSI_VAR_FONT = 12,
    SYSTEM_FONT = 13,
    DEVICE_DEFAULT_FONT = 14,
    DEFAULT_PALETTE = 15,
    SYSTEM_FIXED_FONT = 16,
    DEFAULT_GUI_FONT = 17,
    DC_BRUSH = 18,
    DC_PEN = 19,
}

#[win32_derive::dllexport]
pub fn GetStockObject(sys: &dyn System, i: Result<GetStockObjectArg, u32>) -> HGDIOBJ {
    use GetStockObjectArg::*;
    let mut state = get_state(sys);
    let obj = match i.unwrap() {
        WHITE_BRUSH => state.objects.add(Object::Brush(Brush {
            color: Some(COLORREF::white()),
        })),
        LTGRAY_BRUSH => state.objects.add(Object::Brush(Brush {
            color: Some(COLORREF::from_rgb(0xc0, 0xc0, 0xc0)),
        })),
        GRAY_BRUSH => todo!(),
        DKGRAY_BRUSH => state.objects.add(Object::Brush(Brush {
            color: Some(COLORREF::from_rgb(0x40, 0x40, 0x40)),
        })),
        BLACK_BRUSH => state.objects.add(Object::Brush(Brush {
            color: Some(COLORREF::from_rgb(0x00, 0x00, 0x00)),
        })),
        NULL_BRUSH => state.objects.add(Object::Brush(Brush { color: None })),
        DC_BRUSH => todo!(),

        WHITE_PEN | BLACK_PEN | NULL_PEN | DC_PEN => HGDIOBJ::null(),

        OEM_FIXED_FONT | ANSI_FIXED_FONT | ANSI_VAR_FONT | SYSTEM_FONT | DEVICE_DEFAULT_FONT
        | SYSTEM_FIXED_FONT | DEFAULT_GUI_FONT => HGDIOBJ::null(),

        DEFAULT_PALETTE => HGDIOBJ::null(),
    };
    if obj.is_null() {
        // TODO: once all of these are implemented, null is not a possible return.
        log::warn!("GetStockObject: TODO: returning null stock object");
    }
    obj
}

#[win32_derive::dllexport]
pub fn SelectObject(sys: &dyn System, hdc: HDC, hGdiObj: HGDIOBJ) -> HGDIOBJ {
    let state = get_state(sys);
    let mut dc = match state.dcs.get(hdc) {
        None => return HGDIOBJ::null(), // TODO: HGDI_ERROR
        Some(dc) => dc,
    }
    .borrow_mut();

    let obj = match state.objects.get(hGdiObj) {
        None => return HGDIOBJ::null(), // TODO: HGDI_ERROR
        Some(obj) => obj,
    };

    match obj {
        Object::Bitmap(bitmap) => {
            dc.target.select_bitmap(bitmap.clone());
            hGdiObj
        }
        Object::Brush(_) => std::mem::replace(&mut dc.brush, hGdiObj),
        Object::Pen(_) => std::mem::replace(&mut dc.pen, hGdiObj),
    }
}

#[win32_derive::dllexport]
pub fn GetObjectA(sys: &dyn System, handle: HGDIOBJ, bytes: u32, out: u32) -> u32 {
    let state = get_state(sys);
    let obj = match state.objects.get(handle) {
        None => return 0, // fail
        Some(obj) => obj,
    };

    match obj {
        Object::Brush(_) => todo!(),
        Object::Bitmap(bitmap) => {
            assert_eq!(bytes as usize, std::mem::size_of::<BITMAP>());
            let bitmap = bitmap.borrow();
            sys.mem().put_pod::<BITMAP>(
                out,
                BITMAP {
                    bmType: 0,
                    bmWidth: bitmap.width,
                    bmHeight: bitmap.height,
                    bmWidthBytes: 0,
                    bmPlanes: 0,
                    bmBitsPixel: bitmap.format.bits_per_pixel() as u16,
                    bmBits: 0,
                },
            );
            bytes
        }
        Object::Pen(_) => todo!(),
    }
}

#[win32_derive::dllexport]
pub fn DeleteObject(sys: &dyn System, handle: HGDIOBJ) -> bool {
    // TODO: leak
    true
}
