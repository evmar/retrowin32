//! WaitFor* functions that can block on various types of kernel objects.

use super::EventObject;
use crate::{
    Machine,
    winapi::{HANDLE, kernel32::KernelObject},
};
use memory::Extensions;
use win32_system::{Wait, WaitResult};

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
    objects: Box<[KernelObject]>,
    wait_all: bool,
    wait: Wait,
) -> WaitResult {
    if wait_all {
        todo!("WaitForMultipleObjects: bWaitAll");
    }

    let wait = wait.to_absolute(&*machine.host);
    loop {
        for (i, object) in objects.iter().enumerate() {
            let event = object.get_event();
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
                if machine.host.ticks() >= until {
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
    let object = machine.state.kernel32.objects.get(handle).unwrap();
    wait_for_objects(
        machine,
        Box::new([object.clone()]),
        false,
        Wait::from_millis(dwMilliseconds),
    )
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
    let objects = machine
        .mem()
        .iter_pod::<HANDLE<()>>(lpHandles, nCount)
        .map(|handle| machine.state.kernel32.objects.get(handle).unwrap().clone())
        .collect::<Vec<_>>();
    wait_for_objects(
        machine,
        objects.into(),
        false,
        Wait::from_millis(dwMilliseconds),
    )
    .await
    .to_code()
}
