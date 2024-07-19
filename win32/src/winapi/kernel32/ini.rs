#![allow(non_snake_case, unused_variables)]
//! Functions that work with .ini files.

use crate::{
    winapi::{stack_args::ArrayWithSizeMut, types::Str16},
    Machine,
};

const TRACE_CONTEXT: &'static str = "kernel32/ini";

#[win32_derive::dllexport]
pub fn GetPrivateProfileIntW(
    _machine: &mut Machine,
    lpAppName: Option<&Str16>,
    lpKeyName: Option<&Str16>,
    nDefault: u32,
    lpFileName: Option<&Str16>,
) -> u32 {
    nDefault // not found
}

#[win32_derive::dllexport]
pub fn GetPrivateProfileStringW(
    _machine: &mut Machine,
    lpAppName: Option<&Str16>,
    lpKeyName: Option<&Str16>,
    lpDefault: Option<&Str16>,
    lpReturnedString: ArrayWithSizeMut<u16>,
    lpFileName: Option<&Str16>,
) -> u32 {
    let dst = lpReturnedString.unwrap();
    let src = lpDefault.unwrap();
    let copy_len = std::cmp::min(dst.len() - 1, src.len());
    dst[..copy_len].copy_from_slice(&src.buf()[..copy_len]);
    dst[copy_len] = 0;
    copy_len as u32
}
