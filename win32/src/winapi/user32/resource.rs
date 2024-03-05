use memory::Mem;

use crate::{
    pe,
    winapi::{
        gdi32::{self, HGDIOBJ},
        kernel32::ResourceId,
        types::*,
    },
    Machine,
};

const TRACE_CONTEXT: &'static str = "user32/resource";

// TODO: switch to the HANDLE<T> type?
pub type HCURSOR = u32;
pub type HICON = u32;
pub type HBRUSH = HGDIOBJ;
pub type HMENU = u32;

#[win32_derive::dllexport]
pub fn LoadIconA(_machine: &mut Machine, hInstance: u32, lpIconName: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn LoadIconW(_machine: &mut Machine, hInstance: u32, lpIconName: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn LoadCursorA(_machine: &mut Machine, hInstance: u32, lpCursorName: u32) -> HCURSOR {
    0
}

#[win32_derive::dllexport]
pub fn LoadCursorW(_machine: &mut Machine, hInstance: u32, lpCursorName: u32) -> HCURSOR {
    0
}

#[win32_derive::dllexport]
pub fn CreateCursor(
    _machine: &mut Machine,
    hInst: u32,
    xHotSpot: u32,
    yHotSpot: u32,
    nWidth: u32,
    nHeight: u32,
    pvANDPlane: u32,
    pvXORPlane: u32,
) -> HCURSOR {
    0
}

#[win32_derive::dllexport]
pub fn ShowCursor(_machine: &mut Machine, bShow: bool) -> u32 {
    // TODO: increment/decrement refcount
    1 // ref=1
}

#[win32_derive::dllexport]
pub fn SetCursor(_machine: &mut Machine, hCursor: u32) -> u32 {
    0 // previous: null
}

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

pub enum Pixels {
    /// RGBA
    Owned(Box<[[u8; 4]]>),
    Ptr(u32),
}

pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub pixels: Pixels,
}

impl Bitmap {
    pub fn pixels_slice<'a>(&'a self, mem: Mem<'a>) -> &'a [[u8; 4]] {
        match self.pixels {
            Pixels::Owned(ref slice) => &*slice,
            Pixels::Ptr(addr) => {
                let len = self.width * self.height;
                mem.view_n::<[u8; 4]>(addr, len)
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

        if header.biClrUsed != 0 {
            todo!();
        }

        let palette = match header.compression().unwrap() {
            BI::RGB => {
                let palette_len = match header.biBitCount {
                    8 => 256,
                    4 => 16,
                    _ => unimplemented!(),
                };
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
        let stride = ((width * header.biBitCount as usize / 8) + 3) & !3;

        let (src, height) = match pixels {
            Some(p) => p,
            None => unsafe {
                let ptr = (palette as *const _ as *const u8).add(palette.len() * 4);
                let height = header.height() as usize;
                let len = width * height * header.biBitCount as usize / 8;
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
            pixels: Pixels::Owned(dst.into_boxed_slice()),
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

#[win32_derive::dllexport]
pub fn LoadImageA(
    machine: &mut Machine,
    hInstance: u32,
    name: ResourceId<&str>,
    typ: u32,
    cx: u32,
    cy: u32,
    fuLoad: u32,
) -> HGDIOBJ {
    assert!(hInstance == machine.state.kernel32.image_base);

    if fuLoad != 0 {
        log::error!("unimplemented fuLoad {:x}", fuLoad);
        return HGDIOBJ::null();
    }

    let namebuf: String16;
    let name = match name {
        ResourceId::Id(id) => ResourceId::Id(id),
        ResourceId::Name(name) => {
            namebuf = String16::from(name);
            ResourceId::Name(namebuf.as_str16())
        }
    };

    // TODO: it's unclear whether the width/height is obeyed when loading an image.

    const IMAGE_BITMAP: u32 = 0;
    match typ {
        IMAGE_BITMAP => {
            let buf = crate::winapi::kernel32::find_resource(
                machine,
                ResourceId::Id(pe::RT::BITMAP as u32),
                name,
            )
            .unwrap();
            let bmp = Bitmap::parse(buf.view::<BITMAPINFOHEADER>(0), None);
            machine.state.gdi32.objects.add(gdi32::Object::Bitmap(bmp))
        }
        _ => {
            log::error!("unimplemented image type {:x}", typ);
            return HGDIOBJ::null();
        }
    }
}

fn find_string(machine: &Machine, uID: u32) -> Option<Mem> {
    // Strings are stored as blocks of 16 consecutive strings.
    let (resource_id, index) = ((uID >> 4) + 1, uID & 0xF);

    let block = crate::winapi::kernel32::find_resource(
        machine,
        ResourceId::Id(pe::RT::STRING as u32),
        ResourceId::Id(resource_id),
    )?;

    // Each block is a sequence of two byte length-prefixed strings.
    // Iterate through them to find the requested index.
    let mut ofs = 0;
    for _ in 0..index {
        let len = block.get::<u16>(ofs) as u32;
        ofs += (1 + len) * 2;
    }
    let len = block.get::<u16>(ofs) as u32;
    let str = block.sub(ofs + 2, len * 2);
    Some(str)
}

#[win32_derive::dllexport]
pub fn LoadStringA(
    machine: &mut Machine,
    hInstance: u32,
    uID: u32,
    lpBuffer: u32,
    cchBufferMax: u32,
) -> u32 {
    let str = match find_string(machine, uID) {
        Some(str) => Str16::from_bytes(str.as_slice_todo()),
        None => return 0,
    };
    assert!(cchBufferMax != 0); // MSDN claims this is invalid

    let dst = machine
        .mem()
        .sub(lpBuffer, cchBufferMax)
        .as_mut_slice_todo();
    let copy_len = std::cmp::min(dst.len() as usize - 1, str.len());
    for i in 0..copy_len {
        dst[i] = str.buf()[i] as u8;
    }
    dst[copy_len] = 0;
    copy_len as u32
}

#[win32_derive::dllexport]
pub fn LoadStringW(
    machine: &mut Machine,
    hInstance: u32,
    uID: u32,
    lpBuffer: u32,
    cchBufferMax: u32,
) -> u32 {
    let str = match find_string(machine, uID) {
        Some(str) => str,
        None => return 0,
    };
    if cchBufferMax == 0 {
        machine
            .mem()
            .put::<u32>(lpBuffer, str.offset_from(machine.mem()));
        str.len()
    } else {
        let dst = machine.mem().sub(lpBuffer, cchBufferMax * 2);
        let copy_len = std::cmp::min(dst.len() - 2, str.len()) as usize;
        dst.as_mut_slice_todo()[..copy_len].copy_from_slice(&str.as_slice_todo()[..copy_len]);
        dst.put::<u16>(copy_len as u32, 0);
        copy_len as u32
    }
}

#[win32_derive::dllexport]
pub fn LoadMenuW(_machine: &mut Machine, hInstance: u32, lpMenuName: u32) -> HMENU {
    0
}

#[win32_derive::dllexport]
pub fn LoadAcceleratorsW(_machine: &mut Machine, hInstance: u32, lpTableName: u32) -> HMENU {
    0
}
