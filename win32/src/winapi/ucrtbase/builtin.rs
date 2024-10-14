#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::{
    shims::{Handler, Shim},
    winapi::builtin::BuiltinDLL,
};
mod wrappers {
    use crate::{
        machine::Machine,
        winapi::{self, stack_args::*, types::*},
    };
    use ::memory::Extensions;
    use winapi::ucrtbase::*;
    pub unsafe fn _XcptFilter(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let xcptnum = <u32>::from_stack(mem, stack_args + 0u32);
        let pxcptinfoptrs = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_XcptFilter",
                &[("xcptnum", &xcptnum), ("pxcptinfoptrs", &pxcptinfoptrs)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_XcptFilter(machine, xcptnum, pxcptinfoptrs);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_XcptFilter_pos.0,
                winapi::ucrtbase::_XcptFilter_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn __dllonexit(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let func = <u32>::from_stack(mem, stack_args + 0u32);
        let d = <u32>::from_stack(mem, stack_args + 4u32);
        let f = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "__dllonexit",
                &[("func", &func), ("d", &d), ("f", &f)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::__dllonexit(machine, func, d, f);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::__dllonexit_pos.0,
                winapi::ucrtbase::__dllonexit_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn __getmainargs(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let argc = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
        let argv = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let env = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
        let doWildCard = <u32>::from_stack(mem, stack_args + 12u32);
        let startInfo = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "__getmainargs",
                &[
                    ("argc", &argc),
                    ("argv", &argv),
                    ("env", &env),
                    ("doWildCard", &doWildCard),
                    ("startInfo", &startInfo),
                ],
            ))
        } else {
            None
        };
        let result =
            winapi::ucrtbase::__getmainargs(machine, argc, argv, env, doWildCard, startInfo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::__getmainargs_pos.0,
                winapi::ucrtbase::__getmainargs_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn __p___argc(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin("ucrtbase", "__p___argc", &[]))
        } else {
            None
        };
        let result = winapi::ucrtbase::__p___argc(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::__p___argc_pos.0,
                winapi::ucrtbase::__p___argc_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn __p___argv(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin("ucrtbase", "__p___argv", &[]))
        } else {
            None
        };
        let result = winapi::ucrtbase::__p___argv(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::__p___argv_pos.0,
                winapi::ucrtbase::__p___argv_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn __p__commode(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin("ucrtbase", "__p__commode", &[]))
        } else {
            None
        };
        let result = winapi::ucrtbase::__p__commode(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::__p__commode_pos.0,
                winapi::ucrtbase::__p__commode_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn __p__fmode(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin("ucrtbase", "__p__fmode", &[]))
        } else {
            None
        };
        let result = winapi::ucrtbase::__p__fmode(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::__p__fmode_pos.0,
                winapi::ucrtbase::__p__fmode_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn __set_app_type(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _app_type = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "__set_app_type",
                &[("app_type", &_app_type)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::__set_app_type(machine, _app_type);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::__set_app_type_pos.0,
                winapi::ucrtbase::__set_app_type_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn __setusermatherr(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let pf = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "__setusermatherr",
                &[("pf", &pf)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::__setusermatherr(machine, pf);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::__setusermatherr_pos.0,
                winapi::ucrtbase::__setusermatherr_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _configthreadlocale(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let per_thread_locale_type = <i32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_configthreadlocale",
                &[("per_thread_locale_type", &per_thread_locale_type)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_configthreadlocale(machine, per_thread_locale_type);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_configthreadlocale_pos.0,
                winapi::ucrtbase::_configthreadlocale_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _configure_narrow_argv(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _mode = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_configure_narrow_argv",
                &[("mode", &_mode)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_configure_narrow_argv(machine, _mode);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_configure_narrow_argv_pos.0,
                winapi::ucrtbase::_configure_narrow_argv_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _controlfp(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _new = <u32>::from_stack(mem, stack_args + 0u32);
        let _mask = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_controlfp",
                &[("new", &_new), ("mask", &_mask)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_controlfp(machine, _new, _mask);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_controlfp_pos.0,
                winapi::ucrtbase::_controlfp_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _controlfp_s(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _currentControl = <u32>::from_stack(mem, stack_args + 0u32);
        let _newControl = <u32>::from_stack(mem, stack_args + 4u32);
        let _mask = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_controlfp_s",
                &[
                    ("currentControl", &_currentControl),
                    ("newControl", &_newControl),
                    ("mask", &_mask),
                ],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_controlfp_s(machine, _currentControl, _newControl, _mask);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_controlfp_s_pos.0,
                winapi::ucrtbase::_controlfp_s_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _crt_atexit(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _function = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_crt_atexit",
                &[("function", &_function)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_crt_atexit(machine, _function);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_crt_atexit_pos.0,
                winapi::ucrtbase::_crt_atexit_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _except_handler3(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let exception_record = <u32>::from_stack(mem, stack_args + 0u32);
        let registration = <u32>::from_stack(mem, stack_args + 4u32);
        let context = <u32>::from_stack(mem, stack_args + 8u32);
        let dispatcher = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_except_handler3",
                &[
                    ("exception_record", &exception_record),
                    ("registration", &registration),
                    ("context", &context),
                    ("dispatcher", &dispatcher),
                ],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_except_handler3(
            machine,
            exception_record,
            registration,
            context,
            dispatcher,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_except_handler3_pos.0,
                winapi::ucrtbase::_except_handler3_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _exit(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let status = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_exit",
                &[("status", &status)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_exit(machine, status);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_exit_pos.0,
                winapi::ucrtbase::_exit_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _get_initial_narrow_environment(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_get_initial_narrow_environment",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_get_initial_narrow_environment(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_get_initial_narrow_environment_pos.0,
                winapi::ucrtbase::_get_initial_narrow_environment_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _initialize_narrow_environment(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_initialize_narrow_environment",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_initialize_narrow_environment(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_initialize_narrow_environment_pos.0,
                winapi::ucrtbase::_initialize_narrow_environment_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _initterm(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let start = <u32>::from_stack(mem, stack_args + 0u32);
        let end = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_initterm",
                &[("start", &start), ("end", &end)],
            ))
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::ucrtbase::_initterm(machine, start, end).await;
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ucrtbase::_initterm_pos.0,
                    winapi::ucrtbase::_initterm_pos.1,
                    &result,
                );
            }
            result.to_raw()
        })
    }
    pub unsafe fn _initterm_e(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let start = <u32>::from_stack(mem, stack_args + 0u32);
        let end = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_initterm_e",
                &[("start", &start), ("end", &end)],
            ))
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::ucrtbase::_initterm_e(machine, start, end).await;
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ucrtbase::_initterm_e_pos.0,
                    winapi::ucrtbase::_initterm_e_pos.1,
                    &result,
                );
            }
            result.to_raw()
        })
    }
    pub unsafe fn _lock(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let locknum = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_lock",
                &[("locknum", &locknum)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_lock(machine, locknum);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_lock_pos.0,
                winapi::ucrtbase::_lock_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _set_app_type(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _app_type = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_set_app_type",
                &[("app_type", &_app_type)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_set_app_type(machine, _app_type);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_set_app_type_pos.0,
                winapi::ucrtbase::_set_app_type_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _set_fmode(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _mode = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_set_fmode",
                &[("mode", &_mode)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_set_fmode(machine, _mode);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_set_fmode_pos.0,
                winapi::ucrtbase::_set_fmode_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _set_new_mode(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let newhandlermode = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_set_new_mode",
                &[("newhandlermode", &newhandlermode)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_set_new_mode(machine, newhandlermode);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_set_new_mode_pos.0,
                winapi::ucrtbase::_set_new_mode_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _time64(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let destTime = <Option<&mut u64>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_time64",
                &[("destTime", &destTime)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_time64(machine, destTime);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_time64_pos.0,
                winapi::ucrtbase::_time64_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _unlock(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let locknum = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "_unlock",
                &[("locknum", &locknum)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::_unlock(machine, locknum);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::_unlock_pos.0,
                winapi::ucrtbase::_unlock_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn exit(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let status = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "exit",
                &[("status", &status)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::exit(machine, status);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::exit_pos.0,
                winapi::ucrtbase::exit_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn free(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let ptr = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "free",
                &[("ptr", &ptr)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::free(machine, ptr);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::free_pos.0,
                winapi::ucrtbase::free_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn malloc(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let size = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "malloc",
                &[("size", &size)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::malloc(machine, size);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::malloc_pos.0,
                winapi::ucrtbase::malloc_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn rand(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin("ucrtbase", "rand", &[]))
        } else {
            None
        };
        let result = winapi::ucrtbase::rand(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::rand_pos.0,
                winapi::ucrtbase::rand_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn srand(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let seed = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "srand",
                &[("seed", &seed)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::srand(machine, seed);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::srand_pos.0,
                winapi::ucrtbase::srand_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn time(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let destTime = <Option<&mut u64>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ucrtbase") {
            Some(crate::trace::trace_begin(
                "ucrtbase",
                "time",
                &[("destTime", &destTime)],
            ))
        } else {
            None
        };
        let result = winapi::ucrtbase::time(machine, destTime);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ucrtbase::time_pos.0,
                winapi::ucrtbase::time_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
}
const SHIMS: [Shim; 32usize] = [
    Shim {
        name: "_XcptFilter",
        func: Handler::Sync(wrappers::_XcptFilter),
    },
    Shim {
        name: "__dllonexit",
        func: Handler::Sync(wrappers::__dllonexit),
    },
    Shim {
        name: "__getmainargs",
        func: Handler::Sync(wrappers::__getmainargs),
    },
    Shim {
        name: "__p___argc",
        func: Handler::Sync(wrappers::__p___argc),
    },
    Shim {
        name: "__p___argv",
        func: Handler::Sync(wrappers::__p___argv),
    },
    Shim {
        name: "__p__commode",
        func: Handler::Sync(wrappers::__p__commode),
    },
    Shim {
        name: "__p__fmode",
        func: Handler::Sync(wrappers::__p__fmode),
    },
    Shim {
        name: "__set_app_type",
        func: Handler::Sync(wrappers::__set_app_type),
    },
    Shim {
        name: "__setusermatherr",
        func: Handler::Sync(wrappers::__setusermatherr),
    },
    Shim {
        name: "_configthreadlocale",
        func: Handler::Sync(wrappers::_configthreadlocale),
    },
    Shim {
        name: "_configure_narrow_argv",
        func: Handler::Sync(wrappers::_configure_narrow_argv),
    },
    Shim {
        name: "_controlfp",
        func: Handler::Sync(wrappers::_controlfp),
    },
    Shim {
        name: "_controlfp_s",
        func: Handler::Sync(wrappers::_controlfp_s),
    },
    Shim {
        name: "_crt_atexit",
        func: Handler::Sync(wrappers::_crt_atexit),
    },
    Shim {
        name: "_except_handler3",
        func: Handler::Sync(wrappers::_except_handler3),
    },
    Shim {
        name: "_exit",
        func: Handler::Sync(wrappers::_exit),
    },
    Shim {
        name: "_get_initial_narrow_environment",
        func: Handler::Sync(wrappers::_get_initial_narrow_environment),
    },
    Shim {
        name: "_initialize_narrow_environment",
        func: Handler::Sync(wrappers::_initialize_narrow_environment),
    },
    Shim {
        name: "_initterm",
        func: Handler::Async(wrappers::_initterm),
    },
    Shim {
        name: "_initterm_e",
        func: Handler::Async(wrappers::_initterm_e),
    },
    Shim {
        name: "_lock",
        func: Handler::Sync(wrappers::_lock),
    },
    Shim {
        name: "_set_app_type",
        func: Handler::Sync(wrappers::_set_app_type),
    },
    Shim {
        name: "_set_fmode",
        func: Handler::Sync(wrappers::_set_fmode),
    },
    Shim {
        name: "_set_new_mode",
        func: Handler::Sync(wrappers::_set_new_mode),
    },
    Shim {
        name: "_time64",
        func: Handler::Sync(wrappers::_time64),
    },
    Shim {
        name: "_unlock",
        func: Handler::Sync(wrappers::_unlock),
    },
    Shim {
        name: "exit",
        func: Handler::Sync(wrappers::exit),
    },
    Shim {
        name: "free",
        func: Handler::Sync(wrappers::free),
    },
    Shim {
        name: "malloc",
        func: Handler::Sync(wrappers::malloc),
    },
    Shim {
        name: "rand",
        func: Handler::Sync(wrappers::rand),
    },
    Shim {
        name: "srand",
        func: Handler::Sync(wrappers::srand),
    },
    Shim {
        name: "time",
        func: Handler::Sync(wrappers::time),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "ucrtbase.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/ucrtbase.dll"),
};
