use bitflags::bitflags;
use win32_system::System;
use win32_winapi::{HWND, RECT};

pub type HMENU = u32;

#[win32_derive::dllexport]
pub fn CreatePopupMenu(sys: &dyn System) -> HMENU {
    0 // null
}

#[win32_derive::dllexport]
pub fn CheckMenuItem(sys: &dyn System, hMenu: HMENU, uIDCheckItem: u32, uCheck: u32) -> u32 {
    0 // previous state: unchecked
}

bitflags! {
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct MF: u32 {
        const BYCOMMAND = 0x00000000;
        const BYPOSITION = 0x00000400;
        const DISABLED = 0x00000002;
        const ENABLED = 0x00000000;
        const GRAYED = 0x00000001;
    }
}

#[win32_derive::dllexport]
pub fn EnableMenuItem(
    sys: &dyn System,
    hMenu: HMENU,
    uIDEnableItem: u32,
    uEnable: Result<MF, u32>,
) -> i32 {
    let previous_state = MF::ENABLED;
    previous_state.bits() as i32
}

#[win32_derive::dllexport]
pub fn GetMenu(sys: &dyn System, hWnd: HWND) -> HMENU {
    0 // null
}

#[win32_derive::dllexport]
pub fn GetSubMenu(sys: &dyn System, hMenu: HMENU, nPos: i32) -> HMENU {
    0 // null
}

#[win32_derive::dllexport]
pub fn LoadMenuA(sys: &dyn System, hInstance: u32, lpMenuName: u32) -> HMENU {
    0 // null
}

#[win32_derive::dllexport]
pub fn SetMenu(sys: &dyn System, hWnd: HWND, hMenu: HMENU) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn SetMenuItemInfoA(
    sys: &dyn System,
    hMenu: HMENU,
    item: u32,
    fByPosition: bool,
    lpmii: u32,
) -> bool {
    false // fail
}

#[win32_derive::dllexport]
pub fn GetSystemMenu(sys: &dyn System, hWnd: HWND, bRevert: bool) -> HMENU {
    0 // null
}

#[win32_derive::dllexport]
pub fn AppendMenuA(
    sys: &dyn System,
    hMenu: HMENU,
    uFlags: u32,
    uIDNewItem: u32,
    lpNewItem: Option<&str>,
) -> bool {
    false // fail
}

#[win32_derive::dllexport]
pub fn GetMenuItemRect(
    sys: &dyn System,
    hWnd: HWND,
    hMenu: HMENU,
    uItem: u32,
    lprcItem: Option<&mut RECT>,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn DeleteMenu(
    sys: &dyn System,
    hMenu: HMENU,
    uPosition: u32,
    uFlags: u32, /* MENU_ITEM_FLAGS */
) -> bool {
    false // fail
}

#[win32_derive::dllexport]
pub fn DrawMenuBar(sys: &dyn System, hWnd: HWND) -> bool {
    false // fail
}
