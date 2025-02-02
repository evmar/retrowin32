//! Functions that work with .ini files.

use crate::{
    winapi::{calling_convention::ArrayWithSizeMut, types::Str16},
    Machine,
};

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
pub fn GetPrivateProfileStringA(
    _machine: &mut Machine,
    lpAppName: Option<&str>,
    lpKeyName: Option<&str>,
    lpDefault: Option<&str>,
    lpReturnedString: Option<&str>,
    nSize: u32,
    lpFileName: Option<&str>,
) -> u32 {
    todo!()
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

#[win32_derive::dllexport]
pub fn GetProfileIntW(
    _machine: &mut Machine,
    lpAppName: Option<&Str16>,
    lpKeyName: Option<&Str16>,
    nDefault: i32,
) -> u32 {
    nDefault as u32
}

#[win32_derive::dllexport]
pub fn GetProfileStringW(
    _machine: &mut Machine,
    lpAppName: Option<&Str16>,
    lpKeyName: Option<&Str16>,
    lpDefault: Option<&Str16>,
    lpReturnedString: ArrayWithSizeMut<u16>,
) -> u32 {
    let dst = lpReturnedString.unwrap();
    let src = lpDefault.unwrap();
    let copy_len = std::cmp::min(dst.len() - 1, src.len());
    dst[..copy_len].copy_from_slice(&src.buf()[..copy_len]);
    dst[copy_len] = 0;
    copy_len as u32
}

#[win32_derive::dllexport]
pub fn WriteProfileStringW(
    _machine: &mut Machine,
    lpAppName: Option<&Str16>,
    lpKeyName: Option<&Str16>,
    lpString: Option<&Str16>,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn WritePrivateProfileStringA(
    _machine: &mut Machine,
    lpAppName: Option<&str>,
    lpKeyName: Option<&str>,
    lpString: Option<&str>,
    lpFileName: Option<&str>,
) -> bool {
    todo!()
}
