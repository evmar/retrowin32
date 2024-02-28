#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::{
    handle::Handles,
    kernel32,
    stack_args::ArrayWithSize,
    types::*,
    user32::{BI, BITMAPINFOHEADER},
};
use crate::{machine::Machine, winapi::user32};

const TRACE_CONTEXT: &'static str = "gdi32";

/// GDI Object, as identified by HANDLEs.
#[derive(Debug)]
pub enum Object {
    Brush(Brush),
    Bitmap(user32::Bitmap),
}

#[derive(Debug)]
pub struct Brush {
    pub color: u32,
}

#[derive(Debug, Default)]
pub struct DC {
    // TODO: it's unclear to me what the representation of a DC ought to be.
    // The SelectObject() API returns the previously selected object of a given
    // type, which implies that we need a storage field per object type.
    // But then DirectDraw can also create a DC, and DirectDraw (as a DLL that came
    // later can't retrofit the DC type with a DirectDraw field.
    // Wine appears to use a vtable (for generic behavior) *and* per-object-type fields.
    bitmap: HGDIOBJ,
    pub brush: HGDIOBJ,
    pub ddraw_surface: u32,
}

pub type HDC = HANDLE<DC>;
pub struct HGDIOBJT {}
pub type HGDIOBJ = HANDLE<HGDIOBJT>;

pub struct State {
    pub dcs: Handles<HDC, DC>,
    pub objects: Handles<HGDIOBJ, Object>,
}

impl Default for State {
    fn default() -> Self {
        State {
            dcs: Default::default(),
            objects: Default::default(),
        }
    }
}

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum GetStockObjectArg {
    WHITE_BRUSH = 0,
    LTGRAY_BRUSH = 1,
    GRAY_BRUSH = 2,
    DKGRAY_BRUSH = 3,
    BLACK_BRUSH = 4,
}

#[win32_derive::dllexport]
pub fn GetStockObject(machine: &mut Machine, i: Result<GetStockObjectArg, u32>) -> HGDIOBJ {
    match i {
        Ok(GetStockObjectArg::LTGRAY_BRUSH) => machine
            .state
            .gdi32
            .objects
            .add(Object::Brush(Brush { color: 0xDDDDDD })),
        _ => {
            log::error!("returning null stock object");
            HGDIOBJ::null()
        }
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
        Object::Bitmap(_) => std::mem::replace(&mut dc.bitmap, hGdiObj),
        Object::Brush(_) => std::mem::replace(&mut dc.brush, hGdiObj),
    }
}

#[win32_derive::dllexport]
pub fn GetObjectA(machine: &mut Machine, handle: HGDIOBJ, _bytes: u32, _out: u32) -> u32 {
    let obj = match machine.state.gdi32.objects.get(handle) {
        None => return 0, // fail
        Some(obj) => obj,
    };
    log::warn!("unimp GetObjectA: got {:?}, unimplemented return", obj);
    // TODO: it turns out BasicDD.exe doesn't depend on this working anyway.
    0 // fail
}

