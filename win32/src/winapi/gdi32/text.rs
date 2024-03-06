use super::*;
use crate::winapi::stack_args::ArrayWithSize;

const TRACE_CONTEXT: &'static str = "gdi32/text";

pub struct FONT {}
pub type HFONT = HANDLE<FONT>;

#[win32_derive::dllexport]
pub fn CreateFontA(
    _machine: &mut Machine,
    cHeight: i32,
    cWidth: i32,
    cEscapement: i32,
    cOrientation: i32,
    cWeight: u32,
    bItalic: u32,
    bUnderline: u32,
    bStrikeOut: u32,
    iCharSet: u32,
    iOutPrecision: u32,
    iClipPrecision: u32,
    iQuality: u32,
    iPitchAndFamily: u32,
    pszFaceName: Option<&str>,
) -> HFONT {
    HFONT::null()
}

#[win32_derive::dllexport]
pub fn SetTextColor(_machine: &mut Machine, hdc: HDC, color: u32) -> u32 {
    CLR_INVALID // fail
}

#[win32_derive::dllexport]
pub fn TextOutA(
    _machine: &mut Machine,
    hdc: HDC,
    x: u32,
    y: u32,
    lpString: ArrayWithSize<u8>,
) -> bool {
    let _text = std::str::from_utf8(lpString.unwrap()).unwrap();
    true
}
