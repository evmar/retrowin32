use win32_system::System;
use win32_winapi::Str16;

#[win32_derive::dllexport]
pub fn PlaySoundW(sys: &dyn System, pszSound: Option<&Str16>, hmod: u32, fdwSound: u32) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn sndPlaySoundA(sys: &dyn System, pszSound: Option<&str>, fuSound: u32) -> bool {
    false
}