#[win32_derive::dllexport]
pub fn CreateCompatibleDC(machine: &mut Machine, hdc: HDC) -> HDC {
    let dc = DC::default();
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
    hdc: HDC,
    x: u32,
    y: u32,
    cx: u32,
    cy: u32,
    hdcSrc: HDC,
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
    if hdc.ddraw_surface == 0 {
        log::error!("unimp: BitBlt with non-directdraw surface");
        return 0;
    }
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
        _ => unimplemented!("{:?}", obj),
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
    hdcDest: HDC,
    xDest: u32,
    yDest: u32,
    wDest: u32,
    hDest: u32,
    hdcSrc: HDC,
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
) -> HGDIOBJ {
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
    cHeight: i32,
    cWidth: i32,
    cEscapement: i32,
    cOrientation: i32,
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

#[win32_derive::dllexport]
pub fn SetBkMode(_machine: &mut Machine, hdc: HDC, mode: i32) -> i32 {
    0 // fail
}

const CLR_INVALID: u32 = 0xFFFF_FFFF;

#[win32_derive::dllexport]
pub fn SetBkColor(_machine: &mut Machine, hdc: HDC, color: u32) -> u32 {
    CLR_INVALID // fail
}

#[win32_derive::dllexport]
pub fn SetTextColor(_machine: &mut Machine, hdc: HDC, color: u32) -> u32 {
    CLR_INVALID // fail
}

#[win32_derive::dllexport]
pub fn TextOutA(
    _machine: &mut Machine,
    hdc: HDC,
    x: u32,
    y: u32,
    lpString: ArrayWithSize<u8>,
) -> bool {
    let _text = std::str::from_utf8(lpString.unwrap()).unwrap();
    true
}

#[allow(dead_code)]
#[derive(Debug, win32_derive::TryFromEnum)]
#[repr(u32)]
pub enum GetDeviceCapsArg {
    DRIVERVERSION = 0,
    TECHNOLOGY = 2,
    HORZSIZE = 4,
    VERTSIZE = 6,
    HORZRES = 8,
    VERTRES = 10,
    BITSPIXEL = 12,
    PLANES = 14,
    NUMBRUSHES = 16,
    NUMPENS = 18,
    NUMMARKERS = 20,
    NUMFONTS = 22,
    NUMCOLORS = 24,
    PDEVICESIZE = 26,
    CURVECAPS = 28,
    LINECAPS = 30,
    POLYGONALCAPS = 32,
    TEXTCAPS = 34,
    CLIPCAPS = 36,
    RASTERCAPS = 38,
    ASPECTX = 40,
    ASPECTY = 42,
    ASPECTXY = 44,
    LOGPIXELSX = 88,
    LOGPIXELSY = 90,
    SIZEPALETTE = 104,
    NUMRESERVED = 106,
    COLORRES = 108,
    PHYSICALWIDTH = 110,
    PHYSICALHEIGHT = 111,
    PHYSICALOFFSETX = 112,
    PHYSICALOFFSETY = 113,
    SCALINGFACTORX = 114,
    SCALINGFACTORY = 115,
    VREFRESH = 116,
    DESKTOPVERTRES = 117,
    DESKTOPHORZRES = 118,
    BLTALIGNMENT = 119,
}

#[win32_derive::dllexport]
pub fn GetDeviceCaps(
    _machine: &mut Machine,
    hdc: HDC,
    index: Result<GetDeviceCapsArg, u32>,
) -> u32 {
    match index.unwrap() {
        GetDeviceCapsArg::NUMCOLORS => -1i32 as u32, // true color
        _ => unimplemented!(),
    }
}

pub struct PEN {}
pub type HPEN = HANDLE<PEN>;

#[win32_derive::dllexport]
pub fn CreatePen(_machine: &mut Machine, iStyle: u32, cWidth: u32, color: u32) -> HPEN {
    HPEN::null()
}

#[win32_derive::dllexport]
pub fn CreateCompatibleBitmap(_machine: &mut Machine, hdc: HDC, cx: u32, cy: u32) -> u32 {
    0 // fail
}

#[win32_derive::dllexport]
pub fn SetDIBitsToDevice(
    _machine: &mut Machine,
    hdc: HDC,
    xDest: u32,
    yDest: u32,
    w: u32,
    h: u32,
    xSrc: u32,
    ySrc: u32,
    StartScan: u32,
    cLines: u32,
    lpvBits: u32,
    lpbmi: u32,
    ColorUse: u32,
) -> u32 {
    0 // fail
}

#[win32_derive::dllexport]
pub fn GetLayout(_machine: &mut Machine, hdc: HDC) -> u32 {
    0 // LTR
}

#[win32_derive::dllexport]
pub fn SetROP2(_machine: &mut Machine, hdc: HDC, rop2: u32) -> u32 {
    0 // fail
}

#[win32_derive::dllexport]
pub fn MoveToEx(
    _machine: &mut Machine,
    hdc: HDC,
    x: u32,
    y: u32,
    lppt: Option<&mut POINT>,
) -> bool {
    false // fail
}

#[win32_derive::dllexport]
pub fn LineTo(_machine: &mut Machine, hdc: HDC, x: u32, y: u32) -> bool {
    false // fail
}
