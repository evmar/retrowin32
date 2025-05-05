use crate::{
    Machine, System,
    winapi::{HWND, ddraw::DD},
};
use memory::ExtensionsMut;
use win32_winapi::com::vtable;

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
        let clipper = machine.memory.process_heap.alloc(machine.memory.mem(), 4);
        let vtable = crate::loader::get_symbol(machine, "ddraw.dll", "IDirectDrawClipper");
        machine.mem().put_pod::<u32>(clipper, vtable);
        clipper
    }

    #[win32_derive::dllexport]
    pub fn Release(sys: &dyn System, this: u32) -> u32 {
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    pub fn SetHWnd(sys: &dyn System, this: u32, unused: u32, hwnd: HWND) -> DD {
        DD::OK
    }
}
