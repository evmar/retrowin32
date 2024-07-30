use super::{BitmapType, DCTarget, Object, BITMAPINFOHEADER, HDC, HGDIOBJ};
use crate::{
    machine::Machine,
    winapi::{
        bitmap::{BitmapMono, BitmapRGBA32, PixelData, BI},
        kernel32,
    },
};
use std::cmp::min;

const TRACE_CONTEXT: &'static str = "gdi32/bitmap";

#[allow(dead_code)]
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

/// Copy pixels from src to dst.  Asserts that everything has been appropriately clipped.
/// flush_alpha is true when the output drops alpha channel (e.g. Window backing store).
fn bit_blt(
    dst: &mut [[u8; 4]],
    mut dx: isize,
    mut dy: isize,
    dstride: usize,
    mut w: isize,
    mut h: isize,
    src: &[[u8; 4]],
    mut sx: isize,
    mut sy: isize,
    sstride: usize,
    flush_alpha: bool,
    rop: u32,
) {
    let min_x = min(dx, sx);
    let min_y = min(dy, sy);
    if min_x < 0 {
        dx -= min_x;
        sx -= min_x;
        w += min_x;
    }
    if min_y < 0 {
        dy -= min_y;
        sy -= min_y;
        h += min_y;
    }
    w = min(w, dstride as isize - dx);
    w = min(w, sstride as isize - sx);
    if w <= 0 || h <= 0 {
        return;
    }

    for row in 0..h {
        let dst_off = ((dy + row) * dstride as isize) + dx;
        let src_off = ((sy + row) * sstride as isize) + sx;
        if dst_off + w > dst.len() as isize || src_off + w > src.len() as isize {
            break;
        }
        let dst_row = &mut dst[dst_off as usize..][..w as usize];
        let src_row = &src[src_off as usize..][..w as usize];
        match rop {
            SRCCOPY => {
                dst_row.copy_from_slice(src_row);
            }
            NOTSRCCOPY => {
                for (d, s) in dst_row.iter_mut().zip(src_row.iter()) {
                    d[0] = !s[0];
                    d[1] = !s[1];
                    d[2] = !s[2];
                    d[3] = s[3];
                }
            }
            SRCAND => {
                for (d, s) in dst_row.iter_mut().zip(src_row.iter()) {
                    d[0] &= s[0];
                    d[1] &= s[1];
                    d[2] &= s[2];
                    d[3] &= s[3];
                }
            }
            _ => todo!("unimplemented BitBlt with rop={:x}", rop),
        }
        if flush_alpha {
            for p in dst_row {
                p[3] = 0xFF;
            }
        }
    }
}

fn pat_blt(
    dst: &mut [[u8; 4]],
    mut x: isize,
    mut y: isize,
    stride: usize,
    mut w: isize,
    mut h: isize,
    color: [u8; 4],
    rop: u32,
) {
    if x < 0 {
        w += x;
        x = 0;
    }
    if y < 0 {
        h += y;
        y = 0;
    }
    if w <= 0 || h <= 0 {
        return;
    }
    for row in 0..h {
        let dst_off = ((y + row) * stride as isize) + x;
        if dst_off + w > dst.len() as isize {
            break;
        }
        let dst_row = &mut dst[dst_off as usize..][..w as usize];
        match rop {
            PATCOPY => {
                dst_row.fill(color);
            }
            BLACKNESS => {
                dst_row.fill([0, 0, 0, 0xFF]);
            }
            _ => todo!("unimplemented PatBlt with rop={:x}", rop),
        }
    }
}

const SRCCOPY: u32 = 0xcc0020;
const NOTSRCCOPY: u32 = 0x330008;
const SRCAND: u32 = 0x8800c6;
const PATCOPY: u32 = 0xf00021;
const BLACKNESS: u32 = 0x000042;

