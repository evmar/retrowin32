use super::HFILE;
use crate::{
    System,
    winapi::{HANDLE, kernel32::SECURITY_ATTRIBUTES},
};

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
