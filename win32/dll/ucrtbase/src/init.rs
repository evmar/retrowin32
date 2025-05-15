use memory::Extensions;
use win32_system::System;

#[win32_derive::dllexport(cdecl)]
pub async fn _initterm(sys: &mut dyn System, start: u32, end: u32) -> u32 {
    if (end - start) % 4 != 0 {
        panic!("unaligned _initterm");
    }
    let slice = sys.mem().sub32(start, end - start).to_vec();
    for addr in slice.into_iter_pod::<u32>() {
        if addr != 0 {
            sys.call_x86(addr, vec![]).await;
        }
    }
    0
}

#[win32_derive::dllexport(cdecl)]
pub async fn _initterm_e(sys: &mut dyn System, start: u32, end: u32) -> u32 {
    if (end - start) % 4 != 0 {
        panic!("unaligned _initterm_e");
    }
    let slice = sys.mem().sub32(start, end - start).to_vec();
    for addr in slice.into_iter_pod::<u32>() {
        if addr != 0 {
            let err = sys.call_x86(addr, vec![]).await;
            if err != 0 {
                return err;
            }
        }
    }
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _get_initial_narrow_environment(sys: &dyn System) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __p___argv(sys: &dyn System) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __p___argc(sys: &dyn System) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __p__environ(sys: &dyn System) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __p__fmode(sys: &dyn System) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __p__commode(sys: &dyn System) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _set_app_type(sys: &dyn System, _app_type: u32) -> u32 {
    0
}

// Not a typo!  Both __set_app_type and _set_app_type are defined and even have different documentation.
#[win32_derive::dllexport(cdecl)]
pub fn __set_app_type(sys: &dyn System, _app_type: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _set_fmode(sys: &dyn System, _mode: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _setmode(sys: &dyn System, fd: u32, mode: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _crt_atexit(sys: &dyn System, _function: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _configure_narrow_argv(sys: &dyn System, _mode: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _lock(sys: &dyn System, locknum: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _unlock(sys: &dyn System, locknum: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __dllonexit(sys: &dyn System, func: u32, d: u32, f: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _controlfp(sys: &dyn System, _new: u32, _mask: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _controlfp_s(sys: &dyn System, _currentControl: u32, _newControl: u32, _mask: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _configthreadlocale(sys: &dyn System, per_thread_locale_type: i32) -> i32 {
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
pub fn _initialize_narrow_environment(sys: &dyn System) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _set_new_mode(sys: &dyn System, newhandlermode: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __getmainargs(
    sys: &dyn System,
    argc: Option<&mut u32>,
    argv: Option<&mut u32>,
    env: Option<&mut u32>,
    doWildCard: u32,
    startInfo: u32,
) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _XcptFilter(sys: &mut dyn System, xcptnum: u32, pxcptinfoptrs: u32) -> u32 {
    todo!();
}

#[win32_derive::dllexport(cdecl)]
pub fn _except_handler3(
    sys: &dyn System,
    exception_record: u32,
    registration: u32,
    context: u32,
    dispatcher: u32,
) -> i32 {
    todo!();
}
