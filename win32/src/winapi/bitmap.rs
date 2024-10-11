//! Code dealing with pixel buffers, as found in both gdi32 and user32.
//! This module does not become its own DLL.

#![allow(non_snake_case)]

use super::types::*;
use memory::{Extensions, ExtensionsMut, Mem};
use std::cell::RefCell;

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
    pub fn is_top_down(&self) -> bool {
        (self.biHeight as i32) < 0
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
    pub is_top_down: bool,
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
        let palette_len = match header.bcBitCount {
            8 => 256,
            4 => 16,
            1 => 2,
            _ => unimplemented!(),
        };
        let palette_entry_size = 3usize;
        let palette_size = palette_len * palette_entry_size;
        let palette = buf.sub32(0, palette_size as u32);

        BitmapInfo {
            width: header.bcWidth as usize,
            height: header.bcHeight as usize,
            stride: header.stride(),
            is_top_down: false,
            bit_count: header.bcBitCount as u8,
            compression: BI::RGB,
            palette_entry_size,
            palette,
            header_length: 12 + palette_size,
        }
    }

    /// buf is the bytes following the header.
    fn parseBMPv3(header: &BITMAPINFOHEADER, buf: &'a [u8]) -> Self {
        let palette_len = match header.biBitCount {
            32 => 0,
            8 => 256,
            4 => 16,
            1 => 2,
            _ => unimplemented!(),
        };
        // TODO: biClrUsed determines the size of the palette.
        let palette_entry_size = 4usize;
        let palette_size = palette_len * palette_entry_size;
        let palette = buf.sub32(0, palette_size as u32);

        BitmapInfo {
            width: header.biWidth as usize,
            height: header.height() as usize,
            stride: header.stride(),
            is_top_down: false,
            bit_count: header.biBitCount as u8,
            compression: BI::RGB,
            palette_entry_size,
            palette,
            header_length: 40 + palette_size,
        }
    }
}

pub enum PixelData<T> {
    Owned(RefCell<Box<[T]>>),
    Ptr(u32, u32),
}

pub fn transmute_pixels<T: memory::Pod>(bytes: &[u8]) -> &[T] {
    assert!(bytes.len() % std::mem::size_of::<T>() == 0);
    unsafe {
        std::slice::from_raw_parts(
            bytes.as_ptr() as *const _,
            bytes.len() / std::mem::size_of::<T>(),
        )
    }
}

pub fn transmute_pixels_mut<T: memory::Pod>(bytes: &mut [u8]) -> &mut [T] {
    assert!(bytes.len() % std::mem::size_of::<T>() == 0);
    unsafe {
        std::slice::from_raw_parts_mut(
            bytes.as_mut_ptr() as *mut _,
            bytes.len() / std::mem::size_of::<T>(),
        )
    }
}

impl<T: memory::Pod + Clone + Default> PixelData<T> {
    pub fn new_owned(size: usize) -> Self {
        let buf = {
            let mut p = Vec::with_capacity(size);
            p.resize(size, Default::default());
            p.into_boxed_slice()
        };
        PixelData::new_with_owned(buf)
    }

    pub fn new_with_owned(buf: Box<[T]>) -> Self {
        PixelData::Owned(RefCell::new(buf))
    }

    // It would be nice to be able to get a slice without involving a callback,
    // but the lifetimes with the RefCell mean we cannot return a slice directly.
    pub fn with_slice<'a>(&self, mem: Mem, f: impl FnOnce(&mut [T])) {
        match self {
            PixelData::Owned(b) => f(&mut *b.borrow_mut()),
            &PixelData::Ptr(addr, len) => {
                let bytes = mem.sub32_mut(addr, len);
                f(transmute_pixels_mut::<T>(bytes))
            }
        }
    }
}

pub struct BitmapRGBA32 {
    pub width: u32,
    pub height: u32,
    pub pixels: PixelData<[u8; 4]>,
}

impl BitmapRGBA32 {
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
    pub fn parse(buf: &[u8], pixels: Option<(&[u8], usize)>) -> BitmapRGBA32 {
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
    fn parse_pixels(header: &BitmapInfo, pixels: &[u8], lines: Option<usize>) -> BitmapRGBA32 {
        match header.compression {
            BI::RGB => {}
            BI::RLE8 => todo!(),
            BI::RLE4 => todo!(),
            BI::BITFIELDS => {}
            BI::JPEG => todo!(),
            BI::PNG => todo!(),
        };

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
            let y_src = if header.is_top_down {
                y
            } else {
                height - y - 1
            };
            let row = &src[y_src * stride..][..stride];
            match header.bit_count {
                32 => {
                    // TODO: might need to swizzle here, depending on BI::BITFIELDS.
                    dst.extend_from_slice(transmute_pixels(&row[..width * 4]));
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

        BitmapRGBA32 {
            width: header.width as u32,
            height: height as u32,
            pixels: PixelData::new_with_owned(dst.into_boxed_slice()),
        }
    }
}

impl std::fmt::Debug for BitmapRGBA32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bitmap")
            .field("width", &self.width)
            .field("height", &self.height)
            //.field("pixels", &&self.pixels[0..16])
            .finish()
    }
}

pub struct BitmapMono {
    pub width: u32,
    pub height: u32,
    pub pixels: PixelData<u8>,
}

impl BitmapMono {
    pub fn stride(width: u32) -> u32 {
        ((width + 31) & !31) >> 3
    }
}

impl std::fmt::Debug for BitmapMono {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bitmap")
            .field("width", &self.width)
            .field("height", &self.height)
            //.field("pixels", &&self.pixels[0..16])
            .finish()
    }
}
