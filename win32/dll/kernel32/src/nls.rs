//! "National Language Support", e.g. code page conversions.

use bitflags::bitflags;
use memory::{Extensions, ExtensionsMut};
use win32_system::System;
use win32_winapi::{Str16, calling_convention::ArrayOut};

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
pub fn GetACP(sys: &dyn System) -> u32 {
    1252 // windows-1252
}

#[win32_derive::dllexport]
pub fn GetOEMCP(sys: &dyn System) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetConsoleOutputCP(sys: &dyn System) -> u32 {
    CP::ACP as u32
}

#[win32_derive::dllexport]
pub fn IsValidCodePage(sys: &dyn System, CodePage: u32) -> bool {
    CodePage == 1252
}

#[win32_derive::dllexport]
pub fn GetCPInfo(sys: &dyn System, _CodePage: u32, _lpCPInfo: u32) -> u32 {
    0 // fail
}

bitflags! {
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct MB: u32 {
        const PRECOMPOSED = 0x00000001;
        const COMPOSITE = 0x00000002;
        const USEGLYPHCHARS = 0x00000004;
        const ERR_INVALID_CHARS = 0x00000008;
    }
}

#[win32_derive::dllexport]
pub fn MultiByteToWideChar(
    sys: &dyn System,
    CodePage: Result<CP, u32>,
    dwFlags: Result<MB, u32>,
    lpMultiByteStr: u32,
    cbMultiByte: i32,
    mut lpWideCharStr: Option<ArrayOut<u16>>,
) -> u32 {
    match CodePage {
        Err(value) => unimplemented!("MultiByteToWideChar code page {value}"),
        _ => {} // treat all others as ansi for now
    }
    // TODO: obey dwFlags
    dwFlags.unwrap();

    let src_addr = lpMultiByteStr;
    let src_len = match cbMultiByte {
        0 => return 0,                                     // TODO: invalid param
        -1 => sys.mem().slicez(src_addr).len() as u32 + 1, // include nul
        len => len as u32,
    };

    let dst = &mut lpWideCharStr;
    if let Some(buf) = dst {
        if buf.len() == 0 {
            *dst = None;
        }
    }

    // TODO: reuse the conversion in winapi/string.rs.
    match dst {
        None => src_len,
        Some(dst) => {
            let src = sys.mem().sub32(src_addr, src_len);
            let mut len = 0;
            for &c in src {
                if c > 0x7f {
                    unimplemented!("unicode");
                }
                dst.put_pod(len, c as u16);
                len += 1;
            }
            len
        }
    }
}

bitflags! {
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct WC: u32 {
        const COMPOSITECHECK = 0x00000200;
        const DISCARDNS = 0x00000010;
        const SEPCHARS = 0x00000020;
        const DEFAULTCHAR = 0x00000040;
        const ERR_INVALID_CHARS = 0x00000080;
        const NO_BEST_FIT_CHARS = 0x00000400;
    }
}

fn strlen16(buf: &[u8]) -> usize {
    buf.into_iter_pod::<u16>().position(|c| c == 0).unwrap()
}

#[win32_derive::dllexport]
pub fn WideCharToMultiByte(
    sys: &dyn System,
    CodePage: Result<CP, u32>,
    dwFlags: Result<WC, u32>,
    lpWideCharStr: u32,
    cchWideChar: i32,
    lpMultiByteStr: u32,
    cbMultiByte: i32,
    lpDefaultChar: Option<&mut u32>,
    lpUsedDefaultChar: Option<&mut u32>,
) -> u32 {
    match CodePage {
        Err(value) => unimplemented!("WideCharToMultiByte code page {value}"),
        _ => {} // treat all others as ansi for now
    }
    dwFlags.unwrap();

    let src = {
        let len = match cchWideChar {
            0 => todo!(),
            -1 => strlen16(sys.mem().slice(lpWideCharStr..)) + 1, // include nul
            len => len as usize,
        };
        sys.mem().sub32(lpWideCharStr, len as u32 * 2)
    };

    let dst = if cbMultiByte > 0 {
        sys.mem().sub32_mut(lpMultiByteStr, cbMultiByte as u32)
    } else {
        &mut []
    };

    for (i, c) in src.into_iter_pod::<u16>().enumerate() {
        if c > 0x7f {
            unimplemented!("unicode");
        }
        if i < dst.len() {
            dst[i] = c as u8;
        }
    }

    if let Some(used) = lpUsedDefaultChar {
        *used = 0;
    }

    src.len() as u32 / 2
}

#[win32_derive::dllexport]
pub fn IsDBCSLeadByteEx(sys: &dyn System, _TestChar: u8, _CodePage: u32) -> bool {
    // TODO
    false
}

#[win32_derive::dllexport]
pub fn IsDBCSLeadByte(sys: &dyn System, _TestChar: u8) -> bool {
    false
}

pub type LCID = u32;

/// en-US locale identifier.
pub const LCID_EN_US: u32 = 0x409;

#[win32_derive::dllexport]
pub fn GetThreadLocale(sys: &dyn System) -> LCID {
    LCID_EN_US
}

#[win32_derive::dllexport]
pub fn GetLocaleInfoW(
    sys: &dyn System,
    Locale: u32,
    LCType: u32,
    lpLCData: Option<&Str16>,
    cchData: i32,
) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetLocaleInfoA(
    sys: &dyn System,
    Locale: u32,
    LCType: u32,
    lpLCData: Option<&str>,
    cchData: i32,
) -> i32 {
    0 // fail
}

#[win32_derive::dllexport]
pub fn LCMapStringA(
    sys: &dyn System,
    locale: LCID,
    dwMapFlags: u32,
    lpSrcStr: u32,
    cchSrc: i32,
    lpDestStr: ArrayOut<u8>,
) -> i32 {
    todo!();
}

#[win32_derive::dllexport]
pub fn LCMapStringW(
    sys: &dyn System,
    locale: LCID,
    dwMapFlags: u32,
    lpSrcStr: u32,
    cchSrc: i32,
    lpDestStr: ArrayOut<u16>,
) -> i32 {
    todo!();
}

#[win32_derive::dllexport]
pub fn GetStringTypeA(
    sys: &dyn System,
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
    sys: &dyn System,
    dwInfoType: u32,
    lpSrcStr: u32,
    cchSrc: i32,
    lpCharType: Option<&mut u32>,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn EnumSystemLocalesA(
    sys: &dyn System,
    lpLocaleEnumProc: u32, /* LOCALE_ENUMPROCA */
    dwFlags: u32,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn IsValidLocale(
    sys: &dyn System,
    Locale: u32,
    dwFlags: u32, /* IS_VALID_LOCALE_FLAGS */
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetUserDefaultLCID(sys: &dyn System) -> u32 {
    todo!()
}
