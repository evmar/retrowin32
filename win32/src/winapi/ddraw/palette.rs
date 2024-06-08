use crate::{winapi::com::vtable, Machine};

const TRACE_CONTEXT: &'static str = "ddraw/palette";

#[win32_derive::shims_from_x86]
pub mod IDirectDrawPalette {

    use crate::winapi::ddraw::{DD_OK, PALETTEENTRY};

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
        let lpDirectDrawPalette = ddraw.heap.alloc(machine.emu.memory.mem(), 4);
        let vtable = *ddraw.vtable_IDirectDrawPalette.get_or_insert_with(|| {
            vtable(machine.emu.memory.mem(), &mut ddraw.heap, |shim| {
                machine.emu.shims.add(shim)
            })
        });
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
        let entries = machine
            .emu
            .memory
            .mem()
            .view_n::<PALETTEENTRY>(entries, count);
        palette[start as usize..][..count as usize].clone_from_slice(entries);
        DD_OK
    }
}
