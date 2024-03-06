#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::{handle::Handles, kernel32, stack_args::ArrayWithSize, types::*, user32::BI};
use crate::{machine::Machine, winapi::user32};
pub use user32::BITMAPINFOHEADER;

const TRACE_CONTEXT: &'static str = "gdi32";

/// GDI Object, as identified by HANDLEs.
#[derive(Debug)]
pub enum Object {
    Brush(Brush),
    Bitmap(user32::Bitmap),
    Pen(Pen),
}

#[derive(Debug)]
pub struct COLORREF(pub (u8, u8, u8));
impl COLORREF {
    pub fn from_u32(raw: u32) -> Self {
        Self((raw as u8, (raw >> 8) as u8, (raw >> 16) as u8))
    }
    pub fn to_pixel(&self) -> [u8; 4] {
        let (r, g, b) = self.0;
        [r, g, b, 0xff]
    }
}

#[derive(Debug)]
pub struct Brush {
    pub color: COLORREF,
}

#[derive(Debug)]
pub struct Pen {
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
    pub pen: HGDIOBJ,
}

impl DC {
    pub fn new(target: DCTarget) -> Self {
        DC {
            target,
            r2: R2::default(),
            x: 0,
            y: 0,
            brush: Default::default(),
            pen: Default::default(),
        }
    }

    pub fn new_memory(machine: &mut Machine) -> Self {
        // MSDN says: "When a memory device context is created, it initially has a 1-by-1 monochrome bitmap selected into it."
        // SkiFree depends on this!
        let bitmap = user32::Bitmap {
            width: 1,
            height: 1,
            pixels: user32::Pixels::Ptr(0),
        };
        let hobj = machine.state.gdi32.objects.add(Object::Bitmap(bitmap));
        Self::new(DCTarget::Memory(hobj))
    }
}

pub type HDC = HANDLE<DC>;
pub type HGDIOBJ = HANDLE<Object>;

pub struct State {
    pub dcs: Handles<HDC, DC>,
    pub desktop_dc: HDC,
    pub objects: Handles<HGDIOBJ, Object>,
}

impl Default for State {
    fn default() -> Self {
        let mut dcs: Handles<HDC, DC> = Default::default();
        let desktop_dc = dcs.reserve();
        State {
            dcs,
            desktop_dc,
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
                color: COLORREF((0xc0, 0xc0, 0xc0)),
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
        Object::Pen(_) => std::mem::replace(&mut dc.pen, hGdiObj),
    }
}

#[allow(dead_code)]
struct BITMAP {
    bmType: u32,
    bmWidth: u32,
    bmHeight: u32,
    bmWidthBytes: u32,
    bmPlanes: u16,
    bmBitsPixel: u16,
    bmBits: u32,
}
unsafe impl memory::Pod for BITMAP {}

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
            let out = machine.mem().view_mut::<BITMAP>(out);
            *out = BITMAP {
                bmType: 0,
                bmWidth: bitmap.width,
                bmHeight: bitmap.height,
                bmWidthBytes: 0,
                bmPlanes: 0,
                bmBitsPixel: 0,
                bmBits: 0,
            };
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

#[win32_derive::dllexport]
pub fn CreateCompatibleDC(machine: &mut Machine, hdc: HDC) -> HDC {
    let dc = DC::new_memory(machine);
    let handle = machine.state.gdi32.dcs.add(dc);
    handle
}

#[win32_derive::dllexport]
pub fn DeleteDC(_machine: &mut Machine, hdc: u32) -> u32 {
    log::warn!("todo: DeleteDC({hdc:x})");
    0 // fail
}

fn bit_blt(
    dst: &mut [[u8; 4]],
    dx: usize,
    dy: usize,
    dstride: usize,
    w: usize,
    h: usize,
    src: &[[u8; 4]],
    sx: usize,
    sy: usize,
    sstride: usize,
) {
    for row in 0..h {
        let dst_row = &mut dst[(((dy + row) * dstride) + dx)..][..w];
        let src_row = &src[(((sy + row) * sstride) + sx)..][..w];
        dst_row.copy_from_slice(src_row);
    }
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
        DCTarget::Window(hwnd) => {
            let window = machine.state.user32.windows.get_mut(hwnd).unwrap();

            let src = match &bitmap.pixels {
                user32::Pixels::Owned(p) => &**p,
                _ => todo!(),
            };
            let window_width = window.width;
            let dst = &mut *window.pixels_mut(&mut *machine.host).raw;

            bit_blt(
                dst,
                x as usize,
                y as usize,
                window_width as usize,
                cx as usize,
                cy as usize,
                src,
                x1 as usize,
                y1 as usize,
                bitmap.width as usize,
            );

            window.flush_pixels();
        }
        DCTarget::DirectDrawSurface(ptr) => {
            let surface = machine.state.ddraw.surfaces.get_mut(&ptr).unwrap();

            assert!(x == 0 && y == 0 && x1 == 0 && y1 == 0);
            assert!(cx == surface.width && cy == surface.height);
            assert!(surface.width == bitmap.width && surface.height == bitmap.height);

            surface
                .host
                .write_pixels(bitmap.pixels_slice(machine.memory.mem()));
        }
    }
    true
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

const DIB_RGB_COLORS: u32 = 0;
// const DIB_PAL_COLORS: u32 = 1;

#[win32_derive::dllexport]
pub fn CreateDIBSection(
    machine: &mut Machine,
    hdc: HDC,
    pbmi: Option<&BITMAPINFOHEADER>,
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

    let bi = &pbmi.unwrap();
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
        GetDeviceCapsArg::HORZRES => 640,
        GetDeviceCapsArg::VERTRES => 480,
        _ => unimplemented!(),
    }
}

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum PS {
    SOLID = 0,
}

