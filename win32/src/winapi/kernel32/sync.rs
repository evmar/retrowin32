//! Synchronization.  Currently all no-ops as we don't support threads.

use crate::{winapi::types::HANDLE, Machine};

const TRACE_CONTEXT: &'static str = "kernel32/misc";

#[win32_derive::dllexport]
pub fn WaitForSingleObject(
    _machine: &mut Machine,
    hHandle: HANDLE<()>,
    dwMilliseconds: u32,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn CreateEventA(
    _machine: &mut Machine,
    lpEventAttributes: u32,
    bManualReset: bool,
    bInitialState: bool,
    lpName: Option<&str>,
) -> HANDLE<()> {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetEvent(_machine: &mut Machine, hEvent: HANDLE<()>) -> bool {
    todo!()
}
