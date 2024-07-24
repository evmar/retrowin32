#![allow(non_snake_case)]

use super::kernel32::ExitProcess;
use crate::Machine;
use memory::Extensions;

const TRACE_CONTEXT: &'static str = "ucrtbase";

#[win32_derive::dllexport(cdecl)]
pub async fn _initterm(machine: &mut Machine, start: u32, end: u32) -> u32 {
    if (end - start) % 4 != 0 {
        panic!("unaligned _initterm");
    }
    // Safety: iterating memory while calling x86 can potentially invalidate the memory.
    let slice = unsafe { machine.mem().sub(start, end - start).detach() };
    for addr in slice.into_iter_pod::<u32>() {
        if addr != 0 {
            machine.call_x86(addr, vec![]).await;
        }
    }
    0
}

#[win32_derive::dllexport(cdecl)]
pub async fn _initterm_e(machine: &mut Machine, start: u32, end: u32) -> u32 {
    if (end - start) % 4 != 0 {
        panic!("unaligned _initterm_e");
    }
    // Safety: iterating memory while calling x86 can potentially invalidate the memory.
    let slice = unsafe { machine.mem().sub(start, end - start).detach() };
    for addr in slice.into_iter_pod() {
        if addr != 0 {
            let err = machine.call_x86(addr, vec![]).await;
            if err != 0 {
                return err;
            }
        }
    }
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _get_initial_narrow_environment(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __p___argv(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __p___argc(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn exit(machine: &mut Machine, status: u32) -> u32 {
    ExitProcess(machine, status)
}

#[win32_derive::dllexport(cdecl)]
pub fn _lock(_machine: &mut Machine, locknum: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _unlock(_machine: &mut Machine, locknum: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __dllonexit(_machine: &mut Machine, func: u32, d: u32, f: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn malloc(machine: &mut Machine, size: u32) -> u32 {
    let heap = machine
        .state
        .kernel32
        .get_process_heap(&mut machine.emu.memory); // lazy init process_heap
    heap.alloc(machine.emu.memory.mem(), size)
}

#[win32_derive::dllexport(cdecl)]
pub fn free(machine: &mut Machine, ptr: u32) -> u32 {
    let heap = machine
        .state
        .kernel32
        .get_process_heap(&mut machine.emu.memory); // lazy init process_heap
    heap.free(machine.emu.memory.mem(), ptr);
    0
}
