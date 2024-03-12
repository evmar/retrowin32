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
        DCTarget::Window(_) => todo!(),
        DCTarget::DirectDrawSurface(_) => todo!(),
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

            let window_width = window.width;
            let dst = window.pixels_mut(&mut *machine.host);

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
                src_bitmap.width as usize,
            );

            window.flush_pixels();
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

    let bitmap = BitmapRGBA32 {
        width: bi.width(),
        height: bi.height(),
        pixels: PixelData::Ptr(pixels),
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
        DCTarget::Memory(_) => todo!(),
        DCTarget::Window(_) => {} // screen has known format
        DCTarget::DirectDrawSurface(_) => todo!(),
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
    let (dst, dst_stride) = match dc.target {
        DCTarget::Memory(hbitmap) => {
            let dst_bitmap = match machine.state.gdi32.objects.get_mut(hbitmap).unwrap() {
                Object::Bitmap(BitmapType::RGBA32(b)) => b,
                _ => todo!(),
            };
            (dst_bitmap.pixels.as_slice_mut(), dst_bitmap.width)
        }
        DCTarget::Window(hwnd) => {
            let window = machine.state.user32.windows.get_mut(hwnd).unwrap();
            let window_width = window.width;
            let pixels = window.pixels_mut(&mut *machine.host);
            (pixels, window_width)
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
