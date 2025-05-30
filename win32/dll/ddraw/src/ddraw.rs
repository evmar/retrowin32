//! DirectDraw shared API.  All the ddraw1 through ddraw7 interfaces back onto shared
//! implementation defined here.

use super::types::*;
use super::{ddraw1, ddraw7, palette::Palette};
use builtin_gdi32::bitmap::{Bitmap, PixelData, PixelFormat, transmute_pixels_mut};
use builtin_user32 as user32;
use memory::{Extensions, ExtensionsMut, Mem};
use std::{cell::RefMut, collections::HashMap, rc::Rc};
use win32_system::{Heap, System, generic_get_state, host};
use win32_winapi::{HWND, RECT, com::GUID};

pub struct Surface {
    pub host: Box<dyn host::Surface>,
    pub width: u32,
    pub height: u32,
    pub palette: Option<Rc<Palette>>,
    /// x86 address to pixel buffer, or 0 if unused.
    pub pixels: u32,
    pub bytes_per_pixel: u32,
    pub primary: bool,
    /// Address of attached surface, e.g. back buffer.
    pub attached: u32,
}

impl Surface {
    fn new(sys: &mut dyn System, hwnd: HWND, opts: &host::SurfaceOptions) -> Self {
        if opts.width == 0 || opts.height == 0 {
            panic!("cannot create 0-sized surface");
        }
        Surface {
            host: sys.host().create_surface(hwnd.to_raw(), &opts),
            width: opts.width,
            height: opts.height,
            palette: None,
            pixels: 0,
            primary: opts.primary,
            bytes_per_pixel: opts.bytes_per_pixel,
            attached: 0,
        }
    }

    pub fn create(sys: &mut dyn System, hwnd: HWND, desc: &DDSURFACEDESC2) -> Vec<Surface> {
        assert!(std::mem::size_of::<DDSURFACEDESC2>() == desc.dwSize as usize);

        let mut surfaces = Vec::new();

        let mut opts = host::SurfaceOptions::default();
        if desc.dwFlags.contains(DDSD::WIDTH) {
            opts.width = desc.dwWidth;
        }
        if desc.dwFlags.contains(DDSD::HEIGHT) {
            opts.height = desc.dwHeight;
        }

        // win003 doesn't set desc.dwFlags at all, so don't consider whether they provided DDSD::CAPS.
        let caps = &desc.ddsCaps.dwCaps;
        if caps.contains(DDSCAPS::PRIMARYSURFACE) {
            opts.primary = true;
        }

        if opts.width == 0 || opts.height == 0 {
            // Take width/height from window dimensions
            if let Some(wnd) = user32::get_state(sys).windows.get(hwnd) {
                let wnd = wnd.borrow();
                opts.width = wnd.width;
                opts.height = wnd.height;
            }
        }

        if opts.bytes_per_pixel == 0 {
            opts.bytes_per_pixel = get_state(sys).screen_bytes_per_pixel;
        }

        surfaces.push(Surface::new(sys, hwnd, &opts));

        if let Some(count) = desc.back_buffer_count() {
            opts.primary = false;
            for _ in 0..count {
                surfaces.push(Surface::new(sys, hwnd, &opts));
            }
        }

        surfaces
    }

    pub fn to_rect(&self) -> RECT {
        RECT {
            left: 0,
            top: 0,
            right: self.width as i32,
            bottom: self.height as i32,
        }
    }

    pub fn pixel_format(&self) -> PixelFormat {
        match self.bytes_per_pixel {
            2 => PixelFormat::RGB555, // TODO: 565 etc.
            4 => PixelFormat::RGBA32,
            _ => todo!(),
        }
    }

    /// Create a Bitmap representing the backing pixel buffer.
    pub fn to_bitmap(&self) -> Bitmap {
        if self.pixels == 0 {
            todo!()
        }
        Bitmap {
            width: self.width,
            height: self.height,
            format: self.pixel_format(),
            pixels: PixelData::Ptr(self.pixels, self.width * self.height * self.bytes_per_pixel),
        }
    }

    pub fn lock(&mut self, mem: Mem, heap: &Heap) -> u32 {
        if self.pixels == 0 {
            self.pixels = heap.alloc(mem, self.width * self.height * self.bytes_per_pixel);
        }
        self.pixels
    }

    pub fn unlock(&mut self, mem: Mem) {
        self.flush(mem);
    }

