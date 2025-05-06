use crate::winapi::{HWND, ddraw::DD};
use win32_system::System;
use win32_winapi::com::vtable;

#[win32_derive::dllexport]
pub fn DirectDrawCreateClipper(
    sys: &mut dyn System,
    dwFlags: u32,
    lplpDDClipper: Option<&mut u32>,
    pUnkOuter: u32,
) -> DD {
    assert!(dwFlags == 0);
    *lplpDDClipper.unwrap() = IDirectDrawClipper::new(sys);
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

    pub fn new(sys: &mut dyn System) -> u32 {
        let vtable = sys.get_symbol("ddraw.dll", "IDirectDrawClipper");
        sys.memory().store(vtable)
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
