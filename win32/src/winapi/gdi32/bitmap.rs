use super::{BITMAPINFOHEADER, COLORREF, DCTarget, HDC, HGDIOBJ, Object};
use crate::{
    Machine, System,
    winapi::{
        POINT, RECT,
        bitmap::{BI, Bitmap, PixelData, PixelFormat},
        gdi32::GDIHandles,
        kernel32,
    },
};
use memory::{Extensions, Mem};

pub type HBITMAP = HGDIOBJ;

#[derive(Clone)]
pub struct BITMAP {
    pub bmType: u32,
    pub bmWidth: u32,
    pub bmHeight: u32,
    pub bmWidthBytes: u32,
    pub bmPlanes: u16,
    pub bmBitsPixel: u16,
    pub bmBits: u32,
}
unsafe impl memory::Pod for BITMAP {}

/// For the subset of dst within r, write pixels from op(x, y).
/// Performs no clipping; caller must have already clipped.
// TODO: remove me in favor of pattern used in StretchBlt
fn fill_pixels(
    mem: Mem,
    dst: &mut Bitmap,
    r: &RECT,
    mut op: impl FnMut(POINT, [u8; 4]) -> [u8; 4],
) {
    let stride = dst.width as i32;
    let pixels = dst.as_rgba_mut(mem);
    for y in r.top..r.bottom {
        for x in r.left..r.right {
            let p = &mut pixels[(y * stride + x) as usize];
            *p = op(POINT { x, y }, *p);
        }
    }
}

#[derive(Debug, win32_derive::TryFromEnum, PartialEq, Eq)]
pub enum RasterOp {
    SRCCOPY = 0xcc0020,
    NOTSRCCOPY = 0x330008,
    SRCAND = 0x8800c6,
    PATCOPY = 0xf00021,
    BLACKNESS = 0x000042,
}

impl RasterOp {
    /// Return true if this uses the src surface at all.
    fn uses_src(&self) -> bool {
        match *self {
            RasterOp::PATCOPY | RasterOp::BLACKNESS => false,
            RasterOp::SRCCOPY | RasterOp::NOTSRCCOPY | RasterOp::SRCAND => true,
        }
    }

    /// Return a function that combines a dst and src pixel.
    fn to_binop(&self) -> fn([u8; 4], [u8; 4]) -> [u8; 4] {
        match self {
            RasterOp::SRCCOPY => |_, src| src,
            RasterOp::SRCAND => |dst, src| {
                // https://godbolt.org/z/aT1qc7EKo
                (u32::from_le_bytes(dst) & u32::from_le_bytes(src)).to_le_bytes()
            },
            _ => todo!(),
        }
    }
}

