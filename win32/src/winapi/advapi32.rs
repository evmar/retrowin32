#![allow(non_snake_case)]

use memory::ExtensionsMut;

use super::types::Str16;
use crate::machine::Machine;

const TRACE_CONTEXT: &'static str = "advapi32";

const REG_SZ: u32 = 0x00000001; // A string.

pub type HKEY = u32;

#[win32_derive::dllexport]
pub fn RegCreateKeyA(
    _machine: &mut Machine,
    hKey: HKEY,
    lpSubKey: Option<&str>,
    phkResult: Option<&mut u32>,
) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn RegCreateKeyExW(
    _machine: &mut Machine,
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
pub fn RegCloseKey(_machine: &mut Machine, hKey: HKEY) -> u32 {
    0 // success
}

#[win32_derive::dllexport]
pub fn RegQueryValueExA(
    machine: &mut Machine,
    hKey: HKEY,
    lpValueName: Option<&str>,
    lpReserved: u32,
    lpType: Option<&mut u32>,
    lpData: u32,
    lpcbData: Option<&mut u32>,
) -> u32 {
    if matches!(lpValueName, Some("ProgramFilesDir")) {
        // const PROGRAM_FILES: &str = "C:\\Program Files";
        const PROGRAM_FILES: &str = "C:\\Users\\linus\\Program Files";

        if let Some(lpType) = lpType {
            *lpType = REG_SZ;
        }

        if lpData != 0 {
            let in_out_len = lpcbData.unwrap();

            if *in_out_len <= PROGRAM_FILES.len() as u32 {
                return 234; // ERROR_MORE_DATA
            }

            *in_out_len = (PROGRAM_FILES.len() as u32) + 1;

            let buf = machine.mem().sub32_mut(lpData, *in_out_len);

            buf[..PROGRAM_FILES.len()].copy_from_slice(PROGRAM_FILES.as_bytes());
            buf[PROGRAM_FILES.len()] = 0;
        } else if let Some(lpcbData) = lpcbData {
            *lpcbData = (PROGRAM_FILES.len() as u32) + 1;
        }

        return 0;
    }

    2 // ERROR_FILE_NOT_FOUND
}

#[win32_derive::dllexport]
pub fn RegQueryValueExW(
    _machine: &mut Machine,
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
pub fn RegSetValueExW(
    _machine: &mut Machine,
    hKey: HKEY,
    lpValueName: Option<&Str16>,
    lpReserved: u32,
    lpType: u32,
    lpData: u32,
    cbData: u32,
) -> u32 {
    0 // success
}
