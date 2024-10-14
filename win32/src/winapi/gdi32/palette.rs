use super::{HDC, HGDIOBJ};
use crate::Machine;

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
pub fn CreatePalette(machine: &mut Machine, plpal: u32) -> HGDIOBJ {
    HGDIOBJ::null()
}

#[win32_derive::dllexport]
pub fn RealizePalette(_machine: &mut Machine, hdc: HDC) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn SelectPalette(
    _machine: &mut Machine,
    hdc: HDC,
    hPal: HPALETTE,
    bForceBkgd: bool,
) -> HPALETTE {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetPaletteEntries(
    _machine: &mut Machine,
    hpal: HPALETTE,
    iStart: u32,
    cEntries: u32,
    pPalEntries: Option<&mut PALETTEENTRY>,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetPaletteEntries(
    _machine: &mut Machine,
    hpal: HPALETTE,
    iStart: u32,
    cEntries: u32,
    pPalEntries: Option<&mut PALETTEENTRY>,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetSystemPaletteEntries(
    _machine: &mut Machine,
    hdc: HDC,
    iStart: u32,
    cEntries: u32,
    pPalEntries: Option<&mut PALETTEENTRY>,
) -> u32 {
    todo!()
}
