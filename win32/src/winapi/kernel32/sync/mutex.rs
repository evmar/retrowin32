use crate::{
    Machine,
    winapi::{HANDLE, kernel32::SECURITY_ATTRIBUTES},
};

#[win32_derive::dllexport]
pub fn CreateMutexA(
    _machine: &mut Machine,
    lpMutexAttributes: Option<&mut SECURITY_ATTRIBUTES>,
    bInitialOwner: bool,
    lpName: Option<&str>,
) -> HANDLE<()> {
    HANDLE::null() // fail
}

#[win32_derive::dllexport]
pub fn OpenMutexA(
    _machine: &mut Machine,
    dwDesiredAccess: u32,
    bInheritHandle: bool,
    lpName: Option<&str>,
) -> HANDLE<()> {
    HANDLE::null() // fail
}
