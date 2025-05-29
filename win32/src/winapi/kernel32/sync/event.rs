use crate::winapi::kernel32;
use kernel32::{KernelObject, KernelObjectsMethods, get_state};
use win32_system::{Event, System};
use win32_winapi::HANDLE;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HEVENTT;
pub type HEVENT = HANDLE<HEVENTT>;

#[win32_derive::dllexport]
pub fn CreateEventA(
    sys: &dyn System,
    lpEventAttributes: u32,
    bManualReset: bool,
    bInitialState: bool,
    lpName: Option<&str>,
) -> HEVENT {
    let name = if let Some(name) = lpName {
        let state = get_state(sys);
        if let Some(ev) = state.objects.iter().find(|(_, ev)| {
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
        get_state(sys)
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
pub fn SetEvent(sys: &dyn System, hEvent: HEVENT) -> bool {
    get_state(sys).objects.get_event(hEvent).unwrap().signal();
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
