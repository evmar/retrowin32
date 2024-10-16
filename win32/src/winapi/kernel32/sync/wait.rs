//! WaitFor* functions that can block on various types of kernel objects.

use crate::{
    winapi::{kernel32::KernelObject, types::HANDLE},
    Machine,
};
use memory::Extensions;

use super::HEVENT;

pub const WAIT_OBJECT_0: u32 = 0;
//const WAIT_ABANDONED_0: u32 = 0x80;

#[win32_derive::dllexport]
pub fn WaitForSingleObject(machine: &mut Machine, handle: HEVENT, dwMilliseconds: u32) -> u32 {
    let event = machine.state.kernel32.objects.get_event(handle).unwrap();
    if event.signaled.get() {
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
        match machine.state.kernel32.objects.get(handle).as_ref().unwrap() {
            KernelObject::Event(event) => {
                if event.signaled.get() {
                    if event.manual_reset {
                        event.signaled.set(false);
                    }
                    return WAIT_OBJECT_0 + i as u32;
                }
            }
            KernelObject::Thread(thread) => todo!(),
        }
    }
    todo!()
}
