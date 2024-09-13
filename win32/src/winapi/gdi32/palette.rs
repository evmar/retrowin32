use crate::{winapi::{gdi32::{Object, HGDIOBJ}, types::WORD}, Machine};

const TRACE_CONTEXT: &'static str = "gdi32/palette";

#[repr(C)]
#[derive(Clone, Debug)]
pub struct LOGPALETTE_HEADER {
    palVersion: WORD,
    palNumEntries: WORD,
}
unsafe impl memory::Pod for LOGPALETTE_HEADER {}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct PALETTEENTRY {
    pub peRed: u8,
    pub peGreen: u8,
    pub peBlue: u8,
    pub peFlags: u8,
}
unsafe impl memory::Pod for PALETTEENTRY {}

#[derive(Debug)]
pub struct Palette(LOGPALETTE_HEADER, Vec<PALETTEENTRY>);

#[win32_derive::dllexport]
pub fn CreatePalette(machine: &mut Machine, plpal: u32) -> HGDIOBJ {
    let header = machine.mem().view::<LOGPALETTE_HEADER>(plpal).clone();

    let entries = plpal + size_of::<LOGPALETTE_HEADER>() as u32;
    let entries = machine.mem().view_n::<PALETTEENTRY>(entries, header.palNumEntries as u32).to_vec();

    machine.state.gdi32.objects.add(Object::Palette(Palette(header, entries)))
}
