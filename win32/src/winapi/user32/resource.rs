use anyhow::bail;
use memory::Mem;

use crate::{
    pe,
    reader::Reader,
    winapi::{gdi32, types::*},
    Machine,
};

const TRACE_CONTEXT: &'static str = "user32/resource";

fn IS_INTRESOURCE(x: u32) -> bool {
    x >> 16 == 0
}

// TODO: switch to the HANDLE<T> type?
pub type HCURSOR = u32;
pub type HICON = u32;
pub type HBRUSH = u32;

#[win32_derive::dllexport]
pub fn LoadIconA(_machine: &mut Machine, _hInstance: u32, _lpIconName: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn LoadCursorA(_machine: &mut Machine, _hInstance: u32, _lpCursorName: u32) -> HCURSOR {
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
pub fn ShowCursor(_machine: &mut Machine, _bShow: bool) -> u32 {
    // TODO: increment/decrement refcount
    1 // ref=1
}

#[win32_derive::dllexport]
pub fn SetCursor(_machine: &mut Machine, hCursor: u32) -> u32 {
    0 // previous: null
}

#[repr(C)]
#[derive(Debug)]
struct BITMAPINFOHEADER {
    biSize: DWORD,
    biWidth: DWORD,
    biHeight: DWORD,
    biPlanes: WORD,
    biBitCount: WORD,
    biCompression: DWORD,
    biSizeImage: DWORD,
    biXPelsPerMeter: DWORD,
    biYPelsPerMeter: DWORD,
    biClrUsed: DWORD,
    biClrImportant: DWORD,
}
unsafe impl memory::Pod for BITMAPINFOHEADER {}
impl BITMAPINFOHEADER {
    fn width(&self) -> u32 {
        self.biWidth
    }
    fn height(&self) -> u32 {
        // Height is negative if top-down DIB.
        (self.biHeight as i32).abs() as u32
    }
    fn is_top_down(&self) -> bool {
        (self.biHeight as i32) < 0
    }
}

pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub pixels: Box<[[u8; 4]]>,
}
impl std::fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bitmap")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("pixels", &&self.pixels[0..16])
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
        pixels: pixels.into_boxed_slice(),
    })
}

#[win32_derive::dllexport]
pub fn LoadImageA(
    machine: &mut Machine,
    hInstance: u32,
    name: u32,
    typ: u32,
    cx: u32,
    cy: u32,
    fuLoad: u32,
) -> u32 {
    assert!(hInstance == machine.state.kernel32.image_base);
    if !IS_INTRESOURCE(name) {
        log::error!("unimplemented image name {name:x}");
        return 0;
    }

    if fuLoad != 0 {
        log::error!("unimplemented fuLoad {:x}", fuLoad);
        return 0;
    }

    // TODO: it's unclear whether the width/height is obeyed when loading an image.

    const IMAGE_BITMAP: u32 = 0;
    match typ {
        IMAGE_BITMAP => {
            let mem = machine.mem().slice(machine.state.kernel32.image_base..);
            let buf = pe::get_resource(mem, &machine.state.user32.resources, pe::RT_BITMAP, name)
                .unwrap();
            let bmp = parse_bitmap(buf).unwrap();
            machine.state.gdi32.objects.push(gdi32::Object::Bitmap(bmp));
            machine.state.gdi32.objects.len() as u32
        }
        _ => {
            log::error!("unimplemented image type {:x}", typ);
            return 0;
        }
    }
}
