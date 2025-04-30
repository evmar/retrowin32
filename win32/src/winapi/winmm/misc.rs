pub use crate::winapi::kernel32::HMODULE;
use crate::{System, winapi::Str16};

#[win32_derive::dllexport]
pub fn PlaySoundW(
    sys: &dyn System,
    pszSound: Option<&Str16>,
    hmod: HMODULE,
    fdwSound: u32,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn mciSendCommandA(sys: &dyn System) {
    todo!();
}
