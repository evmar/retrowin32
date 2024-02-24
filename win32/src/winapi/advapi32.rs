#![allow(non_snake_case)]

use super::types::Str16;
use crate::machine::Machine;

const TRACE_CONTEXT: &'static str = "advapi32";

pub type HKEY = u32;

#[win32_derive::dllexport]
pub fn RegCreateKeyExW(
    _machine: &mut Machine,
    hKey: HKEY,
    lpSubKey: Option<Str16>,
    Reserved: u32,
    lpClass: Option<Str16>,
    dwOptions: u32,
    samDesired: u32,
    lpSecurityAttributes: u32,
    phkResult: Option<&mut u32>,
    lpdwDisposition: Option<&mut u32>,
) -> u32 {
    0 // success
}

#[win32_derive::dllexport]
pub fn RegCloseKey(_machine: &mut Machine, hKey: HKEY) -> u32 {
    0 // success
}

#[win32_derive::dllexport]
pub fn RegQueryValueExW(
    _machine: &mut Machine,
    hKey: HKEY,
    lpValueName: Option<Str16>,
    lpReserved: u32,
    lpType: Option<&mut u32>,
    lpData: u32,
    lpcbData: Option<&mut u32>,
) -> u32 {
    2 // ERROR_FILE_NOT_FOUND
}
