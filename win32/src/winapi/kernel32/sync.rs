//! Synchronization.  Currently all no-ops as we don't support threads.

use crate::{winapi::types::HEVENT, Machine};

pub struct EventObject {
    state: bool,
}

#[win32_derive::dllexport]
pub fn WaitForSingleObject(_machine: &mut Machine, hHandle: HEVENT, dwMilliseconds: u32) -> u32 {
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

    machine
        .state
        .kernel32
        .event_handles
        .add(EventObject { state: false })
}

#[win32_derive::dllexport]
pub fn SetEvent(machine: &mut Machine, hEvent: HEVENT) -> bool {
    match machine.state.kernel32.event_handles.get_mut(hEvent) {
        Some(handle) => {
            handle.state = true;
            true
        }
        None => {
            log::warn!("SetEvent: invalid handle");
            false
        }
    }
}
