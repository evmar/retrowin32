//! WaitFor* functions that can block on various types of kernel objects.

use crate::{
    Machine,
    winapi::{HANDLE, kernel32::KernelObject},
};
use memory::Extensions;
use win32_system::{ArcEvent, System, Wait, WaitResult};

impl KernelObject {
    pub fn get_event(&self) -> &ArcEvent {
        match self {
            KernelObject::Event(event) => event,
            KernelObject::Thread(thread) => &thread.terminated,
        }
    }
}

/// The primitive beneath WaitForMultipleObjects etc.
pub async fn wait_for_events(
    sys: &mut dyn System,
    events: Box<[ArcEvent]>,
    wait_all: bool,
    wait: Wait,
) -> WaitResult {
    if wait_all {
        todo!("WaitForMultipleObjects: bWaitAll");
    }

    loop {
        for (i, event) in events.iter().enumerate() {
            let mut signaled = event.signaled.lock().unwrap();
            if *signaled {
                if !event.manual_reset {
                    // TODO: this should wake up exactly one waiting thread
                    *signaled = false;
                }
                return WaitResult::Object(i as u32);
            }
        }

        let until = match wait {
            Wait::None => return WaitResult::Timeout,
            Wait::Millis(until) => {
                if sys.host().ticks() >= until {
                    return WaitResult::Timeout;
                }
                Some(until)
            }
            Wait::Forever => None,
        };

        // log::info!(
        //     "{:?}: waiting for {:?}",
        //     crate::winapi::kernel32::current_thread(machine),
        //     handles
        // );

        sys.block(until).await;
    }
}

#[win32_derive::dllexport]
pub async fn WaitForSingleObject(
    machine: &mut Machine,
    handle: HANDLE<()>,
    dwMilliseconds: u32,
) -> u32 {
    let event = machine
        .state
        .kernel32
        .objects
        .get(handle)
        .unwrap()
        .get_event()
        .clone();
    let wait = Wait::from_millis(machine.host(), dwMilliseconds);
    wait_for_events(machine, [event].into(), false, wait)
        .await
        .to_code()
}

#[win32_derive::dllexport]
pub async fn WaitForMultipleObjects(
    machine: &mut Machine,
    nCount: u32,
    lpHandles: u32,
    bWaitAll: bool,
    dwMilliseconds: u32,
) -> u32 /* WAIT_EVENT */ {
    let events = machine
        .mem()
        .iter_pod::<HANDLE<()>>(lpHandles, nCount)
        .map(|handle| {
            machine
                .state
                .kernel32
                .objects
                .get(handle)
                .unwrap()
                .get_event()
                .clone()
        })
        .collect::<Vec<_>>();
    let wait = Wait::from_millis(machine.host(), dwMilliseconds);
    wait_for_events(machine, events.into(), false, wait)
        .await
        .to_code()
}
