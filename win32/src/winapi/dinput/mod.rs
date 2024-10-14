#![allow(non_snake_case)]

use crate::Machine;

#[win32_derive::dllexport]
pub fn DirectInputCreateA(
    machine: &mut Machine,
    version: u32,
    ppDI: Option<&mut u32>,
    pUnkOuter: u32,
) -> u32 {
    todo!()
}