#[win32_derive::dllexport]
pub fn StretchBlt(
    machine: &mut Machine,
    hdcDst: HDC,
    xDst: i32,
    yDst: i32,
    wDst: i32,
    hDst: i32,
    hdcSrc: HDC,
    xSrc: i32,
    ySrc: i32,
    wSrc: i32,
    hSrc: i32,
    rop: Result<RasterOp, u32>,
) -> bool {
    let rop = rop.unwrap();
    if !rop.uses_src() {
        return PatBlt(machine, hdcDst, xDst, yDst, wDst, hDst, Ok(rop));
    }
    let op = rop.to_binop();

    let dst_rect = RECT {
        left: xDst,
        top: yDst,
        right: xDst + wDst,
        bottom: yDst + hDst,
    };

    let src_rect = RECT {
        left: xSrc,
        top: ySrc,
        right: xSrc + wSrc,
        bottom: ySrc + hSrc,
    };

    let src_dc = machine.state.gdi32.dcs.get(hdcSrc).unwrap().borrow();
    let src_bitmap = src_dc.target.get_bitmap(machine);
    let src_bitmap = src_bitmap.borrow();

    let dst_dc = machine.state.gdi32.dcs.get(hdcDst).unwrap().borrow();
    let dst_bitmap = dst_dc.target.get_bitmap(machine);
    let mut dst_bitmap = dst_bitmap.borrow_mut();

    // What does it mean when the src_rect isn't within the src bitmap?
    // sol.exe does this when you drag a card off screen.
    // Clipping here means the xform below will stretch slightly, hrmm.
    // log::info!("src_rect={:x?}", src_rect);
    let src_rect = src_rect.clip(&src_bitmap.to_rect());
    // log::info!("src_rect clipped={:x?}", src_rect);

    let copy_rect = dst_rect.clip(&dst_bitmap.to_rect());
    // log::info!("dst_rect={:x?}", dst_rect);
    // log::info!("dst_bitmap.to_rect()={:x?}", dst_bitmap.to_rect());
    // log::info!("copy_rect={:x?}", copy_rect);

    let mem = machine.memory.mem();

    let mut row: Vec<COLORREF> = Vec::with_capacity((src_rect.right - src_rect.left) as usize);
    row.resize_with(row.capacity(), || COLORREF::from_rgb(0xff, 0, 0));
    for dy in copy_rect.top..copy_rect.bottom {
        let sy = (dy - yDst) * hSrc / hDst + ySrc;
        src_bitmap.read_row(
            mem,
            src_rect.left as u32..src_rect.right as u32,
            sy as u32,
            &mut row,
        );
        dst_bitmap.write_row(
            mem,
            copy_rect.left as u32..copy_rect.right as u32,
            dy as u32,
            &row,
        );
    }
    drop(dst_bitmap);

    dst_dc.target.flush(machine);
    true
}

#[win32_derive::dllexport]
pub fn BitBlt(
    machine: &mut Machine,
    hdcDst: HDC,
    xDst: i32,
    yDst: i32,
    w: i32,
    h: i32,
    hdcSrc: HDC,
    xSrc: i32,
    ySrc: i32,
    rop: Result<RasterOp, u32>,
) -> bool {
    StretchBlt(
        machine, hdcDst, xDst, yDst, w, h, hdcSrc, xSrc, ySrc, w, h, rop,
    )
}

#[win32_derive::dllexport]
pub fn PatBlt(
    machine: &mut Machine,
    hdc: HDC,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    rop: Result<RasterOp, u32>,
) -> bool {
    if hdc.is_null() {
        log::warn!("PatBlt: ignoring null hdc, possibly child window");
        return false;
    }
    let rop = rop.unwrap();
    let dc = machine.state.gdi32.dcs.get(hdc).unwrap().borrow();

    const DEFAULT_COLOR: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];

    let color = match rop {
        RasterOp::PATCOPY => {
            // get brush color
            match machine.state.gdi32.objects.get(dc.brush) {
                Some(Object::Brush(brush)) => match brush.color {
                    Some(color) => color.to_pixel(),
                    None => DEFAULT_COLOR,
                },
                _ => DEFAULT_COLOR,
            }
        }
        RasterOp::BLACKNESS => [0, 0, 0, 0xFF],
        _ => todo!("unimplemented PatBlt with rop={rop:?}"),
    };

    let dst_bitmap = dc.target.get_bitmap(machine);
    let mut dst_bitmap = dst_bitmap.borrow_mut();
    dst_bitmap.format.expect_rgba32();
    let dst_rect = RECT {
        left: x,
        top: y,
        right: x + w,
        bottom: y + h,
    }
    .clip(&dst_bitmap.to_rect());

    fill_pixels(machine.mem(), &mut *dst_bitmap, &dst_rect, |_, _| color);
    drop(dst_bitmap);
    dc.target.flush(machine);
    true
}

