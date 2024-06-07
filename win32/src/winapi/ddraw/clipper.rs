use super::{State, DD_OK};
use crate::{machine::Emulator, winapi::vtable, Machine};

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

    vtable![IDirectDraw shims
        QueryInterface todo,
        AddRef todo,
        Release todo,

        GetClipList todo,
        GetHWnd todo,
        Initialize todo,
        IsClipListChanged todo,
        SetClipList todo,
        SetHWnd todo,
    ];

    pub fn new(machine: &mut Machine) -> u32 {
        let ddraw = &mut machine.state.ddraw;
        let clipper = ddraw.heap.alloc(machine.emu.memory.mem(), 4);
        let vtable = ddraw.vtable_IDirectDrawClipper;
        machine.mem().put::<u32>(clipper, vtable);
        clipper
    }

    // #[win32_derive::dllexport]
    // fn QueryInterface(
    //     machine: &mut Machine,
    //     this: u32,
    //     riid: Option<&GUID>,
    //     ppvObject: Option<&mut u32>,
    // ) -> u32 {
    //     match riid.unwrap() {
    //         &ddraw2::IID_IDirectDraw2 => {
    //             *ppvObject.unwrap() = ddraw2::IDirectDraw2::new(machine);
    //             DD_OK
    //         }
    //         _ => {
    //             0x80004002 // E_NOINTERFACE
    //         }
    //     }
    // }
}
