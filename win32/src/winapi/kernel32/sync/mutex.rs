use crate::{
    System,
    winapi::{HANDLE, kernel32::SECURITY_ATTRIBUTES},
};

#[win32_derive::dllexport]
pub fn CreateMutexA(
    sys: &dyn System,
    lpMutexAttributes: Option<&mut SECURITY_ATTRIBUTES>,
    bInitialOwner: bool,
    lpName: Option<&str>,
) -> HANDLE<()> {
    HANDLE::null() // fail
}

#[win32_derive::dllexport]
pub fn OpenMutexA(
    sys: &dyn System,
    dwDesiredAccess: u32,
    bInheritHandle: bool,
    lpName: Option<&str>,
) -> HANDLE<()> {
    HANDLE::null() // fail
}
