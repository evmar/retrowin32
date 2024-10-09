use super::{Bitmap, DCTarget, Object, BITMAPINFOHEADER, HDC, HGDIOBJ};
use crate::{
    machine::Machine,
    winapi::{
        bitmap::{BitmapMono, BitmapRGBA32, PixelData, BI},
        kernel32,
        types::RECT,
    },
};
use memory::Mem;
use std::rc::Rc;

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
fn fill_pixels(
    mem: Mem,
    dst: &BitmapRGBA32,
    r: &RECT,
    mut op: impl FnMut(i32, i32, [u8; 4]) -> [u8; 4],
) {
    dst.pixels.with_slice(mem, |pixels| {
        let stride = dst.width as i32;
        for y in r.top..r.bottom {
            for x in r.left..r.right {
                let p = &mut pixels[(y * stride + x) as usize];
                *p = op(x, y, *p);
            }
        }
    });
}

#[derive(Debug, win32_derive::TryFromEnum, PartialEq, Eq)]
pub enum RasterOp {
    SRCCOPY = 0xcc0020,
    NOTSRCCOPY = 0x330008,
    SRCAND = 0x8800c6,
    PATCOPY = 0xf00021,
    BLACKNESS = 0x000042,
}

#[win32_derive::dllexport]
pub fn BitBlt(
    machine: &mut Machine,
    hdc: HDC,
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
    hdcSrc: HDC,
    x1: i32,
    y1: i32,
    rop: Result<RasterOp, u32>,
) -> bool {
    let dst_rect = RECT {
        left: x,
        top: y,
        right: x + cx,
        bottom: y + cy,
    };

    let src_rect = RECT {
        left: x1,
        top: y1,
        right: x1 + cx,
        bottom: y1 + cy,
    };

    let op = match rop.unwrap() {
        RasterOp::BLACKNESS => {
            // It seems like passing null as `hdcSrc` when using BLACKNESS is supported on Windows.
            return PatBlt(machine, hdc, x, y, cx, cy, Ok(RasterOp::BLACKNESS));
        }
        RasterOp::SRCCOPY => |dst: [u8; 4], src: [u8; 4]| src,
        RasterOp::SRCAND => |dst: [u8; 4], src: [u8; 4]| {
            [
                dst[0] & src[0],
                dst[1] & src[1],
                dst[2] & src[2],
                dst[3] & src[3],
            ]
        },
        _ => todo!(),
    };

    let src_dc = machine.state.gdi32.dcs.get(hdcSrc).unwrap();
    let target = src_dc.target;
    let src_bitmap = target.get_bitmap(machine).unwrap();

    let dst_dc = machine.state.gdi32.dcs.get(hdc).unwrap();
    let target = dst_dc.target;

    // Special case: BitBlt to a DirectDrawSurface does a full copy, and the rest is unimplemented.
    match dst_dc.target {
        DCTarget::DirectDrawSurface(ptr) => {
            let surface = machine.state.ddraw.surfaces.get_mut(&ptr).unwrap();

            assert!(src_rect == dst_rect);
            assert!(surface.to_rect() == dst_rect);

            src_bitmap
                .pixels
                .with_slice(machine.emu.memory.mem(), |src| {
                    surface.host.write_pixels(src)
                });
            return true;
        }
        _ => {}
    }

    let dst_bitmap = target.get_bitmap(machine).unwrap();

    // Copy region is the intersection of the two rectangles, which both also need to be clipped to their bitmaps.
    let dst_copy_rect = dst_rect.clip(&dst_bitmap.to_rect());
    let src_copy_rect = src_rect.clip(&src_bitmap.to_rect());
    let copy_rect = dst_copy_rect.clip(
        // translate src_rect into dst_rect space
        &src_copy_rect.add(dst_rect.origin().sub(src_rect.origin())),
    );

    let mem = machine.emu.memory.mem();
    src_bitmap.pixels.with_slice(mem, |src| {
        fill_pixels(mem, &*dst_bitmap, &copy_rect, |dx, dy, p| {
            let x = x1 + dx - x;
            let y = y1 + dy - y;
            let mut px = op(p, src[(y * src_bitmap.width as i32 + x) as usize]);
            px[3] = 0xFF; // clear alpha
            px
        });
    });

    target.flush(machine);
    true
}