#[win32_derive::dllexport]
pub fn BitBlt(
    machine: &mut Machine,
    hdc: HDC,
    x: i32,
    y: i32,
    cx: u32,
    cy: u32,
    hdcSrc: HDC,
    x1: i32,
    y1: i32,
    rop: u32,
) -> bool {
    let src_dc = machine.state.gdi32.dcs.get(hdcSrc).unwrap();
    let src_bitmap = match src_dc.target {
        DCTarget::Memory(bitmap) => {
            let obj = machine.state.gdi32.objects.get(bitmap).unwrap();
            match obj {
                Object::Bitmap(BitmapType::RGBA32(bmp)) => bmp.clone(),
                _ => unimplemented!("{:?}", obj),
            }
        }
        DCTarget::Window(hwnd) => {
            let window = machine.state.user32.windows.get_mut(hwnd).unwrap();
            window.bitmap_mut().clone()
        }
        _ => todo!(),
    };
    let src = src_bitmap.pixels_slice(machine.emu.memory.mem());

    let dst_dc = machine.state.gdi32.dcs.get(hdc).unwrap();
    match dst_dc.target {
        DCTarget::Memory(obj) => {
            let dst = match machine.state.gdi32.objects.get_mut(obj).unwrap() {
                Object::Bitmap(BitmapType::RGBA32(bmp)) => bmp,
                _ => unimplemented!("{:?}", obj),
            };

            bit_blt(
                dst.pixels.as_slice_mut(),
                x as isize,
                y as isize,
                dst.width as usize,
                cx as isize,
                cy as isize,
                src,
                x1 as isize,
                y1 as isize,
                src_bitmap.width as usize,
                true,
                rop,
            );
        }
        DCTarget::Window(hwnd) => {
            let window = machine.state.user32.windows.get_mut(hwnd).unwrap();
            let dst = window.bitmap_mut();

            // Clip to src/dst regions.
            if x >= dst.width as i32
                || x1 >= src_bitmap.width as i32
                || y >= dst.height as i32
                || y1 >= src_bitmap.height as i32
            {
                return true;
            }

            bit_blt(
                dst.pixels.as_slice_mut(),
                x as isize,
                y as isize,
                dst.width as usize,
                cx as isize,
                cy as isize,
                src,
                x1 as isize,
                y1 as isize,
                src_bitmap.width as usize,
                true,
                rop,
            );

            window.flush_pixels(machine.emu.memory.mem());
        }
        DCTarget::DirectDrawSurface(ptr) => {
            let surface = machine.state.ddraw.surfaces.get_mut(&ptr).unwrap();

            assert!(x == 0 && y == 0 && x1 == 0 && y1 == 0);
            assert!(cx == surface.width && cy == surface.height);
            assert!(surface.width == src_bitmap.width && surface.height == src_bitmap.height);

            surface.host.write_pixels(src);
        }
    }
    true
}

#[win32_derive::dllexport]
pub fn StretchBlt(
    machine: &mut Machine,
    hdcDest: HDC,
    xDest: i32,
    yDest: i32,
    wDest: u32,
    hDest: u32,
    hdcSrc: HDC,
    xSrc: i32,
    ySrc: i32,
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

#[win32_derive::dllexport]
pub fn PatBlt(machine: &mut Machine, hdc: HDC, x: i32, y: i32, w: i32, h: i32, rop: u32) -> bool {
    let dc = machine.state.gdi32.dcs.get(hdc).unwrap();

    const DEFAULT_COLOR: [u8; 4] = [255, 255, 255, 255];
    // get brush color
    let color = match machine.state.gdi32.objects.get(dc.brush) {
        Some(Object::Brush(brush)) => match brush.color {
            Some(color) => color.to_pixel(),
            None => DEFAULT_COLOR,
        },
        _ => DEFAULT_COLOR,
    };

    match dc.target {
        DCTarget::Memory(hbitmap) => {
            let bitmap = match machine.state.gdi32.objects.get_mut(hbitmap).unwrap() {
                Object::Bitmap(BitmapType::RGBA32(bmp)) => bmp,
                _ => unimplemented!(),
            };
            pat_blt(
                bitmap.pixels.as_slice_mut(),
                x as isize,
                y as isize,
                bitmap.width as usize,
                w as isize,
                h as isize,
                color,
                rop,
            );
        }
        DCTarget::Window(hwnd) => {
            if hwnd.to_raw() != 1 {
                log::warn!("PatBlt to sub window");
                return false;
            }
            let window = machine.state.user32.windows.get_mut(hwnd).unwrap();
            let bitmap = window.bitmap_mut();
            pat_blt(
                bitmap.pixels.as_slice_mut(),
                x as isize,
                y as isize,
                bitmap.width as usize,
                w as isize,
                h as isize,
                color,
                rop,
            );
            window.flush_pixels(machine.emu.memory.mem());
        }
        _ => todo!(),
    };
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
                pixels: PixelData::Owned(pixels.into_boxed_slice()),
            };
            BitmapType::Mono(bitmap)
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
        .add(Object::Bitmap(BitmapType::RGBA32(bitmap)))
}