#[win32_derive::dllexport]
pub fn CreateBitmap(
    machine: &mut Machine,
    nWidth: u32,
    nHeight: u32,
    nPlanes: u32,
    nBitCount: u32,
    lpBits: u32,
) -> HGDIOBJ {
    assert_eq!(nPlanes, 1);
    let bitmap = match nBitCount {
        1 => {
            let format = PixelFormat::Mono;
            let stride = format.stride(nWidth);
            let len = (nHeight * stride) as usize;
            let mut pixels = Vec::with_capacity(len);
            pixels.resize(len, 0);
            pixels.copy_from_slice(machine.mem().sub32(lpBits, len as u32));
            Bitmap {
                width: nWidth,
                height: nHeight,
                format,
                pixels: PixelData::new_with_owned(pixels.into_boxed_slice()),
            }
        }
        _ => unimplemented!(),
    };
    machine.state.gdi32.objects.add_bitmap(bitmap)
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
    if bi.is_bottom_up() {
        log::warn!("CreateDIBSection: bottom-up bitmap will need flipping");
    }
    let format = match bi.biBitCount {
        32 => PixelFormat::RGBA32,
        16 => PixelFormat::RGB555,
        _ => todo!(),
    };
    match bi.compression().unwrap() {
        BI::BITFIELDS => {
            // TODO: ought to check that .bmiColors masks are the RGBX we expect.
        }
        BI::RGB => {} // ok
        _ => todo!(),
    };

    let byte_count = bi.stride() as u32 * bi.height();
    let heap = kernel32::GetProcessHeap(machine);
    let pixels = kernel32::HeapAlloc(
        machine,
        heap,
        Ok(kernel32::HeapAllocFlags::default()),
        byte_count,
    );

    *ppvBits.unwrap() = pixels;

    let bitmap = Bitmap {
        width: bi.width(),
        height: bi.height(),
        format,
        pixels: PixelData::Ptr(pixels, byte_count),
    };
    machine.state.gdi32.objects.add_bitmap(bitmap)
}

#[win32_derive::dllexport]
pub fn CreateCompatibleBitmap(machine: &mut Machine, hdc: HDC, cx: u32, cy: u32) -> HGDIOBJ {
    let dc = machine.state.gdi32.dcs.get(hdc).unwrap().borrow();
    match &dc.target {
        DCTarget::Memory(hbitmap) => {
            // Cryogoat does a series of:
            //   let dc1 = GetDC(0); // desktop dc
            //   let dc2 = CreateCompatibleDc(dc1); // memory dc
            //   CreateCompatibleBitmap(dc2);
            // The MSDN docs say this sequence should produce a 1bpp bitmap because
            // the initial state of dc1 is monochrome, but I think cryogoat doesn't expect this (?)
        }
        DCTarget::Window(_) => {} // screen has known format
        _ => todo!(),
    };

    let bitmap = Bitmap {
        width: cx,
        height: cy,
        format: PixelFormat::RGBA32,
        pixels: PixelData::new_owned((cx * cy * 4) as usize),
    };
    machine.state.gdi32.objects.add_bitmap(bitmap)
}

#[win32_derive::dllexport]
pub fn SetDIBitsToDevice(
    machine: &mut Machine,
    hdc: HDC,
    xDst: i32,
    yDst: i32,
    w: i32,
    h: i32,
    xSrc: i32,
    ySrc: i32,
    StartScan: u32,
    cLines: u32,
    lpBits: u32,
    lpBmi: u32,
    iUsage: u32,
) -> u32 {
    if StartScan as i32 != 0 {
        todo!()
    }
    if iUsage != DIB_RGB_COLORS {
        todo!();
    }
    let src_bitmap = Bitmap::parse(
        machine.mem().slice(lpBmi..),
        Some((machine.mem().slice(lpBits..), cLines as usize)),
    );

    let dc = machine.state.gdi32.dcs.get(hdc).unwrap().borrow();
    let dst_bitmap = dc.target.get_bitmap(machine);
    let mut dst_bitmap = dst_bitmap.borrow_mut();
    dst_bitmap.format.expect_rgba32();

    let src_rect = RECT {
        left: xSrc,
        top: ySrc,
        right: xSrc + w,
        bottom: ySrc + h,
    };

    let dst_rect = RECT {
        left: xDst,
        top: yDst,
        right: xDst + w,
        bottom: yDst + h,
    };

    // Clip copy region to the relevant bitmap bounds.
    let src_copy_rect = src_rect.clip(&src_bitmap.to_rect());
    let dst_copy_rect = dst_rect.clip(&dst_bitmap.to_rect());
    let copy_rect =
        dst_copy_rect.clip(&src_copy_rect.add(dst_rect.origin().sub(src_rect.origin())));
    let dst_to_src = src_rect.origin().sub(dst_rect.origin());

    let mem = machine.mem();
    let src = src_bitmap.as_rgba(mem);
    fill_pixels(mem, &mut *dst_bitmap, &copy_rect, |p, _| {
        let p = p.add(dst_to_src);
        let mut pixel = src[(p.y * src_bitmap.width as i32 + p.x) as usize];
        pixel[3] = 0xFF;
        pixel
    });
    drop(dst_bitmap);
    dc.target.flush(machine);

    cLines
}

