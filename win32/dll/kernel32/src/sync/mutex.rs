use crate::SECURITY_ATTRIBUTES;
use win32_system::System;
use win32_winapi::HANDLE;

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
