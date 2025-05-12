//! Code dealing with pixel buffers, as found in both gdi32 and user32.

#![allow(non_snake_case)]

use std::ops::Range;

use super::{gdi32::COLORREF, *};
use memory::{Extensions, ExtensionsMut, Mem};

#[derive(Debug, Eq, PartialEq, win32_derive::TryFromEnum)]
pub enum BI {
    RGB = 0,
    RLE8 = 1,
    RLE4 = 2,
    BITFIELDS = 3,
    JPEG = 4,
    PNG = 5,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BITMAPFILEHEADER {
    pub bfType: u16,
    pub bfSize: u32,
    pub bfReserved1: u16,
    pub bfReserved2: u16,
    pub bfOffBits: u32,
}
unsafe impl memory::Pod for BITMAPFILEHEADER {}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BITMAPCOREHEADER {
    pub bcSize: DWORD,
    pub bcWidth: WORD,
    pub bcHeight: WORD,
    pub bcPlanes: WORD,
    pub bcBitCount: WORD,
}
unsafe impl memory::Pod for BITMAPCOREHEADER {}
impl BITMAPCOREHEADER {
    pub fn stride(&self) -> usize {
        // Bitmap row stride is padded out to 4 bytes per row.
        ((((self.bcWidth * self.bcBitCount) as usize) + 31) & !31) >> 3
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BITMAPINFOHEADER {
    pub biSize: DWORD,
    pub biWidth: DWORD,
    pub biHeight: DWORD,
    pub biPlanes: WORD,
    pub biBitCount: WORD,
    pub biCompression: DWORD,
    pub biSizeImage: DWORD,
    pub biXPelsPerMeter: DWORD,
    pub biYPelsPerMeter: DWORD,
    pub biClrUsed: DWORD,
    pub biClrImportant: DWORD,
}
unsafe impl memory::Pod for BITMAPINFOHEADER {}
impl BITMAPINFOHEADER {
    pub fn width(&self) -> u32 {
        self.biWidth
    }
    pub fn stride(&self) -> usize {
        // Bitmap row stride is padded out to 4 bytes per row.
        (((self.biWidth as usize * self.biBitCount as usize) + 31) & !31) >> 3
    }
    pub fn height(&self) -> u32 {
        // Height is negative if top-down DIB.
        (self.biHeight as i32).abs() as u32
    }
    pub fn is_bottom_up(&self) -> bool {
        (self.biHeight as i32) > 0
    }
    pub fn compression(&self) -> Result<BI, u32> {
        BI::try_from(self.biCompression)
    }
}

/// The parsed header of a bitmap, either v2 (BITMAPCOREHEADER) or v3 (BITMAPINFOHEADER).
struct BitmapInfo<'a> {
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub is_bottom_up: bool,
    pub bit_count: u8,
    pub compression: BI,
    pub palette_entry_size: usize,
    pub palette: &'a [u8],
    /// The total size in memory of the underlying header+palette.
    pub header_length: usize,
}

impl<'a> BitmapInfo<'a> {
    // TODO: when parsing a bitmap from memory it's unclear how much memory we'll need
    // to read until we've read the bitmap header.  This means the caller cannot know how
    // big of a slice to provide.

    fn parse(buf: &'a [u8]) -> Self {
        let header_size = buf.get_pod::<u32>(0);
        match header_size {
            12 => Self::parseBMPv2(&buf.get_pod::<BITMAPCOREHEADER>(0), &buf[12..]),
            40 => Self::parseBMPv3(&buf.get_pod::<BITMAPINFOHEADER>(0), &buf[40..]),
            _ => unimplemented!("unimplemented bitmap header size {}", header_size),
        }
    }

    /// buf is the bytes following the header.
    fn parseBMPv2(header: &BITMAPCOREHEADER, buf: &'a [u8]) -> Self {
        let palette_len = if header.bcBitCount <= 8 {
            2usize.pow(header.bcBitCount as u32)
        } else {
            todo!();
        };
        let palette_entry_size = 3usize;
        let palette_size = palette_len * palette_entry_size;
        let palette = &buf[..palette_size];

        BitmapInfo {
            width: header.bcWidth as usize,
            height: header.bcHeight as usize,
            stride: header.stride(),
            is_bottom_up: true, // MSDN: "BITMAPCOREHEADER bitmaps cannot be top-down bitmaps"
            bit_count: header.bcBitCount as u8,
            compression: BI::RGB,
            palette_entry_size,
            palette,
            header_length: 12 + palette_size,
        }
    }

