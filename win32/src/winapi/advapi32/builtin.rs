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
    use winapi::advapi32::*;
    pub unsafe fn RegCloseKey(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if crate::winapi::trace::enabled("advapi32") {
                crate::winapi::trace::Record::new(
                    winapi::advapi32::RegCloseKey_pos,
                    "advapi32",
                    "RegCloseKey",
                    &[("hKey", &hKey)],
                )
                .enter()
            } else {
                None
            };
            let result = winapi::advapi32::RegCloseKey(sys, hKey);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RegCreateKeyA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpSubKey = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let phkResult = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if crate::winapi::trace::enabled("advapi32") {
                crate::winapi::trace::Record::new(
                    winapi::advapi32::RegCreateKeyA_pos,
                    "advapi32",
                    "RegCreateKeyA",
                    &[
                        ("hKey", &hKey),
                        ("lpSubKey", &lpSubKey),
                        ("phkResult", &phkResult),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = winapi::advapi32::RegCreateKeyA(sys, hKey, lpSubKey, phkResult);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RegCreateKeyExW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpSubKey = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let Reserved = <u32>::from_stack(mem, stack_args + 8u32);
            let lpClass = <Option<&Str16>>::from_stack(mem, stack_args + 12u32);
            let dwOptions = <u32>::from_stack(mem, stack_args + 16u32);
            let samDesired = <u32>::from_stack(mem, stack_args + 20u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, stack_args + 24u32);
            let phkResult = <Option<&mut u32>>::from_stack(mem, stack_args + 28u32);
            let lpdwDisposition = <Option<&mut u32>>::from_stack(mem, stack_args + 32u32);
            let __trace_record = if crate::winapi::trace::enabled("advapi32") {
                crate::winapi::trace::Record::new(
                    winapi::advapi32::RegCreateKeyExW_pos,
                    "advapi32",
                    "RegCreateKeyExW",
                    &[
                        ("hKey", &hKey),
                        ("lpSubKey", &lpSubKey),
                        ("Reserved", &Reserved),
                        ("lpClass", &lpClass),
                        ("dwOptions", &dwOptions),
                        ("samDesired", &samDesired),
                        ("lpSecurityAttributes", &lpSecurityAttributes),
                        ("phkResult", &phkResult),
                        ("lpdwDisposition", &lpdwDisposition),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = winapi::advapi32::RegCreateKeyExW(
                sys,
                hKey,
                lpSubKey,
                Reserved,
                lpClass,
                dwOptions,
                samDesired,
                lpSecurityAttributes,
                phkResult,
                lpdwDisposition,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RegOpenKeyExA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpSubKey = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let ulOptions = <u32>::from_stack(mem, stack_args + 8u32);
            let samDesired = <u32>::from_stack(mem, stack_args + 12u32);
            let phkResult = <Option<&mut HKEY>>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if crate::winapi::trace::enabled("advapi32") {
                crate::winapi::trace::Record::new(
                    winapi::advapi32::RegOpenKeyExA_pos,
                    "advapi32",
                    "RegOpenKeyExA",
                    &[
                        ("hKey", &hKey),
                        ("lpSubKey", &lpSubKey),
                        ("ulOptions", &ulOptions),
                        ("samDesired", &samDesired),
                        ("phkResult", &phkResult),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = winapi::advapi32::RegOpenKeyExA(
                sys, hKey, lpSubKey, ulOptions, samDesired, phkResult,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RegQueryValueExA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpValueName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let lpReserved = <u32>::from_stack(mem, stack_args + 8u32);
            let lpType = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpData = <u32>::from_stack(mem, stack_args + 16u32);
            let lpcbData = <Option<&mut u32>>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if crate::winapi::trace::enabled("advapi32") {
                crate::winapi::trace::Record::new(
                    winapi::advapi32::RegQueryValueExA_pos,
                    "advapi32",
                    "RegQueryValueExA",
                    &[
                        ("hKey", &hKey),
                        ("lpValueName", &lpValueName),
                        ("lpReserved", &lpReserved),
                        ("lpType", &lpType),
                        ("lpData", &lpData),
                        ("lpcbData", &lpcbData),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = winapi::advapi32::RegQueryValueExA(
                sys,
                hKey,
                lpValueName,
                lpReserved,
                lpType,
                lpData,
                lpcbData,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RegQueryValueExW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpValueName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpReserved = <u32>::from_stack(mem, stack_args + 8u32);
            let lpType = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpData = <u32>::from_stack(mem, stack_args + 16u32);
            let lpcbData = <Option<&mut u32>>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if crate::winapi::trace::enabled("advapi32") {
                crate::winapi::trace::Record::new(
                    winapi::advapi32::RegQueryValueExW_pos,
                    "advapi32",
                    "RegQueryValueExW",
                    &[
                        ("hKey", &hKey),
                        ("lpValueName", &lpValueName),
                        ("lpReserved", &lpReserved),
                        ("lpType", &lpType),
                        ("lpData", &lpData),
                        ("lpcbData", &lpcbData),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = winapi::advapi32::RegQueryValueExW(
                sys,
                hKey,
                lpValueName,
                lpReserved,
                lpType,
                lpData,
                lpcbData,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RegSetValueExA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpValueName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let Reserved = <u32>::from_stack(mem, stack_args + 8u32);
            let dwType = <u32>::from_stack(mem, stack_args + 12u32);
            let lpData = <u32>::from_stack(mem, stack_args + 16u32);
            let cbData = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if crate::winapi::trace::enabled("advapi32") {
                crate::winapi::trace::Record::new(
                    winapi::advapi32::RegSetValueExA_pos,
                    "advapi32",
                    "RegSetValueExA",
                    &[
                        ("hKey", &hKey),
                        ("lpValueName", &lpValueName),
                        ("Reserved", &Reserved),
                        ("dwType", &dwType),
                        ("lpData", &lpData),
                        ("cbData", &cbData),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = winapi::advapi32::RegSetValueExA(
                sys,
                hKey,
                lpValueName,
                Reserved,
                dwType,
                lpData,
                cbData,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RegSetValueExW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpValueName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let Reserved = <u32>::from_stack(mem, stack_args + 8u32);
            let dwType = <u32>::from_stack(mem, stack_args + 12u32);
            let lpData = <u32>::from_stack(mem, stack_args + 16u32);
            let cbData = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if crate::winapi::trace::enabled("advapi32") {
                crate::winapi::trace::Record::new(
                    winapi::advapi32::RegSetValueExW_pos,
                    "advapi32",
                    "RegSetValueExW",
                    &[
                        ("hKey", &hKey),
                        ("lpValueName", &lpValueName),
                        ("Reserved", &Reserved),
                        ("dwType", &dwType),
                        ("lpData", &lpData),
                        ("cbData", &cbData),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = winapi::advapi32::RegSetValueExW(
                sys,
                hKey,
                lpValueName,
                Reserved,
                dwType,
                lpData,
                cbData,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
}
const SHIMS: [Shim; 8usize] = [
    Shim {
        name: "RegCloseKey",
        func: Handler::Sync(wrappers::RegCloseKey),
    },
    Shim {
        name: "RegCreateKeyA",
        func: Handler::Sync(wrappers::RegCreateKeyA),
    },
    Shim {
        name: "RegCreateKeyExW",
        func: Handler::Sync(wrappers::RegCreateKeyExW),
    },
    Shim {
        name: "RegOpenKeyExA",
        func: Handler::Sync(wrappers::RegOpenKeyExA),
    },
    Shim {
        name: "RegQueryValueExA",
        func: Handler::Sync(wrappers::RegQueryValueExA),
    },
    Shim {
        name: "RegQueryValueExW",
        func: Handler::Sync(wrappers::RegQueryValueExW),
    },
    Shim {
        name: "RegSetValueExA",
        func: Handler::Sync(wrappers::RegSetValueExA),
    },
    Shim {
        name: "RegSetValueExW",
        func: Handler::Sync(wrappers::RegSetValueExW),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "advapi32.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/advapi32.dll"),
};
