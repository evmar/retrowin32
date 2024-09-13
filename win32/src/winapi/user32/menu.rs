use crate::{winapi::types::HWND, Machine};

const TRACE_CONTEXT: &'static str = "user32/menu";

pub type HMENU = u32;

#[win32_derive::dllexport]
pub fn CheckMenuItem(_machine: &mut Machine, hMenu: HMENU, uIDCheckItem: u32, uCheck: u32) -> u32 {
    0 // previous state: unchecked
}

#[win32_derive::dllexport]
pub fn LoadMenuA(_machine: &mut Machine, hInstance: u32, lpMenuName: u32) -> HMENU {
    0 // null
}

#[win32_derive::dllexport]
pub fn SetMenu(_machine: &mut Machine, hWnd: HWND, hMenu: HMENU) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn SetMenuItemInfoA(_machine: &mut Machine, hMenu: HMENU, item: u32, fByPosition: bool, lpmii: u32) -> bool {
    false // fail
}

#[win32_derive::dllexport]
pub fn GetSystemMenu(_machine: &mut Machine, hWnd: HWND, bRevert: bool) -> HMENU {
    0 // null
}

#[win32_derive::dllexport]
pub fn AppendMenuA(
    _machine: &mut Machine,
    hMenu: HMENU,
    uFlags: u32,
    uIDNewItem: u32,
    lpNewItem: Option<&str>,
) -> bool {
    false // fail
}
