use crate::{
    Machine, System,
    winapi::{
        HANDLE,
        kernel32::{KernelObject, KernelObjectsMethods},
    },
};
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HEVENTT;
pub type HEVENT = HANDLE<HEVENTT>;

/// Event objects are used for synchronization between threads.
//
// In the emulator there are no real threads, but we still use event objects
// for signaling all the way up to the Host layer, which itself may use threads,
// so internally Events use Mutex/Arc instead of Cell/Rc.
// Under wasm Mutex/Arc are implemented as Cell/Rc anyway:
//   https://github.com/rust-lang/rust/issues/77839
pub struct EventObject {
    name: Option<String>,
    pub manual_reset: bool,
    pub signaled: Mutex<bool>,
}

impl EventObject {
    pub fn new(name: Option<String>, manual_reset: bool, signaled: bool) -> Arc<Self> {
        Arc::new(Self {
            name,
            manual_reset,
            signaled: Mutex::new(signaled),
        })
    }

    pub fn signal(&self) {
        *self.signaled.lock().unwrap() = true;
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
        if let Some(ev) = machine.state.kernel32.objects.iter().find(|(_, ev)| {
            if let KernelObject::Event(ev) = ev {
                if let EventObject { name: Some(n), .. } = &**ev {
                    if *n == name {
                        return true;
                    }
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
        .signal();
    true
}

#[win32_derive::dllexport]
pub fn ResetEvent(sys: &dyn System, hEvent: HEVENT) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn PulseEvent(sys: &dyn System, hEvent: HEVENT) -> bool {
    todo!()
}
