use super::{CLR_INVALID, HDC};
use crate::{
    winapi::{stack_args::ArrayWithSize, types::HANDLE},
    Machine,
};
use memory::Pod;

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

#[repr(C)]
#[derive(Debug)]
pub struct TEXTMETRICA {
    pub tmHeight: u32,
    pub tmAscent: u32,
    pub tmDescent: u32,
    pub tmInternalLeading: u32,
    pub tmExternalLeading: u32,
    pub tmAveCharWidth: u32,
    pub tmMaxCharWidth: u32,
    pub tmWeight: u32,
    pub tmOverhang: u32,
    pub tmDigitizedAspectX: u32,
    pub tmDigitizedAspectY: u32,
    pub tmFirstChar: u8,
    pub tmLastChar: u8,
    pub tmDefaultChar: u8,
    pub tmBreakChar: u8,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: u8,
    pub tmCharSet: u8,
}
unsafe impl memory::Pod for TEXTMETRICA {}

#[repr(C)]
#[derive(Debug)]
pub struct TEXTMETRICW {
    pub tmHeight: u32,
    pub tmAscent: u32,
    pub tmDescent: u32,
    pub tmInternalLeading: u32,
    pub tmExternalLeading: u32,
    pub tmAveCharWidth: u32,
    pub tmMaxCharWidth: u32,
    pub tmWeight: u32,
    pub tmOverhang: u32,
    pub tmDigitizedAspectX: u32,
    pub tmDigitizedAspectY: u32,
    pub tmFirstChar: u16,
    pub tmLastChar: u16,
    pub tmDefaultChar: u16,
    pub tmBreakChar: u16,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: u8,
    pub tmCharSet: u8,
}
unsafe impl memory::Pod for TEXTMETRICW {}


#[win32_derive::dllexport]
pub fn GetTextMetricsA(_machine: &mut Machine, hdc: HDC, lptm: Option<&mut TEXTMETRICA>) -> bool {
    let tm = lptm.unwrap();
    *tm = TEXTMETRICA::zeroed();

    // SkiFree only cares about the height, just make something up for now.
    tm.tmHeight = 12;
    true
}

#[win32_derive::dllexport]
pub fn GetTextMetricsW(_machine: &mut Machine, hdc: HDC, lptm: Option<&mut TEXTMETRICW>) -> bool {
    let tm = lptm.unwrap();
    *tm = TEXTMETRICW::zeroed();

    // SkiFree only cares about the height, just make something up for now.
    tm.tmHeight = 12;
    true
}

#[repr(C)]
#[derive(Debug)]
pub struct SIZE {
    pub cx: i32,
    pub cy: i32,
}
unsafe impl memory::Pod for SIZE {}

#[win32_derive::dllexport]
pub fn GetTextExtentPoint32A(
    _machine: &mut Machine,
    hdc: HDC,
    lpString: Option<&str>,
    c: i32,
    psizl: Option<&mut SIZE>,
) -> bool {
    *psizl.unwrap() = SIZE {
        cx: lpString.unwrap().len() as i32 * 10,
        cy: 12,
    };
    true
}