#[win32_derive::dllexport]
pub fn CreatePen(
    machine: &mut Machine,
    iStyle: Result<PS, u32>,
    cWidth: u32,
    color: u32,
) -> HGDIOBJ {
    iStyle.unwrap();
    if cWidth != 1 {
        todo!();
    }

    machine.state.gdi32.objects.add(Object::Pen(Pen {
        color: COLORREF::from_u32(color),
    }))
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
    machine: &mut Machine,
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
    lpbmi: Option<&BITMAPINFOHEADER>,
    ColorUse: u32,
) -> u32 {
    if StartScan != ySrc || cLines != h {
        todo!()
    }
    if ColorUse != DIB_RGB_COLORS {
        todo!();
    }

    let header = lpbmi.unwrap();
    if header.biWidth != w {
        todo!("unclear which width to believe");
    }

    let mem = machine.memory.mem();
    let src_bitmap = user32::Bitmap::parse(
        header,
        Some((
            machine.mem().slice(lpvBits..).as_slice_todo(),
            cLines as usize,
        )),
    );
    let src = src_bitmap.pixels_slice(mem);

    let dc = machine.state.gdi32.dcs.get(hdc).unwrap();
    let (dst, dst_stride) = match dc.target {
        DCTarget::Memory(hbitmap) => {
            let dst_bitmap = match machine.state.gdi32.objects.get_mut(hbitmap).unwrap() {
                Object::Bitmap(b) => b,
                _ => todo!(),
            };
            let pixels = match &mut dst_bitmap.pixels {
                user32::Pixels::Owned(p) => &mut **p,
                user32::Pixels::Ptr(_) => todo!(),
            };
            (pixels, dst_bitmap.width)
        }
        DCTarget::Window(hwnd) => {
            let window = machine.state.user32.windows.get_mut(hwnd).unwrap();
            let window_width = window.width;
            let pixels = window.pixels_mut(&mut *machine.host);
            (&mut *pixels.raw, window_width)
        }
        DCTarget::DirectDrawSurface(_) => todo!(),
    };

    bit_blt(
        dst,
        xDest as usize,
        yDest as usize,
        dst_stride as usize,
        w as usize,
        h as usize,
        src,
        xSrc as usize,
        ySrc as usize,
        src_bitmap.width as usize,
    );

    match dc.target {
        DCTarget::Window(hwnd) => {
            let window = machine.state.user32.windows.get_mut(hwnd).unwrap();
            window.flush_pixels();
        }
        _ => {}
    }

    cLines
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
        R2::COPYPEN => match machine.state.gdi32.objects.get(dc.pen).unwrap() {
            Object::Pen(pen) => pen.color.to_pixel(),
            _ => todo!(),
        },
        R2::WHITE => [0xff, 0xff, 0xff, 0],
    };

    let (dstX, dstY) = (x, y);
    if dstX == dc.x {
        let (y0, y1) = ascending(dstY, dc.y);
        for y in y0..=y1 {
            pixels.raw[((y * stride) + x) as usize] = color;
        }
        dc.y = dstY;
    } else if dstY == dc.y {
        let (x0, x1) = ascending(dstX, dc.x);
        for x in x0..=x1 {
            pixels.raw[((y * stride) + x) as usize] = color;
        }
        dc.x = dstX;
    } else {
        todo!();
    }
    false // fail
}
