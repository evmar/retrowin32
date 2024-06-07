#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

mod clipper;
mod ddraw1;
mod ddraw2;
mod ddraw7;
mod palette;
mod types;

use super::{heap::Heap, types::*};
pub use crate::winapi::com::GUID;
use crate::{host, machine::Machine, SurfaceOptions};
pub use clipper::DirectDrawCreateClipper;
use clipper::IDirectDrawClipper;
use palette::IDirectDrawPalette;
use std::collections::HashMap;
use types::*;

const TRACE_CONTEXT: &'static str = "ddraw";

pub struct Surface {
    pub host: Box<dyn host::Surface>,
    pub width: u32,
    pub height: u32,
    pub palette: u32, // same as key in palettes
    /// x86 address to pixel buffer, or 0 if unused.
    pixels: u32,
    /// Address of attached surface, e.g. back buffer.
    attached: u32,
}

impl Surface {
    fn new(machine: &mut Machine, hwnd: HWND, opts: &SurfaceOptions) -> Self {
        if opts.width == 0 || opts.height == 0 {
            panic!("cannot create 0-sized surface");
        }
        Surface {
            host: machine.host.create_surface(hwnd.to_raw(), &opts),
            width: opts.width,
            height: opts.height,
            palette: 0,
            pixels: 0,
            attached: 0,
        }
    }

    pub fn create(machine: &mut Machine, hwnd: HWND, desc: &DDSURFACEDESC2) -> Vec<Surface> {
        assert!(std::mem::size_of::<DDSURFACEDESC2>() == desc.dwSize as usize);

        let mut surfaces = Vec::new();

        let mut opts = crate::host::SurfaceOptions::default();
        if desc.dwFlags.contains(DDSD::WIDTH) {
            opts.width = desc.dwWidth;
        }
        if desc.dwFlags.contains(DDSD::HEIGHT) {
            opts.height = desc.dwHeight;
        }

        if let Some(caps) = desc.caps() {
            if caps.dwCaps.contains(DDSCAPS::PRIMARYSURFACE) {
                opts.primary = true;
            }
        }

        if opts.width == 0 || opts.height == 0 {
            // Take width/height from window dimensions
            if let Some(wnd) = machine.state.user32.windows.get(hwnd) {
                opts.width = wnd.width;
                opts.height = wnd.height;
            }
        }

        surfaces.push(Surface::new(machine, hwnd, &opts));

        if let Some(count) = desc.back_buffer_count() {
            opts.primary = false;
            for _ in 0..count {
                surfaces.push(Surface::new(machine, hwnd, &opts));
            }
        }

        surfaces
    }
}

pub struct State {
    heap: Heap,
    vtable_IDirectDraw: u32,
    vtable_IDirectDrawSurface: u32,
    vtable_IDirectDraw2: u32,
    vtable_IDirectDrawSurface2: u32,
    vtable_IDirectDraw7: u32,
    vtable_IDirectDrawSurface7: u32,
    vtable_IDirectDrawPalette: u32,
    vtable_IDirectDrawClipper: u32,

    // TODO: this is per-IDirectDraw state.
    hwnd: HWND,
    pub surfaces: HashMap<u32, Surface>,

    bytes_per_pixel: u32,

    palettes: HashMap<u32, Box<[PALETTEENTRY]>>,
    /// XXX monolife attaches palette only to back surface, then flips; we need to rearrange
    /// how surface flipping works for the palettes to work out, so this is hacked for now.
    palette_hack: u32,
}

impl State {
    pub fn new_init(machine: &mut Machine) -> Self {
        let mut ddraw = State::default();
        ddraw.heap = machine.state.kernel32.new_private_heap(
            &mut machine.emu.memory,
            4 << 20,
            "ddraw.dll heap".into(),
        );

        ddraw.vtable_IDirectDraw = ddraw1::IDirectDraw::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDrawSurface = ddraw1::IDirectDrawSurface::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDraw2 = ddraw2::IDirectDraw2::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDrawSurface2 = ddraw2::IDirectDrawSurface2::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDraw7 = ddraw7::IDirectDraw7::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDrawSurface7 = ddraw7::IDirectDrawSurface7::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDrawPalette = IDirectDrawPalette::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDrawClipper = IDirectDrawClipper::vtable(&mut ddraw, machine);

        ddraw
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            heap: Heap::default(),
            vtable_IDirectDraw: 0,
            vtable_IDirectDrawSurface: 0,
            vtable_IDirectDraw2: 0,
            vtable_IDirectDrawSurface2: 0,
            vtable_IDirectDraw7: 0,
            vtable_IDirectDrawSurface7: 0,
            vtable_IDirectDrawPalette: 0,
            vtable_IDirectDrawClipper: 0,
            hwnd: HWND::null(),
            surfaces: HashMap::new(),
            bytes_per_pixel: 4,
            palettes: HashMap::new(),
            palette_hack: 0,
        }
    }
}

const DD_OK: u32 = 0;
// DD error codes are generated with this MAKE_HRESULT macro, maybe it doesn't matter too much.
const DDERR_GENERIC: u32 = 0x80004005;

#[win32_derive::dllexport]
pub fn DirectDrawCreate(
    machine: &mut Machine,
    lpGuid: Option<&GUID>,
    lplpDD: Option<&mut u32>,
    pUnkOuter: u32,
) -> u32 {
    DirectDrawCreateEx(machine, lpGuid, lplpDD, None, pUnkOuter)
}

#[win32_derive::dllexport]
pub fn DirectDrawCreateEx(
    machine: &mut Machine,
    lpGuid: Option<&GUID>,
    lplpDD: Option<&mut u32>,
    iid: Option<&GUID>,
    pUnkOuter: u32,
) -> u32 {
    assert!(lpGuid.is_none());
    assert!(pUnkOuter == 0);

    if machine.state.ddraw.heap.addr == 0 {
        machine.state.ddraw = State::new_init(machine);
    }

    match iid {
        None => {
            // DirectDrawCreate
            *lplpDD.unwrap() = ddraw1::IDirectDraw::new(machine);
            return DD_OK;
        }
        Some(&ddraw7::IID_IDirectDraw7) => {
            *lplpDD.unwrap() = ddraw7::IDirectDraw7::new(machine);
            DD_OK
        }
        _ => {
            log::error!("DirectDrawCreateEx: unknown IID {iid:x?}");
            DDERR_GENERIC
        }
    }
}
