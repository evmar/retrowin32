use super::{Brush, DCTarget, Pen, BITMAP, COLORREF, HDC};
use crate::{
    winapi::{bitmap::Bitmap, HANDLE},
    Machine,
};
use memory::ExtensionsMut;
use std::{cell::RefCell, rc::Rc};

/// GDI Object, as identified by HANDLEs.
#[derive(Debug)]
pub enum Object {
    Brush(Brush),
    Bitmap(Rc<RefCell<Bitmap>>),
    Pen(Pen),
}

pub type HGDIOBJ = HANDLE<Object>;
impl HGDIOBJ {
    /// Some Windows APIs use low values of GDI objects as known system constants,
    /// so start the handles higher.
    pub fn lowest_value() -> u32 {
        0x100
    }
}

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
pub fn GetStockObject(machine: &mut Machine, i: Result<GetStockObjectArg, u32>) -> HGDIOBJ {
    use GetStockObjectArg::*;
    let obj = match i.unwrap() {
        WHITE_BRUSH => machine.state.gdi32.objects.add(Object::Brush(Brush {
            color: Some(COLORREF::white()),
        })),
        LTGRAY_BRUSH => machine.state.gdi32.objects.add(Object::Brush(Brush {
            color: Some(COLORREF::from_rgb(0xc0, 0xc0, 0xc0)),
        })),
        GRAY_BRUSH => todo!(),
        DKGRAY_BRUSH => machine.state.gdi32.objects.add(Object::Brush(Brush {
            color: Some(COLORREF::from_rgb(0x40, 0x40, 0x40)),
        })),
        BLACK_BRUSH => machine.state.gdi32.objects.add(Object::Brush(Brush {
            color: Some(COLORREF::from_rgb(0x00, 0x00, 0x00)),
        })),
        NULL_BRUSH => machine
            .state
            .gdi32
            .objects
            .add(Object::Brush(Brush { color: None })),
        DC_BRUSH => todo!(),

        WHITE_PEN | BLACK_PEN | NULL_PEN | DC_PEN => HGDIOBJ::null(),

        OEM_FIXED_FONT | ANSI_FIXED_FONT | ANSI_VAR_FONT | SYSTEM_FONT | DEVICE_DEFAULT_FONT
        | SYSTEM_FIXED_FONT | DEFAULT_GUI_FONT => HGDIOBJ::null(),

        DEFAULT_PALETTE => todo!(),
    };
    if obj.is_null() {
        // TODO: once all of these are implemented, null is not a possible return.
        log::warn!("GetStockObject: TODO: returning null stock object");
    }
    obj
}

#[win32_derive::dllexport]
pub fn SelectObject(machine: &mut Machine, hdc: HDC, hGdiObj: HGDIOBJ) -> HGDIOBJ {
    let mut dc = match machine.state.gdi32.dcs.get(hdc) {
        None => return HGDIOBJ::null(), // TODO: HGDI_ERROR
        Some(dc) => dc,
    }
    .borrow_mut();

    let obj = match machine.state.gdi32.objects.get(hGdiObj) {
        None => return HGDIOBJ::null(), // TODO: HGDI_ERROR
        Some(obj) => obj,
    };
    match obj {
        Object::Bitmap(bitmap) => match dc.target {
            DCTarget::Memory(_) => {
                // TODO: "A single bitmap cannot be selected into more than one DC at the same time."
                dc.target = DCTarget::Memory(bitmap.clone());
                std::mem::replace(&mut dc.bitmap, hGdiObj)
            }
            _ => {
                // MSDN: "Bitmaps can only be selected into memory DC's [sic].""
                // But kill the clone attempts to select one onto a window, just before using
                // normal drawing calls to draw the same bitmap onto the same window.
                log::warn!(
                    "ignoring invalid SelectObject of bitmap onto {:?}",
                    dc.target
                );
                HGDIOBJ::null()
            }
        },
        Object::Brush(_) => std::mem::replace(&mut dc.brush, hGdiObj),
        Object::Pen(_) => std::mem::replace(&mut dc.pen, hGdiObj),
    }
}

#[win32_derive::dllexport]
pub fn GetObjectA(machine: &mut Machine, handle: HGDIOBJ, bytes: u32, out: u32) -> u32 {
    let obj = match machine.state.gdi32.objects.get(handle) {
        None => return 0, // fail
        Some(obj) => obj,
    };

    match obj {
        Object::Brush(_) => todo!(),
        Object::Bitmap(bitmap) => {
            assert_eq!(bytes as usize, std::mem::size_of::<BITMAP>());
            let bitmap = bitmap.borrow();
            machine.mem().put_pod::<BITMAP>(
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
pub fn DeleteObject(_machine: &mut Machine, handle: HGDIOBJ) -> bool {
    // TODO: leak
    true
}