#[win32_derive::dllexport]
pub fn StretchBlt(
    machine: &mut Machine,
    hdcDest: HDC,
    xDest: i32,
    yDest: i32,
    wDest: i32,
    hDest: i32,
    hdcSrc: HDC,
    xSrc: i32,
    ySrc: i32,
    wSrc: i32,
    hSrc: i32,
    rop: Result<RasterOp, u32>,
) -> bool {
    if wDest != wSrc || hDest != hSrc {
        todo!("unimp: StretchBlt with actual stretching");
    }
    BitBlt(
        machine, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, rop,
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
    let rop = rop.unwrap();
    let Some(dc) = machine.state.gdi32.dcs.get(hdc) else {
        log::warn!("PatBlt: ignoring invalid DC {hdc:?}");
        return false;
    };

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

    let target = dc.target;
    let dst_bitmap = target.get_bitmap(machine).unwrap();
    let dst_rect = RECT {
        left: x,
        top: y,
        right: x + w,
        bottom: y + h,
    }
    .clip(&dst_bitmap.to_rect());

    fill_pixels(machine.mem(), &*dst_bitmap, &dst_rect, |_, _, _| color);
    target.flush(machine);
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
            let stride = BitmapMono::stride(nWidth);
            let len = (nHeight * stride) as usize;
            let mut pixels = Vec::with_capacity(len);
            pixels.resize(len, 0);
            let bitmap = BitmapMono {
                width: nWidth,
                height: nHeight,
                pixels: PixelData::new_with_owned(pixels.into_boxed_slice()),
            };
            Bitmap::Mono(bitmap)
        }
        _ => unimplemented!(),
    };
    machine.state.gdi32.objects.add(Object::Bitmap(bitmap))
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
        log::warn!("CreateDIBSection: bitmap may need flipping");
    }
    if bi.biBitCount != 32 {
        todo!()
    }
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

    let bitmap = BitmapRGBA32 {
        width: bi.width(),
        height: bi.height(),
        pixels: PixelData::Ptr(pixels, byte_count),
    };
    machine
        .state
        .gdi32
        .objects
        .add(Object::Bitmap(Bitmap::RGBA32(Rc::new(bitmap))))
}

#[win32_derive::dllexport]
pub fn CreateCompatibleBitmap(machine: &mut Machine, hdc: HDC, cx: u32, cy: u32) -> HGDIOBJ {
    let dc = machine.state.gdi32.dcs.get(hdc).unwrap();
    match dc.target {
        DCTarget::Memory(hbitmap) => match machine.state.gdi32.objects.get_mut(hbitmap).unwrap() {
            Object::Bitmap(Bitmap::RGBA32(_)) => {}
            Object::Bitmap(_) => {
                // Cryogoat does a series of:
                //   let dc1 = GetDC(0); // desktop dc
                //   let dc2 = CreateCompatibleDc(dc1); // memory dc
                //   CreateCompatibleBitmap(dc2);
                // The MSDN docs say this sequence should produce a 1bpp bitmap because
                // the initial state of dc1 is monochrome, but I think cryogoat doesn't expect this (?)
            }
            _ => todo!(),
        },
        DCTarget::Window(_) => {} // screen has known format
        _ => todo!(),
    };

    let bitmap = BitmapRGBA32 {
        width: cx,
        height: cy,
        pixels: PixelData::new_owned(cx as usize * cy as usize),
    };
    machine
        .state
        .gdi32
        .objects
        .add(Object::Bitmap(Bitmap::RGBA32(Rc::new(bitmap))))
}

#[win32_derive::dllexport]
pub fn SetDIBitsToDevice(
    machine: &mut Machine,
    hdc: HDC,
    xDest: i32,
    yDest: i32,
    w: i32,
    h: i32,
    xSrc: i32,
    ySrc: i32,
    StartScan: u32,
    cLines: u32,
    lpvBits: u32,
    lpbmi: u32,
    ColorUse: u32,
) -> u32 {
    if StartScan as i32 != ySrc || cLines as i32 != h {
        todo!()
    }
    if ColorUse != DIB_RGB_COLORS {
        todo!();
    }
    let src_bitmap = BitmapRGBA32::parse(
        machine.mem().slice(lpbmi..),
        Some((machine.mem().slice(lpvBits..), cLines as usize)),
    );

    let dc = machine.state.gdi32.dcs.get(hdc).unwrap();
    let target = dc.target;
    let dst_bitmap = target.get_bitmap(machine).unwrap();

    let src_rect = RECT {
        left: xSrc,
        top: ySrc,
        right: xSrc + w,
        bottom: ySrc + h,
    }
    .clip(&src_bitmap.to_rect());

    let dst_rect = RECT {
        left: xDest,
        top: yDest,
        right: xDest + w,
        bottom: yDest + h,
    }
    .clip(&dst_bitmap.to_rect());

    let copy_rect = dst_rect.clip(&src_rect.add(dst_rect.origin().sub(src_rect.origin())));

    let mem = machine.emu.memory.mem();
    src_bitmap.pixels.with_slice(mem, |src| {
        fill_pixels(mem, &*dst_bitmap, &copy_rect, |dx, dy, _| {
            let x = dx - xDest + xSrc;
            let y = dy - yDest + ySrc;
            let mut pixel = src[(y * src_bitmap.width as i32 + x) as usize];
            pixel[3] = 0xFF;
            pixel
        });
    });

    target.flush(machine);

    cLines
}

#[win32_derive::dllexport]
pub fn StretchDIBits(
    machine: &mut Machine,
    hdc: HDC,
    xDest: i32,
    yDest: i32,
    DestWidth: i32,
    DestHeight: i32,
    xSrc: i32,
    ySrc: i32,
    SrcWidth: i32,
    SrcHeight: i32,
    lpBits: u32,
    lpbmi: u32,
    iUsage: u32,
    rop: Result<RasterOp, u32>,
) -> u32 {
    if SrcWidth != DestWidth || SrcHeight != DestHeight {
        log::warn!("TODO: StretchDIBits doesn't stretch");
    }

    match rop.unwrap() {
        RasterOp::SRCCOPY => {}
        _ => todo!(),
    }

    SetDIBitsToDevice(
        machine,
        hdc,
        xDest,
        yDest,
        SrcWidth,
        SrcHeight,
        xSrc,
        ySrc,
        0,
        SrcHeight as u32,
        lpBits,
        lpbmi,
        iUsage,
    )
}
