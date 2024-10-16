use crate::{
    winapi::{kernel32::KernelObject, types::HANDLE},
    Machine,
};
use std::cell::Cell;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HEVENTT;
pub type HEVENT = HANDLE<HEVENTT>;

pub struct EventObject {
    name: Option<String>,
    pub manual_reset: bool,
    pub signaled: Cell<bool>,
}

impl EventObject {
    pub fn new(name: Option<String>, manual_reset: bool, signaled: bool) -> Self {
        Self {
            name,
            manual_reset,
            signaled: Cell::new(signaled),
        }
    }
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
        if let Some(ev) = machine.state.kernel32.objects.iter().find(|ev| {
            if let KernelObject::Event(EventObject { name: Some(n), .. }) = ev {
                if *n == name {
                    return true;
                }
            }
            false
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
            .objects
            .add(KernelObject::Event(EventObject::new(
                name,
                bManualReset,
                bInitialState,
            )))
            .to_raw(),
    )
}

#[win32_derive::dllexport]
pub fn SetEvent(machine: &mut Machine, hEvent: HEVENT) -> bool {
    machine
        .state
        .kernel32
        .objects
        .get_event(hEvent)
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
