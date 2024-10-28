use std::{cell::RefCell, rc::Rc};

use crate::{
    winapi::{com::vtable, ddraw::DD, gdi32::PALETTEENTRY, kernel32::get_symbol},
    Machine,
};
use memory::{Extensions, ExtensionsMut};

pub type Palette = Rc<RefCell<Box<[PALETTEENTRY]>>>;

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

    pub fn new(machine: &mut Machine) -> u32 {
        let lpDirectDrawPalette = machine
            .state
            .kernel32
            .process_heap
            .alloc(machine.emu.memory.mem(), 4);
        let vtable = get_symbol(machine, "ddraw.dll", "IDirectDrawPalette");
        machine.mem().put_pod::<u32>(lpDirectDrawPalette, vtable);
        lpDirectDrawPalette
    }

    #[win32_derive::dllexport]
    pub fn Release(_machine: &mut Machine, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    pub fn SetEntries(
        machine: &mut Machine,
        this: u32,
        unused: u32,
        start: u32,
        count: u32,
        entries: u32,
    ) -> DD {
        let mut palette = machine
            .state
            .ddraw
            .palettes
            .get(&this)
            .unwrap()
            .borrow_mut();
        // TODO: if palette is DDPCAPS_8BITENTRIES then entries are one byte, not 4.
        let entries = machine
            .emu
            .memory
            .mem()
            .iter_pod::<PALETTEENTRY>(entries, count);
        for (dst, src) in palette[start as usize..][..count as usize]
            .iter_mut()
            .zip(entries)
        {
            *dst = src;
        }
        DD::OK
    }
}
