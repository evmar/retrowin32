//! "National Language Support", e.g. code page conversions.

use crate::{winapi::stack_args::ArrayWithSizeMut, Machine};
use memory::Extensions;

const TRACE_CONTEXT: &'static str = "kernel32/nls";

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
pub fn IsValidCodePage(_machine: &mut Machine, CodePage: u32) -> bool {
    CodePage == 1252
}

#[win32_derive::dllexport]
pub fn GetCPInfo(_machine: &mut Machine, _CodePage: u32, _lpCPInfo: u32) -> u32 {
    0 // fail
}

#[win32_derive::dllexport]
pub fn MultiByteToWideChar(
    machine: &mut Machine,
    CodePage: Result<CP, u32>,
    dwFlags: u32,
    lpMultiByteStr: u32,
    cbMultiByte: i32,
    lpWideCharStr: ArrayWithSizeMut<u16>,
) -> u32 {
    match CodePage {
        Err(value) => unimplemented!("MultiByteToWideChar code page {value}"),
        _ => {} // treat all others as ansi for now
    }
    // TODO: dwFlags

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

#[win32_derive::dllexport]
pub fn IsDBCSLeadByteEx(_machine: &mut Machine, _TestChar: u8, _CodePage: u32) -> bool {
    // TODO
    false
}

#[win32_derive::dllexport]
pub fn IsDBCSLeadByte(_machine: &mut Machine, _TestChar: u8) -> bool {
    false
}