    pub fn fill(&mut self, mem: Mem, heap: &Heap, color: u32) {
        let pixels = self.lock(mem, heap);
        let pixels = mem.sub32_mut(pixels, self.width * self.height * self.bytes_per_pixel);
        match self.bytes_per_pixel {
            1 => {
                pixels.fill(color as u8);
            }
            2 => {
                let pixels = transmute_pixels_mut::<u8, u16>(pixels);
                pixels.fill(color as u16);
            }
            4 => {
                let pixels = transmute_pixels_mut::<u8, u32>(pixels);
                pixels.fill(color);
            }
            _ => unreachable!(),
        }
        self.unlock(mem);
    }

    /// Copy pixels from emulator .pixels memory to the host's surface.
    /// Called after GDI drawing calls or Lock/Unlock.
    pub fn flush(&self, mem: Mem) {
        assert!(self.pixels != 0);

        // We need to copy self.pixels to convert its format to the RGBA expected by the write_pixels API.
        let mut pixels_bytes = Vec::with_capacity((self.width * self.height * 4) as usize);
        unsafe { pixels_bytes.set_len(pixels_bytes.capacity()) };
        let pixels_quads: &mut [[u8; 4]] = transmute_pixels_mut(&mut pixels_bytes);
        match self.bytes_per_pixel {
            1 => {
                let pixels = mem.iter_pod::<u8>(self.pixels, self.width * self.height);
                let Some(palette) = self.palette.as_ref() else {
                    // On startup, no palette may mean all black?
                    return;
                };
                let entries = palette.entries.borrow();
                for (pSrc, pDst) in pixels.zip(pixels_quads) {
                    let p = &entries[pSrc as usize];
                    *pDst = [p.peRed, p.peGreen, p.peBlue, 0xFF];
                }
            }
            2 => {
                let pixels = mem.iter_pod::<u16>(self.pixels, self.width * self.height);

                for (pSrc, pDst) in pixels.zip(pixels_quads) {
                    // TODO: this depends on whether 16bpp is 555 or 565 etc.
                    let r = ((pSrc & 0b0111_1100_0000_0000) >> 7) as u8;
                    let g = ((pSrc & 0b0000_0011_1110_0000) >> 2) as u8;
                    let b = ((pSrc & 0b0000_0000_0001_1111) << 3) as u8;
                    *pDst = [r, g, b, 0xFF];
                }
            }
            4 => {
                let pixels = mem.iter_pod::<[u8; 4]>(self.pixels, self.width * self.height);
                // Ignore alpha channel in input; output is always opaque.
                for (pSrc, pDst) in pixels.zip(pixels_quads) {
                    *pDst = [pSrc[0], pSrc[1], pSrc[2], 0xFF];
                }
            }
            bpp => todo!("Unlock for {bpp}bpp"),
        }
        self.host.write_pixels(&pixels_bytes);
        if self.primary {
            self.host.show();
        }
    }
}

pub struct State {
    // TODO: the fields in this struct are really per-IDirectDraw state.
    pub hwnd: HWND,

    /// Maps interface pointer to Surface objects.
    /// Note that you cannot have multiple pointers to the same surface;
    /// instead, DirectDraw refcounts the interfaces themselves.
    pub surfaces: HashMap<u32, Surface>,

    /// bpp of the current display mode.
    pub screen_bytes_per_pixel: u32,

    pub palettes: HashMap<u32, Rc<Palette>>,
}

impl Default for State {
    fn default() -> Self {
        State {
            hwnd: HWND::null(),
            surfaces: HashMap::new(),
            screen_bytes_per_pixel: 4,
            palettes: HashMap::new(),
        }
    }
}

#[inline]
pub fn get_state(sys: &dyn System) -> RefMut<State> {
    generic_get_state::<State>(sys)
}

#[win32_derive::dllexport]
pub fn DirectDrawCreate(
    sys: &mut dyn System,
    lpGuid: Option<&GUID>,
    lplpDD: Option<&mut u32>,
    pUnkOuter: u32,
) -> DD {
    DirectDrawCreateEx(sys, lpGuid, lplpDD, None, pUnkOuter)
}

#[win32_derive::dllexport]
pub fn DirectDrawCreateEx(
    sys: &mut dyn System,
    lpGuid: Option<&GUID>,
    lplpDD: Option<&mut u32>,
    iid: Option<&GUID>,
    pUnkOuter: u32,
) -> DD {
    assert!(lpGuid.is_none());
    assert!(pUnkOuter == 0);

    match iid {
        None => {
            // DirectDrawCreate
            *lplpDD.unwrap() = ddraw1::IDirectDraw::new(sys);
            return DD::OK;
        }
        Some(&ddraw7::IID_IDirectDraw7) => {
            *lplpDD.unwrap() = ddraw7::IDirectDraw7::new(sys);
            DD::OK
        }
        _ => {
            log::error!("DirectDrawCreateEx: unknown IID {iid:x?}");
            DD::ERR_GENERIC
        }
    }
}
