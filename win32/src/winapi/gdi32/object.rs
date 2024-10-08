use super::{Brush, DCTarget, Pen, BITMAP, COLORREF, HDC};
use crate::{
    winapi::{
        bitmap::{BitmapMono, BitmapRGBA32},
        types::HANDLE,
    },
    Machine,
};
use memory::ExtensionsMut;
use std::rc::Rc;

#[derive(Debug)]
pub enum Bitmap {
    RGBA32(Rc<BitmapRGBA32>),
    Mono(BitmapMono),
}

/// GDI Object, as identified by HANDLEs.
#[derive(Debug)]
pub enum Object {
    Brush(Brush),
    Bitmap(Bitmap),
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
    OEM_FIXED_FONT = 10,
}

#[win32_derive::dllexport]
pub fn GetStockObject(machine: &mut Machine, i: Result<GetStockObjectArg, u32>) -> HGDIOBJ {
    match i.unwrap() {
        GetStockObjectArg::WHITE_BRUSH => machine.state.gdi32.objects.add(Object::Brush(Brush {
            color: Some(COLORREF::white()),
        })),
        GetStockObjectArg::LTGRAY_BRUSH => machine.state.gdi32.objects.add(Object::Brush(Brush {
            color: Some(COLORREF::from_rgb(0xc0, 0xc0, 0xc0)),
        })),
        GetStockObjectArg::BLACK_BRUSH => machine.state.gdi32.objects.add(Object::Brush(Brush {
            color: Some(COLORREF::from_rgb(0x00, 0x00, 0x00)),
        })),
        GetStockObjectArg::NULL_BRUSH => machine
            .state
            .gdi32
            .objects
            .add(Object::Brush(Brush { color: None })),
        GetStockObjectArg::OEM_FIXED_FONT => {
            log::error!("returning null stock object");
            HGDIOBJ::null()
        }
        _ => todo!(),
    }
}

#[win32_derive::dllexport]
pub fn SelectObject(machine: &mut Machine, hdc: HDC, hGdiObj: HGDIOBJ) -> HGDIOBJ {
    let dc = match machine.state.gdi32.dcs.get_mut(hdc) {
        None => return HGDIOBJ::null(), // TODO: HGDI_ERROR
        Some(dc) => dc,
    };

    let obj = match machine.state.gdi32.objects.get(hGdiObj) {
        None => return HGDIOBJ::null(), // TODO: HGDI_ERROR
        Some(obj) => obj,
    };
    match obj {
        Object::Bitmap(_) => match dc.target {
            DCTarget::Memory(prev) => {
                dc.target = DCTarget::Memory(hGdiObj);
                prev
            }
            DCTarget::Window(_) => todo!(),
            DCTarget::DirectDrawSurface(_) => todo!(),
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
            machine.mem().put_pod::<BITMAP>(
                out,
                match bitmap {
                    Bitmap::RGBA32(bitmap) => BITMAP {
                        bmType: 0,
                        bmWidth: bitmap.width,
                        bmHeight: bitmap.height,
                        bmWidthBytes: 0,
                        bmPlanes: 0,
                        bmBitsPixel: 32,
                        bmBits: 0,
                    },
                    Bitmap::Mono(bitmap) => BITMAP {
                        bmType: 0,
                        bmWidth: bitmap.width,
                        bmHeight: bitmap.height,
                        bmWidthBytes: 0,
                        bmPlanes: 0,
                        bmBitsPixel: 1,
                        bmBits: 0,
                    },
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