    /// buf is the bytes following the header.
    fn parseBMPv3(header: &BITMAPINFOHEADER, buf: &'a [u8]) -> Self {
        if header.biCompression != BI::RGB as u32 {
            todo!("compression {:?}", header.biCompression);
        }
        let palette_len = if header.biClrUsed > 0 {
            header.biClrUsed as usize
        } else if header.biBitCount <= 8 {
            2usize.pow(header.biBitCount as u32)
        } else {
            todo!()
        };
        let palette_entry_size = 4usize;
        let palette_size = palette_len * palette_entry_size;
        let palette = &buf[..palette_size];

        BitmapInfo {
            width: header.biWidth as usize,
            height: header.height() as usize,
            stride: header.stride(),
            is_bottom_up: header.is_bottom_up(),
            bit_count: header.biBitCount as u8,
            compression: BI::RGB,
            palette_entry_size,
            palette,
            header_length: 40 + palette_size,
        }
    }
}

pub enum PixelData {
    Owned(Box<[u8]>),
    Ptr(u32, u32),
}

pub fn transmute_pixels<Tin: memory::Pod, Tout: memory::Pod>(px: &[Tin]) -> &[Tout] {
    unsafe {
        std::slice::from_raw_parts(
            px.as_ptr() as *const _,
            px.len() * std::mem::size_of::<Tin>() / std::mem::size_of::<Tout>(),
        )
    }
}

pub fn transmute_pixels_mut<Tin: memory::Pod, Tout: memory::Pod>(px: &mut [Tin]) -> &mut [Tout] {
    unsafe {
        std::slice::from_raw_parts_mut(
            px.as_mut_ptr() as *mut _,
            px.len() * std::mem::size_of::<Tin>() / std::mem::size_of::<Tout>(),
        )
    }
}

impl PixelData {
    pub fn new_owned(size: usize) -> Self {
        let buf = {
            let mut p = Vec::with_capacity(size);
            p.resize(size, Default::default());
            p.into_boxed_slice()
        };
        PixelData::new_with_owned(buf)
    }

    pub fn new_with_owned(buf: Box<[u8]>) -> Self {
        PixelData::Owned(buf)
    }

    pub fn bytes<'a>(&'a self, mem: Mem<'a>) -> &'a [u8] {
        match *self {
            PixelData::Owned(ref b) => &*b,
            PixelData::Ptr(addr, len) => mem.sub32(addr, len),
        }
    }

