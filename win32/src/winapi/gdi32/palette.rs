use win32_system::System;

use super::{HDC, HGDIOBJ};

pub type HPALETTE = u32; // TODO

#[repr(C)]
#[derive(Clone, Debug)]
pub struct PALETTEENTRY {
    pub peRed: u8,
    pub peGreen: u8,
    pub peBlue: u8,
    pub peFlags: u8,
}
unsafe impl memory::Pod for PALETTEENTRY {}

#[win32_derive::dllexport]
pub fn CreatePalette(sys: &dyn System, plpal: u32) -> HGDIOBJ {
    HGDIOBJ::null()
}

#[win32_derive::dllexport]
pub fn RealizePalette(sys: &dyn System, hdc: HDC) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn SelectPalette(sys: &dyn System, hdc: HDC, hPal: HPALETTE, bForceBkgd: bool) -> HPALETTE {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetPaletteEntries(
    sys: &dyn System,
    hpal: HPALETTE,
    iStart: u32,
    cEntries: u32,
    pPalEntries: Option<&mut PALETTEENTRY>,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetPaletteEntries(
    sys: &dyn System,
    hpal: HPALETTE,
    iStart: u32,
    cEntries: u32,
    pPalEntries: Option<&mut PALETTEENTRY>,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetSystemPaletteEntries(
    sys: &dyn System,
    hdc: HDC,
    iStart: u32,
    cEntries: u32,
    pPalEntries: Option<&mut PALETTEENTRY>,
) -> u32 {
    todo!()
}
