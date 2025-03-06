#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::{
    shims::{Handler, Shim},
    winapi::builtin::BuiltinDLL,
};
mod wrappers {
    use crate::{
        calling_convention::*,
        machine::Machine,
        winapi::{self, *},
    };
    use ::memory::Extensions;
    use winapi::ucrtbase::*;
    pub unsafe fn _XcptFilter(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let xcptnum = <u32>::from_stack(mem, stack_args + 0u32);
        let pxcptinfoptrs = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_XcptFilter_pos,
                "ucrtbase/init",
                "_XcptFilter",
                &[("xcptnum", &xcptnum), ("pxcptinfoptrs", &pxcptinfoptrs)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_XcptFilter(machine, xcptnum, pxcptinfoptrs);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn __dllonexit(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let func = <u32>::from_stack(mem, stack_args + 0u32);
        let d = <u32>::from_stack(mem, stack_args + 4u32);
        let f = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::__dllonexit_pos,
                "ucrtbase/init",
                "__dllonexit",
                &[("func", &func), ("d", &d), ("f", &f)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::__dllonexit(machine, func, d, f);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn __getmainargs(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let argc = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
        let argv = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let env = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
        let doWildCard = <u32>::from_stack(mem, stack_args + 12u32);
        let startInfo = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::__getmainargs_pos,
                "ucrtbase/init",
                "__getmainargs",
                &[
                    ("argc", &argc),
                    ("argv", &argv),
                    ("env", &env),
                    ("doWildCard", &doWildCard),
                    ("startInfo", &startInfo),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::ucrtbase::__getmainargs(machine, argc, argv, env, doWildCard, startInfo);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn __p___argc(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::__p___argc_pos,
                "ucrtbase/init",
                "__p___argc",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::__p___argc(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn __p___argv(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::__p___argv_pos,
                "ucrtbase/init",
                "__p___argv",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::__p___argv(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn __p__commode(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::__p__commode_pos,
                "ucrtbase/init",
                "__p__commode",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::__p__commode(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn __p__fmode(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::__p__fmode_pos,
                "ucrtbase/init",
                "__p__fmode",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::__p__fmode(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn __set_app_type(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let _app_type = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::__set_app_type_pos,
                "ucrtbase/init",
                "__set_app_type",
                &[("app_type", &_app_type)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::__set_app_type(machine, _app_type);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn __setusermatherr(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let pf = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/math") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::__setusermatherr_pos,
                "ucrtbase/math",
                "__setusermatherr",
                &[("pf", &pf)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::__setusermatherr(machine, pf);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _configthreadlocale(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let per_thread_locale_type = <i32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_configthreadlocale_pos,
                "ucrtbase/init",
                "_configthreadlocale",
                &[("per_thread_locale_type", &per_thread_locale_type)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_configthreadlocale(machine, per_thread_locale_type);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _configure_narrow_argv(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let _mode = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_configure_narrow_argv_pos,
                "ucrtbase/init",
                "_configure_narrow_argv",
                &[("mode", &_mode)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_configure_narrow_argv(machine, _mode);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _controlfp(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let _new = <u32>::from_stack(mem, stack_args + 0u32);
        let _mask = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_controlfp_pos,
                "ucrtbase/init",
                "_controlfp",
                &[("new", &_new), ("mask", &_mask)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_controlfp(machine, _new, _mask);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _controlfp_s(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let _currentControl = <u32>::from_stack(mem, stack_args + 0u32);
        let _newControl = <u32>::from_stack(mem, stack_args + 4u32);
        let _mask = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_controlfp_s_pos,
                "ucrtbase/init",
                "_controlfp_s",
                &[
                    ("currentControl", &_currentControl),
                    ("newControl", &_newControl),
                    ("mask", &_mask),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_controlfp_s(machine, _currentControl, _newControl, _mask);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _crt_atexit(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let _function = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_crt_atexit_pos,
                "ucrtbase/init",
                "_crt_atexit",
                &[("function", &_function)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_crt_atexit(machine, _function);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _except_handler3(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let exception_record = <u32>::from_stack(mem, stack_args + 0u32);
        let registration = <u32>::from_stack(mem, stack_args + 4u32);
        let context = <u32>::from_stack(mem, stack_args + 8u32);
        let dispatcher = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_except_handler3_pos,
                "ucrtbase/init",
                "_except_handler3",
                &[
                    ("exception_record", &exception_record),
                    ("registration", &registration),
                    ("context", &context),
                    ("dispatcher", &dispatcher),
                ],
            )
            .enter()
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
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _exit(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let status = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/misc") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_exit_pos,
                "ucrtbase/misc",
                "_exit",
                &[("status", &status)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_exit(machine, status);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _ftol(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/math") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_ftol_pos,
                "ucrtbase/math",
                "_ftol",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_ftol(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _get_initial_narrow_environment(
        machine: &mut Machine,
        stack_args: u32,
    ) -> ABIReturn {
        let mem = machine.mem().detach();
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_get_initial_narrow_environment_pos,
                "ucrtbase/init",
                "_get_initial_narrow_environment",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_get_initial_narrow_environment(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _initialize_narrow_environment(
        machine: &mut Machine,
        stack_args: u32,
    ) -> ABIReturn {
        let mem = machine.mem().detach();
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_initialize_narrow_environment_pos,
                "ucrtbase/init",
                "_initialize_narrow_environment",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_initialize_narrow_environment(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _initterm(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn>>> {
        let mem = machine.mem().detach();
        let start = <u32>::from_stack(mem, stack_args + 0u32);
        let end = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_initterm_pos,
                "ucrtbase/init",
                "_initterm",
                &[("start", &start), ("end", &end)],
            )
            .enter()
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::ucrtbase::_initterm(machine, start, end).await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        })
    }
    pub unsafe fn _initterm_e(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn>>> {
        let mem = machine.mem().detach();
        let start = <u32>::from_stack(mem, stack_args + 0u32);
        let end = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_initterm_e_pos,
                "ucrtbase/init",
                "_initterm_e",
                &[("start", &start), ("end", &end)],
            )
            .enter()
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::ucrtbase::_initterm_e(machine, start, end).await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        })
    }
    pub unsafe fn _lock(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let locknum = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_lock_pos,
                "ucrtbase/init",
                "_lock",
                &[("locknum", &locknum)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_lock(machine, locknum);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _set_app_type(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let _app_type = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_set_app_type_pos,
                "ucrtbase/init",
                "_set_app_type",
                &[("app_type", &_app_type)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_set_app_type(machine, _app_type);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _set_fmode(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let _mode = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_set_fmode_pos,
                "ucrtbase/init",
                "_set_fmode",
                &[("mode", &_mode)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_set_fmode(machine, _mode);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _set_new_mode(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let newhandlermode = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_set_new_mode_pos,
                "ucrtbase/init",
                "_set_new_mode",
                &[("newhandlermode", &newhandlermode)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_set_new_mode(machine, newhandlermode);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _time64(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let destTime = <Option<&mut u64>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/time") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_time64_pos,
                "ucrtbase/time",
                "_time64",
                &[("destTime", &destTime)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_time64(machine, destTime);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn _unlock(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let locknum = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/init") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::_unlock_pos,
                "ucrtbase/init",
                "_unlock",
                &[("locknum", &locknum)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::_unlock(machine, locknum);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn cos(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let x = <f64>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/math") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::cos_pos,
                "ucrtbase/math",
                "cos",
                &[("x", &x)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::cos(machine, x);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn exit(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let status = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/misc") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::exit_pos,
                "ucrtbase/misc",
                "exit",
                &[("status", &status)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::exit(machine, status);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn free(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let ptr = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/memory") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::free_pos,
                "ucrtbase/memory",
                "free",
                &[("ptr", &ptr)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::free(machine, ptr);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn malloc(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let size = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/memory") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::malloc_pos,
                "ucrtbase/memory",
                "malloc",
                &[("size", &size)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::malloc(machine, size);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn memcpy(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let dest = <u32>::from_stack(mem, stack_args + 0u32);
        let src = <u32>::from_stack(mem, stack_args + 4u32);
        let count = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/misc") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::memcpy_pos,
                "ucrtbase/misc",
                "memcpy",
                &[("dest", &dest), ("src", &src), ("count", &count)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::memcpy(machine, dest, src, count);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn printf(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let fmt = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let args = <VarArgs>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/misc") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::printf_pos,
                "ucrtbase/misc",
                "printf",
                &[("fmt", &fmt), ("args", &args)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::printf(machine, fmt, args);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn rand(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/rand") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::rand_pos,
                "ucrtbase/rand",
                "rand",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::rand(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn sin(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let x = <f64>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/math") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::sin_pos,
                "ucrtbase/math",
                "sin",
                &[("x", &x)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::sin(machine, x);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn sqrt(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let x = <f64>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/math") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::sqrt_pos,
                "ucrtbase/math",
                "sqrt",
                &[("x", &x)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::sqrt(machine, x);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn srand(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let seed = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/rand") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::srand_pos,
                "ucrtbase/rand",
                "srand",
                &[("seed", &seed)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::srand(machine, seed);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn strlen(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let lpString = <Option<&CStr>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/misc") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::strlen_pos,
                "ucrtbase/misc",
                "strlen",
                &[("lpString", &lpString)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::strlen(machine, lpString);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn time(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let destTime = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ucrtbase/time") {
            crate::winapi::trace::Record::new(
                winapi::ucrtbase::time_pos,
                "ucrtbase/time",
                "time",
                &[("destTime", &destTime)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ucrtbase::time(machine, destTime);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
}
const SHIMS: [Shim; 39usize] = [
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
        name: "_ftol",
        func: Handler::Sync(wrappers::_ftol),
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
        name: "cos",
        func: Handler::Sync(wrappers::cos),
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
        name: "memcpy",
        func: Handler::Sync(wrappers::memcpy),
    },
    Shim {
        name: "printf",
        func: Handler::Sync(wrappers::printf),
    },
    Shim {
        name: "rand",
        func: Handler::Sync(wrappers::rand),
    },
    Shim {
        name: "sin",
        func: Handler::Sync(wrappers::sin),
    },
    Shim {
        name: "sqrt",
        func: Handler::Sync(wrappers::sqrt),
    },
    Shim {
        name: "srand",
        func: Handler::Sync(wrappers::srand),
    },
    Shim {
        name: "strlen",
        func: Handler::Sync(wrappers::strlen),
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
