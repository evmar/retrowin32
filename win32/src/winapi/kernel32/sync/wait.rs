//! WaitFor* functions that can block on various types of kernel objects.

use super::EventObject;
use crate::{
    Machine,
    winapi::{HANDLE, kernel32::KernelObject},
};
use memory::Extensions;

pub const WAIT_OBJECT_0: u32 = 0;
//const WAIT_ABANDONED_0: u32 = 0x80;
const WAIT_TIMEOUT: u32 = 0x102;
const INFINITE: u32 = 0xffff_ffff;

pub enum Wait {
    None,
    Forever,
    Millis(u32),
}

impl Wait {
    pub fn from_millis(ms: u32) -> Self {
        if ms == 0 {
            Wait::None
        } else if ms == INFINITE {
            Wait::Forever
        } else {
            Wait::Millis(ms)
        }
    }

    pub fn to_absolute(self, machine: &mut Machine) -> Self {
        match self {
            Wait::None => Wait::None,
            Wait::Forever => Wait::Forever,
            Wait::Millis(ms) => {
                assert!(ms > 0);
                Wait::Millis(machine.host.ticks() + ms)
            }
        }
    }
}

impl KernelObject {
    pub fn get_event(&self) -> &EventObject {
        match self {
            KernelObject::Event(event) => event,
            KernelObject::Thread(thread) => &thread.terminated,
        }
    }
}

/// The primitive beneath WaitForMultipleObjects etc.
pub async fn wait_for_objects(
    machine: &mut Machine,
    handles: &[HANDLE<()>],
    bWaitAll: bool,
    wait: Wait,
) -> u32 {
    if bWaitAll {
        todo!("WaitForMultipleObjects: bWaitAll");
    }

    let wait = wait.to_absolute(machine);
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

        let until = match wait {
            Wait::None => return WAIT_TIMEOUT,
            Wait::Millis(until) => {
                if machine.host.ticks() >= until {
                    return WAIT_TIMEOUT;
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

        #[cfg(feature = "x86-emu")]
        {
            machine.emu.x86.cpu_mut().block(until).await;
        }
        #[cfg(not(feature = "x86-emu"))]
        todo!();
    }
}

#[win32_derive::dllexport]
pub async fn WaitForSingleObject(
    machine: &mut Machine,
    handle: HANDLE<()>,
    dwMilliseconds: u32,
) -> u32 {
    wait_for_objects(machine, &[handle], false, Wait::from_millis(dwMilliseconds)).await
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
    wait_for_objects(machine, &handles, false, Wait::from_millis(dwMilliseconds)).await
}
