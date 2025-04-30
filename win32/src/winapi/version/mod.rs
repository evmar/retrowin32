#![allow(non_snake_case)]

mod builtin;

pub use builtin::DLL;

use crate::System;

#[win32_derive::dllexport]
pub fn GetFileVersionInfoSizeA(
    sys: &dyn System,
    lptstrFilename: Option<&str>,
    lpdwHandle: Option<&mut u32>,
) -> u32 {
    0 // TODO
}

#[win32_derive::dllexport]
pub fn GetFileVersionInfoA(
    sys: &dyn System,
    lptstrFilename: Option<&str>,
    dwHandle: u32,
    dwLen: u32,
    lpData: u32,
) -> bool {
    false // fail
}

#[win32_derive::dllexport]
pub fn VerQueryValueA(
    sys: &dyn System,
    pBlock: u32,
    lpSubBlock: Option<&str>,
    lplpBuffer: u32,
    puLen: Option<&mut u32>,
) -> bool {
    false // fail
}
