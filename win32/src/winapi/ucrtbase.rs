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
    let slice = machine.mem().sub32(start, end - start).to_vec();
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
    let slice = machine.mem().sub32(start, end - start).to_vec();
    for addr in slice.into_iter_pod::<u32>() {
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
pub fn __p__fmode(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __p__commode(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _set_app_type(_machine: &mut Machine, _app_type: u32) -> u32 {
    0
}

// Not a typo!  Both __set_app_type and _set_app_type are defined and even have different documentation.
#[win32_derive::dllexport(cdecl)]
pub fn __set_app_type(_machine: &mut Machine, _app_type: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _set_fmode(_machine: &mut Machine, _mode: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _crt_atexit(_machine: &mut Machine, _function: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _configure_narrow_argv(_machine: &mut Machine, _mode: u32) -> u32 {
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
pub fn _controlfp_s(
    _machine: &mut Machine,
    _currentControl: u32,
    _newControl: u32,
    _mask: u32,
) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _configthreadlocale(_machine: &mut Machine, per_thread_locale_type: i32) -> i32 {
    const _ENABLE_PER_THREAD_LOCALE: i32 = 1;
    const _DISABLE_PER_THREAD_LOCALE: i32 = 2;

    match per_thread_locale_type {
        0 => _DISABLE_PER_THREAD_LOCALE, // retrieve current setting
        _ENABLE_PER_THREAD_LOCALE | _DISABLE_PER_THREAD_LOCALE => {
            // ignore setting
            _DISABLE_PER_THREAD_LOCALE // return previous setting
        }
        _ => -1,
    }
}

#[win32_derive::dllexport(cdecl)]
pub fn _initialize_narrow_environment(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _set_new_mode(_machine: &mut Machine, newhandlermode: u32) -> u32 {
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
