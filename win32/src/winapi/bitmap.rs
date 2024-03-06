//! Code dealing with pixel buffers, as found in both gdi32 and user32.
//! This module does not become its own DLL.

#![allow(non_snake_case)]

use super::types::*;
use memory::Mem;

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

pub fn bytes_as_rgba(bytes: &[u8]) -> &[[u8; 4]] {
    assert!(bytes.len() % 4 == 0);
    unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const _, bytes.len() / 4) }
}

pub fn bytes_as_rgba_mut(bytes: &mut [u8]) -> &mut [[u8; 4]] {
    assert!(bytes.len() % 4 == 0);
    unsafe { std::slice::from_raw_parts_mut(bytes.as_ptr() as *mut _, bytes.len() / 4) }
}

pub enum PixelData {
    Owned(Box<[u8]>),
    Ptr(u32),
}

impl PixelData {
    pub fn from_rgba32(mut pixels: Box<[[u8; 4]]>) -> Self {
        let slice: &mut [u8] = unsafe {
            std::slice::from_raw_parts_mut(pixels.as_mut_ptr() as *mut _, pixels.len() * 4)
        };
        std::mem::forget(pixels);
        unsafe { PixelData::Owned(Box::from_raw(slice)) }
    }

    pub fn as_slice(&self) -> &[u8] {
        match self {
            PixelData::Owned(b) => &*b,
            _ => unimplemented!(),
        }
    }
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        match self {
            PixelData::Owned(b) => &mut *b,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PixelFormat {
    Mono,
    BPP4,
    RGBA32,
}

impl PixelFormat {
    pub fn bits_per_pixel(&self) -> u32 {
        match *self {
            PixelFormat::Mono => 1,
            PixelFormat::BPP4 => 4,
            PixelFormat::RGBA32 => 32,
        }
    }

    pub fn aligned_stride(&self, width: u32) -> u32 {
        ((self.bits_per_pixel() * width * 8) + 31) & !31
    }
}

pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    pub pixels: PixelData,
}

impl Bitmap {
    pub fn pixels_slice<'a>(&'a self, mem: Mem<'a>) -> &'a [u8] {
        match self.pixels {
            PixelData::Owned(ref slice) => &*slice,
            PixelData::Ptr(addr) => {
                let len = self.format.aligned_stride(self.width) * self.height;
                mem.view_n::<u8>(addr, len)
            }
        }
    }

    /// Parse a BITMAPINFO/HEADER and pixel data.
    /// If pixels is None, pixel data immediately follows header.
    pub fn parse(header: &BITMAPINFOHEADER, pixels: Option<(&[u8], usize)>) -> Bitmap {
        let header_size = std::mem::size_of::<BITMAPINFOHEADER>();
        if header.biSize as usize != header_size {
            panic!("bad bitmap header");
        }

        let palette = match header.compression().unwrap() {
            BI::RGB => {
                let palette_len = match header.biBitCount {
                    8 => 256,
                    4 => 16,
                    _ => unimplemented!(),
                };
                if header.biClrUsed != 0 && header.biClrUsed != palette_len {
                    todo!();
                }
                unsafe {
                    let ptr = (header as *const _ as *const u8).add(header_size) as *const _;
                    std::slice::from_raw_parts(ptr as *const [u8; 4], palette_len as usize)
                }
            }
            BI::RLE8 => todo!(),
            BI::RLE4 => todo!(),
            BI::BITFIELDS => todo!(),
            BI::JPEG => todo!(),
            BI::PNG => todo!(),
        };

        fn get_pixel(palette: &[[u8; 4]], val: u8) -> [u8; 4] {
            // BMP palette is BGRx
            let [b, g, r, _] = palette[val as usize];
            [r, g, b, 255]
        }

        let width = header.width() as usize;
        // Bitmap row stride is padded out to 4 bytes per row.
        let stride = (((width * header.biBitCount as usize) + 31) & !31) >> 3;

        let (src, height) = match pixels {
            Some(p) => p,
            None => unsafe {
                let ptr = (palette as *const _ as *const u8).add(palette.len() * 4);
                let height = header.height() as usize;
                let len = stride * height;
                (std::slice::from_raw_parts(ptr, len), height)
            },
        };

        let mut dst = Vec::with_capacity(width * height);
        for y in 0..height {
            let y_src = if header.is_top_down() {
                y
            } else {
                height - y - 1
            };
            let row = &src[y_src * stride..][..stride];
            match header.biBitCount {
                8 => {
                    for &p in &row[..width] {
                        dst.push(get_pixel(palette, p));
                    }
                }
                4 => {
                    for i in 0..width {
                        let p = row[i / 2];
                        if i % 2 == 0 {
                            dst.push(get_pixel(palette, p >> 4));
                        } else {
                            dst.push(get_pixel(palette, p & 0xF));
                        }
                    }
                }
                _ => unimplemented!(),
            }
        }

        Bitmap {
            width: header.width(),
            height: height as u32,
            format: PixelFormat::RGBA32,
            pixels: PixelData::from_rgba32(dst.into_boxed_slice()),
        }
    }
}

impl std::fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bitmap")
            .field("width", &self.width)
            .field("height", &self.height)
            //.field("pixels", &&self.pixels[0..16])
            .finish()
    }
}
