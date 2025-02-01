use crate::winapi::types::CStr;
use crate::Machine;

#[win32_derive::dllexport(cdecl)]
pub fn _exit(machine: &mut Machine, status: u32) {
    machine.exit(status);
}

#[win32_derive::dllexport(cdecl)]
pub fn exit(machine: &mut Machine, status: u32) {
    machine.exit(status);
}

#[win32_derive::dllexport(cdecl)]
pub fn strlen(_machine: &mut Machine, lpString: Option<&CStr>) -> u32 {
    // The mapping to str already computes the string length.
    lpString.unwrap().count_bytes() as u32
}
