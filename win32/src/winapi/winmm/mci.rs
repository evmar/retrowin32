use crate::{winapi::types::HWND, Machine};

#[win32_derive::dllexport]
pub fn mciGetErrorStringA(
    _machine: &mut Machine,
    mcierr: u32,
    pszText: Option<&str>,
    cchText: u32,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn mciSendStringA(
    _machine: &mut Machine,
    lpstrCommand: Option<&str>,
    lpstrReturnString: Option<&str>,
    uReturnLength: u32,
    hwndCallback: HWND,
) -> u32 {
    todo!()
}
