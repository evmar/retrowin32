//! WaitFor* functions that can block on various types of kernel objects.

use crate::{KernelObject, get_state};
use memory::Extensions;
use win32_system::{ArcEvent, System, Wait, WaitResult};
use win32_winapi::HANDLE;

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
        //     crate::winapi::kernel32::current_thread(sys),
        //     handles
        // );

        sys.block(until).await;
    }
}

#[win32_derive::dllexport]
pub async fn WaitForSingleObject(
    sys: &mut dyn System,
    handle: HANDLE<()>,
    dwMilliseconds: u32,
) -> u32 {
    let event = {
        let state = get_state(sys);
        state.objects.get(handle).unwrap().get_event().clone()
    };
    let wait = Wait::from_millis(sys.host(), dwMilliseconds);
    wait_for_events(sys, [event].into(), false, wait)
        .await
        .to_code()
}

#[win32_derive::dllexport]
pub async fn WaitForMultipleObjects(
    sys: &mut dyn System,
    nCount: u32,
    lpHandles: u32,
    bWaitAll: bool,
    dwMilliseconds: u32,
) -> u32 /* WAIT_EVENT */ {
    let events = {
        let state = get_state(sys);
        sys.mem()
            .iter_pod::<HANDLE<()>>(lpHandles, nCount)
            .map(|handle| state.objects.get(handle).unwrap().get_event().clone())
            .collect::<Vec<_>>()
    };
    let wait = Wait::from_millis(sys.host(), dwMilliseconds);
    wait_for_events(sys, events.into(), false, wait)
        .await
        .to_code()
}
