//! Synchronization.  Currently all no-ops as we don't support threads.

use crate::{winapi::types::HEVENT, Machine};

pub struct EventObject {
    name: String,
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
    let name = if let Some(name) = lpName {
        if let Some(ev) = machine
            .state
            .kernel32
            .event_handles
            .iter()
            .find(|ev| ev.name == name)
        {
            todo!("CreateEventA: reusing named event");
        }
        name.to_string()
    } else {
        "".into()
    };

    machine
        .state
        .kernel32
        .event_handles
        .add(EventObject { name, state: false })
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
