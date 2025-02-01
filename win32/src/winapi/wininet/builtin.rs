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
    use winapi::wininet::*;
    pub unsafe fn InternetOpenA(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let lpszAgent = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let dwAccessType = <u32>::from_stack(mem, stack_args + 4u32);
        let lpszProxy = <Option<&str>>::from_stack(mem, stack_args + 8u32);
        let lpszProxyBypass = <Option<&str>>::from_stack(mem, stack_args + 12u32);
        let dwFlags = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("wininet") {
            crate::trace::Record::new(
                winapi::wininet::InternetOpenA_pos,
                "wininet",
                "InternetOpenA",
                &[
                    ("lpszAgent", &lpszAgent),
                    ("dwAccessType", &dwAccessType),
                    ("lpszProxy", &lpszProxy),
                    ("lpszProxyBypass", &lpszProxyBypass),
                    ("dwFlags", &dwFlags),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::wininet::InternetOpenA(
            machine,
            lpszAgent,
            dwAccessType,
            lpszProxy,
            lpszProxyBypass,
            dwFlags,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_abireturn()
    }
}
const SHIMS: [Shim; 1usize] = [Shim {
    name: "InternetOpenA",
    func: Handler::Sync(wrappers::InternetOpenA),
}];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "wininet.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/wininet.dll"),
};
