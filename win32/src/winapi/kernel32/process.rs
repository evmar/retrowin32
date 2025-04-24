use super::{SECURITY_ATTRIBUTES, STARTUPINFOA};
use crate::{
    Machine,
    winapi::{HANDLE, Str16},
};

pub type HPROCESS = HANDLE<()>;

// MSDN: "A pseudo handle is a special constant, currently (HANDLE)-1, that is interpreted as the current process handle."
pub const CURRENT_PROCESS_HANDLE: HPROCESS = HPROCESS::from_raw(-1i32 as u32);

#[win32_derive::dllexport]
pub fn GetCurrentProcess(_machine: &mut Machine) -> HPROCESS {
    CURRENT_PROCESS_HANDLE
}

#[win32_derive::dllexport]
pub fn GetExitCodeProcess(
    _machine: &mut Machine,
    hProcess: HPROCESS,
    lpExitCode: Option<&mut u32>,
) -> bool {
    todo!()
}

pub type PROCESS_INFORMATION = u32; // TODO

pub type STARTUPINFOW = STARTUPINFOA; // TODO: same layout, different strings

#[win32_derive::dllexport]
pub fn CreateProcessW(
    _machine: &mut Machine,
    lpApplicationName: Option<&Str16>,
    lpCommandLine: Option<&Str16>,
    lpProcessAttributes: Option<&mut SECURITY_ATTRIBUTES>,
    lpThreadAttributes: Option<&mut SECURITY_ATTRIBUTES>,
    bInheritHandles: bool,
    dwCreationFlags: u32, /* PROCESS_CREATION_FLAGS */
    lpEnvironment: Option<&mut u32>,
    lpCurrentDirectory: Option<&Str16>,
    lpStartupInfo: Option<&mut STARTUPINFOW>,
    lpProcessInformation: Option<&mut PROCESS_INFORMATION>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn CreateProcessA(
    _machine: &mut Machine,
    lpApplicationName: Option<&str>,
    lpCommandLine: Option<&str>,
    lpProcessAttributes: Option<&mut SECURITY_ATTRIBUTES>,
    lpThreadAttributes: Option<&mut SECURITY_ATTRIBUTES>,
    bInheritHandles: bool,
    dwCreationFlags: u32, /* PROCESS_CREATION_FLAGS */
    lpEnvironment: Option<&mut u8>,
    lpCurrentDirectory: Option<&str>,
    lpStartupInfo: Option<&mut STARTUPINFOA>,
    lpProcessInformation: Option<&mut PROCESS_INFORMATION>,
) -> bool {
    todo!()
}
