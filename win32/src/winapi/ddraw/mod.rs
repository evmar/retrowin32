#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

mod ddraw1;
mod ddraw7;
mod types;

use super::{heap::Heap, types::*};
use crate::{host, machine::Machine, winapi::vtable};
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
}

pub struct State {
    heap: Heap,
    vtable_IDirectDraw: u32,
    vtable_IDirectDrawSurface: u32,
    vtable_IDirectDraw7: u32,
    vtable_IDirectDrawSurface7: u32,
    vtable_IDirectDrawPalette: u32,

    // TODO: this is per-IDirectDraw state.
    hwnd: HWND,
    /// Display width, after SetDisplayMode.
    width: u32,
    /// Display height, after SetDisplayMode.
    height: u32,
    pub surfaces: HashMap<u32, Surface>,
    palettes: HashMap<u32, Box<[PALETTEENTRY]>>,
    /// XXX monolife attaches palette only to back surface, then flips; we need to rearrange
    /// how surface flipping works for the palettes to work out, so this is hacked for now.
    palette_hack: u32,
}

impl State {
    pub fn new_init(machine: &mut Machine) -> Self {
        let mut ddraw = State::default();
        ddraw.heap = machine.state.kernel32.new_private_heap(
            &mut machine.memory,
            1 << 20,
            "ddraw.dll heap".into(),
        );

        ddraw.vtable_IDirectDraw = ddraw1::IDirectDraw::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDrawSurface = ddraw1::IDirectDrawSurface::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDraw7 = ddraw7::IDirectDraw7::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDrawSurface7 = ddraw7::IDirectDrawSurface7::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDrawPalette = IDirectDrawPalette::vtable(&mut ddraw, machine);

        ddraw
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            heap: Heap::default(),
            vtable_IDirectDraw: 0,
            vtable_IDirectDrawSurface: 0,
            vtable_IDirectDraw7: 0,
            vtable_IDirectDrawSurface7: 0,
            vtable_IDirectDrawPalette: 0,
            hwnd: HWND::null(),
            width: 0,
            height: 0,
            surfaces: HashMap::new(),
            palettes: HashMap::new(),
            palette_hack: 0,
        }
    }
}

const DD_OK: u32 = 0;
// DD error codes are generated with this MAKE_HRESULT macro, maybe it doesn't matter too much.
const DDERR_GENERIC: u32 = 0x80004005;

#[win32_derive::shims_from_x86]
mod IDirectDrawPalette {
    use super::*;

    vtable![IDirectDrawPalette shims
        QueryInterface todo,
        AddRef todo,
        Release ok,
        GetCaps todo,
        GetEntries todo,
        Initialize todo,
        SetEntries ok,
    ];

    pub fn new(machine: &mut Machine) -> u32 {
        let ddraw = &mut machine.state.ddraw;
        let lpDirectDrawPalette = ddraw.heap.alloc(machine.memory.mem(), 4);
        let vtable = ddraw.vtable_IDirectDrawPalette;
        machine.mem().put::<u32>(lpDirectDrawPalette, vtable);
        lpDirectDrawPalette
    }

    #[win32_derive::dllexport]
    fn Release(_machine: &mut Machine, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    fn SetEntries(
        machine: &mut Machine,
        this: u32,
        unused: u32,
        start: u32,
        count: u32,
        entries: u32,
    ) -> u32 {
        let palette = machine.state.ddraw.palettes.get_mut(&this).unwrap();
        // TODO: if palette is DDPCAPS_8BITENTRIES then entries are one byte, not 4.
        let entries = machine.memory.mem().view_n::<PALETTEENTRY>(entries, count);
        palette[start as usize..][..count as usize].clone_from_slice(entries);
        DD_OK
    }
}

#[win32_derive::dllexport]
pub fn DirectDrawCreate(machine: &mut Machine, lpGuid: u32, lplpDD: u32, pUnkOuter: u32) -> u32 {
    DirectDrawCreateEx(machine, lpGuid, lplpDD, 0, pUnkOuter)
}

#[win32_derive::dllexport]
pub fn DirectDrawCreateEx(
    machine: &mut Machine,
    lpGuid: u32,
    lplpDD: u32,
    iid: u32,
    pUnkOuter: u32,
) -> u32 {
    assert!(lpGuid == 0);
    assert!(pUnkOuter == 0);

    if machine.state.ddraw.heap.addr == 0 {
        machine.state.ddraw = State::new_init(machine);
    }
    let ddraw = &mut machine.state.ddraw;

    if iid == 0 {
        // DirectDrawCreate
        let lpDirectDraw = ddraw.heap.alloc(machine.memory.mem(), 4);
        let vtable = ddraw.vtable_IDirectDraw;
        machine.mem().put::<u32>(lpDirectDraw, vtable);
        machine.mem().put::<u32>(lplpDD, lpDirectDraw);
        return DD_OK;
    }

    let iid_slice = machine.memory.mem().sub(iid, 16).as_slice_todo();
    if iid_slice == ddraw7::IID_IDirectDraw7 {
        // Caller gives us:
        //   pointer (lplpDD) that they want us to fill in to point to ->
        //   [vtable, ...] (lpDirectDraw7), where vtable is pointer to ->
        //   [fn1, fn2, ...] (vtable_IDirectDraw7)
        let lpDirectDraw7 = ddraw.heap.alloc(machine.memory.mem(), 4);
        let vtable = ddraw.vtable_IDirectDraw7;
        machine.mem().put::<u32>(lpDirectDraw7, vtable);
        machine.mem().put::<u32>(lplpDD, lpDirectDraw7);
        DD_OK
    } else {
        log::error!("DirectDrawCreateEx: unknown IID {iid_slice:x?}");
        DDERR_GENERIC
    }
}
