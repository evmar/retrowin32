use anyhow::bail;
use memory::Mem;

use crate::{
    pe,
    reader::Reader,
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
pub type HBRUSH = u32;
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

fn parse_bitmap(buf: Mem) -> anyhow::Result<Bitmap> {
    let mut r = Reader::new(buf);
    let header = r.read::<BITMAPINFOHEADER>();
    let header_size = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    if header.biSize != header_size {
        bail!("bad bitmap header");
    }

    let palette_count = match header.biBitCount {
        8 => match header.biClrUsed {
            0 => 256,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };
    let palette_buf = r.read_n::<u32>(palette_count)?;
    let palette = unsafe {
        std::slice::from_raw_parts(
            palette_buf.as_ptr() as *const [u8; 4],
            palette_count as usize,
        )
    };
    let width = header.width();
    let height = header.height();
    let pixels = r.read_n::<u8>(width * height)?;
    assert!(r.done());

    // Bitmap pixel data is tricky.
    // - Likely bottom-up (first row of data is bottom row of pixels)
    // - Palette will have 0s for the 4th component, while canvas interprets those 0s
    //   as an alpha channel.  We swap there here for 255 for now.
    // It's plausible some software expects the pixel data within a bitmap to be
    // exactly as in the underlying file and we ought to not monkey with it here,
    // but for now let's just transform it into the form the canvas expects.

    fn get_pixel(palette: &[[u8; 4]], val: u8) -> [u8; 4] {
        let [r, g, b, _] = palette[val as usize];
        [r, g, b, 255]
    }

    let pixels = if header.is_top_down() {
        pixels.iter().map(|&p| get_pixel(palette, p)).collect()
    } else {
        let mut v = Vec::with_capacity(pixels.len() as usize);
        for y in (0..height).rev() {
            for &p in pixels[(y * width) as usize..][..width as usize].iter() {
                v.push(get_pixel(palette, p));
            }
        }
        v
    };

    Ok(Bitmap {
        width,
        height,
        pixels: Pixels::Owned(pixels.into_boxed_slice()),
    })
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
            let bmp = parse_bitmap(buf).unwrap();
            machine.state.gdi32.objects.add(gdi32::Object::Bitmap(bmp))
        }
        _ => {
            log::error!("unimplemented image type {:x}", typ);
            return HGDIOBJ::null();
        }
    }
}

#[win32_derive::dllexport]
pub fn LoadStringW(
    machine: &mut Machine,
    hInstance: u32,
    uID: u32,
    lpBuffer: u32,
    cchBufferMax: u32,
) -> u32 {
    // Strings are stored as blocks of 16 consecutive strings.
    let (resource_id, index) = ((uID >> 4) + 1, uID & 0xF);

    let block = match crate::winapi::kernel32::find_resource(
        machine,
        ResourceId::Id(pe::RT::STRING as u32),
        ResourceId::Id(resource_id),
    ) {
        Some(block) => block,
        None => todo!(),
    };

    // Each block is a sequence of two byte length-prefixed strings.
    // Iterate through them to find the requested index.
    let mut ofs = 0;
    for _ in 0..index {
        let len = block.get::<u16>(ofs) as u32;
        ofs += (1 + len) * 2;
    }
    let len = block.get::<u16>(ofs) as u32;
    let str = block.sub(ofs + 2, len * 2);

    if cchBufferMax == 0 {
        machine
            .mem()
            .put::<u32>(lpBuffer, str.offset_from(machine.mem()));
        len
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
