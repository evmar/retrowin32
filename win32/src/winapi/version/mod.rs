#![allow(non_snake_case)]

mod builtin;

pub use builtin::DLL;

use crate::machine::Machine;

#[win32_derive::dllexport]
pub fn GetFileVersionInfoSizeA(
    _machine: &mut Machine,
    lptstrFilename: Option<&str>,
    lpdwHandle: Option<&mut u32>,
) -> u32 {
    0 // TODO
}

#[win32_derive::dllexport]
pub fn GetFileVersionInfoA(
    _machine: &mut Machine,
    lptstrFilename: Option<&str>,
    dwHandle: u32,
    dwLen: u32,
    lpData: u32,
) -> bool {
    false // fail
}

#[win32_derive::dllexport]
pub fn VerQueryValueA(
    _machine: &mut Machine,
    pBlock: u32,
    lpSubBlock: Option<&str>,
    lplpBuffer: u32,
    puLen: Option<&mut u32>,
) -> bool {
    false // fail
}
