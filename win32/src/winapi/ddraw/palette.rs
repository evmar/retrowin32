use super::ddraw::get_state;
use super::types::*;
use builtin_gdi32::PALETTEENTRY;
use memory::Extensions;
use std::cell::RefCell;
use win32_system::System;
use win32_winapi::vtable;

pub struct Palette {
    pub ptr: u32,
    pub entries: RefCell<Box<[PALETTEENTRY]>>,
}

#[win32_derive::dllexport]
pub mod IDirectDrawPalette {
    use super::*;

    vtable![
        QueryInterface: todo,
        AddRef: todo,
        Release: ok,
        GetCaps: todo,
        GetEntries: todo,
        Initialize: todo,
        SetEntries: ok,
    ];

    pub fn new(sys: &mut dyn System) -> u32 {
        let vtable = sys.get_symbol("ddraw.dll", "IDirectDrawPalette");
        sys.memory().store(vtable)
    }

    #[win32_derive::dllexport]
    pub fn Release(sys: &dyn System, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    pub fn SetEntries(
        sys: &mut dyn System,
        this: u32,
        unused: u32,
        start: u32,
        count: u32,
        entries: u32,
    ) -> DD {
        let state = get_state(sys);
        let mut dst = state.palettes.get(&this).unwrap().entries.borrow_mut();
        // TODO: if palette is DDPCAPS_8BITENTRIES then entries are one byte, not 4.
        let src = sys.mem().iter_pod::<PALETTEENTRY>(entries, count);
        for (dst, src) in dst[start as usize..][..count as usize].iter_mut().zip(src) {
            *dst = src;
        }
        DD::OK
    }
}
