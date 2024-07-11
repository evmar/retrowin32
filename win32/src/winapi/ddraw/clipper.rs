use super::DD_OK;
use crate::{
    winapi::{com::vtable, types::HWND},
    Machine,
};
use memory::Extensions;

const TRACE_CONTEXT: &'static str = "ddraw/clipper";

#[win32_derive::dllexport]
pub fn DirectDrawCreateClipper(
    machine: &mut Machine,
    dwFlags: u32,
    lplpDDClipper: Option<&mut u32>,
    pUnkOuter: u32,
) -> u32 {
    assert!(dwFlags == 0);
    *lplpDDClipper.unwrap() = IDirectDrawClipper::new(machine);
    DD_OK
}

#[win32_derive::shims_from_x86]
pub(super) mod IDirectDrawClipper {

    use super::*;

    vtable![IDirectDrawClipper shims
        QueryInterface: todo,
        AddRef: todo,
        Release: ok,

        GetClipList: todo,
        GetHWnd: todo,
        Initialize: todo,
        IsClipListChanged: todo,
        SetClipList: todo,
        SetHWnd: ok,
    ];

    pub fn new(machine: &mut Machine) -> u32 {
        let ddraw = &mut machine.state.ddraw;
        let clipper = ddraw.heap.alloc(machine.emu.memory.mem(), 4);
        let vtable = *ddraw.vtable_IDirectDrawClipper.get_or_insert_with(|| {
            vtable(machine.emu.memory.mem(), &mut ddraw.heap, |shim| {
                machine.emu.shims.add(shim)
            })
        });
        machine.mem().put_pod::<u32>(clipper, vtable);
        clipper
    }

    #[win32_derive::dllexport]
    fn Release(_machine: &mut Machine, this: u32) -> u32 {
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    fn SetHWnd(_machine: &mut Machine, this: u32, unused: u32, hwnd: HWND) -> u32 {
        DD_OK
    }
}
