//! WaitFor* functions that can block on various types of kernel objects.

use super::EventObject;
use crate::{
    winapi::{kernel32::KernelObject, types::HANDLE},
    Machine,
};
use memory::Extensions;

pub const WAIT_OBJECT_0: u32 = 0;
//const WAIT_ABANDONED_0: u32 = 0x80;
const WAIT_TIMEOUT: u32 = 0x102;
const INFINITE: u32 = 0xffff_ffff;

impl KernelObject {
    pub fn get_event(&self) -> &EventObject {
        match self {
            KernelObject::Event(event) => event,
            KernelObject::Thread(thread) => &thread.terminated,
        }
    }
}

/// Convert a dwMilliseconds value to a wait-until time.
/// Returns:
/// - None => no waiting requested
/// - Some(until) => wait until "until", which is None in the case of forever
fn wait_from_milliseconds(machine: &mut Machine, dwMilliseconds: u32) -> Option<Option<u32>> {
    if dwMilliseconds == 0 {
        None
    } else if dwMilliseconds == INFINITE {
        Some(None)
    } else {
        Some(Some(machine.host.ticks() + dwMilliseconds))
    }
}

async fn wait_for_objects(
    machine: &mut Machine,
    handles: &[HANDLE<()>],
    bWaitAll: bool,
    dwMilliseconds: u32,
) -> u32 {
    if bWaitAll {
        todo!("WaitForMultipleObjects: bWaitAll");
    }

    let until = wait_from_milliseconds(machine, dwMilliseconds);
    loop {
        for (i, &handle) in handles.iter().enumerate() {
            let event = machine
                .state
                .kernel32
                .objects
                .get(handle)
                .unwrap()
                .get_event();
            if event.signaled.get() {
                if !event.manual_reset {
                    // TODO: this should wake up exactly one waiting thread
                    event.signaled.set(false);
                }
                return WAIT_OBJECT_0 + i as u32;
            }
        }
        let Some(until) = until else {
            return WAIT_TIMEOUT; // no waiting at all
        };

        if let Some(until) = until {
            if machine.host.ticks() >= until {
                return WAIT_TIMEOUT;
            }
        }
        machine.emu.x86.cpu_mut().block(until).await;
    }
}

#[win32_derive::dllexport]
pub async fn WaitForSingleObject(
    machine: &mut Machine,
    handle: HANDLE<()>,
    dwMilliseconds: u32,
) -> u32 {
    wait_for_objects(machine, &[handle], false, dwMilliseconds).await
}

#[win32_derive::dllexport]
pub async fn WaitForMultipleObjects(
    machine: &mut Machine,
    nCount: u32,
    lpHandles: u32,
    bWaitAll: bool,
    dwMilliseconds: u32,
) -> u32 /* WAIT_EVENT */ {
    let handles = machine
        .mem()
        .iter_pod::<HANDLE<()>>(lpHandles, nCount)
        .collect::<Vec<_>>();
    wait_for_objects(machine, &handles, false, dwMilliseconds).await
}
