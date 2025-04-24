use crate::{Machine, calling_convention::ArrayOut, winapi::HWND};

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
