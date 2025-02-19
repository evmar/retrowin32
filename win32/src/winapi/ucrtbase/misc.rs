use crate::winapi::CStr;
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

#[win32_derive::dllexport(cdecl)]
pub fn memcpy(machine: &mut Machine, dest: u32, src: u32, count: u32) -> u32 {
    machine.mem().copy(src, dest, count);
    dest
}
