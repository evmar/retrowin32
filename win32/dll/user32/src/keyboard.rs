use win32_system::System;

pub type HKL = u32;

#[win32_derive::dllexport]
pub fn GetKeyState(sys: &dyn System, nVirtKey: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn GetKeyboardState(sys: &dyn System, lpKeyState: Option<&mut u8>) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn keybd_event(
    sys: &dyn System,
    bVk: u8,
    bScan: u8,
    dwFlags: u32, /* KEYBD_EVENT_FLAGS */
    dwExtraInfo: u32,
) {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetKeyboardType(sys: &dyn System, nTypeFlag: i32) -> i32 {
    0 // fail
}

#[win32_derive::dllexport]
pub fn GetKeyboardLayout(sys: &dyn System, idThread: u32) -> u32 {
    log::warn!("GetKeyboardLayout: stub");
    0 // garbage value, unclear if callers care
}

#[win32_derive::dllexport]
pub fn GetKeyboardLayoutList(sys: &dyn System, nBuff: i32, lpList: Option<&mut HKL>) -> i32 {
    log::warn!("GetKeyboardLayoutList: stub");
    0 // no layouts
}

#[win32_derive::dllexport]
pub fn MapVirtualKeyA(
    sys: &dyn System,
    uCode: u32,
    uMapType: u32, /* MAP_VIRTUAL_KEY_TYPE */
) -> u32 {
    0 // no translation
}
