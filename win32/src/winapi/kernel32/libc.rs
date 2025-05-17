//! For some reason kernel32 exports functions that I would've expected to find in the libc...

use crate::{Machine, System, winapi::Str16};
use memory::{Extensions, ExtensionsMut};

#[win32_derive::dllexport]
pub fn lstrlenA(sys: &dyn System, lpString: Option<&str>) -> u32 {
    match lpString {
        None => 0,
        // The mapping to str already computes the string length.
        Some(str) => str.len() as u32,
    }
}

#[win32_derive::dllexport]
pub fn lstrlenW(sys: &dyn System, lpString: Option<&Str16>) -> u32 {
    match lpString {
        None => 0,
        // The mapping to Str16 already computes the string length.
        Some(str) => str.len() as u32,
    }
}

#[win32_derive::dllexport]
pub fn lstrcpyA(machine: &mut Machine, lpString1: u32, lpString2: Option<&str>) -> u32 {
    let src = lpString2.unwrap();
    let dst = machine.mem().sub32_mut(lpString1, (src.len() + 1) as u32);
    dst[..src.len()].copy_from_slice(src.as_bytes());
    dst[src.len()] = 0;
    lpString1
}

#[win32_derive::dllexport]
pub fn lstrcpyW(machine: &mut Machine, lpString1: u32, lpString2: Option<&Str16>) -> u32 {
    let lpString2 = lpString2.unwrap();
    // lpString1 is a buffer of unspecified size!
    let copy_len = lpString2.len();
    let dst = machine.mem().sub32_mut(lpString1, copy_len as u32 + 2);
    let src = lpString2.as_bytes();
    dst[..copy_len].copy_from_slice(src);
    dst[copy_len..copy_len + 2].copy_from_slice(&[0, 0]);
    lpString1
}

#[win32_derive::dllexport]
pub fn lstrcpynA(
    sys: &dyn System,
    lpString1: u32,
    lpString2: Option<&str>,
    iMaxLength: u32,
) -> u32 {
    let lpString2 = lpString2.unwrap();
    let len = lpString2.len().min((iMaxLength - 1) as usize);
    let mem = sys.mem();
    let dst = mem.sub32_mut(lpString1, len as u32 + 1);
    dst[..len].copy_from_slice(lpString2.as_bytes());
    dst[len] = 0;
    lpString1
}

#[win32_derive::dllexport]
pub fn lstrcmpA(sys: &dyn System, lpString1: Option<&str>, lpString2: Option<&str>) -> i32 {
    let str1 = lpString1.unwrap();
    let str2 = lpString2.unwrap();

    for (a, b) in str1.bytes().zip(str2.bytes()) {
        if a < b {
            return -1;
        } else if a > b {
            return 1;
        }
    }

    str1.len().cmp(&str2.len()) as i32
}

#[win32_derive::dllexport]
pub fn lstrcmpiA(sys: &dyn System, lpString1: Option<&str>, lpString2: Option<&str>) -> i32 {
    let str1 = lpString1.unwrap();
    let str2 = lpString2.unwrap();

    for (a, b) in str1.bytes().zip(str2.bytes()) {
        let a = a.to_ascii_lowercase();
        let b = b.to_ascii_lowercase();
        if a < b {
            return -1;
        } else if a > b {
            return 1;
        }
    }

    str1.len().cmp(&str2.len()) as i32
}

#[win32_derive::dllexport]
pub fn lstrcatA(sys: &dyn System, lpString1: u32, lpString2: Option<&str>) -> u32 {
    let mem = sys.mem();
    let dst_len = mem.slicez(lpString1).len() as u32;
    let src = lpString2.unwrap();
    let dst = mem.sub32_mut(lpString1, dst_len + src.len() as u32 + 1);
    dst[..src.len()].copy_from_slice(src.as_bytes());
    dst[src.len()] = 0;
    lpString1
}