    pub fn bytes_mut<'a>(&'a mut self, mem: Mem<'a>) -> &'a mut [u8] {
        match *self {
            PixelData::Owned(ref mut b) => &mut *b,
            PixelData::Ptr(addr, len) => mem.sub32_mut(addr, len),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PixelFormat {
    RGBA32,
    RGB555,
    Mono,
}

impl PixelFormat {
    pub fn expect_rgba32(&self) {
        match self {
            PixelFormat::RGBA32 => {}
            _ => todo!("expected RGBA32 bitmap"),
        }
    }

    pub fn stride(&self, width: u32) -> u32 {
        // Align to 2 byte boundary.
        let bpp = self.bits_per_pixel();
        ((width * bpp + 0xF) & !0xF) >> 3
    }

    pub fn bits_per_pixel(&self) -> u32 {
        match self {
            PixelFormat::RGBA32 => 32,
            PixelFormat::RGB555 => 16,
            PixelFormat::Mono => 1,
        }
    }

    fn decode_rgb555(val: u16) -> COLORREF {
        // xrrr_rrgg_gggb_bbbb
        let r = ((val >> 10) & 0x1F) as u8;
        let g = ((val >> 5) & 0x1F) as u8;
        let b = (val & 0x1F) as u8;
        COLORREF::from_rgb(r << 3, g << 3, b << 3)
    }

    fn encode_rgb555(val: COLORREF) -> u16 {
        let (mut r, mut g, mut b) = val.to_rgb();
        r = r >> 3;
        g = g >> 3;
        b = b >> 3;
        let out = ((r as u16) << 10) | ((g as u16) << 5) | b as u16;
        out
    }

    #[allow(dead_code)]
    fn decode_rgb565(val: u16) -> COLORREF {
        // rrrr_rggg_gggb_bbbb
        let r = ((val >> 11) & 0x1F) as u8;
        let g = ((val >> 5) & 0x3F) as u8;
        let b = (val & 0x1F) as u8;
        COLORREF::from_rgb(r << 3, g << 3, b << 3)
    }

    #[allow(dead_code)]
    fn encode_rgb565(val: COLORREF) -> u16 {
        let (mut r, mut g, mut b) = val.to_rgb();
        r = r >> 3;
        g = g >> 2;
        b = b >> 3;
        ((r as u16) << 11) | ((g as u16) << 5) | b as u16
    }
}

pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    pub pixels: PixelData,
}

impl Bitmap {
    pub fn to_rect(&self) -> RECT {
        RECT {
            left: 0,
            right: self.width as i32,
            top: 0,
            bottom: self.height as i32,
        }
    }

    /// If pixels is not None, only parse the given number of lines from the given pixels.
    /// Otherwise pixels are expected to immediately follow the header in memory.
    pub fn parse(buf: &[u8], pixels: Option<(&[u8], usize)>) -> Bitmap {
        let header = BitmapInfo::parse(buf);
        let (pixels, lines) = match pixels {
            Some((pixels, lines)) => (pixels, Some(lines)),
            _ => {
                let len = header.height * header.stride;
                (&buf[header.header_length..][..len], None)
            }
        };
        Self::parse_pixels(&header, pixels, lines)
    }

    /// Parse a BITMAPINFO/HEADER and pixel data.
    fn parse_pixels(header: &BitmapInfo, pixels: &[u8], lines: Option<usize>) -> Bitmap {
        match header.compression {
            BI::RGB => {}
            BI::RLE8 => todo!(),
            BI::RLE4 => todo!(),
            BI::BITFIELDS => {}
            BI::JPEG => todo!(),
            BI::PNG => todo!(),
        };

        // TODO: remove all this; we should be able to use bitmaps in their native pixel format.

        fn get_pixel(header: &BitmapInfo, val: u8) -> [u8; 4] {
            // BMP palette is BGRx
            let offset = val as usize * header.palette_entry_size as usize;
            let slice = &header.palette[offset..][..3];
            [slice[2], slice[1], slice[0], 0xFF]
        }

        let src = pixels;
        let width = header.width;
        let stride = header.stride;
        let height = lines.unwrap_or(header.height as usize);

        let mut dst: Vec<[u8; 4]> = Vec::with_capacity(width * height);
        for y in 0..height {
            let y_src = if header.is_bottom_up {
                height - y - 1
            } else {
                y
            };
            let row = &src[y_src * stride..][..stride];
            match header.bit_count {
                32 => {
                    // TODO: might need to swizzle here, depending on BI::BITFIELDS.
                    dst.extend_from_slice(transmute_pixels(&row[..width * 4]));
                }
                16 => {
                    let row: &[u16] = transmute_pixels(row);
                    for &p in row {
                        dst.push(PixelFormat::decode_rgb555(p).to_pixel());
                    }
                }
                8 => {
                    for &p in &row[..width] {
                        dst.push(get_pixel(header, p));
                    }
                }
                4 => {
                    for i in 0..width {
                        let p = row[i / 2];
                        if i % 2 == 0 {
                            dst.push(get_pixel(header, p >> 4));
                        } else {
                            dst.push(get_pixel(header, p & 0xF));
                        }
                    }
                }
                1 => {
                    for i in 0..width {
                        let p = row[i / 8];
                        let bit = 7 - (i % 8);
                        let p = (p >> bit) & 1;
                        dst.push(get_pixel(header, p));
                    }
                }
                _ => unimplemented!(),
            }
        }

        let mut buf: Vec<u8> = Vec::with_capacity(dst.len() * 4);
        unsafe { buf.set_len(buf.capacity()) };
        buf.copy_from_slice(transmute_pixels(&dst));

        Bitmap {
            width: header.width as u32,
            height: height as u32,
            format: PixelFormat::RGBA32,
            pixels: PixelData::new_with_owned(buf.into_boxed_slice()),
        }
    }

    pub fn as_rgba<'a>(&'a self, mem: Mem<'a>) -> &'a [[u8; 4]] {
        self.format.expect_rgba32();
        let bytes = self.pixels.bytes(mem);
        transmute_pixels(bytes)
    }

    pub fn as_rgba_mut<'a>(&'a mut self, mem: Mem<'a>) -> &'a mut [[u8; 4]] {
        self.format.expect_rgba32();
        let bytes = self.pixels.bytes_mut(mem);
        transmute_pixels_mut(bytes)
    }

    pub fn read_row(&self, mem: Mem, x: Range<u32>, y: u32, out: &mut [COLORREF]) {
        let stride = self.format.stride(self.width);
        let pixels = self.pixels.bytes(mem);
        match self.format {
            PixelFormat::RGBA32 => {
                let row = &pixels[((y * stride) + x.start * 4) as usize..]
                    [..(x.end - x.start) as usize * 4];
                for (dst, src) in out.iter_mut().zip(row.chunks_exact(4)) {
                    *dst = COLORREF::from_rgb(src[0], src[1], src[2]);
                }
            }
            PixelFormat::RGB555 => {
                let row = &pixels[((y * stride) + x.start * 2) as usize..]
                    [..(x.end - x.start) as usize * 2];
                for (dst, src) in out.iter_mut().zip(row.chunks_exact(2)) {
                    *dst = PixelFormat::decode_rgb555(u16::from_le_bytes([src[0], src[1]]));
                }
            }
            PixelFormat::Mono => {
                let fullrow = &pixels[(y * stride) as usize..][..stride as usize];
                for (i, dst) in out.iter_mut().enumerate() {
                    let x = x.start as usize + i;
                    let byte = fullrow[x / 8];
                    let bit = 7 - (x % 8);
                    let val = ((byte >> bit) & 1) * 0xff;
                    *dst = COLORREF::from_rgb(val, val, val);
                }
            }
        }
    }

    pub fn write_row(&mut self, mem: Mem, x: Range<u32>, y: u32, src: &[COLORREF]) {
        let stride = self.format.stride(self.width);
        let pixels = self.pixels.bytes_mut(mem);
        match self.format {
            PixelFormat::RGBA32 => {
                let fullrow = &mut pixels[(y * stride) as usize..][..stride as usize];
                let row = &mut fullrow[x.start as usize * 4..x.end as usize * 4];
                let dst_width = (x.end - x.start) as usize;
                for (i, dst) in row.chunks_exact_mut(4).enumerate() {
                    let color = src[i * src.len() / dst_width];
                    dst.copy_from_slice(&color.to_pixel());
                }
            }
            PixelFormat::RGB555 => {
                let fullrow = &mut pixels[(y * stride) as usize..][..stride as usize];
                let row = &mut fullrow[x.start as usize * 2..x.end as usize * 2];
                let dst_width = (x.end - x.start) as usize;
                for (i, dst) in row.chunks_exact_mut(2).enumerate() {
                    let color = src[i * src.len() / dst_width];
                    dst.copy_from_slice(&PixelFormat::encode_rgb555(color).to_le_bytes());
                }
            }
            PixelFormat::Mono => {
                todo!();
            }
        }
    }
}

impl std::fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bitmap")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("format", &self.format)
            //.field("pixels", &&self.pixels[0..16])
            .finish()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_format_stride() {
        assert_eq!(PixelFormat::RGBA32.stride(11), 44);
        assert_eq!(PixelFormat::RGB555.stride(11), 22);
        assert_eq!(PixelFormat::Mono.stride(11), 2);
    }

    #[test]
    fn test_pixel_format_rgb555() {
        let red = COLORREF::from_rgb(0xff, 0, 0);
        let enc = PixelFormat::encode_rgb555(red);
        assert_eq!(enc, 0b0_11111_00000_00000);
        let dec = PixelFormat::decode_rgb555(enc);
        assert_eq!(dec.to_rgb(), (0xf8, 0, 0));

        let cyan = COLORREF::from_rgb(0x20, 0x10, 0);
        let enc = PixelFormat::encode_rgb555(cyan);
        assert_eq!(enc, 0b0_00100_00010_00000);
        let dec = PixelFormat::decode_rgb555(enc);
        assert_eq!(dec.to_rgb(), (0x20, 0x10, 0));
    }
}
