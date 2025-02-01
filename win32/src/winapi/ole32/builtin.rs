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
    use winapi::ole32::*;
    pub unsafe fn CoCreateInstance(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let rclsid = <u32>::from_stack(mem, stack_args + 0u32);
        let pUnkOuter = <u32>::from_stack(mem, stack_args + 4u32);
        let dwClsContext = <u32>::from_stack(mem, stack_args + 8u32);
        let riid = <u32>::from_stack(mem, stack_args + 12u32);
        let ppv = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("ole32") {
            crate::trace::Record::new(
                winapi::ole32::CoCreateInstance_pos,
                "ole32",
                "CoCreateInstance",
                &[
                    ("rclsid", &rclsid),
                    ("pUnkOuter", &pUnkOuter),
                    ("dwClsContext", &dwClsContext),
                    ("riid", &riid),
                    ("ppv", &ppv),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::ole32::CoCreateInstance(machine, rclsid, pUnkOuter, dwClsContext, riid, ppv);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_abireturn()
    }
    pub unsafe fn CoInitialize(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let pvReserved = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("ole32") {
            crate::trace::Record::new(
                winapi::ole32::CoInitialize_pos,
                "ole32",
                "CoInitialize",
                &[("pvReserved", &pvReserved)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ole32::CoInitialize(machine, pvReserved);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_abireturn()
    }
    pub unsafe fn CoInitializeEx(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let pvReserved = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
        let dwCoInit = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("ole32") {
            crate::trace::Record::new(
                winapi::ole32::CoInitializeEx_pos,
                "ole32",
                "CoInitializeEx",
                &[("pvReserved", &pvReserved), ("dwCoInit", &dwCoInit)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ole32::CoInitializeEx(machine, pvReserved, dwCoInit);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_abireturn()
    }
    pub unsafe fn CoUninitialize(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("ole32") {
            crate::trace::Record::new(
                winapi::ole32::CoUninitialize_pos,
                "ole32",
                "CoUninitialize",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ole32::CoUninitialize(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_abireturn()
    }
    pub unsafe fn OleInitialize(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let _pvReserved = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("ole32") {
            crate::trace::Record::new(
                winapi::ole32::OleInitialize_pos,
                "ole32",
                "OleInitialize",
                &[("pvReserved", &_pvReserved)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ole32::OleInitialize(machine, _pvReserved);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_abireturn()
    }
}
const SHIMS: [Shim; 5usize] = [
    Shim {
        name: "CoCreateInstance",
        func: Handler::Sync(wrappers::CoCreateInstance),
    },
    Shim {
        name: "CoInitialize",
        func: Handler::Sync(wrappers::CoInitialize),
    },
    Shim {
        name: "CoInitializeEx",
        func: Handler::Sync(wrappers::CoInitializeEx),
    },
    Shim {
        name: "CoUninitialize",
        func: Handler::Sync(wrappers::CoUninitialize),
    },
    Shim {
        name: "OleInitialize",
        func: Handler::Sync(wrappers::OleInitialize),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "ole32.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/ole32.dll"),
};
