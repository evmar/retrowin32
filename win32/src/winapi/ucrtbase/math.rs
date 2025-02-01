use crate::Machine;

#[win32_derive::dllexport(cdecl)]
pub fn __setusermatherr(machine: &mut Machine, pf: u32) {
    todo!();
}

#[win32_derive::dllexport]
pub const _adjust_fdiv: &'static str = "_adjust_fdiv";

#[win32_derive::dllexport]
pub const _acmdln: &'static str = "_acmdln";
