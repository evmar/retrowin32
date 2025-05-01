#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate as version;
    use crate::*;
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
    pub unsafe fn GetFileVersionInfoA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lptstrFilename = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let dwHandle = <u32>::from_stack(mem, stack_args + 4u32);
            let dwLen = <u32>::from_stack(mem, stack_args + 8u32);
            let lpData = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("src/lib") {
                trace::Record::new(
                    version::GetFileVersionInfoA_pos,
                    "src/lib",
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
            let result = version::GetFileVersionInfoA(sys, lptstrFilename, dwHandle, dwLen, lpData);
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
            let __trace_record = if trace::enabled("src/lib") {
                trace::Record::new(
                    version::GetFileVersionInfoSizeA_pos,
                    "src/lib",
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
            let result = version::GetFileVersionInfoSizeA(sys, lptstrFilename, lpdwHandle);
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
            let __trace_record = if trace::enabled("src/lib") {
                trace::Record::new(
                    version::VerQueryValueA_pos,
                    "src/lib",
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
            let result = version::VerQueryValueA(sys, pBlock, lpSubBlock, lplpBuffer, puLen);
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
    raw: std::include_bytes!("../version.dll"),
};
