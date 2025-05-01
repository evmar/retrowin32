#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod builtin;

pub use builtin::DLL;

use win32_system::System;
use win32_winapi::HRESULT;

#[win32_derive::dllexport]
pub fn OleInitialize(sys: &dyn System, _pvReserved: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn CoInitialize(sys: &dyn System, pvReserved: u32) -> HRESULT {
    todo!();
}

#[win32_derive::dllexport]
pub fn CoInitializeEx(sys: &dyn System, pvReserved: Option<&mut u32>, dwCoInit: u32) -> u32 {
    0 // ok
}

#[win32_derive::dllexport]
pub fn CoUninitialize(sys: &dyn System) {}

#[win32_derive::dllexport]
pub fn CoCreateInstance(
    sys: &dyn System,
    rclsid: u32,
    pUnkOuter: u32,
    dwClsContext: u32,
    riid: u32,
    ppv: u32,
) -> HRESULT {
    todo!();
}
