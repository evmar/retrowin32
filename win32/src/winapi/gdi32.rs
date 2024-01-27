#![allow(non_snake_case)]

use std::collections::HashMap;

use super::{
    kernel32,
    types::*,
    user32::{BI, BITMAPINFOHEADER},
};
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
    pub fn new() -> Self {
        DC {
            bitmap: 0,
            ddraw_surface: 0,
        }
    }
}

pub type HDC = HANDLE<DC>;

/// Maintains a mapping of u32 -> T, vending out new u32 ids.
pub struct Handles<T> {
    map: HashMap<u32, T>,
    next: u32,
}

impl<T> Default for Handles<T> {
    fn default() -> Self {
        Handles {
            map: HashMap::default(),
            next: 1,
        }
    }
}

impl<T> Handles<T> {
    pub fn add(&mut self, t: T) -> u32 {
        let handle = self.next;
        self.next += 1;
        self.map.insert(handle, t);
        handle
    }

    pub fn get(&self, handle: u32) -> Option<&T> {
        self.map.get(&handle)
    }

    pub fn get_mut(&mut self, handle: u32) -> Option<&mut T> {
        self.map.get_mut(&handle)
    }
}

pub struct State {
    pub dcs: Handles<DC>,
    pub objects: Handles<Object>,
}

impl Default for State {
    fn default() -> Self {
        State {
            dcs: Default::default(),
            objects: Default::default(),
        }
    }
}

#[win32_derive::dllexport]
pub fn GetStockObject(_machine: &mut Machine, _i: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn SelectObject(machine: &mut Machine, hdc: u32, hGdiObj: u32) -> u32 {
    let dc = match machine.state.gdi32.dcs.get_mut(hdc) {
        None => return 0, // TODO: HGDI_ERROR
        Some(dc) => dc,
    };

    let obj = match machine.state.gdi32.objects.get(hGdiObj) {
        None => return 0, // TODO: HGDI_ERROR
        Some(obj) => obj,
    };
    match obj {
        Object::Bitmap(_) => std::mem::replace(&mut dc.bitmap, hGdiObj),
    }
}

#[win32_derive::dllexport]
pub fn GetObjectA(machine: &mut Machine, handle: u32, _bytes: u32, _out: u32) -> u32 {
    let obj = match machine.state.gdi32.objects.get(handle) {
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
    let dc = DC::new();
    let handle = machine.state.gdi32.dcs.add(dc);
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
    let hdc = machine.state.gdi32.dcs.get(hdc).unwrap();
    let surface = machine
        .state
        .ddraw
        .surfaces
        .get_mut(&hdc.ddraw_surface)
        .unwrap();
    let hdcSrc = machine.state.gdi32.dcs.get(hdcSrc).unwrap();
    let obj = machine.state.gdi32.objects.get(hdcSrc.bitmap).unwrap();
    let bitmap = match obj {
        Object::Bitmap(bmp) => bmp,
    };
    assert!(x == 0 && y == 0 && x1 == 0 && y1 == 0);
    assert!(cx == surface.width && cy == surface.height);
    assert!(surface.width == bitmap.width && surface.height == bitmap.height);

    surface
        .host
        .write_pixels(bitmap.pixels_slice(machine.memory.mem()));
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

const DIB_RGB_COLORS: u32 = 0;
// const DIB_PAL_COLORS: u32 = 1;

#[win32_derive::dllexport]
pub fn CreateDIBSection(
    machine: &mut Machine,
    hdc: HDC,
    pbmi: Option<&BITMAPINFO>,
    usage: u32,
    ppvBits: Option<&mut u32>, // **void
    hSection: u32,
    offset: u32,
) -> u32 {
    if usage != DIB_RGB_COLORS {
        todo!()
    }
    if hSection != 0 || offset != 0 {
        todo!()
    }

    let bi = &pbmi.unwrap().bmiHeader;
    if bi.biSize != std::mem::size_of::<BITMAPINFOHEADER>() as u32 {
        todo!()
    }
    if !bi.is_top_down() {
        todo!()
    }
    if bi.biBitCount != 32 {
        todo!()
    }
    if bi.compression().unwrap() != BI::BITFIELDS {
        todo!()
    }
    // TODO: ought to check that .bmiColors masks are the RGBX we expect.

    let byte_count = bi.width() * bi.height() * bi.biBitCount as u32;
    let heap = kernel32::GetProcessHeap(machine);
    let pixels = kernel32::HeapAlloc(
        machine,
        heap,
        Ok(kernel32::HeapAllocFlags::default()),
        byte_count,
    );

    *ppvBits.unwrap() = pixels;

    let bitmap = user32::Bitmap {
        width: bi.width(),
        height: bi.height(),
        pixels: user32::Pixels::Ptr(pixels),
    };
    machine.state.gdi32.objects.add(Object::Bitmap(bitmap))
}

pub struct FONT {}
pub type HFONT = HANDLE<FONT>;

#[win32_derive::dllexport]
pub fn CreateFontA(
    _machine: &mut Machine,
    cHeight: u32,
    cWidth: u32,
    cEscapement: u32,
    cOrientation: u32,
    cWeight: u32,
    bItalic: u32,
    bUnderline: u32,
    bStrikeOut: u32,
    iCharSet: u32,
    iOutPrecision: u32,
    iClipPrecision: u32,
    iQuality: u32,
    iPitchAndFamily: u32,
    pszFaceName: Option<&str>,
) -> HFONT {
    HFONT::null()
}
