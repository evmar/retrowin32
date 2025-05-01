#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate::{
        System,
        calling_convention::*,
        machine::Machine,
        winapi::{self, *},
    };
    use ::memory::Extensions;
    use winapi::version::*;
    pub unsafe fn GetFileVersionInfoA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lptstrFilename = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let dwHandle = <u32>::from_stack(mem, stack_args + 4u32);
            let dwLen = <u32>::from_stack(mem, stack_args + 8u32);
            let lpData = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if crate::winapi::trace::enabled("version") {
                crate::winapi::trace::Record::new(
                    winapi::version::GetFileVersionInfoA_pos,
                    "version",
                    "GetFileVersionInfoA",
                    &[
                        ("lptstrFilename", &lptstrFilename),
                        ("dwHandle", &dwHandle),
                        ("dwLen", &dwLen),
                        ("lpData", &lpData),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                winapi::version::GetFileVersionInfoA(sys, lptstrFilename, dwHandle, dwLen, lpData);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetFileVersionInfoSizeA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lptstrFilename = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpdwHandle = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if crate::winapi::trace::enabled("version") {
                crate::winapi::trace::Record::new(
                    winapi::version::GetFileVersionInfoSizeA_pos,
                    "version",
                    "GetFileVersionInfoSizeA",
                    &[
                        ("lptstrFilename", &lptstrFilename),
                        ("lpdwHandle", &lpdwHandle),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = winapi::version::GetFileVersionInfoSizeA(sys, lptstrFilename, lpdwHandle);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn VerQueryValueA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let pBlock = <u32>::from_stack(mem, stack_args + 0u32);
            let lpSubBlock = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let lplpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
            let puLen = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if crate::winapi::trace::enabled("version") {
                crate::winapi::trace::Record::new(
                    winapi::version::VerQueryValueA_pos,
                    "version",
                    "VerQueryValueA",
                    &[
                        ("pBlock", &pBlock),
                        ("lpSubBlock", &lpSubBlock),
                        ("lplpBuffer", &lplpBuffer),
                        ("puLen", &puLen),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                winapi::version::VerQueryValueA(sys, pBlock, lpSubBlock, lplpBuffer, puLen);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
}
const SHIMS: [Shim; 3usize] = [
    Shim {
        name: "GetFileVersionInfoA",
        func: Handler::Sync(wrappers::GetFileVersionInfoA),
    },
    Shim {
        name: "GetFileVersionInfoSizeA",
        func: Handler::Sync(wrappers::GetFileVersionInfoSizeA),
    },
    Shim {
        name: "VerQueryValueA",
        func: Handler::Sync(wrappers::VerQueryValueA),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "version.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/version.dll"),
};
