use super::{HDC, HGDIOBJ};
use win32_system::System;
use win32_winapi::HANDLE;

pub type HPALETTE = HANDLE<()>; // TODO

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
    0 // number of entries mapped
}

#[win32_derive::dllexport]
pub fn SelectPalette(sys: &dyn System, hdc: HDC, hPal: HPALETTE, bForceBkgd: bool) -> HPALETTE {
    if hPal.is_null() {
        // ok, we assume all palettes are null for now
        return hPal;
    }
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
    if hpal.is_null() {
        // ok, we assume all palettes are null for now
        return 0;
    }
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
    0 // no system palette entries
}

#[win32_derive::dllexport]
pub fn SetSystemPaletteUse(
    sys: &dyn System,
    hdc: HDC,
    use_: u32, /* SYSTEM_PALETTE_USE */
) -> u32 {
    /// System does not support palettes.
    const SYSPAL_ERROR: u32 = 0;
    SYSPAL_ERROR
}

#[win32_derive::dllexport]
pub fn ResizePalette(sys: &dyn System, hpal: HPALETTE, n: u32) -> bool {
    if hpal.is_null() {
        // ok, we assume all palettes are null for now
        return true; // success
    }
    todo!()
}
