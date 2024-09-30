use crate::{winapi::gdi32::HGDIOBJ, Machine};

#[win32_derive::dllexport]
pub fn CreatePalette(machine: &mut Machine, plpal: u32) -> HGDIOBJ {
    HGDIOBJ::null()
}
