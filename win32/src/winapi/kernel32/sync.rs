//! Synchronization.  Currently all no-ops as we don't support threads.

use crate::{winapi::types::HEVENT, Machine};

const TRACE_CONTEXT: &'static str = "kernel32/misc";

pub struct EventObject;

#[win32_derive::dllexport]
pub fn WaitForSingleObject(
    _machine: &mut Machine,
    hHandle: HEVENT,
    dwMilliseconds: u32,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn CreateEventA(
    machine: &mut Machine,
    lpEventAttributes: u32,
    bManualReset: bool,
    bInitialState: bool,
    lpName: Option<&str>,
) -> HEVENT {
    if lpName.is_some() {
        todo!("CreateEventA: named events not supported");
    }

    machine.state.kernel32.event_handles.add(EventObject)
}

#[win32_derive::dllexport]
pub fn SetEvent(_machine: &mut Machine, hEvent: HEVENT) -> bool {
    todo!()
}
