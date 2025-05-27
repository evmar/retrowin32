#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate as ucrtbase;
    use crate::*;
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
    pub unsafe fn _EH_prolog(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::memory::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("ucrtbase/memory") {
                trace::Record::new(
                    ucrtbase::memory::_EH_prolog_pos,
                    "ucrtbase/memory",
                    "_EH_prolog",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::memory::_EH_prolog(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _XcptFilter(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let xcptnum = <u32>::from_stack(mem, stack_args + 0u32);
            let pxcptinfoptrs = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_XcptFilter_pos,
                    "ucrtbase/init",
                    "_XcptFilter",
                    &[("xcptnum", &xcptnum), ("pxcptinfoptrs", &pxcptinfoptrs)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::_XcptFilter(sys, xcptnum, pxcptinfoptrs);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn __CxxFrameHandler(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::memory::*;
        unsafe {
            let mem = sys.mem().detach();
            let pExcept = <u32>::from_stack(mem, stack_args + 0u32);
            let pRN = <u32>::from_stack(mem, stack_args + 4u32);
            let pContext = <u32>::from_stack(mem, stack_args + 8u32);
            let pDC = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("ucrtbase/memory") {
                trace::Record::new(
                    ucrtbase::memory::__CxxFrameHandler_pos,
                    "ucrtbase/memory",
                    "__CxxFrameHandler",
                    &[
                        ("pExcept", &pExcept),
                        ("pRN", &pRN),
                        ("pContext", &pContext),
                        ("pDC", &pDC),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::memory::__CxxFrameHandler(sys, pExcept, pRN, pContext, pDC);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn __dllonexit(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let func = <u32>::from_stack(mem, stack_args + 0u32);
            let d = <u32>::from_stack(mem, stack_args + 4u32);
            let f = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::__dllonexit_pos,
                    "ucrtbase/init",
                    "__dllonexit",
                    &[("func", &func), ("d", &d), ("f", &f)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::__dllonexit(sys, func, d, f);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn __getmainargs(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let argc = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
            let argv = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let env = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let doWildCard = <u32>::from_stack(mem, stack_args + 12u32);
            let startInfo = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::__getmainargs_pos,
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
            let result = ucrtbase::init::__getmainargs(sys, argc, argv, env, doWildCard, startInfo);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn __p___argc(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::__p___argc_pos,
                    "ucrtbase/init",
                    "__p___argc",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::__p___argc(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn __p___argv(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::__p___argv_pos,
                    "ucrtbase/init",
                    "__p___argv",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::__p___argv(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn __p__environ(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::__p__environ_pos,
                    "ucrtbase/init",
                    "__p__environ",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::__p__environ(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn __set_app_type(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let _app_type = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::__set_app_type_pos,
                    "ucrtbase/init",
                    "__set_app_type",
                    &[("app_type", &_app_type)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::__set_app_type(sys, _app_type);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn __setusermatherr(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::math::*;
        unsafe {
            let mem = sys.mem().detach();
            let pf = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/math") {
                trace::Record::new(
                    ucrtbase::math::__setusermatherr_pos,
                    "ucrtbase/math",
                    "__setusermatherr",
                    &[("pf", &pf)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::math::__setusermatherr(sys, pf);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _cexit(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("ucrtbase/misc") {
                trace::Record::new(ucrtbase::misc::_cexit_pos, "ucrtbase/misc", "_cexit", &[])
                    .enter()
            } else {
                None
            };
            let result = ucrtbase::misc::_cexit(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _configthreadlocale(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let per_thread_locale_type = <i32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_configthreadlocale_pos,
                    "ucrtbase/init",
                    "_configthreadlocale",
                    &[("per_thread_locale_type", &per_thread_locale_type)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::_configthreadlocale(sys, per_thread_locale_type);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _configure_narrow_argv(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let _mode = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_configure_narrow_argv_pos,
                    "ucrtbase/init",
                    "_configure_narrow_argv",
                    &[("mode", &_mode)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::_configure_narrow_argv(sys, _mode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _controlfp(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let _new = <u32>::from_stack(mem, stack_args + 0u32);
            let _mask = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_controlfp_pos,
                    "ucrtbase/init",
                    "_controlfp",
                    &[("new", &_new), ("mask", &_mask)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::_controlfp(sys, _new, _mask);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _controlfp_s(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let _currentControl = <u32>::from_stack(mem, stack_args + 0u32);
            let _newControl = <u32>::from_stack(mem, stack_args + 4u32);
            let _mask = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_controlfp_s_pos,
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
            let result = ucrtbase::init::_controlfp_s(sys, _currentControl, _newControl, _mask);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _crt_atexit(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let _function = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_crt_atexit_pos,
                    "ucrtbase/init",
                    "_crt_atexit",
                    &[("function", &_function)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::_crt_atexit(sys, _function);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _except_handler3(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let exception_record = <u32>::from_stack(mem, stack_args + 0u32);
            let registration = <u32>::from_stack(mem, stack_args + 4u32);
            let context = <u32>::from_stack(mem, stack_args + 8u32);
            let dispatcher = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_except_handler3_pos,
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
            let result = ucrtbase::init::_except_handler3(
                sys,
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
    }
    pub unsafe fn _exit(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let status = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/misc") {
                trace::Record::new(
                    ucrtbase::misc::_exit_pos,
                    "ucrtbase/misc",
                    "_exit",
                    &[("status", &status)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::misc::_exit(sys, status);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _get_initial_narrow_environment(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_get_initial_narrow_environment_pos,
                    "ucrtbase/init",
                    "_get_initial_narrow_environment",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::_get_initial_narrow_environment(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _initialize_narrow_environment(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_initialize_narrow_environment_pos,
                    "ucrtbase/init",
                    "_initialize_narrow_environment",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::_initialize_narrow_environment(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _initterm(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let start = <u32>::from_stack(mem, stack_args + 0u32);
            let end = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_initterm_pos,
                    "ucrtbase/init",
                    "_initterm",
                    &[("start", &start), ("end", &end)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = ucrtbase::init::_initterm(sys, start, end).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn _initterm_e(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let start = <u32>::from_stack(mem, stack_args + 0u32);
            let end = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_initterm_e_pos,
                    "ucrtbase/init",
                    "_initterm_e",
                    &[("start", &start), ("end", &end)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = ucrtbase::init::_initterm_e(sys, start, end).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn _lock(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let locknum = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_lock_pos,
                    "ucrtbase/init",
                    "_lock",
                    &[("locknum", &locknum)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::_lock(sys, locknum);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _onexit(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let func = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/misc") {
                trace::Record::new(
                    ucrtbase::misc::_onexit_pos,
                    "ucrtbase/misc",
                    "_onexit",
                    &[("func", &func)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::misc::_onexit(sys, func);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _set_app_type(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let _app_type = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_set_app_type_pos,
                    "ucrtbase/init",
                    "_set_app_type",
                    &[("app_type", &_app_type)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::_set_app_type(sys, _app_type);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _set_new_mode(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let newhandlermode = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_set_new_mode_pos,
                    "ucrtbase/init",
                    "_set_new_mode",
                    &[("newhandlermode", &newhandlermode)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::_set_new_mode(sys, newhandlermode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _setmode(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let fd = <u32>::from_stack(mem, stack_args + 0u32);
            let mode = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_setmode_pos,
                    "ucrtbase/init",
                    "_setmode",
                    &[("fd", &fd), ("mode", &mode)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::_setmode(sys, fd, mode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _time64(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::time::*;
        unsafe {
            let mem = sys.mem().detach();
            let destTime = <Option<&mut u64>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/time") {
                trace::Record::new(
                    ucrtbase::time::_time64_pos,
                    "ucrtbase/time",
                    "_time64",
                    &[("destTime", &destTime)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::time::_time64(sys, destTime);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _unlock(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::init::*;
        unsafe {
            let mem = sys.mem().detach();
            let locknum = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/init") {
                trace::Record::new(
                    ucrtbase::init::_unlock_pos,
                    "ucrtbase/init",
                    "_unlock",
                    &[("locknum", &locknum)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::init::_unlock(sys, locknum);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn abort(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("ucrtbase/misc") {
                trace::Record::new(ucrtbase::misc::abort_pos, "ucrtbase/misc", "abort", &[]).enter()
            } else {
                None
            };
            let result = ucrtbase::misc::abort(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn atexit(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let func = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/misc") {
                trace::Record::new(
                    ucrtbase::misc::atexit_pos,
                    "ucrtbase/misc",
                    "atexit",
                    &[("func", &func)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::misc::atexit(sys, func);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn calloc(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::memory::*;
        unsafe {
            let mem = sys.mem().detach();
            let count = <u32>::from_stack(mem, stack_args + 0u32);
            let size = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ucrtbase/memory") {
                trace::Record::new(
                    ucrtbase::memory::calloc_pos,
                    "ucrtbase/memory",
                    "calloc",
                    &[("count", &count), ("size", &size)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::memory::calloc(sys, count, size);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn cos(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::math::*;
        unsafe {
            let mem = sys.mem().detach();
            let x = <f64>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/math") {
                trace::Record::new(
                    ucrtbase::math::cos_pos,
                    "ucrtbase/math",
                    "cos",
                    &[("x", &x)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::math::cos(sys, x);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn exit(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let status = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/misc") {
                trace::Record::new(
                    ucrtbase::misc::exit_pos,
                    "ucrtbase/misc",
                    "exit",
                    &[("status", &status)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::misc::exit(sys, status);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn floor(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::math::*;
        unsafe {
            let mem = sys.mem().detach();
            let x = <f64>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/math") {
                trace::Record::new(
                    ucrtbase::math::floor_pos,
                    "ucrtbase/math",
                    "floor",
                    &[("x", &x)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::math::floor(sys, x);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn free(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::memory::*;
        unsafe {
            let mem = sys.mem().detach();
            let ptr = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/memory") {
                trace::Record::new(
                    ucrtbase::memory::free_pos,
                    "ucrtbase/memory",
                    "free",
                    &[("ptr", &ptr)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::memory::free(sys, ptr);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn fwrite(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let filename = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let mode = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ucrtbase/misc") {
                trace::Record::new(
                    ucrtbase::misc::fwrite_pos,
                    "ucrtbase/misc",
                    "fwrite",
                    &[("filename", &filename), ("mode", &mode)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::misc::fwrite(sys, filename, mode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn malloc(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::memory::*;
        unsafe {
            let mem = sys.mem().detach();
            let size = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/memory") {
                trace::Record::new(
                    ucrtbase::memory::malloc_pos,
                    "ucrtbase/memory",
                    "malloc",
                    &[("size", &size)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::memory::malloc(sys, size);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn memcpy(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::memory::*;
        unsafe {
            let mem = sys.mem().detach();
            let dest = <u32>::from_stack(mem, stack_args + 0u32);
            let src = <u32>::from_stack(mem, stack_args + 4u32);
            let count = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ucrtbase/memory") {
                trace::Record::new(
                    ucrtbase::memory::memcpy_pos,
                    "ucrtbase/memory",
                    "memcpy",
                    &[("dest", &dest), ("src", &src), ("count", &count)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::memory::memcpy(sys, dest, src, count);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn memset(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::memory::*;
        unsafe {
            let mem = sys.mem().detach();
            let dst = <u32>::from_stack(mem, stack_args + 0u32);
            let val = <u32>::from_stack(mem, stack_args + 4u32);
            let len = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ucrtbase/memory") {
                trace::Record::new(
                    ucrtbase::memory::memset_pos,
                    "ucrtbase/memory",
                    "memset",
                    &[("dst", &dst), ("val", &val), ("len", &len)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::memory::memset(sys, dst, val, len);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn operator_delete(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::memory::*;
        unsafe {
            let mem = sys.mem().detach();
            let size = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/memory") {
                trace::Record::new(
                    ucrtbase::memory::operator_delete_pos,
                    "ucrtbase/memory",
                    "operator_delete",
                    &[("size", &size)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::memory::operator_delete(sys, size);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn operator_new(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::memory::*;
        unsafe {
            let mem = sys.mem().detach();
            let size = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/memory") {
                trace::Record::new(
                    ucrtbase::memory::operator_new_pos,
                    "ucrtbase/memory",
                    "operator_new",
                    &[("size", &size)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::memory::operator_new(sys, size);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn printf(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let fmt = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let args = <VarArgs>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ucrtbase/misc") {
                trace::Record::new(
                    ucrtbase::misc::printf_pos,
                    "ucrtbase/misc",
                    "printf",
                    &[("fmt", &fmt), ("args", &args)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::misc::printf(sys, fmt, args);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn rand(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::rand::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("ucrtbase/rand") {
                trace::Record::new(ucrtbase::rand::rand_pos, "ucrtbase/rand", "rand", &[]).enter()
            } else {
                None
            };
            let result = ucrtbase::rand::rand(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn signal(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let sig = <u32>::from_stack(mem, stack_args + 0u32);
            let func = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ucrtbase/misc") {
                trace::Record::new(
                    ucrtbase::misc::signal_pos,
                    "ucrtbase/misc",
                    "signal",
                    &[("sig", &sig), ("func", &func)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::misc::signal(sys, sig, func);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn sin(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::math::*;
        unsafe {
            let mem = sys.mem().detach();
            let x = <f64>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/math") {
                trace::Record::new(
                    ucrtbase::math::sin_pos,
                    "ucrtbase/math",
                    "sin",
                    &[("x", &x)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::math::sin(sys, x);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn sprintf(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let buf = <u32>::from_stack(mem, stack_args + 0u32);
            let fmt = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let args = <VarArgs>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ucrtbase/misc") {
                trace::Record::new(
                    ucrtbase::misc::sprintf_pos,
                    "ucrtbase/misc",
                    "sprintf",
                    &[("buf", &buf), ("fmt", &fmt), ("args", &args)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::misc::sprintf(sys, buf, fmt, args);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn sqrt(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::math::*;
        unsafe {
            let mem = sys.mem().detach();
            let x = <f64>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/math") {
                trace::Record::new(
                    ucrtbase::math::sqrt_pos,
                    "ucrtbase/math",
                    "sqrt",
                    &[("x", &x)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::math::sqrt(sys, x);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn srand(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::rand::*;
        unsafe {
            let mem = sys.mem().detach();
            let seed = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/rand") {
                trace::Record::new(
                    ucrtbase::rand::srand_pos,
                    "ucrtbase/rand",
                    "srand",
                    &[("seed", &seed)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::rand::srand(sys, seed);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn strlen(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::memory::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpString = <Option<&CStr>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/memory") {
                trace::Record::new(
                    ucrtbase::memory::strlen_pos,
                    "ucrtbase/memory",
                    "strlen",
                    &[("lpString", &lpString)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::memory::strlen(sys, lpString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn terminate(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("ucrtbase/misc") {
                trace::Record::new(
                    ucrtbase::misc::terminate_pos,
                    "ucrtbase/misc",
                    "terminate",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::misc::terminate(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn time(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::time::*;
        unsafe {
            let mem = sys.mem().detach();
            let destTime = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ucrtbase/time") {
                trace::Record::new(
                    ucrtbase::time::time_pos,
                    "ucrtbase/time",
                    "time",
                    &[("destTime", &destTime)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::time::time(sys, destTime);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn vfprintf(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ucrtbase::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let buf = <u32>::from_stack(mem, stack_args + 0u32);
            let fmt = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let args = <VarArgs>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ucrtbase/misc") {
                trace::Record::new(
                    ucrtbase::misc::vfprintf_pos,
                    "ucrtbase/misc",
                    "vfprintf",
                    &[("buf", &buf), ("fmt", &fmt), ("args", &args)],
                )
                .enter()
            } else {
                None
            };
            let result = ucrtbase::misc::vfprintf(sys, buf, fmt, args);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
}
const SHIMS: [Shim; 53usize] = [
    Shim {
        name: "_EH_prolog",
        func: Handler::Sync(wrappers::_EH_prolog),
    },
    Shim {
        name: "_XcptFilter",
        func: Handler::Sync(wrappers::_XcptFilter),
    },
    Shim {
        name: "__CxxFrameHandler",
        func: Handler::Sync(wrappers::__CxxFrameHandler),
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
        name: "__p__environ",
        func: Handler::Sync(wrappers::__p__environ),
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
        name: "_cexit",
        func: Handler::Sync(wrappers::_cexit),
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
        name: "_onexit",
        func: Handler::Sync(wrappers::_onexit),
    },
    Shim {
        name: "_set_app_type",
        func: Handler::Sync(wrappers::_set_app_type),
    },
    Shim {
        name: "_set_new_mode",
        func: Handler::Sync(wrappers::_set_new_mode),
    },
    Shim {
        name: "_setmode",
        func: Handler::Sync(wrappers::_setmode),
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
        name: "abort",
        func: Handler::Sync(wrappers::abort),
    },
    Shim {
        name: "atexit",
        func: Handler::Sync(wrappers::atexit),
    },
    Shim {
        name: "calloc",
        func: Handler::Sync(wrappers::calloc),
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
        name: "floor",
        func: Handler::Sync(wrappers::floor),
    },
    Shim {
        name: "free",
        func: Handler::Sync(wrappers::free),
    },
    Shim {
        name: "fwrite",
        func: Handler::Sync(wrappers::fwrite),
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
        name: "memset",
        func: Handler::Sync(wrappers::memset),
    },
    Shim {
        name: "operator_delete",
        func: Handler::Sync(wrappers::operator_delete),
    },
    Shim {
        name: "operator_new",
        func: Handler::Sync(wrappers::operator_new),
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
        name: "signal",
        func: Handler::Sync(wrappers::signal),
    },
    Shim {
        name: "sin",
        func: Handler::Sync(wrappers::sin),
    },
    Shim {
        name: "sprintf",
        func: Handler::Sync(wrappers::sprintf),
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
        name: "terminate",
        func: Handler::Sync(wrappers::terminate),
    },
    Shim {
        name: "time",
        func: Handler::Sync(wrappers::time),
    },
    Shim {
        name: "vfprintf",
        func: Handler::Sync(wrappers::vfprintf),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "ucrtbase.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../ucrtbase.dll"),
};
