#![allow(non_snake_case)]

use crate::machine::Machine;

const TRACE_CONTEXT: &'static str = "version";

#[win32_derive::dllexport]
pub fn GetFileVersionInfoSizeA(
    _machine: &mut Machine,
    lptstrFilename: Option<&str>,
    lpdwHandle: Option<&mut u32>,
) -> u32 {
    0 // TODO
}
