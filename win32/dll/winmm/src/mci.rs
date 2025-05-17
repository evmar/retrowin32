use win32_system::System;
use win32_winapi::{HWND, calling_convention::ArrayOut};

const MCIERR_BASE: u32 = 256;
const MCIERR_DRIVER: u32 = MCIERR_BASE + 22;

#[win32_derive::dllexport]
pub fn mciGetErrorStringA(
    sys: &dyn System,
    mcierr: u32,
    pszText: Option<&str>,
    cchText: u32,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn mciSendStringA(
    sys: &dyn System,
    lpstrCommand: Option<&str>,
    lpstrReturnString: ArrayOut<u8>,
    hwndCallback: HWND,
) -> u32 {
    let cmd = lpstrCommand.unwrap();
    log::info!("mci: {:?}", cmd);
    if cmd.find("notify").is_some() {
        todo!("mci notify not implemented");
    }
    0 // success
}

#[win32_derive::dllexport]
pub fn mciSendCommandA(
    sys: &dyn System,
    mciId: u32,
    uMsg: u32,
    dwParam1: u32,
    dwParam2: u32,
) -> u32 {
    MCIERR_DRIVER
}