#[win32_derive::dllexport]
pub fn StretchDIBits(
    machine: &mut Machine,
    hdc: HDC,
    xDst: i32,
    yDst: i32,
    wDst: i32,
    hDst: i32,
    xSrc: i32,
    ySrc: i32,
    wSrc: i32,
    hSrc: i32,
    lpBits: u32,
    lpBmi: u32,
    iUsage: u32,
    rop: Result<RasterOp, u32>,
) -> i32 {
    match rop.unwrap() {
        RasterOp::SRCCOPY => {}
        _ => todo!(),
    }

    let src_bitmap = Bitmap::parse(
        machine.mem().slice(lpBmi..),
        Some((machine.mem().slice(lpBits..), hSrc as usize)),
    );

    let dc = machine.state.gdi32.dcs.get(hdc).unwrap().borrow();
    let dst_bitmap = dc.target.get_bitmap(machine);
    let mut dst_bitmap = dst_bitmap.borrow_mut();
    dst_bitmap.format.expect_rgba32();

    let src_rect = RECT {
        left: xSrc,
        top: ySrc,
        right: xSrc + wSrc,
        bottom: ySrc + hSrc,
    };

    let dst_rect = RECT {
        left: xDst,
        top: yDst,
        right: xDst + wDst,
        bottom: yDst + hDst,
    };

    // TODO: do we need to clip src or dst rect?

    let copy_rect = dst_rect;

    let mem = machine.mem();
    let src = src_bitmap.as_rgba(mem);
    fill_pixels(mem, &mut *dst_bitmap, &dst_rect, |p, _| {
        // Translate p from dst to src space.
        // Because we're stretching, we scale to src space rather than clipping.
        let p = p
            .sub(dst_rect.origin())
            .mul(src_rect.size())
            .div(dst_rect.size())
            .add(src_rect.origin());
        let mut pixel = src[(p.y * src_bitmap.width as i32 + p.x) as usize];
        pixel[3] = 0xFF;
        pixel
    });
    drop(dst_bitmap);

    dc.target.flush(machine);

    hSrc
}

pub type BITMAPINFO = u32; // TODO

#[win32_derive::dllexport]
pub fn GetDIBits(
    sys: &dyn System,
    hdc: HDC,
    hbm: HBITMAP,
    start: u32,
    cLines: u32,
    lpvBits: Option<&mut u8>,
    lpbmi: Option<&mut BITMAPINFO>,
    usage: u32, /* DIB_USAGE */
) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn CreateDIBitmap(
    sys: &dyn System,
    hdc: HDC,
    pbmih: Option<&mut BITMAPINFOHEADER>,
    flInit: u32,
    pjBits: Option<&mut u8>,
    pbmi: Option<&mut BITMAPINFO>,
    iUsage: u32, /* DIB_USAGE */
) -> HBITMAP {
    todo!()
}
