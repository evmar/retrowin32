use super::HFILE;
use crate::SECURITY_ATTRIBUTES;
use win32_system::System;
use win32_winapi::HANDLE;

#[win32_derive::dllexport]
pub fn CreateFileMappingA(
    sys: &dyn System,
    hFile: HFILE,
    lpFileMappingAttributes: Option<&mut SECURITY_ATTRIBUTES>,
    flProtect: u32, /* PAGE_PROTECTION_FLAGS */
    dwMaximumSizeHigh: u32,
    dwMaximumSizeLow: u32,
    lpName: Option<&str>,
) -> HANDLE<()> {
    todo!()
}
