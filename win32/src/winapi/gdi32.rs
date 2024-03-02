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
pub struct COLORREF(pub (u8, u8, u8));
impl COLORREF {
    pub fn to_pixel(&self) -> [u8; 4] {
        let (r, g, b) = self.0;
        [r, g, b, 0]
    }
}

#[derive(Debug)]
pub struct Brush {
    pub color: COLORREF,
}

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
    target: DCTarget,

    r2: R2,
    pub x: u32,
    pub y: u32,

    // The SelectObject() API sets a drawing-related field on the DC and returns the
    // previously selected object of a given type, which means we need a storage field
    // per object type.
    pub brush: HGDIOBJ,
}

impl DC {
    pub fn new(target: DCTarget) -> Self {
        DC {
            target,
            r2: R2::default(),
            x: 0,
            y: 0,
            brush: Default::default(),
        }
    }
}

pub type HDC = HANDLE<DC>;
pub type HGDIOBJ = HANDLE<Object>;

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
        Ok(GetStockObjectArg::LTGRAY_BRUSH) => {
            machine.state.gdi32.objects.add(Object::Brush(Brush {
                color: COLORREF((0xDD, 0xDD, 0xDD)),
            }))
        }
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
        Object::Bitmap(_) => match dc.target {
            DCTarget::Memory(prev) => {
                dc.target = DCTarget::Memory(hGdiObj);
                prev
            }
            DCTarget::Window(_) => todo!(),
            DCTarget::DirectDrawSurface(_) => todo!(),
        },
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
    let dc = DC::new(DCTarget::Memory(HGDIOBJ::null()));
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
) -> bool {
    // TODO: we special case only a few specific BitBlts.
    if rop != SRCCOPY {
        todo!();
    }

    let dcSrc = machine.state.gdi32.dcs.get(hdcSrc).unwrap();
    let bitmap = match dcSrc.target {
        DCTarget::Memory(bitmap) => {
            let obj = machine.state.gdi32.objects.get(bitmap).unwrap();
            match obj {
                Object::Bitmap(bmp) => bmp,
                _ => unimplemented!("{:?}", obj),
            }
        }
        DCTarget::Window(_) => todo!(),
        DCTarget::DirectDrawSurface(_) => todo!(),
    };

    let dcDst = machine.state.gdi32.dcs.get(hdc).unwrap();
    match dcDst.target {
        DCTarget::Memory(_) => todo!(),
        DCTarget::Window(_) => {
            log::error!("todo: BltBlt to Window");
            false
        }
        DCTarget::DirectDrawSurface(ptr) => {
            let surface = machine.state.ddraw.surfaces.get_mut(&ptr).unwrap();

            assert!(x == 0 && y == 0 && x1 == 0 && y1 == 0);
            assert!(cx == surface.width && cy == surface.height);
            assert!(surface.width == bitmap.width && surface.height == bitmap.height);

            surface
                .host
                .write_pixels(bitmap.pixels_slice(machine.memory.mem()));
            true
        }
    }
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
) -> bool {
    if wDest != wSrc || hDest != hSrc {
        todo!("unimp: StretchBlt with actual stretching");
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
pub fn CreateCompatibleBitmap(machine: &mut Machine, hdc: HDC, cx: u32, cy: u32) -> HGDIOBJ {
    let dc = machine.state.gdi32.dcs.get(hdc).unwrap();
    match dc.target {
        DCTarget::Memory(_) => todo!(),
        DCTarget::Window(_) => {} // screen has known format
        DCTarget::DirectDrawSurface(_) => todo!(),
    }

    let mut pixels = Vec::new();
    pixels.resize((cx * cy) as usize, [0; 4]);
    let bitmap = user32::Bitmap {
        width: cx,
        height: cy,
        pixels: user32::Pixels::Owned(pixels.into_boxed_slice()),
    };
    machine.state.gdi32.objects.add(Object::Bitmap(bitmap))
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

#[derive(Debug, Default, win32_derive::TryFromEnum)]
pub enum R2 {
    #[default]
    COPYPEN = 13,
    WHITE = 16,
}

#[win32_derive::dllexport]
pub fn SetROP2(machine: &mut Machine, hdc: HDC, rop2: Result<R2, u32>) -> u32 {
    let dc = machine.state.gdi32.dcs.get_mut(hdc).unwrap();
    std::mem::replace(&mut dc.r2, rop2.unwrap()) as u32
}

#[win32_derive::dllexport]
pub fn MoveToEx(machine: &mut Machine, hdc: HDC, x: u32, y: u32, lppt: Option<&mut POINT>) -> bool {
    let dc = machine.state.gdi32.dcs.get_mut(hdc).unwrap();
    if let Some(pt) = lppt {
        *pt = POINT { x: dc.x, y: dc.y };
    }
    dc.x = x;
    dc.y = y;
    true
}

fn ascending(a: u32, b: u32) -> (u32, u32) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

#[win32_derive::dllexport]
pub fn LineTo(machine: &mut Machine, hdc: HDC, x: u32, y: u32) -> bool {
    let dc = machine.state.gdi32.dcs.get_mut(hdc).unwrap();
    let hwnd = match dc.target {
        DCTarget::Memory(_) => todo!(),
        DCTarget::Window(hwnd) => hwnd,
        DCTarget::DirectDrawSurface(_) => todo!(),
    };
    let window = machine.state.user32.windows.get_mut(hwnd).unwrap();
    let stride = window.width;
    let pixels = window.pixels_mut(&mut *machine.host);

    let color = match dc.r2 {
        R2::COPYPEN => [0, 0, 0, 0],
        R2::WHITE => [0xff, 0xff, 0xff, 0],
    };

    let (dstX, dstY) = (x, y);
    if dstX == dc.x {
        let (y0, y1) = ascending(dstY, dc.x);
        for y in y0..y1 {
            pixels.raw[((y * stride) + x) as usize] = color;
        }
        dc.y = dstY;
    } else if dstY == dc.y {
        let (x0, x1) = ascending(dstX, dc.x);
        for x in x0..x1 {
            pixels.raw[((y * stride) + x) as usize] = color;
        }
        dc.x = dstX;
    } else {
        todo!();
    }
    false // fail
}
