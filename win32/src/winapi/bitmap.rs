//! Code dealing with pixel buffers, as found in both gdi32 and user32.
//! This module does not become its own DLL.

#![allow(non_snake_case)]

use super::types::*;
use memory::{Extensions, Mem};

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
#[derive(Debug)]
pub struct BITMAPCOREHEADER {
    pub bcSize: DWORD,
    pub bcWidth: WORD,
    pub bcHeight: WORD,
    pub bcPlanes: WORD,
    pub bcBitCount: WORD,
}
unsafe impl memory::Pod for BITMAPCOREHEADER {}
impl BITMAPCOREHEADER {
    pub fn stride(&self) -> u32 {
        // Bitmap row stride is padded out to 4 bytes per row.
        ((((self.bcWidth * self.bcBitCount) as u32) + 31) & !31) >> 3
    }
}

#[repr(C)]
#[derive(Debug)]
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
    pub fn stride(&self) -> u32 {
        // Bitmap row stride is padded out to 4 bytes per row.
        (((self.biWidth * self.biBitCount as u32) + 31) & !31) >> 3
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

pub struct BitmapInfo<'a> {
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub is_top_down: bool,
    pub bit_count: u8,
    pub compression: BI,
    pub palette_entry_size: usize,
    pub palette: &'a [u8],
    pub pixels: &'a [u8],
}

pub trait Bitmap {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

pub enum PixelData<T> {
    Owned(Box<[T]>),
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

impl<T: memory::Pod> PixelData<T> {
    pub fn as_slice<'a>(&'a self, mem: Mem<'a>) -> &'a [T] {
        match self {
            PixelData::Owned(b) => &*b,
            &PixelData::Ptr(addr, len) => {
                let bytes = mem.sub(addr, len).as_slice_todo();
                transmute_pixels::<T>(bytes)
            }
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [T] {
        match self {
            PixelData::Owned(b) => &mut *b,
            _ => unimplemented!(),
        }
    }
}

pub struct BitmapRGBA32 {
    pub width: u32,
    pub height: u32,
    pub pixels: PixelData<[u8; 4]>,
}

impl BitmapRGBA32 {
    pub fn pixels_slice<'a>(&'a self, mem: Mem<'a>) -> &'a [[u8; 4]] {
        self.pixels.as_slice(mem)
    }

    pub fn clone(&self) -> BitmapRGBA32 {
        let pixels = match &self.pixels {
            PixelData::Owned(b) => PixelData::Owned(b.clone()),
            PixelData::Ptr(addr, len) => PixelData::Ptr(*addr, *len),
        };
        BitmapRGBA32 {
            width: self.width,
            height: self.height,
            pixels,
        }
    }

    pub fn parse(buf: Mem) -> BitmapRGBA32 {
        let headerSize = buf.get_pod::<u32>(0);
        let bmp = match headerSize {
            12 => BitmapRGBA32::parseBMPv2(buf.view::<BITMAPCOREHEADER>(0)),
            40 => BitmapRGBA32::parseBMPv3(buf.view::<BITMAPINFOHEADER>(0), None),
            _ => unimplemented!("unimplemented bitmap header size {}", headerSize),
        };
        bmp
    }

    pub fn parseBMPv2(header: &BITMAPCOREHEADER) -> BitmapRGBA32 {
        let header_size = std::mem::size_of::<BITMAPCOREHEADER>();
        if header.bcSize as usize != header_size {
            panic!("bad bitmap header");
        }

        let palette_len = match header.bcBitCount {
            8 => 256,
            4 => 16,
            1 => 2,
            _ => unimplemented!(),
        };
        let palette_entry_size = 3;
        let palette_size = palette_len * palette_entry_size;
        let palette = unsafe {
            std::slice::from_raw_parts(
                (header as *const _ as *const u8).add(header_size),
                palette_size,
            )
        };
        let height = header.bcHeight as usize;
        let stride = header.stride() as usize;
        let pixels = unsafe {
            std::slice::from_raw_parts(
                (header as *const _ as *const u8).add(header_size + palette_size),
                height * stride,
            )
        };
        let bi = &BitmapInfo {
            width: header.bcWidth as usize,
            height,
            stride,
            is_top_down: false,
            bit_count: header.bcBitCount as u8,
            compression: BI::RGB,
            palette_entry_size,
            palette,
            pixels,
        };
        BitmapRGBA32::parseBMP(bi, None)
    }

    /// This accepts optional pixels/lines to support SetDIBitsToDevice.
    pub fn parseBMPv3(header: &BITMAPINFOHEADER, pixels: Option<(&[u8], usize)>) -> BitmapRGBA32 {
        let palette_len = match header.biBitCount {
            8 => 256,
            4 => 16,
            1 => 2,
            _ => unimplemented!(),
        };
        let palette_entry_size = 4;
        let palette_size = palette_len * palette_entry_size;
        let palette = unsafe {
            std::slice::from_raw_parts(
                (header as *const _ as *const u8).add(header.biSize as usize),
                palette_size,
            )
        };
        let height = header.biHeight as usize;
        let stride = header.stride() as usize;
        let (pixels, lines) = match pixels {
            Some((pixels, lines)) => (pixels, Some(lines)),
            _ => {
                let pixels = unsafe {
                    std::slice::from_raw_parts(
                        (header as *const _ as *const u8)
                            .add(header.biSize as usize + palette_size),
                        height * stride,
                    )
                };
                (pixels, None)
            }
        };
        let bi = &BitmapInfo {
            width: header.biWidth as usize,
            height,
            stride,
            is_top_down: false,
            bit_count: header.biBitCount as u8,
            compression: BI::RGB,
            palette_entry_size,
            palette,
            pixels,
        };
        BitmapRGBA32::parseBMP(bi, lines)
    }

    /// Parse a BITMAPINFO/HEADER and pixel data.
    /// If lines is not None, only parse the given number of lines from the data.
    pub fn parseBMP(header: &BitmapInfo, lines: Option<usize>) -> BitmapRGBA32 {
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
            let offset = val as usize * header.palette_entry_size;
            let slice = &header.palette[offset..][..3];
            [slice[2], slice[1], slice[0], 255]
        }

        let src = header.pixels;
        let width = header.width;
        let stride = header.stride;
        let height = lines.unwrap_or(header.height);

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
            pixels: PixelData::Owned(dst.into_boxed_slice()),
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

impl Bitmap for BitmapRGBA32 {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
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

impl Bitmap for BitmapMono {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}
