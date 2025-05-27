#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate as vcruntime140;
    use crate::*;
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
    pub unsafe fn _CxxThrowException(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use vcruntime140::*;
        unsafe {
            let mem = sys.mem().detach();
            let pExceptionObject = <u32>::from_stack(mem, stack_args + 0u32);
            let pThrowInfo = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("vcruntime140") {
                trace::Record::new(
                    vcruntime140::_CxxThrowException_pos,
                    "vcruntime140",
                    "_CxxThrowException",
                    &[
                        ("pExceptionObject", &pExceptionObject),
                        ("pThrowInfo", &pThrowInfo),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = vcruntime140::_CxxThrowException(sys, pExceptionObject, pThrowInfo);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn memcmp(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use vcruntime140::*;
        unsafe {
            let mem = sys.mem().detach();
            let lhs = <u32>::from_stack(mem, stack_args + 0u32);
            let rhs = <u32>::from_stack(mem, stack_args + 4u32);
            let len = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("vcruntime140") {
                trace::Record::new(
                    vcruntime140::memcmp_pos,
                    "vcruntime140",
                    "memcmp",
                    &[("lhs", &lhs), ("rhs", &rhs), ("len", &len)],
                )
                .enter()
            } else {
                None
            };
            let result = vcruntime140::memcmp(sys, lhs, rhs, len);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn memcpy(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use vcruntime140::*;
        unsafe {
            let mem = sys.mem().detach();
            let dst = <u32>::from_stack(mem, stack_args + 0u32);
            let src = <u32>::from_stack(mem, stack_args + 4u32);
            let len = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("vcruntime140") {
                trace::Record::new(
                    vcruntime140::memcpy_pos,
                    "vcruntime140",
                    "memcpy",
                    &[("dst", &dst), ("src", &src), ("len", &len)],
                )
                .enter()
            } else {
                None
            };
            let result = vcruntime140::memcpy(sys, dst, src, len);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn memset(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use vcruntime140::*;
        unsafe {
            let mem = sys.mem().detach();
            let dst = <u32>::from_stack(mem, stack_args + 0u32);
            let val = <u32>::from_stack(mem, stack_args + 4u32);
            let len = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("vcruntime140") {
                trace::Record::new(
                    vcruntime140::memset_pos,
                    "vcruntime140",
                    "memset",
                    &[("dst", &dst), ("val", &val), ("len", &len)],
                )
                .enter()
            } else {
                None
            };
            let result = vcruntime140::memset(sys, dst, val, len);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
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
    raw: std::include_bytes!("../vcruntime140.dll"),
};