#[win32_derive::dllexport]
pub fn CreateCompatibleBitmap(machine: &mut Machine, hdc: HDC, cx: u32, cy: u32) -> HGDIOBJ {
    let dc = machine.state.gdi32.dcs.get(hdc).unwrap();
    match dc.target {
        DCTarget::Memory(hbitmap) => match machine.state.gdi32.objects.get_mut(hbitmap).unwrap() {
            Object::Bitmap(BitmapType::RGBA32(_)) => {}
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

    let mut pixels = Vec::new();
    pixels.resize((cx * cy) as usize, [0; 4]);
    let bitmap = BitmapRGBA32 {
        width: cx,
        height: cy,
        pixels: PixelData::Owned(pixels.into_boxed_slice()),
    };
    machine
        .state
        .gdi32
        .objects
        .add(Object::Bitmap(BitmapType::RGBA32(bitmap)))
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
    lpbmi: u32,
    ColorUse: u32,
) -> u32 {
    if StartScan != ySrc || cLines != h {
        todo!()
    }
    if ColorUse != DIB_RGB_COLORS {
        todo!();
    }
    let src_bitmap = BitmapRGBA32::parseBMPv3(
        machine.mem().slice(lpbmi..),
        Some((
            machine.mem().slice(lpvBits..).as_slice_todo(),
            cLines as usize,
        )),
    );
    let src = src_bitmap.pixels_slice(machine.emu.memory.mem());

    let dc = machine.state.gdi32.dcs.get(hdc).unwrap();
    let (dst, flush_alpha) = match dc.target {
        DCTarget::Memory(hbitmap) => match machine.state.gdi32.objects.get_mut(hbitmap).unwrap() {
            Object::Bitmap(BitmapType::RGBA32(b)) => (b, false),
            _ => todo!(),
        },
        DCTarget::Window(hwnd) => {
            let window = machine.state.user32.windows.get_mut(hwnd).unwrap();
            (window.bitmap_mut(), true)
        }
        _ => todo!(),
    };

    bit_blt(
        &mut dst.pixels.as_slice_mut(),
        xDest as isize,
        yDest as isize,
        dst.width as usize,
        w as isize,
        std::cmp::min(h, dst.height - yDest) as isize,
        src,
        xSrc as isize,
        ySrc as isize,
        src_bitmap.width as usize,
        flush_alpha,
        SRCCOPY,
    );

    match dc.target {
        DCTarget::Window(hwnd) => {
            let window = machine.state.user32.windows.get_mut(hwnd).unwrap();
            window.flush_pixels(machine.emu.memory.mem());
        }
        _ => {}
    }

    cLines
}

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum RasterOp {
    SRCCOPY = 0xcc0020,
}

#[win32_derive::dllexport]
pub fn StretchDIBits(
    machine: &mut Machine,
    hdc: HDC,
    // XXX these are signed ints in the MSDN docs
    xDest: u32,
    yDest: u32,
    DestWidth: u32,
    DestHeight: u32,
    xSrc: u32,
    ySrc: u32,
    SrcWidth: u32,
    SrcHeight: u32,
    lpBits: u32,
    lpbmi: u32,
    iUsage: u32,
    rop: Result<RasterOp, u32>,
) -> u32 {
    if SrcWidth != DestWidth || SrcHeight != DestHeight {
        log::warn!("TODO: StretchDIBits doesn't stretch");
    }

    SetDIBitsToDevice(
        machine, hdc, xDest, yDest, SrcWidth, SrcHeight, xSrc, ySrc, 0, SrcHeight, lpBits, lpbmi,
        iUsage,
    )
}
