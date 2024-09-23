pub use crate::winapi::kernel32::HMODULE;
use crate::{str16::Str16, Machine};

const TRACE_CONTEXT: &'static str = "winmm/misc";

#[win32_derive::dllexport]
pub fn PlaySoundW(
    _machine: &mut Machine,
    pszSound: Option<&Str16>,
    hmod: HMODULE,
    fdwSound: u32,
) -> bool {
    todo!();
}
