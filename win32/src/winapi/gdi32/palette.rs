use crate::{winapi::gdi32::HGDIOBJ, Machine};

const TRACE_CONTEXT: &'static str = "gdi32/palette";

#[win32_derive::dllexport]
pub fn CreatePalette(machine: &mut Machine, plpal: u32) -> HGDIOBJ {
    HGDIOBJ::null()
}
