use super::{BitmapType, DCTarget, Object, BITMAPINFOHEADER, HDC, HGDIOBJ};
use crate::{
    machine::Machine,
    winapi::{
        bitmap::{BitmapMono, BitmapRGBA32, PixelData, BI},
        kernel32,
    },
};

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
    dx: usize,
    dy: usize,
    dstride: usize,
    w: usize,
    h: usize,
    src: &[[u8; 4]],
    sx: usize,
    sy: usize,
    sstride: usize,
    flush_alpha: bool,
) {
    assert!(dstride >= w);
    assert!(sstride >= w);
    for row in 0..h {
        let dst_row = &mut dst[(((dy + row) * dstride) + dx)..][..w];
        let src_row = &src[(((sy + row) * sstride) + sx)..][..w];
        dst_row.copy_from_slice(src_row);
        if flush_alpha {
            for p in dst_row {
                p[3] = 0xFF;
            }
        }
    }
}

const SRCCOPY: u32 = 0xcc0020;
const NOTSRCCOPY: u32 = 0x330008;

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
    match rop {
        SRCCOPY => {}
        NOTSRCCOPY => log::warn!("TODO: ignoring NOTSRCCOPY"),
        _ => todo!(),
    }

    let src_dc = machine.state.gdi32.dcs.get(hdcSrc).unwrap();
    let src_bitmap = match src_dc.target {
        DCTarget::Memory(bitmap) => {
            let obj = machine.state.gdi32.objects.get(bitmap).unwrap();
            match obj {
                Object::Bitmap(BitmapType::RGBA32(bmp)) => bmp,
                _ => unimplemented!("{:?}", obj),
            }
        }
        _ => todo!(),
    };
    let src = src_bitmap.pixels_slice(machine.emu.memory.mem());

    let dst_dc = machine.state.gdi32.dcs.get(hdc).unwrap();
    match dst_dc.target {
        DCTarget::Memory(obj) => {
            let _bitmap = match machine.state.gdi32.objects.get_mut(obj).unwrap() {
                Object::Bitmap(bmp) => bmp,
                _ => unimplemented!("{:?}", obj),
            };

            // TODO: we can't BltBlt here because of borrow checker -- can't have two
            // bitmaps at the same time.  Need to revisit how object handles work.
            log::warn!("TODO: BltBlt between bitmaps");
            // assert_eq!(bitmap.format, PixelFormat::RGBA32);
            // let dst = bitmap::bytes_as_rgba_mut(bitmap.pixels.as_slice_mut());
            // bit_blt(
            //     dst,
            //     x as usize,
            //     y as usize,
            //     bitmap.width as usize,
            //     cx as usize,
            //     cy as usize,
            //     src,
            //     x1 as usize,
            //     y1 as usize,
            //     src_bitmap.width as usize,
            // );
        }
        DCTarget::Window(hwnd) => {
            let window = machine.state.user32.windows.get_mut(hwnd).unwrap();
            let dst = window.bitmap_mut(&mut *machine.host);

            // Clip to src/dst regions.
            if x >= dst.width
                || x1 >= src_bitmap.width
                || y >= dst.height
                || y1 >= src_bitmap.height
            {
                return true;
            }
            let cx = std::cmp::min(cx, std::cmp::min(dst.width - x, src_bitmap.width - x1));
            let cy = std::cmp::min(cy, std::cmp::min(dst.height - y, src_bitmap.height - y1));

            bit_blt(
                dst.pixels.as_slice_mut(),
                x as usize,
                y as usize,
                dst.width as usize,
                cx as usize,
                cy as usize,
                src,
                x1 as usize,
                y1 as usize,
                src_bitmap.width as usize,
                true,
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

    let byte_count = bi.stride() * bi.height();
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

    let src_bitmap = BitmapRGBA32::parse(
        header,
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
            (window.bitmap_mut(&mut *machine.host), true)
        }
        _ => todo!(),
    };

    bit_blt(
        &mut dst.pixels.as_slice_mut(),
        xDest as usize,
        yDest as usize,
        dst.width as usize,
        w as usize,
        h as usize,
        src,
        xSrc as usize,
        ySrc as usize,
        src_bitmap.width as usize,
        flush_alpha,
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

#[win32_derive::dllexport]
pub fn StretchDIBits(
    _machine: &mut Machine,
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
    lpbmi: Option<&BITMAPINFOHEADER>,
    iUsage: u32,
    rop: u32,
) -> i32 {
    log::warn!("TODO: StretchDIBits");
    SrcHeight // success
}
