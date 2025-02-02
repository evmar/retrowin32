//! "National Language Support", e.g. code page conversions.

use crate::{str16::Str16, winapi::calling_convention::ArrayWithSizeMut, Machine};
use bitflags::bitflags;
use memory::Extensions;

/// Code pages
#[derive(Debug, win32_derive::TryFromEnum)]
pub enum CP {
    /// The system default Windows ANSI code page.
    ACP = 0,
    OEMCP = 1,
    WINDOWS_1252 = 1252,
    UTF8 = 65001,
}

#[win32_derive::dllexport]
pub fn GetACP(_machine: &mut Machine) -> u32 {
    1252 // windows-1252
}

#[win32_derive::dllexport]
pub fn GetOEMCP(_machine: &mut Machine) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn IsValidCodePage(_machine: &mut Machine, CodePage: u32) -> bool {
    CodePage == 1252
}

#[win32_derive::dllexport]
pub fn GetCPInfo(_machine: &mut Machine, _CodePage: u32, _lpCPInfo: u32) -> u32 {
    0 // fail
}

bitflags! {
    #[derive(win32_derive::TryFromBitflags)]
    pub struct MB: u32 {
        const PRECOMPOSED = 0x00000001;
        const COMPOSITE = 0x00000002;
        const USEGLYPHCHARS = 0x00000004;
        const ERR_INVALID_CHARS = 0x00000008;
    }
}

#[win32_derive::dllexport]
pub fn MultiByteToWideChar(
    machine: &mut Machine,
    CodePage: Result<CP, u32>,
    dwFlags: Result<MB, u32>,
    lpMultiByteStr: u32,
    cbMultiByte: i32,
    lpWideCharStr: ArrayWithSizeMut<u16>,
) -> u32 {
    match CodePage {
        Err(value) => unimplemented!("MultiByteToWideChar code page {value}"),
        _ => {} // treat all others as ansi for now
    }
    // TODO: obey dwFlags
    dwFlags.unwrap();

    let input_len = match cbMultiByte {
        0 => return 0,                                               // TODO: invalid param
        -1 => machine.mem().slicez(lpMultiByteStr).len() as u32 + 1, // include nul
        len => len as u32,
    };

    let mut lpWideCharStr = lpWideCharStr.to_option();
    match lpWideCharStr {
        Some(buf) if buf.len() == 0 => lpWideCharStr = None,
        _ => (),
    };

    match lpWideCharStr {
        None => input_len,
        Some(buf) => {
            let input = machine.mem().sub32(lpMultiByteStr, input_len);
            let mut len = 0;
            for (&c_in, c_out) in std::iter::zip(input, buf) {
                if c_in > 0x7f {
                    unimplemented!("unicode");
                }
                *c_out = c_in as u16;
                len += 1;
            }
            len
        }
    }
}

bitflags! {
    #[derive(win32_derive::TryFromBitflags)]
    pub struct WC: u32 {
        const COMPOSITECHECK = 0x00000200;
        const DISCARDNS = 0x00000010;
        const SEPCHARS = 0x00000020;
        const DEFAULTCHAR = 0x00000040;
        const ERR_INVALID_CHARS = 0x00000080;
        const NO_BEST_FIT_CHARS = 0x00000400;
    }
}

#[win32_derive::dllexport]
pub fn WideCharToMultiByte(
    machine: &mut Machine,
    CodePage: Result<CP, u32>,
    dwFlags: Result<WC, u32>,
    lpWideCharStr: u32,
    cchWideChar: i32,
    lpMultiByteStr: u32,
    cbMultiByte: i32,
    lpUsedDefaultChar: Option<&mut u32>,
) -> u32 {
    match CodePage {
        Err(value) => unimplemented!("WideCharToMultiByte code page {value}"),
        _ => {} // treat all others as ansi for now
    }
    dwFlags.unwrap();

    0
}

#[win32_derive::dllexport]
pub fn IsDBCSLeadByteEx(_machine: &mut Machine, _TestChar: u8, _CodePage: u32) -> bool {
    // TODO
    false
}

#[win32_derive::dllexport]
pub fn IsDBCSLeadByte(_machine: &mut Machine, _TestChar: u8) -> bool {
    false
}

pub type LCID = u32;

/// en-US locale identifier.
pub const LCID_EN_US: u32 = 0x409;

#[win32_derive::dllexport]
pub fn GetThreadLocale(_machine: &mut Machine) -> LCID {
    LCID_EN_US
}

#[win32_derive::dllexport]
pub fn GetLocaleInfoW(
    _machine: &mut Machine,
    Locale: u32,
    LCType: u32,
    lpLCData: Option<&Str16>,
    cchData: i32,
) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetLocaleInfoA(
    _machine: &mut Machine,
    Locale: u32,
    LCType: u32,
    lpLCData: Option<&str>,
    cchData: i32,
) -> i32 {
    0 // fail
}

#[win32_derive::dllexport]
pub fn LCMapStringA(
    _machine: &mut Machine,
    locale: LCID,
    dwMapFlags: u32,
    lpSrcStr: u32,
    cchSrc: i32,
    lpDestStr: ArrayWithSizeMut<u8>,
) -> i32 {
    todo!();
}

#[win32_derive::dllexport]
pub fn LCMapStringW(
    _machine: &mut Machine,
    locale: LCID,
    dwMapFlags: u32,
    lpSrcStr: u32,
    cchSrc: i32,
    lpDestStr: ArrayWithSizeMut<u16>,
) -> i32 {
    todo!();
}

#[win32_derive::dllexport]
pub fn GetStringTypeA(
    _machine: &mut Machine,
    Locale: LCID,
    dwInfoType: u32,
    lpSrcStr: u32,
    cchSrc: i32,
    lpCharType: Option<&mut u32>,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn GetStringTypeW(
    _machine: &mut Machine,
    dwInfoType: u32,
    lpSrcStr: u32,
    cchSrc: i32,
    lpCharType: Option<&mut u32>,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn EnumSystemLocalesA(
    _machine: &mut Machine,
    lpLocaleEnumProc: u32, /* LOCALE_ENUMPROCA */
    dwFlags: u32,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn IsValidLocale(
    _machine: &mut Machine,
    Locale: u32,
    dwFlags: u32, /* IS_VALID_LOCALE_FLAGS */
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetUserDefaultLCID(_machine: &mut Machine) -> u32 {
    todo!()
}
