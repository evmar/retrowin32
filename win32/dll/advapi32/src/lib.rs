#![allow(non_snake_case)]

mod builtin;

pub use builtin::DLL;

use win32_system::System;
use win32_winapi::{ERROR, Str16};

pub type HKEY = u32;

#[win32_derive::dllexport]
pub fn RegCreateKeyA(
    sys: &dyn System,
    hKey: HKEY,
    lpSubKey: Option<&str>,
    phkResult: Option<&mut u32>,
) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn RegCreateKeyExW(
    sys: &dyn System,
    hKey: HKEY,
    lpSubKey: Option<&Str16>,
    Reserved: u32,
    lpClass: Option<&Str16>,
    dwOptions: u32,
    samDesired: u32,
    lpSecurityAttributes: u32,
    phkResult: Option<&mut u32>,
    lpdwDisposition: Option<&mut u32>,
) -> u32 {
    0 // success
}

#[win32_derive::dllexport]
pub fn RegOpenKeyExA(
    sys: &dyn System,
    hKey: HKEY,
    lpSubKey: Option<&str>,
    ulOptions: u32,
    samDesired: u32,
    phkResult: Option<&mut HKEY>,
) -> ERROR {
    ERROR::FILE_NOT_FOUND
}

#[win32_derive::dllexport]
pub fn RegCloseKey(sys: &dyn System, hKey: HKEY) -> u32 {
    0 // success
}

#[win32_derive::dllexport]
pub fn RegQueryValueExA(
    sys: &dyn System,
    hKey: HKEY,
    lpValueName: Option<&str>,
    lpReserved: u32,
    lpType: Option<&mut u32>,
    lpData: u32,
    lpcbData: Option<&mut u32>,
) -> u32 {
    2 // ERROR_FILE_NOT_FOUND 
}

#[win32_derive::dllexport]
pub fn RegQueryValueExW(
    sys: &dyn System,
    hKey: HKEY,
    lpValueName: Option<&Str16>,
    lpReserved: u32,
    lpType: Option<&mut u32>,
    lpData: u32,
    lpcbData: Option<&mut u32>,
) -> u32 {
    2 // ERROR_FILE_NOT_FOUND
}

#[win32_derive::dllexport]
pub fn RegSetValueExA(
    sys: &dyn System,
    hKey: HKEY,
    lpValueName: Option<&str>,
    Reserved: u32,
    dwType: u32,
    lpData: u32,
    cbData: u32,
) -> u32 {
    0 // success
}

#[win32_derive::dllexport]
pub fn RegSetValueExW(
    sys: &dyn System,
    hKey: HKEY,
    lpValueName: Option<&Str16>,
    Reserved: u32,
    dwType: u32,
    lpData: u32,
    cbData: u32,
) -> u32 {
    0 // success
}
