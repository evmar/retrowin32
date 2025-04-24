use crate::{
    Machine,
    winapi::{HWND, com::vtable, ddraw::DD, kernel32::get_symbol},
};
use memory::ExtensionsMut;

#[win32_derive::dllexport]
pub fn DirectDrawCreateClipper(
    machine: &mut Machine,
    dwFlags: u32,
    lplpDDClipper: Option<&mut u32>,
    pUnkOuter: u32,
) -> DD {
    assert!(dwFlags == 0);
    *lplpDDClipper.unwrap() = IDirectDrawClipper::new(machine);
    DD::OK
}

#[win32_derive::dllexport]
pub mod IDirectDrawClipper {
    use super::*;

    vtable![
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
        let clipper = machine
            .state
            .kernel32
            .process_heap
            .alloc(machine.memory.mem(), 4);
        let vtable = get_symbol(machine, "ddraw.dll", "IDirectDrawClipper");
        machine.mem().put_pod::<u32>(clipper, vtable);
        clipper
    }

    #[win32_derive::dllexport]
    pub fn Release(_machine: &mut Machine, this: u32) -> u32 {
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    pub fn SetHWnd(_machine: &mut Machine, this: u32, unused: u32, hwnd: HWND) -> DD {
        DD::OK
    }
}
