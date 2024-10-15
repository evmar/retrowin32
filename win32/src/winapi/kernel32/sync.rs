//! Synchronization.  Currently all no-ops as we don't support threads.

use crate::{
    winapi::types::{HANDLE, HEVENT},
    Machine,
};
use memory::Extensions;
use std::cell::Cell;

pub struct EventObject {
    name: Option<String>,
    manual_reset: bool,
    signaled: Cell<bool>,
}

pub const WAIT_OBJECT_0: u32 = 0;
//const WAIT_ABANDONED_0: u32 = 0x80;

#[win32_derive::dllexport]
pub fn WaitForSingleObject(machine: &mut Machine, handle: HANDLE<()>, dwMilliseconds: u32) -> u32 {
    let handle = machine.state.kernel32.handles.get(handle).unwrap();
    if handle.signaled.get() {
        return WAIT_OBJECT_0;
    }
    todo!()
}

#[win32_derive::dllexport]
pub fn WaitForMultipleObjects(
    machine: &mut Machine,
    nCount: u32,
    lpHandles: u32,
    bWaitAll: bool,
    dwMilliseconds: u32,
) -> u32 /* WAIT_EVENT */ {
    let handles = machine.mem().iter_pod::<HANDLE<()>>(lpHandles, nCount);

    if bWaitAll {
        todo!("WaitForMultipleObjects: bWaitAll");
    }
    for (i, handle) in handles.enumerate() {
        let handle = machine.state.kernel32.handles.get(handle).unwrap();
        if handle.signaled.get() {
            if handle.manual_reset {
                handle.signaled.set(false);
            }
            return WAIT_OBJECT_0 + i as u32;
        }
    }
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
        if let Some(ev) = machine.state.kernel32.handles.iter().find(|ev| {
            if let Some(n) = &ev.name {
                n == name
            } else {
                false
            }
        }) {
            todo!("CreateEventA: reusing named event");
        }
        Some(name.to_string())
    } else {
        None
    };

    HEVENT::from_raw(
        machine
            .state
            .kernel32
            .handles
            .add(EventObject {
                name,
                manual_reset: bManualReset,
                signaled: Cell::new(bInitialState),
            })
            .to_raw(),
    )
}

#[win32_derive::dllexport]
pub fn SetEvent(machine: &mut Machine, hEvent: HEVENT) -> bool {
    machine
        .state
        .kernel32
        .handles
        .get_raw(hEvent.to_raw())
        .unwrap()
        .signaled
        .set(true);
    true
}

#[win32_derive::dllexport]
pub fn ResetEvent(_machine: &mut Machine, hEvent: HEVENT) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn PulseEvent(_machine: &mut Machine, hEvent: HEVENT) -> bool {
    todo!()
}
