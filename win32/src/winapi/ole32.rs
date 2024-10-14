#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::types::HRESULT;
use crate::Machine;

#[win32_derive::dllexport]
pub fn OleInitialize(_machine: &mut Machine, _pvReserved: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn CoInitialize(_machine: &mut Machine, pvReserved: u32) -> HRESULT {
    todo!();
}

#[win32_derive::dllexport]
pub fn CoInitializeEx(_machine: &mut Machine, pvReserved: Option<&mut u32>, dwCoInit: u32) -> u32 {
    0 // ok
}

#[win32_derive::dllexport]
pub fn CoUninitialize(_machine: &mut Machine) {
    todo!();
}

#[win32_derive::dllexport]
pub fn CoCreateInstance(
    _machine: &mut Machine,
    rclsid: u32,
    pUnkOuter: u32,
    dwClsContext: u32,
    riid: u32,
    ppv: u32,
) -> HRESULT {
    todo!();
}
