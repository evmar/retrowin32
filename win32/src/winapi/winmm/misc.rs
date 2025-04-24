pub use crate::winapi::kernel32::HMODULE;
use crate::{Machine, winapi::Str16};

#[win32_derive::dllexport]
pub fn PlaySoundW(
    _machine: &mut Machine,
    pszSound: Option<&Str16>,
    hmod: HMODULE,
    fdwSound: u32,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn mciSendCommandA(_machine: &mut Machine) {
    todo!();
}
