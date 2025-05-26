use crate::{
    Machine,
    winapi::kernel32::{KernelObject, KernelObjectsMethods},
};
use win32_system::{Event, System};
use win32_winapi::HANDLE;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HEVENTT;
pub type HEVENT = HANDLE<HEVENTT>;

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
                if let Some(n) = &ev.name {
                    if n == name {
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
            .add(KernelObject::Event(Event::new(
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
