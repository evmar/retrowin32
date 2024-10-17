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
    use winapi::vcruntime140::*;
    pub unsafe fn _CxxThrowException(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let pExceptionObject = <u32>::from_stack(mem, stack_args + 0u32);
        let pThrowInfo = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("vcruntime140") {
            Some(crate::trace::trace_begin(
                "vcruntime140",
                "_CxxThrowException",
                &[
                    ("pExceptionObject", &pExceptionObject),
                    ("pThrowInfo", &pThrowInfo),
                ],
            ))
        } else {
            None
        };
        let result =
            winapi::vcruntime140::_CxxThrowException(machine, pExceptionObject, pThrowInfo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::vcruntime140::_CxxThrowException_pos.0,
                winapi::vcruntime140::_CxxThrowException_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn memcmp(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lhs = <u32>::from_stack(mem, stack_args + 0u32);
        let rhs = <u32>::from_stack(mem, stack_args + 4u32);
        let len = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("vcruntime140") {
            Some(crate::trace::trace_begin(
                "vcruntime140",
                "memcmp",
                &[("lhs", &lhs), ("rhs", &rhs), ("len", &len)],
            ))
        } else {
            None
        };
        let result = winapi::vcruntime140::memcmp(machine, lhs, rhs, len);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::vcruntime140::memcmp_pos.0,
                winapi::vcruntime140::memcmp_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn memcpy(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let dst = <u32>::from_stack(mem, stack_args + 0u32);
        let src = <u32>::from_stack(mem, stack_args + 4u32);
        let len = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("vcruntime140") {
            Some(crate::trace::trace_begin(
                "vcruntime140",
                "memcpy",
                &[("dst", &dst), ("src", &src), ("len", &len)],
            ))
        } else {
            None
        };
        let result = winapi::vcruntime140::memcpy(machine, dst, src, len);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::vcruntime140::memcpy_pos.0,
                winapi::vcruntime140::memcpy_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn memset(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let dst = <u32>::from_stack(mem, stack_args + 0u32);
        let val = <u32>::from_stack(mem, stack_args + 4u32);
        let len = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("vcruntime140") {
            Some(crate::trace::trace_begin(
                "vcruntime140",
                "memset",
                &[("dst", &dst), ("val", &val), ("len", &len)],
            ))
        } else {
            None
        };
        let result = winapi::vcruntime140::memset(machine, dst, val, len);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::vcruntime140::memset_pos.0,
                winapi::vcruntime140::memset_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
}
const SHIMS: [Shim; 4usize] = [
    Shim {
        name: "_CxxThrowException",
        func: Handler::Sync(wrappers::_CxxThrowException),
    },
    Shim {
        name: "memcmp",
        func: Handler::Sync(wrappers::memcmp),
    },
    Shim {
        name: "memcpy",
        func: Handler::Sync(wrappers::memcpy),
    },
    Shim {
        name: "memset",
        func: Handler::Sync(wrappers::memset),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "vcruntime140.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/vcruntime140.dll"),
};