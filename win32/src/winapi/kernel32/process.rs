use super::STARTUPINFOA;
use crate::{winapi::types::HANDLE, Machine};

#[win32_derive::dllexport]
pub fn GetExitCodeProcess(
    _machine: &mut Machine,
    hProcess: HANDLE<()>,
    lpExitCode: Option<&mut u32>,
) -> bool {
    todo!()
}

pub type SECURITY_ATTRIBUTES = u32; // TODO
pub type PROCESS_INFORMATION = u32; // TODO

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
