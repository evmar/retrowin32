use super::{CLR_INVALID, HDC};
use crate::{
    calling_convention::Array,
    winapi::{gdi32::COLORREF, HANDLE, LPARAM},
    Machine,
};
use memory::Pod;

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

pub const LF_FACESIZE: usize = 32;
#[repr(C)]
#[derive(Debug)]
pub struct LOGFONTA {
    pub lfHeight: u32,
    pub lfWidth: u32,
    pub lfEscapement: u32,
    pub lfOrientation: u32,
    pub lfWeight: u32,
    pub lfItalic: u8,
    pub lfUnderline: u8,
    pub lfStrikeOut: u8,
    pub lfCharSet: u8,
    pub lfOutPrecision: u8,
    pub lfClipPrecision: u8,
    pub lfQuality: u8,
    pub lfPitchAndFamily: u8,
    pub lfFaceName: [u8; LF_FACESIZE],
}
unsafe impl memory::Pod for LOGFONTA {}

#[win32_derive::dllexport]
pub fn EnumFontFamiliesExA(
    _machine: &mut Machine,
    hdc: HDC,
    lpLogfont: Option<&mut LOGFONTA>,
    lpProc: u32, /* FONTENUMPROCA */
    lParam: LPARAM,
    dwFlags: u32,
) -> i32 {
    // no calls to lpProc callback made
    0
}

#[win32_derive::dllexport]
pub fn SetTextAlign(_machine: &mut Machine, hdc: HDC, fMode: u32) -> u32 {
    0 // TA_LEFT | TA_TOP | TA_NOUPDATECP
}

#[win32_derive::dllexport]
pub fn SetTextColor(_machine: &mut Machine, hdc: HDC, color: COLORREF) -> COLORREF {
    CLR_INVALID // fail
}

#[win32_derive::dllexport]
pub fn TextOutA(_machine: &mut Machine, hdc: HDC, x: u32, y: u32, lpString: Array<u8>) -> bool {
    true
}

#[win32_derive::dllexport]
pub fn TextOutW(_machine: &mut Machine, hdc: HDC, x: u32, y: u32, lpString: Array<u16>) -> bool {
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

#[win32_derive::dllexport]
pub fn GetTextExtentPoint32W(
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
