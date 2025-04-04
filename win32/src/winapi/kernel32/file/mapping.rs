use super::HFILE;
use crate::{
    winapi::{kernel32::SECURITY_ATTRIBUTES, HANDLE},
    Machine,
};

#[win32_derive::dllexport]
pub fn CreateFileMappingA(
    _machine: &mut Machine,
    hFile: HFILE,
    lpFileMappingAttributes: Option<&mut SECURITY_ATTRIBUTES>,
    flProtect: u32, /* PAGE_PROTECTION_FLAGS */
    dwMaximumSizeHigh: u32,
    dwMaximumSizeLow: u32,
    lpName: Option<&str>,
) -> HANDLE<()> {
    todo!()
}
