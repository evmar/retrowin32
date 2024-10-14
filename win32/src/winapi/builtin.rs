#![allow(non_snake_case)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#[doc = r" Generated code, do not edit."]
use crate::shims::{Handler, Shim};
pub struct BuiltinDLL {
    pub file_name: &'static str,
    #[doc = r" The xth function in the DLL represents a call to shims[x]."]
    pub shims: &'static [Shim],
    #[doc = r" Raw bytes of generated .dll."]
    pub raw: &'static [u8],
}
pub mod advapi32 {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::advapi32::*;
        pub unsafe fn RegCloseKey(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("advapi32") {
                Some(crate::trace::trace_begin(
                    "advapi32",
                    "RegCloseKey",
                    &[("hKey", &hKey)],
                ))
            } else {
                None
            };
            let result = winapi::advapi32::RegCloseKey(machine, hKey);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::advapi32::RegCloseKey_pos.0,
                    winapi::advapi32::RegCloseKey_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RegCreateKeyA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpSubKey = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let phkResult = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("advapi32") {
                Some(crate::trace::trace_begin(
                    "advapi32",
                    "RegCreateKeyA",
                    &[
                        ("hKey", &hKey),
                        ("lpSubKey", &lpSubKey),
                        ("phkResult", &phkResult),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::advapi32::RegCreateKeyA(machine, hKey, lpSubKey, phkResult);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::advapi32::RegCreateKeyA_pos.0,
                    winapi::advapi32::RegCreateKeyA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RegCreateKeyExW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpSubKey = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let Reserved = <u32>::from_stack(mem, stack_args + 8u32);
            let lpClass = <Option<&Str16>>::from_stack(mem, stack_args + 12u32);
            let dwOptions = <u32>::from_stack(mem, stack_args + 16u32);
            let samDesired = <u32>::from_stack(mem, stack_args + 20u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, stack_args + 24u32);
            let phkResult = <Option<&mut u32>>::from_stack(mem, stack_args + 28u32);
            let lpdwDisposition = <Option<&mut u32>>::from_stack(mem, stack_args + 32u32);
            let __trace_context = if crate::trace::enabled("advapi32") {
                Some(crate::trace::trace_begin(
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
                ))
            } else {
                None
            };
            let result = winapi::advapi32::RegCreateKeyExW(
                machine,
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
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::advapi32::RegCreateKeyExW_pos.0,
                    winapi::advapi32::RegCreateKeyExW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RegOpenKeyExA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpSubKey = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let ulOptions = <u32>::from_stack(mem, stack_args + 8u32);
            let samDesired = <u32>::from_stack(mem, stack_args + 12u32);
            let phkResult = <Option<&mut HKEY>>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("advapi32") {
                Some(crate::trace::trace_begin(
                    "advapi32",
                    "RegOpenKeyExA",
                    &[
                        ("hKey", &hKey),
                        ("lpSubKey", &lpSubKey),
                        ("ulOptions", &ulOptions),
                        ("samDesired", &samDesired),
                        ("phkResult", &phkResult),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::advapi32::RegOpenKeyExA(
                machine, hKey, lpSubKey, ulOptions, samDesired, phkResult,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::advapi32::RegOpenKeyExA_pos.0,
                    winapi::advapi32::RegOpenKeyExA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RegQueryValueExA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpValueName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let lpReserved = <u32>::from_stack(mem, stack_args + 8u32);
            let lpType = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpData = <u32>::from_stack(mem, stack_args + 16u32);
            let lpcbData = <Option<&mut u32>>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("advapi32") {
                Some(crate::trace::trace_begin(
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
                ))
            } else {
                None
            };
            let result = winapi::advapi32::RegQueryValueExA(
                machine,
                hKey,
                lpValueName,
                lpReserved,
                lpType,
                lpData,
                lpcbData,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::advapi32::RegQueryValueExA_pos.0,
                    winapi::advapi32::RegQueryValueExA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RegQueryValueExW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpValueName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpReserved = <u32>::from_stack(mem, stack_args + 8u32);
            let lpType = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpData = <u32>::from_stack(mem, stack_args + 16u32);
            let lpcbData = <Option<&mut u32>>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("advapi32") {
                Some(crate::trace::trace_begin(
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
                ))
            } else {
                None
            };
            let result = winapi::advapi32::RegQueryValueExW(
                machine,
                hKey,
                lpValueName,
                lpReserved,
                lpType,
                lpData,
                lpcbData,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::advapi32::RegQueryValueExW_pos.0,
                    winapi::advapi32::RegQueryValueExW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RegSetValueExA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpValueName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let Reserved = <u32>::from_stack(mem, stack_args + 8u32);
            let dwType = <u32>::from_stack(mem, stack_args + 12u32);
            let lpData = <u32>::from_stack(mem, stack_args + 16u32);
            let cbData = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("advapi32") {
                Some(crate::trace::trace_begin(
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
                ))
            } else {
                None
            };
            let result = winapi::advapi32::RegSetValueExA(
                machine,
                hKey,
                lpValueName,
                Reserved,
                dwType,
                lpData,
                cbData,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::advapi32::RegSetValueExA_pos.0,
                    winapi::advapi32::RegSetValueExA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RegSetValueExW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpValueName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let Reserved = <u32>::from_stack(mem, stack_args + 8u32);
            let dwType = <u32>::from_stack(mem, stack_args + 12u32);
            let lpData = <u32>::from_stack(mem, stack_args + 16u32);
            let cbData = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("advapi32") {
                Some(crate::trace::trace_begin(
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
                ))
            } else {
                None
            };
            let result = winapi::advapi32::RegSetValueExW(
                machine,
                hKey,
                lpValueName,
                Reserved,
                dwType,
                lpData,
                cbData,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::advapi32::RegSetValueExW_pos.0,
                    winapi::advapi32::RegSetValueExW_pos.1,
                    &result,
                );
            }
            result.to_raw()
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
        raw: std::include_bytes!("../../dll/advapi32.dll"),
    };
}
pub mod bass {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::bass::*;
        pub unsafe fn BASS_ChannelGetPosition(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let mode = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("bass") {
                Some(crate::trace::trace_begin(
                    "bass",
                    "BASS_ChannelGetPosition",
                    &[("mode", &mode)],
                ))
            } else {
                None
            };
            let result = winapi::bass::BASS_ChannelGetPosition(machine, mode);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::bass::BASS_ChannelGetPosition_pos.0,
                    winapi::bass::BASS_ChannelGetPosition_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn BASS_Free(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("bass") {
                Some(crate::trace::trace_begin(
                    "bass",
                    "BASS_Free",
                    &[("arg1", &arg1)],
                ))
            } else {
                None
            };
            let result = winapi::bass::BASS_Free(machine, arg1);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::bass::BASS_Free_pos.0,
                    winapi::bass::BASS_Free_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn BASS_Init(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
            let arg2 = <u32>::from_stack(mem, stack_args + 4u32);
            let arg3 = <u32>::from_stack(mem, stack_args + 8u32);
            let arg4 = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("bass") {
                Some(crate::trace::trace_begin(
                    "bass",
                    "BASS_Init",
                    &[
                        ("arg1", &arg1),
                        ("arg2", &arg2),
                        ("arg3", &arg3),
                        ("arg4", &arg4),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::bass::BASS_Init(machine, arg1, arg2, arg3, arg4);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::bass::BASS_Init_pos.0,
                    winapi::bass::BASS_Init_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn BASS_MusicLoad(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
            let arg2 = <u32>::from_stack(mem, stack_args + 4u32);
            let arg3 = <u32>::from_stack(mem, stack_args + 8u32);
            let arg4 = <u32>::from_stack(mem, stack_args + 12u32);
            let arg5 = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("bass") {
                Some(crate::trace::trace_begin(
                    "bass",
                    "BASS_MusicLoad",
                    &[
                        ("arg1", &arg1),
                        ("arg2", &arg2),
                        ("arg3", &arg3),
                        ("arg4", &arg4),
                        ("arg5", &arg5),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::bass::BASS_MusicLoad(machine, arg1, arg2, arg3, arg4, arg5);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::bass::BASS_MusicLoad_pos.0,
                    winapi::bass::BASS_MusicLoad_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn BASS_MusicPlay(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("bass") {
                Some(crate::trace::trace_begin(
                    "bass",
                    "BASS_MusicPlay",
                    &[("arg1", &arg1)],
                ))
            } else {
                None
            };
            let result = winapi::bass::BASS_MusicPlay(machine, arg1);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::bass::BASS_MusicPlay_pos.0,
                    winapi::bass::BASS_MusicPlay_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn BASS_MusicSetPositionScaler(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
            let arg2 = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("bass") {
                Some(crate::trace::trace_begin(
                    "bass",
                    "BASS_MusicSetPositionScaler",
                    &[("arg1", &arg1), ("arg2", &arg2)],
                ))
            } else {
                None
            };
            let result = winapi::bass::BASS_MusicSetPositionScaler(machine, arg1, arg2);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::bass::BASS_MusicSetPositionScaler_pos.0,
                    winapi::bass::BASS_MusicSetPositionScaler_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn BASS_Start(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("bass") {
                Some(crate::trace::trace_begin("bass", "BASS_Start", &[]))
            } else {
                None
            };
            let result = winapi::bass::BASS_Start(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::bass::BASS_Start_pos.0,
                    winapi::bass::BASS_Start_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
    }
    const SHIMS: [Shim; 7usize] = [
        Shim {
            name: "BASS_ChannelGetPosition",
            func: Handler::Sync(wrappers::BASS_ChannelGetPosition),
        },
        Shim {
            name: "BASS_Free",
            func: Handler::Sync(wrappers::BASS_Free),
        },
        Shim {
            name: "BASS_Init",
            func: Handler::Sync(wrappers::BASS_Init),
        },
        Shim {
            name: "BASS_MusicLoad",
            func: Handler::Sync(wrappers::BASS_MusicLoad),
        },
        Shim {
            name: "BASS_MusicPlay",
            func: Handler::Sync(wrappers::BASS_MusicPlay),
        },
        Shim {
            name: "BASS_MusicSetPositionScaler",
            func: Handler::Sync(wrappers::BASS_MusicSetPositionScaler),
        },
        Shim {
            name: "BASS_Start",
            func: Handler::Sync(wrappers::BASS_Start),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "bass.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/bass.dll"),
    };
}
pub mod ddraw {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ddraw::*;
        pub unsafe fn DirectDrawCreate(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, stack_args + 0u32);
            let lplpDD = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("ddraw/mod") {
                Some(crate::trace::trace_begin(
                    "ddraw/mod",
                    "DirectDrawCreate",
                    &[
                        ("lpGuid", &lpGuid),
                        ("lplpDD", &lplpDD),
                        ("pUnkOuter", &pUnkOuter),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::DirectDrawCreate_pos.0,
                    winapi::ddraw::DirectDrawCreate_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn DirectDrawCreateClipper(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, stack_args + 0u32);
            let lplpDDClipper = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("ddraw/clipper") {
                Some(crate::trace::trace_begin(
                    "ddraw/clipper",
                    "DirectDrawCreateClipper",
                    &[
                        ("dwFlags", &dwFlags),
                        ("lplpDDClipper", &lplpDDClipper),
                        ("pUnkOuter", &pUnkOuter),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::ddraw::DirectDrawCreateClipper(machine, dwFlags, lplpDDClipper, pUnkOuter);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::DirectDrawCreateClipper_pos.0,
                    winapi::ddraw::DirectDrawCreateClipper_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn DirectDrawCreateEx(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, stack_args + 0u32);
            let lplpDD = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let iid = <Option<&GUID>>::from_stack(mem, stack_args + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("ddraw/mod") {
                Some(crate::trace::trace_begin(
                    "ddraw/mod",
                    "DirectDrawCreateEx",
                    &[
                        ("lpGuid", &lpGuid),
                        ("lplpDD", &lplpDD),
                        ("iid", &iid),
                        ("pUnkOuter", &pUnkOuter),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::DirectDrawCreateEx_pos.0,
                    winapi::ddraw::DirectDrawCreateEx_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw2_CreateSurface(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let desc = <Option<&DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
            let lplpDDSurface = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw2") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw2",
                    "IDirectDraw2::CreateSurface",
                    &[
                        ("this", &this),
                        ("desc", &desc),
                        ("lplpDDSurface", &lplpDDSurface),
                        ("pUnkOuter", &pUnkOuter),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDraw2::CreateSurface(
                machine,
                this,
                desc,
                lplpDDSurface,
                pUnkOuter,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw2::CreateSurface_pos.0,
                    winapi::ddraw::IDirectDraw2::CreateSurface_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw2_EnumDisplayModes(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpSurfaceDesc = <Option<&DDSURFACEDESC>>::from_stack(mem, stack_args + 8u32);
            let lpContext = <u32>::from_stack(mem, stack_args + 12u32);
            let lpEnumCallback = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw2") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw2",
                    "IDirectDraw2::EnumDisplayModes",
                    &[
                        ("this", &this),
                        ("dwFlags", &dwFlags),
                        ("lpSurfaceDesc", &lpSurfaceDesc),
                        ("lpContext", &lpContext),
                        ("lpEnumCallback", &lpEnumCallback),
                    ],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::ddraw::IDirectDraw2::EnumDisplayModes(
                    machine,
                    this,
                    dwFlags,
                    lpSurfaceDesc,
                    lpContext,
                    lpEnumCallback,
                )
                .await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::ddraw::IDirectDraw2::EnumDisplayModes_pos.0,
                        winapi::ddraw::IDirectDraw2::EnumDisplayModes_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn IDirectDraw2_GetDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSurfaceDesc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw2") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw2",
                    "IDirectDraw2::GetDisplayMode",
                    &[("this", &this), ("lpDDSurfaceDesc", &lpDDSurfaceDesc)],
                ))
            } else {
                None
            };
            let result =
                winapi::ddraw::IDirectDraw2::GetDisplayMode(machine, this, lpDDSurfaceDesc);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw2::GetDisplayMode_pos.0,
                    winapi::ddraw::IDirectDraw2::GetDisplayMode_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw2_QueryInterface(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let riid = <Option<&GUID>>::from_stack(mem, stack_args + 4u32);
            let ppvObject = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw2") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw2",
                    "IDirectDraw2::QueryInterface",
                    &[("this", &this), ("riid", &riid), ("ppvObject", &ppvObject)],
                ))
            } else {
                None
            };
            let result =
                winapi::ddraw::IDirectDraw2::QueryInterface(machine, this, riid, ppvObject);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw2::QueryInterface_pos.0,
                    winapi::ddraw::IDirectDraw2::QueryInterface_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw2_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw2") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw2",
                    "IDirectDraw2::Release",
                    &[("this", &this)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDraw2::Release(machine, this);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw2::Release_pos.0,
                    winapi::ddraw::IDirectDraw2::Release_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw2_SetDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let width = <u32>::from_stack(mem, stack_args + 4u32);
            let height = <u32>::from_stack(mem, stack_args + 8u32);
            let bpp = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw2") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw2",
                    "IDirectDraw2::SetDisplayMode",
                    &[
                        ("this", &this),
                        ("width", &width),
                        ("height", &height),
                        ("bpp", &bpp),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::ddraw::IDirectDraw2::SetDisplayMode(machine, this, width, height, bpp);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw2::SetDisplayMode_pos.0,
                    winapi::ddraw::IDirectDraw2::SetDisplayMode_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw7_CreatePalette(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let flags = <Result<DDPCAPS, u32>>::from_stack(mem, stack_args + 4u32);
            let entries = <u32>::from_stack(mem, stack_args + 8u32);
            let lplpPalette = <u32>::from_stack(mem, stack_args + 12u32);
            let unused = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDraw7::CreatePalette",
                    &[
                        ("this", &this),
                        ("flags", &flags),
                        ("entries", &entries),
                        ("lplpPalette", &lplpPalette),
                        ("unused", &unused),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDraw7::CreatePalette(
                machine,
                this,
                flags,
                entries,
                lplpPalette,
                unused,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw7::CreatePalette_pos.0,
                    winapi::ddraw::IDirectDraw7::CreatePalette_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw7_CreateSurface(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let desc = <Option<&DDSURFACEDESC2>>::from_stack(mem, stack_args + 4u32);
            let lpDirectDrawSurface7 = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let unused = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDraw7::CreateSurface",
                    &[
                        ("this", &this),
                        ("desc", &desc),
                        ("lpDirectDrawSurface7", &lpDirectDrawSurface7),
                        ("unused", &unused),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDraw7::CreateSurface(
                machine,
                this,
                desc,
                lpDirectDrawSurface7,
                unused,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw7::CreateSurface_pos.0,
                    winapi::ddraw::IDirectDraw7::CreateSurface_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw7_EnumDisplayModes(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpSurfaceDesc = <Option<&DDSURFACEDESC2>>::from_stack(mem, stack_args + 8u32);
            let lpContext = <u32>::from_stack(mem, stack_args + 12u32);
            let lpEnumCallback = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDraw7::EnumDisplayModes",
                    &[
                        ("this", &this),
                        ("dwFlags", &dwFlags),
                        ("lpSurfaceDesc", &lpSurfaceDesc),
                        ("lpContext", &lpContext),
                        ("lpEnumCallback", &lpEnumCallback),
                    ],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::ddraw::IDirectDraw7::EnumDisplayModes(
                    machine,
                    this,
                    dwFlags,
                    lpSurfaceDesc,
                    lpContext,
                    lpEnumCallback,
                )
                .await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::ddraw::IDirectDraw7::EnumDisplayModes_pos.0,
                        winapi::ddraw::IDirectDraw7::EnumDisplayModes_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn IDirectDraw7_GetDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSurfaceDesc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDraw7::GetDisplayMode",
                    &[("this", &this), ("lpDDSurfaceDesc", &lpDDSurfaceDesc)],
                ))
            } else {
                None
            };
            let result =
                winapi::ddraw::IDirectDraw7::GetDisplayMode(machine, this, lpDDSurfaceDesc);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw7::GetDisplayMode_pos.0,
                    winapi::ddraw::IDirectDraw7::GetDisplayMode_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw7_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDraw7::Release",
                    &[("this", &this)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDraw7::Release(machine, this);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw7::Release_pos.0,
                    winapi::ddraw::IDirectDraw7::Release_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw7_RestoreDisplayMode(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDraw7::RestoreDisplayMode",
                    &[("this", &this)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDraw7::RestoreDisplayMode(machine, this);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw7::RestoreDisplayMode_pos.0,
                    winapi::ddraw::IDirectDraw7::RestoreDisplayMode_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw7_SetCooperativeLevel(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let hwnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let flags = <Result<DDSCL, u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDraw7::SetCooperativeLevel",
                    &[("this", &this), ("hwnd", &hwnd), ("flags", &flags)],
                ))
            } else {
                None
            };
            let result =
                winapi::ddraw::IDirectDraw7::SetCooperativeLevel(machine, this, hwnd, flags);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw7::SetCooperativeLevel_pos.0,
                    winapi::ddraw::IDirectDraw7::SetCooperativeLevel_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw7_SetDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let width = <u32>::from_stack(mem, stack_args + 4u32);
            let height = <u32>::from_stack(mem, stack_args + 8u32);
            let bpp = <u32>::from_stack(mem, stack_args + 12u32);
            let refresh = <u32>::from_stack(mem, stack_args + 16u32);
            let flags = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDraw7::SetDisplayMode",
                    &[
                        ("this", &this),
                        ("width", &width),
                        ("height", &height),
                        ("bpp", &bpp),
                        ("refresh", &refresh),
                        ("flags", &flags),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDraw7::SetDisplayMode(
                machine, this, width, height, bpp, refresh, flags,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw7::SetDisplayMode_pos.0,
                    winapi::ddraw::IDirectDraw7::SetDisplayMode_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw7_WaitForVerticalBlank(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let flags = <u32>::from_stack(mem, stack_args + 4u32);
            let _unused = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDraw7::WaitForVerticalBlank",
                    &[("this", &this), ("flags", &flags), ("unused", &_unused)],
                ))
            } else {
                None
            };
            let result =
                winapi::ddraw::IDirectDraw7::WaitForVerticalBlank(machine, this, flags, _unused);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw7::WaitForVerticalBlank_pos.0,
                    winapi::ddraw::IDirectDraw7::WaitForVerticalBlank_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawClipper_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("ddraw/clipper") {
                Some(crate::trace::trace_begin(
                    "ddraw/clipper",
                    "IDirectDrawClipper::Release",
                    &[("this", &this)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawClipper::Release(machine, this);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawClipper::Release_pos.0,
                    winapi::ddraw::IDirectDrawClipper::Release_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawClipper_SetHWnd(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let unused = <u32>::from_stack(mem, stack_args + 4u32);
            let hwnd = <HWND>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("ddraw/clipper") {
                Some(crate::trace::trace_begin(
                    "ddraw/clipper",
                    "IDirectDrawClipper::SetHWnd",
                    &[("this", &this), ("unused", &unused), ("hwnd", &hwnd)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawClipper::SetHWnd(machine, this, unused, hwnd);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawClipper::SetHWnd_pos.0,
                    winapi::ddraw::IDirectDrawClipper::SetHWnd_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawPalette_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("ddraw/palette") {
                Some(crate::trace::trace_begin(
                    "ddraw/palette",
                    "IDirectDrawPalette::Release",
                    &[("this", &this)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawPalette::Release(machine, this);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawPalette::Release_pos.0,
                    winapi::ddraw::IDirectDrawPalette::Release_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawPalette_SetEntries(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let unused = <u32>::from_stack(mem, stack_args + 4u32);
            let start = <u32>::from_stack(mem, stack_args + 8u32);
            let count = <u32>::from_stack(mem, stack_args + 12u32);
            let entries = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("ddraw/palette") {
                Some(crate::trace::trace_begin(
                    "ddraw/palette",
                    "IDirectDrawPalette::SetEntries",
                    &[
                        ("this", &this),
                        ("unused", &unused),
                        ("start", &start),
                        ("count", &count),
                        ("entries", &entries),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawPalette::SetEntries(
                machine, this, unused, start, count, entries,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawPalette::SetEntries_pos.0,
                    winapi::ddraw::IDirectDrawPalette::SetEntries_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_GetAttachedSurface(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCaps = <Option<&DDSCAPS>>::from_stack(mem, stack_args + 4u32);
            let lpDirectDrawSurface = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw2") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw2",
                    "IDirectDrawSurface2::GetAttachedSurface",
                    &[
                        ("this", &this),
                        ("lpDDSCaps", &lpDDSCaps),
                        ("lpDirectDrawSurface", &lpDirectDrawSurface),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface2::GetAttachedSurface(
                machine,
                this,
                lpDDSCaps,
                lpDirectDrawSurface,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface2::GetAttachedSurface_pos.0,
                    winapi::ddraw::IDirectDrawSurface2::GetAttachedSurface_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_GetCaps(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCAPS = <Option<&mut DDSCAPS>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw2") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw2",
                    "IDirectDrawSurface2::GetCaps",
                    &[("this", &this), ("lpDDSCAPS", &lpDDSCAPS)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface2::GetCaps(machine, this, lpDDSCAPS);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface2::GetCaps_pos.0,
                    winapi::ddraw::IDirectDrawSurface2::GetCaps_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_GetSurfaceDesc(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw2") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw2",
                    "IDirectDrawSurface2::GetSurfaceDesc",
                    &[("this", &this), ("desc", &desc)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface2::GetSurfaceDesc(machine, this, desc);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface2::GetSurfaceDesc_pos.0,
                    winapi::ddraw::IDirectDrawSurface2::GetSurfaceDesc_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_Lock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let rect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, stack_args + 8u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, stack_args + 12u32);
            let event = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw2") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw2",
                    "IDirectDrawSurface2::Lock",
                    &[
                        ("this", &this),
                        ("rect", &rect),
                        ("desc", &desc),
                        ("flags", &flags),
                        ("event", &event),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::ddraw::IDirectDrawSurface2::Lock(machine, this, rect, desc, flags, event);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface2::Lock_pos.0,
                    winapi::ddraw::IDirectDrawSurface2::Lock_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw2") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw2",
                    "IDirectDrawSurface2::Release",
                    &[("this", &this)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface2::Release(machine, this);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface2::Release_pos.0,
                    winapi::ddraw::IDirectDrawSurface2::Release_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_Unlock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let ptr = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw2") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw2",
                    "IDirectDrawSurface2::Unlock",
                    &[("this", &this), ("ptr", &ptr)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface2::Unlock(machine, this, ptr);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface2::Unlock_pos.0,
                    winapi::ddraw::IDirectDrawSurface2::Unlock_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Blt(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDstRect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let lpSrc = <u32>::from_stack(mem, stack_args + 8u32);
            let lpSrcRect = <Option<&RECT>>::from_stack(mem, stack_args + 12u32);
            let flags = <Result<DDBLT, u32>>::from_stack(mem, stack_args + 16u32);
            let lpDDBLTFX = <Option<&DDBLTFX>>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::Blt",
                    &[
                        ("this", &this),
                        ("lpDstRect", &lpDstRect),
                        ("lpSrc", &lpSrc),
                        ("lpSrcRect", &lpSrcRect),
                        ("flags", &flags),
                        ("lpDDBLTFX", &lpDDBLTFX),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface7::Blt(
                machine, this, lpDstRect, lpSrc, lpSrcRect, flags, lpDDBLTFX,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::Blt_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::Blt_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_BltFast(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let lpSrc = <u32>::from_stack(mem, stack_args + 12u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 16u32);
            let flags = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::BltFast",
                    &[
                        ("this", &this),
                        ("x", &x),
                        ("y", &y),
                        ("lpSrc", &lpSrc),
                        ("lpRect", &lpRect),
                        ("flags", &flags),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface7::BltFast(
                machine, this, x, y, lpSrc, lpRect, flags,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::BltFast_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::BltFast_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Flip(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpSurf = <u32>::from_stack(mem, stack_args + 4u32);
            let flags = <Result<DDFLIP, u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::Flip",
                    &[("this", &this), ("lpSurf", &lpSurf), ("flags", &flags)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface7::Flip(machine, this, lpSurf, flags);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::Flip_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::Flip_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetAttachedSurface(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCaps2 = <Option<&DDSCAPS2>>::from_stack(mem, stack_args + 4u32);
            let lpDirectDrawSurface7 = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::GetAttachedSurface",
                    &[
                        ("this", &this),
                        ("lpDDSCaps2", &lpDDSCaps2),
                        ("lpDirectDrawSurface7", &lpDirectDrawSurface7),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface7::GetAttachedSurface(
                machine,
                this,
                lpDDSCaps2,
                lpDirectDrawSurface7,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::GetAttachedSurface_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::GetAttachedSurface_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetCaps(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCAPS2 = <Option<&mut DDSCAPS2>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::GetCaps",
                    &[("this", &this), ("lpDDSCAPS2", &lpDDSCAPS2)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface7::GetCaps(machine, this, lpDDSCAPS2);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::GetCaps_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::GetCaps_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetDC(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpHDC = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::GetDC",
                    &[("this", &this), ("lpHDC", &lpHDC)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface7::GetDC(machine, this, lpHDC);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::GetDC_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::GetDC_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetPixelFormat(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let fmt = <Option<&mut DDPIXELFORMAT>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::GetPixelFormat",
                    &[("this", &this), ("fmt", &fmt)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface7::GetPixelFormat(machine, this, fmt);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::GetPixelFormat_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::GetPixelFormat_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetSurfaceDesc(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDesc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::GetSurfaceDesc",
                    &[("this", &this), ("lpDesc", &lpDesc)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface7::GetSurfaceDesc(machine, this, lpDesc);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::GetSurfaceDesc_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::GetSurfaceDesc_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Lock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let rect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let desc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, stack_args + 8u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, stack_args + 12u32);
            let unused = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::Lock",
                    &[
                        ("this", &this),
                        ("rect", &rect),
                        ("desc", &desc),
                        ("flags", &flags),
                        ("unused", &unused),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::ddraw::IDirectDrawSurface7::Lock(machine, this, rect, desc, flags, unused);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::Lock_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::Lock_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::Release",
                    &[("this", &this)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface7::Release(machine, this);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::Release_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::Release_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_ReleaseDC(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _this = <u32>::from_stack(mem, stack_args + 0u32);
            let _hDC = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::ReleaseDC",
                    &[("this", &_this), ("hDC", &_hDC)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface7::ReleaseDC(machine, _this, _hDC);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::ReleaseDC_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::ReleaseDC_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Restore(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::Restore",
                    &[("this", &_this)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface7::Restore(machine, _this);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::Restore_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::Restore_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_SetClipper(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let clipper = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::SetClipper",
                    &[("this", &this), ("clipper", &clipper)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface7::SetClipper(machine, this, clipper);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::SetClipper_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::SetClipper_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_SetPalette(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let palette = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::SetPalette",
                    &[("this", &this), ("palette", &palette)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface7::SetPalette(machine, this, palette);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::SetPalette_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::SetPalette_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Unlock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let rect = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw7") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::Unlock",
                    &[("this", &this), ("rect", &rect)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface7::Unlock(machine, this, rect);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface7::Unlock_pos.0,
                    winapi::ddraw::IDirectDrawSurface7::Unlock_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface_GetAttachedSurface(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCaps = <Option<&DDSCAPS>>::from_stack(mem, stack_args + 4u32);
            let lpDirectDrawSurface = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw1") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw1",
                    "IDirectDrawSurface::GetAttachedSurface",
                    &[
                        ("this", &this),
                        ("lpDDSCaps", &lpDDSCaps),
                        ("lpDirectDrawSurface", &lpDirectDrawSurface),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface::GetAttachedSurface(
                machine,
                this,
                lpDDSCaps,
                lpDirectDrawSurface,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface::GetAttachedSurface_pos.0,
                    winapi::ddraw::IDirectDrawSurface::GetAttachedSurface_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface_GetCaps(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCAPS = <Option<&mut DDSCAPS>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw1") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw1",
                    "IDirectDrawSurface::GetCaps",
                    &[("this", &this), ("lpDDSCAPS", &lpDDSCAPS)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface::GetCaps(machine, this, lpDDSCAPS);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface::GetCaps_pos.0,
                    winapi::ddraw::IDirectDrawSurface::GetCaps_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface_Lock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let rect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, stack_args + 8u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, stack_args + 12u32);
            let event = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw1") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw1",
                    "IDirectDrawSurface::Lock",
                    &[
                        ("this", &this),
                        ("rect", &rect),
                        ("desc", &desc),
                        ("flags", &flags),
                        ("event", &event),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::ddraw::IDirectDrawSurface::Lock(machine, this, rect, desc, flags, event);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface::Lock_pos.0,
                    winapi::ddraw::IDirectDrawSurface::Lock_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw1") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw1",
                    "IDirectDrawSurface::Release",
                    &[("this", &this)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface::Release(machine, this);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface::Release_pos.0,
                    winapi::ddraw::IDirectDrawSurface::Release_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDrawSurface_Unlock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let ptr = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw1") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw1",
                    "IDirectDrawSurface::Unlock",
                    &[("this", &this), ("ptr", &ptr)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDrawSurface::Unlock(machine, this, ptr);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDrawSurface::Unlock_pos.0,
                    winapi::ddraw::IDirectDrawSurface::Unlock_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw_CreateSurface(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let desc = <Option<&DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
            let lplpDDSurface = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw1") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw1",
                    "IDirectDraw::CreateSurface",
                    &[
                        ("this", &this),
                        ("desc", &desc),
                        ("lplpDDSurface", &lplpDDSurface),
                        ("pUnkOuter", &pUnkOuter),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDraw::CreateSurface(
                machine,
                this,
                desc,
                lplpDDSurface,
                pUnkOuter,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw::CreateSurface_pos.0,
                    winapi::ddraw::IDirectDraw::CreateSurface_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw_QueryInterface(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let riid = <Option<&GUID>>::from_stack(mem, stack_args + 4u32);
            let ppvObject = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw1") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw1",
                    "IDirectDraw::QueryInterface",
                    &[("this", &this), ("riid", &riid), ("ppvObject", &ppvObject)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDraw::QueryInterface(machine, this, riid, ppvObject);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw::QueryInterface_pos.0,
                    winapi::ddraw::IDirectDraw::QueryInterface_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw1") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw1",
                    "IDirectDraw::Release",
                    &[("this", &this)],
                ))
            } else {
                None
            };
            let result = winapi::ddraw::IDirectDraw::Release(machine, this);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw::Release_pos.0,
                    winapi::ddraw::IDirectDraw::Release_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectDraw_SetDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let width = <u32>::from_stack(mem, stack_args + 4u32);
            let height = <u32>::from_stack(mem, stack_args + 8u32);
            let bpp = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("ddraw/ddraw1") {
                Some(crate::trace::trace_begin(
                    "ddraw/ddraw1",
                    "IDirectDraw::SetDisplayMode",
                    &[
                        ("this", &this),
                        ("width", &width),
                        ("height", &height),
                        ("bpp", &bpp),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::ddraw::IDirectDraw::SetDisplayMode(machine, this, width, height, bpp);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ddraw::IDirectDraw::SetDisplayMode_pos.0,
                    winapi::ddraw::IDirectDraw::SetDisplayMode_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
    }
    const SHIMS: [Shim; 52usize] = [
        Shim {
            name: "DirectDrawCreate",
            func: Handler::Sync(wrappers::DirectDrawCreate),
        },
        Shim {
            name: "DirectDrawCreateClipper",
            func: Handler::Sync(wrappers::DirectDrawCreateClipper),
        },
        Shim {
            name: "DirectDrawCreateEx",
            func: Handler::Sync(wrappers::DirectDrawCreateEx),
        },
        Shim {
            name: "IDirectDraw2::CreateSurface",
            func: Handler::Sync(wrappers::IDirectDraw2_CreateSurface),
        },
        Shim {
            name: "IDirectDraw2::EnumDisplayModes",
            func: Handler::Async(wrappers::IDirectDraw2_EnumDisplayModes),
        },
        Shim {
            name: "IDirectDraw2::GetDisplayMode",
            func: Handler::Sync(wrappers::IDirectDraw2_GetDisplayMode),
        },
        Shim {
            name: "IDirectDraw2::QueryInterface",
            func: Handler::Sync(wrappers::IDirectDraw2_QueryInterface),
        },
        Shim {
            name: "IDirectDraw2::Release",
            func: Handler::Sync(wrappers::IDirectDraw2_Release),
        },
        Shim {
            name: "IDirectDraw2::SetDisplayMode",
            func: Handler::Sync(wrappers::IDirectDraw2_SetDisplayMode),
        },
        Shim {
            name: "IDirectDraw7::CreatePalette",
            func: Handler::Sync(wrappers::IDirectDraw7_CreatePalette),
        },
        Shim {
            name: "IDirectDraw7::CreateSurface",
            func: Handler::Sync(wrappers::IDirectDraw7_CreateSurface),
        },
        Shim {
            name: "IDirectDraw7::EnumDisplayModes",
            func: Handler::Async(wrappers::IDirectDraw7_EnumDisplayModes),
        },
        Shim {
            name: "IDirectDraw7::GetDisplayMode",
            func: Handler::Sync(wrappers::IDirectDraw7_GetDisplayMode),
        },
        Shim {
            name: "IDirectDraw7::Release",
            func: Handler::Sync(wrappers::IDirectDraw7_Release),
        },
        Shim {
            name: "IDirectDraw7::RestoreDisplayMode",
            func: Handler::Sync(wrappers::IDirectDraw7_RestoreDisplayMode),
        },
        Shim {
            name: "IDirectDraw7::SetCooperativeLevel",
            func: Handler::Sync(wrappers::IDirectDraw7_SetCooperativeLevel),
        },
        Shim {
            name: "IDirectDraw7::SetDisplayMode",
            func: Handler::Sync(wrappers::IDirectDraw7_SetDisplayMode),
        },
        Shim {
            name: "IDirectDraw7::WaitForVerticalBlank",
            func: Handler::Sync(wrappers::IDirectDraw7_WaitForVerticalBlank),
        },
        Shim {
            name: "IDirectDrawClipper::Release",
            func: Handler::Sync(wrappers::IDirectDrawClipper_Release),
        },
        Shim {
            name: "IDirectDrawClipper::SetHWnd",
            func: Handler::Sync(wrappers::IDirectDrawClipper_SetHWnd),
        },
        Shim {
            name: "IDirectDrawPalette::Release",
            func: Handler::Sync(wrappers::IDirectDrawPalette_Release),
        },
        Shim {
            name: "IDirectDrawPalette::SetEntries",
            func: Handler::Sync(wrappers::IDirectDrawPalette_SetEntries),
        },
        Shim {
            name: "IDirectDrawSurface2::GetAttachedSurface",
            func: Handler::Sync(wrappers::IDirectDrawSurface2_GetAttachedSurface),
        },
        Shim {
            name: "IDirectDrawSurface2::GetCaps",
            func: Handler::Sync(wrappers::IDirectDrawSurface2_GetCaps),
        },
        Shim {
            name: "IDirectDrawSurface2::GetSurfaceDesc",
            func: Handler::Sync(wrappers::IDirectDrawSurface2_GetSurfaceDesc),
        },
        Shim {
            name: "IDirectDrawSurface2::Lock",
            func: Handler::Sync(wrappers::IDirectDrawSurface2_Lock),
        },
        Shim {
            name: "IDirectDrawSurface2::Release",
            func: Handler::Sync(wrappers::IDirectDrawSurface2_Release),
        },
        Shim {
            name: "IDirectDrawSurface2::Unlock",
            func: Handler::Sync(wrappers::IDirectDrawSurface2_Unlock),
        },
        Shim {
            name: "IDirectDrawSurface7::Blt",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_Blt),
        },
        Shim {
            name: "IDirectDrawSurface7::BltFast",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_BltFast),
        },
        Shim {
            name: "IDirectDrawSurface7::Flip",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_Flip),
        },
        Shim {
            name: "IDirectDrawSurface7::GetAttachedSurface",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_GetAttachedSurface),
        },
        Shim {
            name: "IDirectDrawSurface7::GetCaps",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_GetCaps),
        },
        Shim {
            name: "IDirectDrawSurface7::GetDC",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_GetDC),
        },
        Shim {
            name: "IDirectDrawSurface7::GetPixelFormat",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_GetPixelFormat),
        },
        Shim {
            name: "IDirectDrawSurface7::GetSurfaceDesc",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_GetSurfaceDesc),
        },
        Shim {
            name: "IDirectDrawSurface7::Lock",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_Lock),
        },
        Shim {
            name: "IDirectDrawSurface7::Release",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_Release),
        },
        Shim {
            name: "IDirectDrawSurface7::ReleaseDC",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_ReleaseDC),
        },
        Shim {
            name: "IDirectDrawSurface7::Restore",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_Restore),
        },
        Shim {
            name: "IDirectDrawSurface7::SetClipper",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_SetClipper),
        },
        Shim {
            name: "IDirectDrawSurface7::SetPalette",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_SetPalette),
        },
        Shim {
            name: "IDirectDrawSurface7::Unlock",
            func: Handler::Sync(wrappers::IDirectDrawSurface7_Unlock),
        },
        Shim {
            name: "IDirectDrawSurface::GetAttachedSurface",
            func: Handler::Sync(wrappers::IDirectDrawSurface_GetAttachedSurface),
        },
        Shim {
            name: "IDirectDrawSurface::GetCaps",
            func: Handler::Sync(wrappers::IDirectDrawSurface_GetCaps),
        },
        Shim {
            name: "IDirectDrawSurface::Lock",
            func: Handler::Sync(wrappers::IDirectDrawSurface_Lock),
        },
        Shim {
            name: "IDirectDrawSurface::Release",
            func: Handler::Sync(wrappers::IDirectDrawSurface_Release),
        },
        Shim {
            name: "IDirectDrawSurface::Unlock",
            func: Handler::Sync(wrappers::IDirectDrawSurface_Unlock),
        },
        Shim {
            name: "IDirectDraw::CreateSurface",
            func: Handler::Sync(wrappers::IDirectDraw_CreateSurface),
        },
        Shim {
            name: "IDirectDraw::QueryInterface",
            func: Handler::Sync(wrappers::IDirectDraw_QueryInterface),
        },
        Shim {
            name: "IDirectDraw::Release",
            func: Handler::Sync(wrappers::IDirectDraw_Release),
        },
        Shim {
            name: "IDirectDraw::SetDisplayMode",
            func: Handler::Sync(wrappers::IDirectDraw_SetDisplayMode),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ddraw.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/ddraw.dll"),
    };
}
pub mod dinput {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::dinput::*;
        pub unsafe fn DirectInputCreateA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let version = <u32>::from_stack(mem, stack_args + 0u32);
            let ppDI = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("dinput") {
                Some(crate::trace::trace_begin(
                    "dinput",
                    "DirectInputCreateA",
                    &[
                        ("version", &version),
                        ("ppDI", &ppDI),
                        ("pUnkOuter", &pUnkOuter),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::dinput::DirectInputCreateA(machine, version, ppDI, pUnkOuter);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::dinput::DirectInputCreateA_pos.0,
                    winapi::dinput::DirectInputCreateA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
    }
    const SHIMS: [Shim; 1usize] = [Shim {
        name: "DirectInputCreateA",
        func: Handler::Sync(wrappers::DirectInputCreateA),
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "dinput.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/dinput.dll"),
    };
}
pub mod dsound {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::dsound::*;
        pub unsafe fn DirectSoundCreate(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, stack_args + 0u32);
            let ppDS = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("dsound") {
                Some(crate::trace::trace_begin(
                    "dsound",
                    "DirectSoundCreate",
                    &[
                        ("lpGuid", &lpGuid),
                        ("ppDS", &ppDS),
                        ("pUnkOuter", &pUnkOuter),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::dsound::DirectSoundCreate(machine, lpGuid, ppDS, pUnkOuter);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::dsound::DirectSoundCreate_pos.0,
                    winapi::dsound::DirectSoundCreate_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn DirectSoundEnumerateA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpDSEnumCallback = <u32>::from_stack(mem, stack_args + 0u32);
            let lpContext = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("dsound") {
                Some(crate::trace::trace_begin(
                    "dsound",
                    "DirectSoundEnumerateA",
                    &[
                        ("lpDSEnumCallback", &lpDSEnumCallback),
                        ("lpContext", &lpContext),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::dsound::DirectSoundEnumerateA(machine, lpDSEnumCallback, lpContext);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::dsound::DirectSoundEnumerateA_pos.0,
                    winapi::dsound::DirectSoundEnumerateA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_GetCurrentPosition(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpdwCurrentPlayCursor = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let lpdwCurrentWriteCursor = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("dsound") {
                Some(crate::trace::trace_begin(
                    "dsound",
                    "IDirectSoundBuffer::GetCurrentPosition",
                    &[
                        ("this", &this),
                        ("lpdwCurrentPlayCursor", &lpdwCurrentPlayCursor),
                        ("lpdwCurrentWriteCursor", &lpdwCurrentWriteCursor),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::dsound::IDirectSoundBuffer::GetCurrentPosition(
                machine,
                this,
                lpdwCurrentPlayCursor,
                lpdwCurrentWriteCursor,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::dsound::IDirectSoundBuffer::GetCurrentPosition_pos.0,
                    winapi::dsound::IDirectSoundBuffer::GetCurrentPosition_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_GetStatus(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpdwStatus = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("dsound") {
                Some(crate::trace::trace_begin(
                    "dsound",
                    "IDirectSoundBuffer::GetStatus",
                    &[("this", &this), ("lpdwStatus", &lpdwStatus)],
                ))
            } else {
                None
            };
            let result = winapi::dsound::IDirectSoundBuffer::GetStatus(machine, this, lpdwStatus);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::dsound::IDirectSoundBuffer::GetStatus_pos.0,
                    winapi::dsound::IDirectSoundBuffer::GetStatus_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_Lock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let dwWriteCursor = <u32>::from_stack(mem, stack_args + 4u32);
            let dwWriteBytes = <u32>::from_stack(mem, stack_args + 8u32);
            let lplpvAudioPtr1 = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpdwAudioBytes1 = <Option<&mut u32>>::from_stack(mem, stack_args + 16u32);
            let lplpvAudioPtr2 = <Option<&mut u32>>::from_stack(mem, stack_args + 20u32);
            let lpdwAudioBytes2 = <Option<&mut u32>>::from_stack(mem, stack_args + 24u32);
            let dwFlags = <Result<DSBLOCK, u32>>::from_stack(mem, stack_args + 28u32);
            let __trace_context = if crate::trace::enabled("dsound") {
                Some(crate::trace::trace_begin(
                    "dsound",
                    "IDirectSoundBuffer::Lock",
                    &[
                        ("this", &this),
                        ("dwWriteCursor", &dwWriteCursor),
                        ("dwWriteBytes", &dwWriteBytes),
                        ("lplpvAudioPtr1", &lplpvAudioPtr1),
                        ("lpdwAudioBytes1", &lpdwAudioBytes1),
                        ("lplpvAudioPtr2", &lplpvAudioPtr2),
                        ("lpdwAudioBytes2", &lpdwAudioBytes2),
                        ("dwFlags", &dwFlags),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::dsound::IDirectSoundBuffer::Lock(
                machine,
                this,
                dwWriteCursor,
                dwWriteBytes,
                lplpvAudioPtr1,
                lpdwAudioBytes1,
                lplpvAudioPtr2,
                lpdwAudioBytes2,
                dwFlags,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::dsound::IDirectSoundBuffer::Lock_pos.0,
                    winapi::dsound::IDirectSoundBuffer::Lock_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_Play(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let dwReserved1 = <u32>::from_stack(mem, stack_args + 4u32);
            let dwReserved2 = <u32>::from_stack(mem, stack_args + 8u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("dsound") {
                Some(crate::trace::trace_begin(
                    "dsound",
                    "IDirectSoundBuffer::Play",
                    &[
                        ("this", &this),
                        ("dwReserved1", &dwReserved1),
                        ("dwReserved2", &dwReserved2),
                        ("dwFlags", &dwFlags),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::dsound::IDirectSoundBuffer::Play(
                machine,
                this,
                dwReserved1,
                dwReserved2,
                dwFlags,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::dsound::IDirectSoundBuffer::Play_pos.0,
                    winapi::dsound::IDirectSoundBuffer::Play_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("dsound") {
                Some(crate::trace::trace_begin(
                    "dsound",
                    "IDirectSoundBuffer::Release",
                    &[("this", &this)],
                ))
            } else {
                None
            };
            let result = winapi::dsound::IDirectSoundBuffer::Release(machine, this);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::dsound::IDirectSoundBuffer::Release_pos.0,
                    winapi::dsound::IDirectSoundBuffer::Release_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_SetFormat(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpcfxFormat = <Option<&WAVEFORMATEX>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("dsound") {
                Some(crate::trace::trace_begin(
                    "dsound",
                    "IDirectSoundBuffer::SetFormat",
                    &[("this", &this), ("lpcfxFormat", &lpcfxFormat)],
                ))
            } else {
                None
            };
            let result = winapi::dsound::IDirectSoundBuffer::SetFormat(machine, this, lpcfxFormat);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::dsound::IDirectSoundBuffer::SetFormat_pos.0,
                    winapi::dsound::IDirectSoundBuffer::SetFormat_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_Unlock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpvAudioPtr1 = <u32>::from_stack(mem, stack_args + 4u32);
            let dwAudioBytes1 = <u32>::from_stack(mem, stack_args + 8u32);
            let lpvAudioPtr2 = <u32>::from_stack(mem, stack_args + 12u32);
            let dwAudioBytes2 = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("dsound") {
                Some(crate::trace::trace_begin(
                    "dsound",
                    "IDirectSoundBuffer::Unlock",
                    &[
                        ("this", &this),
                        ("lpvAudioPtr1", &lpvAudioPtr1),
                        ("dwAudioBytes1", &dwAudioBytes1),
                        ("lpvAudioPtr2", &lpvAudioPtr2),
                        ("dwAudioBytes2", &dwAudioBytes2),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::dsound::IDirectSoundBuffer::Unlock(
                machine,
                this,
                lpvAudioPtr1,
                dwAudioBytes1,
                lpvAudioPtr2,
                dwAudioBytes2,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::dsound::IDirectSoundBuffer::Unlock_pos.0,
                    winapi::dsound::IDirectSoundBuffer::Unlock_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectSound_CreateSoundBuffer(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpcDSBufferDesc = <Option<&DSBUFFERDESC>>::from_stack(mem, stack_args + 4u32);
            let lplpDirectSoundBuffer = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("dsound") {
                Some(crate::trace::trace_begin(
                    "dsound",
                    "IDirectSound::CreateSoundBuffer",
                    &[
                        ("this", &this),
                        ("lpcDSBufferDesc", &lpcDSBufferDesc),
                        ("lplpDirectSoundBuffer", &lplpDirectSoundBuffer),
                        ("pUnkOuter", &pUnkOuter),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::dsound::IDirectSound::CreateSoundBuffer(
                machine,
                this,
                lpcDSBufferDesc,
                lplpDirectSoundBuffer,
                pUnkOuter,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::dsound::IDirectSound::CreateSoundBuffer_pos.0,
                    winapi::dsound::IDirectSound::CreateSoundBuffer_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectSound_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("dsound") {
                Some(crate::trace::trace_begin(
                    "dsound",
                    "IDirectSound::Release",
                    &[("this", &this)],
                ))
            } else {
                None
            };
            let result = winapi::dsound::IDirectSound::Release(machine, this);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::dsound::IDirectSound::Release_pos.0,
                    winapi::dsound::IDirectSound::Release_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IDirectSound_SetCooperativeLevel(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let hwnd = <u32>::from_stack(mem, stack_args + 4u32);
            let dwLevel = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("dsound") {
                Some(crate::trace::trace_begin(
                    "dsound",
                    "IDirectSound::SetCooperativeLevel",
                    &[("this", &this), ("hwnd", &hwnd), ("dwLevel", &dwLevel)],
                ))
            } else {
                None
            };
            let result =
                winapi::dsound::IDirectSound::SetCooperativeLevel(machine, this, hwnd, dwLevel);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::dsound::IDirectSound::SetCooperativeLevel_pos.0,
                    winapi::dsound::IDirectSound::SetCooperativeLevel_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
    }
    const SHIMS: [Shim; 12usize] = [
        Shim {
            name: "DirectSoundCreate",
            func: Handler::Sync(wrappers::DirectSoundCreate),
        },
        Shim {
            name: "DirectSoundEnumerateA",
            func: Handler::Sync(wrappers::DirectSoundEnumerateA),
        },
        Shim {
            name: "IDirectSoundBuffer::GetCurrentPosition",
            func: Handler::Sync(wrappers::IDirectSoundBuffer_GetCurrentPosition),
        },
        Shim {
            name: "IDirectSoundBuffer::GetStatus",
            func: Handler::Sync(wrappers::IDirectSoundBuffer_GetStatus),
        },
        Shim {
            name: "IDirectSoundBuffer::Lock",
            func: Handler::Sync(wrappers::IDirectSoundBuffer_Lock),
        },
        Shim {
            name: "IDirectSoundBuffer::Play",
            func: Handler::Sync(wrappers::IDirectSoundBuffer_Play),
        },
        Shim {
            name: "IDirectSoundBuffer::Release",
            func: Handler::Sync(wrappers::IDirectSoundBuffer_Release),
        },
        Shim {
            name: "IDirectSoundBuffer::SetFormat",
            func: Handler::Sync(wrappers::IDirectSoundBuffer_SetFormat),
        },
        Shim {
            name: "IDirectSoundBuffer::Unlock",
            func: Handler::Sync(wrappers::IDirectSoundBuffer_Unlock),
        },
        Shim {
            name: "IDirectSound::CreateSoundBuffer",
            func: Handler::Sync(wrappers::IDirectSound_CreateSoundBuffer),
        },
        Shim {
            name: "IDirectSound::Release",
            func: Handler::Sync(wrappers::IDirectSound_Release),
        },
        Shim {
            name: "IDirectSound::SetCooperativeLevel",
            func: Handler::Sync(wrappers::IDirectSound_SetCooperativeLevel),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "dsound.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/dsound.dll"),
    };
}
pub mod gdi32 {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::gdi32::*;
        pub unsafe fn BitBlt(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdcDst = <HDC>::from_stack(mem, stack_args + 0u32);
            let xDst = <i32>::from_stack(mem, stack_args + 4u32);
            let yDst = <i32>::from_stack(mem, stack_args + 8u32);
            let w = <i32>::from_stack(mem, stack_args + 12u32);
            let h = <i32>::from_stack(mem, stack_args + 16u32);
            let hdcSrc = <HDC>::from_stack(mem, stack_args + 20u32);
            let xSrc = <i32>::from_stack(mem, stack_args + 24u32);
            let ySrc = <i32>::from_stack(mem, stack_args + 28u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, stack_args + 32u32);
            let __trace_context = if crate::trace::enabled("gdi32/bitmap") {
                Some(crate::trace::trace_begin(
                    "gdi32/bitmap",
                    "BitBlt",
                    &[
                        ("hdcDst", &hdcDst),
                        ("xDst", &xDst),
                        ("yDst", &yDst),
                        ("w", &w),
                        ("h", &h),
                        ("hdcSrc", &hdcSrc),
                        ("xSrc", &xSrc),
                        ("ySrc", &ySrc),
                        ("rop", &rop),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::gdi32::BitBlt(machine, hdcDst, xDst, yDst, w, h, hdcSrc, xSrc, ySrc, rop);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::BitBlt_pos.0,
                    winapi::gdi32::BitBlt_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateBitmap(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nWidth = <u32>::from_stack(mem, stack_args + 0u32);
            let nHeight = <u32>::from_stack(mem, stack_args + 4u32);
            let nPlanes = <u32>::from_stack(mem, stack_args + 8u32);
            let nBitCount = <u32>::from_stack(mem, stack_args + 12u32);
            let lpBits = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("gdi32/bitmap") {
                Some(crate::trace::trace_begin(
                    "gdi32/bitmap",
                    "CreateBitmap",
                    &[
                        ("nWidth", &nWidth),
                        ("nHeight", &nHeight),
                        ("nPlanes", &nPlanes),
                        ("nBitCount", &nBitCount),
                        ("lpBits", &lpBits),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::gdi32::CreateBitmap(machine, nWidth, nHeight, nPlanes, nBitCount, lpBits);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::CreateBitmap_pos.0,
                    winapi::gdi32::CreateBitmap_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateCompatibleBitmap(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let cx = <u32>::from_stack(mem, stack_args + 4u32);
            let cy = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("gdi32/bitmap") {
                Some(crate::trace::trace_begin(
                    "gdi32/bitmap",
                    "CreateCompatibleBitmap",
                    &[("hdc", &hdc), ("cx", &cx), ("cy", &cy)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::CreateCompatibleBitmap(machine, hdc, cx, cy);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::CreateCompatibleBitmap_pos.0,
                    winapi::gdi32::CreateCompatibleBitmap_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateCompatibleDC(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("gdi32/dc") {
                Some(crate::trace::trace_begin(
                    "gdi32/dc",
                    "CreateCompatibleDC",
                    &[("hdc", &hdc)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::CreateCompatibleDC(machine, hdc);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::CreateCompatibleDC_pos.0,
                    winapi::gdi32::CreateCompatibleDC_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateDIBSection(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let pbmi = <Option<&BITMAPINFOHEADER>>::from_stack(mem, stack_args + 4u32);
            let usage = <u32>::from_stack(mem, stack_args + 8u32);
            let ppvBits = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let hSection = <u32>::from_stack(mem, stack_args + 16u32);
            let offset = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("gdi32/bitmap") {
                Some(crate::trace::trace_begin(
                    "gdi32/bitmap",
                    "CreateDIBSection",
                    &[
                        ("hdc", &hdc),
                        ("pbmi", &pbmi),
                        ("usage", &usage),
                        ("ppvBits", &ppvBits),
                        ("hSection", &hSection),
                        ("offset", &offset),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::CreateDIBSection(
                machine, hdc, pbmi, usage, ppvBits, hSection, offset,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::CreateDIBSection_pos.0,
                    winapi::gdi32::CreateDIBSection_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateDIBitmap(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let pbmih = <Option<&mut BITMAPINFOHEADER>>::from_stack(mem, stack_args + 4u32);
            let flInit = <u32>::from_stack(mem, stack_args + 8u32);
            let pjBits = <Option<&mut u8>>::from_stack(mem, stack_args + 12u32);
            let pbmi = <Option<&mut BITMAPINFO>>::from_stack(mem, stack_args + 16u32);
            let iUsage = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("gdi32/bitmap") {
                Some(crate::trace::trace_begin(
                    "gdi32/bitmap",
                    "CreateDIBitmap",
                    &[
                        ("hdc", &hdc),
                        ("pbmih", &pbmih),
                        ("flInit", &flInit),
                        ("pjBits", &pjBits),
                        ("pbmi", &pbmi),
                        ("iUsage", &iUsage),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::gdi32::CreateDIBitmap(machine, hdc, pbmih, flInit, pjBits, pbmi, iUsage);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::CreateDIBitmap_pos.0,
                    winapi::gdi32::CreateDIBitmap_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateFontA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let cHeight = <i32>::from_stack(mem, stack_args + 0u32);
            let cWidth = <i32>::from_stack(mem, stack_args + 4u32);
            let cEscapement = <i32>::from_stack(mem, stack_args + 8u32);
            let cOrientation = <i32>::from_stack(mem, stack_args + 12u32);
            let cWeight = <u32>::from_stack(mem, stack_args + 16u32);
            let bItalic = <u32>::from_stack(mem, stack_args + 20u32);
            let bUnderline = <u32>::from_stack(mem, stack_args + 24u32);
            let bStrikeOut = <u32>::from_stack(mem, stack_args + 28u32);
            let iCharSet = <u32>::from_stack(mem, stack_args + 32u32);
            let iOutPrecision = <u32>::from_stack(mem, stack_args + 36u32);
            let iClipPrecision = <u32>::from_stack(mem, stack_args + 40u32);
            let iQuality = <u32>::from_stack(mem, stack_args + 44u32);
            let iPitchAndFamily = <u32>::from_stack(mem, stack_args + 48u32);
            let pszFaceName = <Option<&str>>::from_stack(mem, stack_args + 52u32);
            let __trace_context = if crate::trace::enabled("gdi32/text") {
                Some(crate::trace::trace_begin(
                    "gdi32/text",
                    "CreateFontA",
                    &[
                        ("cHeight", &cHeight),
                        ("cWidth", &cWidth),
                        ("cEscapement", &cEscapement),
                        ("cOrientation", &cOrientation),
                        ("cWeight", &cWeight),
                        ("bItalic", &bItalic),
                        ("bUnderline", &bUnderline),
                        ("bStrikeOut", &bStrikeOut),
                        ("iCharSet", &iCharSet),
                        ("iOutPrecision", &iOutPrecision),
                        ("iClipPrecision", &iClipPrecision),
                        ("iQuality", &iQuality),
                        ("iPitchAndFamily", &iPitchAndFamily),
                        ("pszFaceName", &pszFaceName),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::CreateFontA(
                machine,
                cHeight,
                cWidth,
                cEscapement,
                cOrientation,
                cWeight,
                bItalic,
                bUnderline,
                bStrikeOut,
                iCharSet,
                iOutPrecision,
                iClipPrecision,
                iQuality,
                iPitchAndFamily,
                pszFaceName,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::CreateFontA_pos.0,
                    winapi::gdi32::CreateFontA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreatePalette(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let plpal = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("gdi32/palette") {
                Some(crate::trace::trace_begin(
                    "gdi32/palette",
                    "CreatePalette",
                    &[("plpal", &plpal)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::CreatePalette(machine, plpal);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::CreatePalette_pos.0,
                    winapi::gdi32::CreatePalette_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreatePen(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let iStyle = <Result<PS, u32>>::from_stack(mem, stack_args + 0u32);
            let cWidth = <u32>::from_stack(mem, stack_args + 4u32);
            let color = <COLORREF>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("gdi32/draw") {
                Some(crate::trace::trace_begin(
                    "gdi32/draw",
                    "CreatePen",
                    &[("iStyle", &iStyle), ("cWidth", &cWidth), ("color", &color)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::CreatePen(machine, iStyle, cWidth, color);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::CreatePen_pos.0,
                    winapi::gdi32::CreatePen_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateSolidBrush(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let color = <COLORREF>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("gdi32/draw") {
                Some(crate::trace::trace_begin(
                    "gdi32/draw",
                    "CreateSolidBrush",
                    &[("color", &color)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::CreateSolidBrush(machine, color);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::CreateSolidBrush_pos.0,
                    winapi::gdi32::CreateSolidBrush_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn DeleteDC(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("gdi32/dc") {
                Some(crate::trace::trace_begin(
                    "gdi32/dc",
                    "DeleteDC",
                    &[("hdc", &hdc)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::DeleteDC(machine, hdc);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::DeleteDC_pos.0,
                    winapi::gdi32::DeleteDC_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn DeleteObject(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let handle = <HGDIOBJ>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("gdi32/object") {
                Some(crate::trace::trace_begin(
                    "gdi32/object",
                    "DeleteObject",
                    &[("handle", &handle)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::DeleteObject(machine, handle);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::DeleteObject_pos.0,
                    winapi::gdi32::DeleteObject_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetDCOrgEx(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpPoint = <Option<&mut POINT>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("gdi32/dc") {
                Some(crate::trace::trace_begin(
                    "gdi32/dc",
                    "GetDCOrgEx",
                    &[("hdc", &hdc), ("lpPoint", &lpPoint)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::GetDCOrgEx(machine, hdc, lpPoint);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::GetDCOrgEx_pos.0,
                    winapi::gdi32::GetDCOrgEx_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetDIBits(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let hbm = <HBITMAP>::from_stack(mem, stack_args + 4u32);
            let start = <u32>::from_stack(mem, stack_args + 8u32);
            let cLines = <u32>::from_stack(mem, stack_args + 12u32);
            let lpvBits = <Option<&mut u8>>::from_stack(mem, stack_args + 16u32);
            let lpbmi = <Option<&mut BITMAPINFO>>::from_stack(mem, stack_args + 20u32);
            let usage = <u32>::from_stack(mem, stack_args + 24u32);
            let __trace_context = if crate::trace::enabled("gdi32/bitmap") {
                Some(crate::trace::trace_begin(
                    "gdi32/bitmap",
                    "GetDIBits",
                    &[
                        ("hdc", &hdc),
                        ("hbm", &hbm),
                        ("start", &start),
                        ("cLines", &cLines),
                        ("lpvBits", &lpvBits),
                        ("lpbmi", &lpbmi),
                        ("usage", &usage),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::gdi32::GetDIBits(machine, hdc, hbm, start, cLines, lpvBits, lpbmi, usage);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::GetDIBits_pos.0,
                    winapi::gdi32::GetDIBits_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetDeviceCaps(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let index = <Result<GetDeviceCapsArg, u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("gdi32/dc") {
                Some(crate::trace::trace_begin(
                    "gdi32/dc",
                    "GetDeviceCaps",
                    &[("hdc", &hdc), ("index", &index)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::GetDeviceCaps(machine, hdc, index);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::GetDeviceCaps_pos.0,
                    winapi::gdi32::GetDeviceCaps_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetLayout(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("gdi32/dc") {
                Some(crate::trace::trace_begin(
                    "gdi32/dc",
                    "GetLayout",
                    &[("hdc", &hdc)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::GetLayout(machine, hdc);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::GetLayout_pos.0,
                    winapi::gdi32::GetLayout_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetObjectA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let handle = <HGDIOBJ>::from_stack(mem, stack_args + 0u32);
            let bytes = <u32>::from_stack(mem, stack_args + 4u32);
            let out = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("gdi32/object") {
                Some(crate::trace::trace_begin(
                    "gdi32/object",
                    "GetObjectA",
                    &[("handle", &handle), ("bytes", &bytes), ("out", &out)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::GetObjectA(machine, handle, bytes, out);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::GetObjectA_pos.0,
                    winapi::gdi32::GetObjectA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetPaletteEntries(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hpal = <HPALETTE>::from_stack(mem, stack_args + 0u32);
            let iStart = <u32>::from_stack(mem, stack_args + 4u32);
            let cEntries = <u32>::from_stack(mem, stack_args + 8u32);
            let pPalEntries = <Option<&mut PALETTEENTRY>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("gdi32/palette") {
                Some(crate::trace::trace_begin(
                    "gdi32/palette",
                    "GetPaletteEntries",
                    &[
                        ("hpal", &hpal),
                        ("iStart", &iStart),
                        ("cEntries", &cEntries),
                        ("pPalEntries", &pPalEntries),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::gdi32::GetPaletteEntries(machine, hpal, iStart, cEntries, pPalEntries);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::GetPaletteEntries_pos.0,
                    winapi::gdi32::GetPaletteEntries_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetPixel(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("gdi32/draw") {
                Some(crate::trace::trace_begin(
                    "gdi32/draw",
                    "GetPixel",
                    &[("hdc", &hdc), ("x", &x), ("y", &y)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::GetPixel(machine, hdc, x, y);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::GetPixel_pos.0,
                    winapi::gdi32::GetPixel_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetStockObject(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let i = <Result<GetStockObjectArg, u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("gdi32/object") {
                Some(crate::trace::trace_begin(
                    "gdi32/object",
                    "GetStockObject",
                    &[("i", &i)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::GetStockObject(machine, i);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::GetStockObject_pos.0,
                    winapi::gdi32::GetStockObject_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetSystemPaletteEntries(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let iStart = <u32>::from_stack(mem, stack_args + 4u32);
            let cEntries = <u32>::from_stack(mem, stack_args + 8u32);
            let pPalEntries = <Option<&mut PALETTEENTRY>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("gdi32/palette") {
                Some(crate::trace::trace_begin(
                    "gdi32/palette",
                    "GetSystemPaletteEntries",
                    &[
                        ("hdc", &hdc),
                        ("iStart", &iStart),
                        ("cEntries", &cEntries),
                        ("pPalEntries", &pPalEntries),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::gdi32::GetSystemPaletteEntries(machine, hdc, iStart, cEntries, pPalEntries);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::GetSystemPaletteEntries_pos.0,
                    winapi::gdi32::GetSystemPaletteEntries_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetTextExtentPoint32A(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let c = <i32>::from_stack(mem, stack_args + 8u32);
            let psizl = <Option<&mut SIZE>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("gdi32/text") {
                Some(crate::trace::trace_begin(
                    "gdi32/text",
                    "GetTextExtentPoint32A",
                    &[
                        ("hdc", &hdc),
                        ("lpString", &lpString),
                        ("c", &c),
                        ("psizl", &psizl),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::GetTextExtentPoint32A(machine, hdc, lpString, c, psizl);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::GetTextExtentPoint32A_pos.0,
                    winapi::gdi32::GetTextExtentPoint32A_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetTextExtentPoint32W(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let c = <i32>::from_stack(mem, stack_args + 8u32);
            let psizl = <Option<&mut SIZE>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("gdi32/text") {
                Some(crate::trace::trace_begin(
                    "gdi32/text",
                    "GetTextExtentPoint32W",
                    &[
                        ("hdc", &hdc),
                        ("lpString", &lpString),
                        ("c", &c),
                        ("psizl", &psizl),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::GetTextExtentPoint32W(machine, hdc, lpString, c, psizl);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::GetTextExtentPoint32W_pos.0,
                    winapi::gdi32::GetTextExtentPoint32W_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetTextMetricsA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lptm = <Option<&mut TEXTMETRICA>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("gdi32/text") {
                Some(crate::trace::trace_begin(
                    "gdi32/text",
                    "GetTextMetricsA",
                    &[("hdc", &hdc), ("lptm", &lptm)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::GetTextMetricsA(machine, hdc, lptm);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::GetTextMetricsA_pos.0,
                    winapi::gdi32::GetTextMetricsA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetTextMetricsW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lptm = <Option<&mut TEXTMETRICW>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("gdi32/text") {
                Some(crate::trace::trace_begin(
                    "gdi32/text",
                    "GetTextMetricsW",
                    &[("hdc", &hdc), ("lptm", &lptm)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::GetTextMetricsW(machine, hdc, lptm);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::GetTextMetricsW_pos.0,
                    winapi::gdi32::GetTextMetricsW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LineDDA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let xStart = <i32>::from_stack(mem, stack_args + 0u32);
            let yStart = <i32>::from_stack(mem, stack_args + 4u32);
            let xEnd = <i32>::from_stack(mem, stack_args + 8u32);
            let yEnd = <i32>::from_stack(mem, stack_args + 12u32);
            let lpProc = <u32>::from_stack(mem, stack_args + 16u32);
            let data = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("gdi32/draw") {
                Some(crate::trace::trace_begin(
                    "gdi32/draw",
                    "LineDDA",
                    &[
                        ("xStart", &xStart),
                        ("yStart", &yStart),
                        ("xEnd", &xEnd),
                        ("yEnd", &yEnd),
                        ("lpProc", &lpProc),
                        ("data", &data),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::LineDDA(machine, xStart, yStart, xEnd, yEnd, lpProc, data);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::LineDDA_pos.0,
                    winapi::gdi32::LineDDA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LineTo(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <i32>::from_stack(mem, stack_args + 4u32);
            let y = <i32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("gdi32/draw") {
                Some(crate::trace::trace_begin(
                    "gdi32/draw",
                    "LineTo",
                    &[("hdc", &hdc), ("x", &x), ("y", &y)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::LineTo(machine, hdc, x, y);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::LineTo_pos.0,
                    winapi::gdi32::LineTo_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn MoveToEx(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <i32>::from_stack(mem, stack_args + 4u32);
            let y = <i32>::from_stack(mem, stack_args + 8u32);
            let lppt = <Option<&mut POINT>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("gdi32/draw") {
                Some(crate::trace::trace_begin(
                    "gdi32/draw",
                    "MoveToEx",
                    &[("hdc", &hdc), ("x", &x), ("y", &y), ("lppt", &lppt)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::MoveToEx(machine, hdc, x, y, lppt);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::MoveToEx_pos.0,
                    winapi::gdi32::MoveToEx_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn PatBlt(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <i32>::from_stack(mem, stack_args + 4u32);
            let y = <i32>::from_stack(mem, stack_args + 8u32);
            let w = <i32>::from_stack(mem, stack_args + 12u32);
            let h = <i32>::from_stack(mem, stack_args + 16u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("gdi32/bitmap") {
                Some(crate::trace::trace_begin(
                    "gdi32/bitmap",
                    "PatBlt",
                    &[
                        ("hdc", &hdc),
                        ("x", &x),
                        ("y", &y),
                        ("w", &w),
                        ("h", &h),
                        ("rop", &rop),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::PatBlt(machine, hdc, x, y, w, h, rop);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::PatBlt_pos.0,
                    winapi::gdi32::PatBlt_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn PtVisible(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <i32>::from_stack(mem, stack_args + 4u32);
            let y = <i32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("gdi32/draw") {
                Some(crate::trace::trace_begin(
                    "gdi32/draw",
                    "PtVisible",
                    &[("hdc", &hdc), ("x", &x), ("y", &y)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::PtVisible(machine, hdc, x, y);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::PtVisible_pos.0,
                    winapi::gdi32::PtVisible_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RealizePalette(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("gdi32/palette") {
                Some(crate::trace::trace_begin(
                    "gdi32/palette",
                    "RealizePalette",
                    &[("hdc", &hdc)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::RealizePalette(machine, hdc);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::RealizePalette_pos.0,
                    winapi::gdi32::RealizePalette_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SelectObject(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let hGdiObj = <HGDIOBJ>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("gdi32/object") {
                Some(crate::trace::trace_begin(
                    "gdi32/object",
                    "SelectObject",
                    &[("hdc", &hdc), ("hGdiObj", &hGdiObj)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::SelectObject(machine, hdc, hGdiObj);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::SelectObject_pos.0,
                    winapi::gdi32::SelectObject_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SelectPalette(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let hPal = <HPALETTE>::from_stack(mem, stack_args + 4u32);
            let bForceBkgd = <bool>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("gdi32/palette") {
                Some(crate::trace::trace_begin(
                    "gdi32/palette",
                    "SelectPalette",
                    &[("hdc", &hdc), ("hPal", &hPal), ("bForceBkgd", &bForceBkgd)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::SelectPalette(machine, hdc, hPal, bForceBkgd);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::SelectPalette_pos.0,
                    winapi::gdi32::SelectPalette_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetBkColor(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let color = <COLORREF>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("gdi32/draw") {
                Some(crate::trace::trace_begin(
                    "gdi32/draw",
                    "SetBkColor",
                    &[("hdc", &hdc), ("color", &color)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::SetBkColor(machine, hdc, color);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::SetBkColor_pos.0,
                    winapi::gdi32::SetBkColor_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetBkMode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let mode = <i32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("gdi32/draw") {
                Some(crate::trace::trace_begin(
                    "gdi32/draw",
                    "SetBkMode",
                    &[("hdc", &hdc), ("mode", &mode)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::SetBkMode(machine, hdc, mode);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::SetBkMode_pos.0,
                    winapi::gdi32::SetBkMode_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetBrushOrgEx(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <i32>::from_stack(mem, stack_args + 4u32);
            let y = <i32>::from_stack(mem, stack_args + 8u32);
            let lppt = <Option<&mut POINT>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("gdi32/draw") {
                Some(crate::trace::trace_begin(
                    "gdi32/draw",
                    "SetBrushOrgEx",
                    &[("hdc", &hdc), ("x", &x), ("y", &y), ("lppt", &lppt)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::SetBrushOrgEx(machine, hdc, x, y, lppt);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::SetBrushOrgEx_pos.0,
                    winapi::gdi32::SetBrushOrgEx_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetDIBitsToDevice(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let xDst = <i32>::from_stack(mem, stack_args + 4u32);
            let yDst = <i32>::from_stack(mem, stack_args + 8u32);
            let w = <i32>::from_stack(mem, stack_args + 12u32);
            let h = <i32>::from_stack(mem, stack_args + 16u32);
            let xSrc = <i32>::from_stack(mem, stack_args + 20u32);
            let ySrc = <i32>::from_stack(mem, stack_args + 24u32);
            let StartScan = <u32>::from_stack(mem, stack_args + 28u32);
            let cLines = <u32>::from_stack(mem, stack_args + 32u32);
            let lpBits = <u32>::from_stack(mem, stack_args + 36u32);
            let lpBmi = <u32>::from_stack(mem, stack_args + 40u32);
            let iUsage = <u32>::from_stack(mem, stack_args + 44u32);
            let __trace_context = if crate::trace::enabled("gdi32/bitmap") {
                Some(crate::trace::trace_begin(
                    "gdi32/bitmap",
                    "SetDIBitsToDevice",
                    &[
                        ("hdc", &hdc),
                        ("xDst", &xDst),
                        ("yDst", &yDst),
                        ("w", &w),
                        ("h", &h),
                        ("xSrc", &xSrc),
                        ("ySrc", &ySrc),
                        ("StartScan", &StartScan),
                        ("cLines", &cLines),
                        ("lpBits", &lpBits),
                        ("lpBmi", &lpBmi),
                        ("iUsage", &iUsage),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::SetDIBitsToDevice(
                machine, hdc, xDst, yDst, w, h, xSrc, ySrc, StartScan, cLines, lpBits, lpBmi,
                iUsage,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::SetDIBitsToDevice_pos.0,
                    winapi::gdi32::SetDIBitsToDevice_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetLayout(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let l = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("gdi32/dc") {
                Some(crate::trace::trace_begin(
                    "gdi32/dc",
                    "SetLayout",
                    &[("hdc", &hdc), ("l", &l)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::SetLayout(machine, hdc, l);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::SetLayout_pos.0,
                    winapi::gdi32::SetLayout_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetPaletteEntries(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hpal = <HPALETTE>::from_stack(mem, stack_args + 0u32);
            let iStart = <u32>::from_stack(mem, stack_args + 4u32);
            let cEntries = <u32>::from_stack(mem, stack_args + 8u32);
            let pPalEntries = <Option<&mut PALETTEENTRY>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("gdi32/palette") {
                Some(crate::trace::trace_begin(
                    "gdi32/palette",
                    "SetPaletteEntries",
                    &[
                        ("hpal", &hpal),
                        ("iStart", &iStart),
                        ("cEntries", &cEntries),
                        ("pPalEntries", &pPalEntries),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::gdi32::SetPaletteEntries(machine, hpal, iStart, cEntries, pPalEntries);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::SetPaletteEntries_pos.0,
                    winapi::gdi32::SetPaletteEntries_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetPixel(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let color = <COLORREF>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("gdi32/draw") {
                Some(crate::trace::trace_begin(
                    "gdi32/draw",
                    "SetPixel",
                    &[("hdc", &hdc), ("x", &x), ("y", &y), ("color", &color)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::SetPixel(machine, hdc, x, y, color);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::SetPixel_pos.0,
                    winapi::gdi32::SetPixel_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetROP2(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let rop2 = <Result<R2, u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("gdi32/draw") {
                Some(crate::trace::trace_begin(
                    "gdi32/draw",
                    "SetROP2",
                    &[("hdc", &hdc), ("rop2", &rop2)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::SetROP2(machine, hdc, rop2);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::SetROP2_pos.0,
                    winapi::gdi32::SetROP2_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetTextAlign(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let fMode = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("gdi32/text") {
                Some(crate::trace::trace_begin(
                    "gdi32/text",
                    "SetTextAlign",
                    &[("hdc", &hdc), ("fMode", &fMode)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::SetTextAlign(machine, hdc, fMode);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::SetTextAlign_pos.0,
                    winapi::gdi32::SetTextAlign_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetTextColor(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let color = <COLORREF>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("gdi32/text") {
                Some(crate::trace::trace_begin(
                    "gdi32/text",
                    "SetTextColor",
                    &[("hdc", &hdc), ("color", &color)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::SetTextColor(machine, hdc, color);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::SetTextColor_pos.0,
                    winapi::gdi32::SetTextColor_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn StretchBlt(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdcDst = <HDC>::from_stack(mem, stack_args + 0u32);
            let xDst = <i32>::from_stack(mem, stack_args + 4u32);
            let yDst = <i32>::from_stack(mem, stack_args + 8u32);
            let wDst = <i32>::from_stack(mem, stack_args + 12u32);
            let hDst = <i32>::from_stack(mem, stack_args + 16u32);
            let hdcSrc = <HDC>::from_stack(mem, stack_args + 20u32);
            let xSrc = <i32>::from_stack(mem, stack_args + 24u32);
            let ySrc = <i32>::from_stack(mem, stack_args + 28u32);
            let wSrc = <i32>::from_stack(mem, stack_args + 32u32);
            let hSrc = <i32>::from_stack(mem, stack_args + 36u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, stack_args + 40u32);
            let __trace_context = if crate::trace::enabled("gdi32/bitmap") {
                Some(crate::trace::trace_begin(
                    "gdi32/bitmap",
                    "StretchBlt",
                    &[
                        ("hdcDst", &hdcDst),
                        ("xDst", &xDst),
                        ("yDst", &yDst),
                        ("wDst", &wDst),
                        ("hDst", &hDst),
                        ("hdcSrc", &hdcSrc),
                        ("xSrc", &xSrc),
                        ("ySrc", &ySrc),
                        ("wSrc", &wSrc),
                        ("hSrc", &hSrc),
                        ("rop", &rop),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::StretchBlt(
                machine, hdcDst, xDst, yDst, wDst, hDst, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::StretchBlt_pos.0,
                    winapi::gdi32::StretchBlt_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn StretchDIBits(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let xDst = <i32>::from_stack(mem, stack_args + 4u32);
            let yDst = <i32>::from_stack(mem, stack_args + 8u32);
            let wDst = <i32>::from_stack(mem, stack_args + 12u32);
            let hDst = <i32>::from_stack(mem, stack_args + 16u32);
            let xSrc = <i32>::from_stack(mem, stack_args + 20u32);
            let ySrc = <i32>::from_stack(mem, stack_args + 24u32);
            let wSrc = <i32>::from_stack(mem, stack_args + 28u32);
            let hSrc = <i32>::from_stack(mem, stack_args + 32u32);
            let lpBits = <u32>::from_stack(mem, stack_args + 36u32);
            let lpBmi = <u32>::from_stack(mem, stack_args + 40u32);
            let iUsage = <u32>::from_stack(mem, stack_args + 44u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, stack_args + 48u32);
            let __trace_context = if crate::trace::enabled("gdi32/bitmap") {
                Some(crate::trace::trace_begin(
                    "gdi32/bitmap",
                    "StretchDIBits",
                    &[
                        ("hdc", &hdc),
                        ("xDst", &xDst),
                        ("yDst", &yDst),
                        ("wDst", &wDst),
                        ("hDst", &hDst),
                        ("xSrc", &xSrc),
                        ("ySrc", &ySrc),
                        ("wSrc", &wSrc),
                        ("hSrc", &hSrc),
                        ("lpBits", &lpBits),
                        ("lpBmi", &lpBmi),
                        ("iUsage", &iUsage),
                        ("rop", &rop),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::StretchDIBits(
                machine, hdc, xDst, yDst, wDst, hDst, xSrc, ySrc, wSrc, hSrc, lpBits, lpBmi,
                iUsage, rop,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::StretchDIBits_pos.0,
                    winapi::gdi32::StretchDIBits_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn TextOutA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let lpString = <ArrayWithSize<u8>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("gdi32/text") {
                Some(crate::trace::trace_begin(
                    "gdi32/text",
                    "TextOutA",
                    &[("hdc", &hdc), ("x", &x), ("y", &y), ("lpString", &lpString)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::TextOutA(machine, hdc, x, y, lpString);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::TextOutA_pos.0,
                    winapi::gdi32::TextOutA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn TextOutW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let lpString = <ArrayWithSize<u16>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("gdi32/text") {
                Some(crate::trace::trace_begin(
                    "gdi32/text",
                    "TextOutW",
                    &[("hdc", &hdc), ("x", &x), ("y", &y), ("lpString", &lpString)],
                ))
            } else {
                None
            };
            let result = winapi::gdi32::TextOutW(machine, hdc, x, y, lpString);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::gdi32::TextOutW_pos.0,
                    winapi::gdi32::TextOutW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
    }
    const SHIMS: [Shim; 47usize] = [
        Shim {
            name: "BitBlt",
            func: Handler::Sync(wrappers::BitBlt),
        },
        Shim {
            name: "CreateBitmap",
            func: Handler::Sync(wrappers::CreateBitmap),
        },
        Shim {
            name: "CreateCompatibleBitmap",
            func: Handler::Sync(wrappers::CreateCompatibleBitmap),
        },
        Shim {
            name: "CreateCompatibleDC",
            func: Handler::Sync(wrappers::CreateCompatibleDC),
        },
        Shim {
            name: "CreateDIBSection",
            func: Handler::Sync(wrappers::CreateDIBSection),
        },
        Shim {
            name: "CreateDIBitmap",
            func: Handler::Sync(wrappers::CreateDIBitmap),
        },
        Shim {
            name: "CreateFontA",
            func: Handler::Sync(wrappers::CreateFontA),
        },
        Shim {
            name: "CreatePalette",
            func: Handler::Sync(wrappers::CreatePalette),
        },
        Shim {
            name: "CreatePen",
            func: Handler::Sync(wrappers::CreatePen),
        },
        Shim {
            name: "CreateSolidBrush",
            func: Handler::Sync(wrappers::CreateSolidBrush),
        },
        Shim {
            name: "DeleteDC",
            func: Handler::Sync(wrappers::DeleteDC),
        },
        Shim {
            name: "DeleteObject",
            func: Handler::Sync(wrappers::DeleteObject),
        },
        Shim {
            name: "GetDCOrgEx",
            func: Handler::Sync(wrappers::GetDCOrgEx),
        },
        Shim {
            name: "GetDIBits",
            func: Handler::Sync(wrappers::GetDIBits),
        },
        Shim {
            name: "GetDeviceCaps",
            func: Handler::Sync(wrappers::GetDeviceCaps),
        },
        Shim {
            name: "GetLayout",
            func: Handler::Sync(wrappers::GetLayout),
        },
        Shim {
            name: "GetObjectA",
            func: Handler::Sync(wrappers::GetObjectA),
        },
        Shim {
            name: "GetPaletteEntries",
            func: Handler::Sync(wrappers::GetPaletteEntries),
        },
        Shim {
            name: "GetPixel",
            func: Handler::Sync(wrappers::GetPixel),
        },
        Shim {
            name: "GetStockObject",
            func: Handler::Sync(wrappers::GetStockObject),
        },
        Shim {
            name: "GetSystemPaletteEntries",
            func: Handler::Sync(wrappers::GetSystemPaletteEntries),
        },
        Shim {
            name: "GetTextExtentPoint32A",
            func: Handler::Sync(wrappers::GetTextExtentPoint32A),
        },
        Shim {
            name: "GetTextExtentPoint32W",
            func: Handler::Sync(wrappers::GetTextExtentPoint32W),
        },
        Shim {
            name: "GetTextMetricsA",
            func: Handler::Sync(wrappers::GetTextMetricsA),
        },
        Shim {
            name: "GetTextMetricsW",
            func: Handler::Sync(wrappers::GetTextMetricsW),
        },
        Shim {
            name: "LineDDA",
            func: Handler::Sync(wrappers::LineDDA),
        },
        Shim {
            name: "LineTo",
            func: Handler::Sync(wrappers::LineTo),
        },
        Shim {
            name: "MoveToEx",
            func: Handler::Sync(wrappers::MoveToEx),
        },
        Shim {
            name: "PatBlt",
            func: Handler::Sync(wrappers::PatBlt),
        },
        Shim {
            name: "PtVisible",
            func: Handler::Sync(wrappers::PtVisible),
        },
        Shim {
            name: "RealizePalette",
            func: Handler::Sync(wrappers::RealizePalette),
        },
        Shim {
            name: "SelectObject",
            func: Handler::Sync(wrappers::SelectObject),
        },
        Shim {
            name: "SelectPalette",
            func: Handler::Sync(wrappers::SelectPalette),
        },
        Shim {
            name: "SetBkColor",
            func: Handler::Sync(wrappers::SetBkColor),
        },
        Shim {
            name: "SetBkMode",
            func: Handler::Sync(wrappers::SetBkMode),
        },
        Shim {
            name: "SetBrushOrgEx",
            func: Handler::Sync(wrappers::SetBrushOrgEx),
        },
        Shim {
            name: "SetDIBitsToDevice",
            func: Handler::Sync(wrappers::SetDIBitsToDevice),
        },
        Shim {
            name: "SetLayout",
            func: Handler::Sync(wrappers::SetLayout),
        },
        Shim {
            name: "SetPaletteEntries",
            func: Handler::Sync(wrappers::SetPaletteEntries),
        },
        Shim {
            name: "SetPixel",
            func: Handler::Sync(wrappers::SetPixel),
        },
        Shim {
            name: "SetROP2",
            func: Handler::Sync(wrappers::SetROP2),
        },
        Shim {
            name: "SetTextAlign",
            func: Handler::Sync(wrappers::SetTextAlign),
        },
        Shim {
            name: "SetTextColor",
            func: Handler::Sync(wrappers::SetTextColor),
        },
        Shim {
            name: "StretchBlt",
            func: Handler::Sync(wrappers::StretchBlt),
        },
        Shim {
            name: "StretchDIBits",
            func: Handler::Sync(wrappers::StretchDIBits),
        },
        Shim {
            name: "TextOutA",
            func: Handler::Sync(wrappers::TextOutA),
        },
        Shim {
            name: "TextOutW",
            func: Handler::Sync(wrappers::TextOutW),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "gdi32.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/gdi32.dll"),
    };
}
pub mod kernel32 {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::kernel32::*;
        pub unsafe fn AcquireSRWLockExclusive(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "AcquireSRWLockExclusive",
                    &[("SRWLock", &SRWLock)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::AcquireSRWLockExclusive(machine, SRWLock);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::AcquireSRWLockExclusive_pos.0,
                    winapi::kernel32::AcquireSRWLockExclusive_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn AcquireSRWLockShared(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "AcquireSRWLockShared",
                    &[("SRWLock", &SRWLock)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::AcquireSRWLockShared(machine, SRWLock);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::AcquireSRWLockShared_pos.0,
                    winapi::kernel32::AcquireSRWLockShared_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn AddVectoredExceptionHandler(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let first = <u32>::from_stack(mem, stack_args + 0u32);
            let handler = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "AddVectoredExceptionHandler",
                    &[("first", &first), ("handler", &handler)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::AddVectoredExceptionHandler(machine, first, handler);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::AddVectoredExceptionHandler_pos.0,
                    winapi::kernel32::AddVectoredExceptionHandler_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CloseHandle(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hObject = <HFILE>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "CloseHandle",
                    &[("hObject", &hObject)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::CloseHandle(machine, hObject);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::CloseHandle_pos.0,
                    winapi::kernel32::CloseHandle_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CompareStringA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let Locale = <u32>::from_stack(mem, stack_args + 0u32);
            let dwCmpFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpString1 = <u32>::from_stack(mem, stack_args + 8u32);
            let cchCount1 = <i32>::from_stack(mem, stack_args + 12u32);
            let lpString2 = <u32>::from_stack(mem, stack_args + 16u32);
            let cchCount2 = <i32>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "CompareStringA",
                    &[
                        ("Locale", &Locale),
                        ("dwCmpFlags", &dwCmpFlags),
                        ("lpString1", &lpString1),
                        ("cchCount1", &cchCount1),
                        ("lpString2", &lpString2),
                        ("cchCount2", &cchCount2),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::CompareStringA(
                machine, Locale, dwCmpFlags, lpString1, cchCount1, lpString2, cchCount2,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::CompareStringA_pos.0,
                    winapi::kernel32::CompareStringA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CompareStringW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let Locale = <u32>::from_stack(mem, stack_args + 0u32);
            let dwCmpFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpString1 = <u32>::from_stack(mem, stack_args + 8u32);
            let cchCount1 = <i32>::from_stack(mem, stack_args + 12u32);
            let lpString2 = <u32>::from_stack(mem, stack_args + 16u32);
            let cchCount2 = <i32>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "CompareStringW",
                    &[
                        ("Locale", &Locale),
                        ("dwCmpFlags", &dwCmpFlags),
                        ("lpString1", &lpString1),
                        ("cchCount1", &cchCount1),
                        ("lpString2", &lpString2),
                        ("cchCount2", &cchCount2),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::CompareStringW(
                machine, Locale, dwCmpFlags, lpString1, cchCount1, lpString2, cchCount2,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::CompareStringW_pos.0,
                    winapi::kernel32::CompareStringW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "CreateDirectoryA",
                    &[
                        ("lpPathName", &lpPathName),
                        ("lpSecurityAttributes", &lpSecurityAttributes),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::kernel32::CreateDirectoryA(machine, lpPathName, lpSecurityAttributes);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::CreateDirectoryA_pos.0,
                    winapi::kernel32::CreateDirectoryA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateEventA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpEventAttributes = <u32>::from_stack(mem, stack_args + 0u32);
            let bManualReset = <bool>::from_stack(mem, stack_args + 4u32);
            let bInitialState = <bool>::from_stack(mem, stack_args + 8u32);
            let lpName = <Option<&str>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/sync") {
                Some(crate::trace::trace_begin(
                    "kernel32/sync",
                    "CreateEventA",
                    &[
                        ("lpEventAttributes", &lpEventAttributes),
                        ("bManualReset", &bManualReset),
                        ("bInitialState", &bInitialState),
                        ("lpName", &lpName),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::CreateEventA(
                machine,
                lpEventAttributes,
                bManualReset,
                bInitialState,
                lpName,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::CreateEventA_pos.0,
                    winapi::kernel32::CreateEventA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateFileA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let dwDesiredAccess = <u32>::from_stack(mem, stack_args + 4u32);
            let dwShareMode = <u32>::from_stack(mem, stack_args + 8u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, stack_args + 12u32);
            let dwCreationDisposition =
                <Result<CreationDisposition, u32>>::from_stack(mem, stack_args + 16u32);
            let dwFlagsAndAttributes =
                <Result<FileAttribute, u32>>::from_stack(mem, stack_args + 20u32);
            let hTemplateFile = <HFILE>::from_stack(mem, stack_args + 24u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "CreateFileA",
                    &[
                        ("lpFileName", &lpFileName),
                        ("dwDesiredAccess", &dwDesiredAccess),
                        ("dwShareMode", &dwShareMode),
                        ("lpSecurityAttributes", &lpSecurityAttributes),
                        ("dwCreationDisposition", &dwCreationDisposition),
                        ("dwFlagsAndAttributes", &dwFlagsAndAttributes),
                        ("hTemplateFile", &hTemplateFile),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::CreateFileA(
                machine,
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::CreateFileA_pos.0,
                    winapi::kernel32::CreateFileA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateFileW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let dwDesiredAccess = <u32>::from_stack(mem, stack_args + 4u32);
            let dwShareMode = <u32>::from_stack(mem, stack_args + 8u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, stack_args + 12u32);
            let dwCreationDisposition =
                <Result<CreationDisposition, u32>>::from_stack(mem, stack_args + 16u32);
            let dwFlagsAndAttributes =
                <Result<FileAttribute, u32>>::from_stack(mem, stack_args + 20u32);
            let hTemplateFile = <HFILE>::from_stack(mem, stack_args + 24u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "CreateFileW",
                    &[
                        ("lpFileName", &lpFileName),
                        ("dwDesiredAccess", &dwDesiredAccess),
                        ("dwShareMode", &dwShareMode),
                        ("lpSecurityAttributes", &lpSecurityAttributes),
                        ("dwCreationDisposition", &dwCreationDisposition),
                        ("dwFlagsAndAttributes", &dwFlagsAndAttributes),
                        ("hTemplateFile", &hTemplateFile),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::CreateFileW(
                machine,
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::CreateFileW_pos.0,
                    winapi::kernel32::CreateFileW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateProcessA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpApplicationName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpCommandLine = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let lpProcessAttributes =
                <Option<&mut SECURITY_ATTRIBUTES>>::from_stack(mem, stack_args + 8u32);
            let lpThreadAttributes =
                <Option<&mut SECURITY_ATTRIBUTES>>::from_stack(mem, stack_args + 12u32);
            let bInheritHandles = <bool>::from_stack(mem, stack_args + 16u32);
            let dwCreationFlags = <u32>::from_stack(mem, stack_args + 20u32);
            let lpEnvironment = <Option<&mut u8>>::from_stack(mem, stack_args + 24u32);
            let lpCurrentDirectory = <Option<&str>>::from_stack(mem, stack_args + 28u32);
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, stack_args + 32u32);
            let lpProcessInformation =
                <Option<&mut PROCESS_INFORMATION>>::from_stack(mem, stack_args + 36u32);
            let __trace_context = if crate::trace::enabled("kernel32/process") {
                Some(crate::trace::trace_begin(
                    "kernel32/process",
                    "CreateProcessA",
                    &[
                        ("lpApplicationName", &lpApplicationName),
                        ("lpCommandLine", &lpCommandLine),
                        ("lpProcessAttributes", &lpProcessAttributes),
                        ("lpThreadAttributes", &lpThreadAttributes),
                        ("bInheritHandles", &bInheritHandles),
                        ("dwCreationFlags", &dwCreationFlags),
                        ("lpEnvironment", &lpEnvironment),
                        ("lpCurrentDirectory", &lpCurrentDirectory),
                        ("lpStartupInfo", &lpStartupInfo),
                        ("lpProcessInformation", &lpProcessInformation),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::CreateProcessA(
                machine,
                lpApplicationName,
                lpCommandLine,
                lpProcessAttributes,
                lpThreadAttributes,
                bInheritHandles,
                dwCreationFlags,
                lpEnvironment,
                lpCurrentDirectory,
                lpStartupInfo,
                lpProcessInformation,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::CreateProcessA_pos.0,
                    winapi::kernel32::CreateProcessA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateThread(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let lpThreadAttributes = <u32>::from_stack(mem, stack_args + 0u32);
            let dwStackSize = <u32>::from_stack(mem, stack_args + 4u32);
            let lpStartAddress = <u32>::from_stack(mem, stack_args + 8u32);
            let lpParameter = <u32>::from_stack(mem, stack_args + 12u32);
            let dwCreationFlags = <u32>::from_stack(mem, stack_args + 16u32);
            let lpThreadId = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "CreateThread",
                    &[
                        ("lpThreadAttributes", &lpThreadAttributes),
                        ("dwStackSize", &dwStackSize),
                        ("lpStartAddress", &lpStartAddress),
                        ("lpParameter", &lpParameter),
                        ("dwCreationFlags", &dwCreationFlags),
                        ("lpThreadId", &lpThreadId),
                    ],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::kernel32::CreateThread(
                    machine,
                    lpThreadAttributes,
                    dwStackSize,
                    lpStartAddress,
                    lpParameter,
                    dwCreationFlags,
                    lpThreadId,
                )
                .await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::kernel32::CreateThread_pos.0,
                        winapi::kernel32::CreateThread_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn DebugBreak(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "DebugBreak",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::DebugBreak(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::DebugBreak_pos.0,
                    winapi::kernel32::DebugBreak_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn DeleteCriticalSection(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "DeleteCriticalSection",
                    &[("lpCriticalSection", &lpCriticalSection)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::DeleteCriticalSection(machine, lpCriticalSection);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::DeleteCriticalSection_pos.0,
                    winapi::kernel32::DeleteCriticalSection_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn DeleteFileA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "DeleteFileA",
                    &[("lpFileName", &lpFileName)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::DeleteFileA(machine, lpFileName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::DeleteFileA_pos.0,
                    winapi::kernel32::DeleteFileA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn DisableThreadLibraryCalls(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hLibModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/dll") {
                Some(crate::trace::trace_begin(
                    "kernel32/dll",
                    "DisableThreadLibraryCalls",
                    &[("hLibModule", &hLibModule)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::DisableThreadLibraryCalls(machine, hLibModule);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::DisableThreadLibraryCalls_pos.0,
                    winapi::kernel32::DisableThreadLibraryCalls_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn DuplicateHandle(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hSourceProcessHandle = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let hSourceHandle = <HANDLE<()>>::from_stack(mem, stack_args + 4u32);
            let hTargetProcessHandle = <HANDLE<()>>::from_stack(mem, stack_args + 8u32);
            let lpTargetHandle = <Option<&mut HANDLE<()>>>::from_stack(mem, stack_args + 12u32);
            let dwDesiredAccess = <u32>::from_stack(mem, stack_args + 16u32);
            let bInheritHandle = <bool>::from_stack(mem, stack_args + 20u32);
            let dwOptions = <u32>::from_stack(mem, stack_args + 24u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "DuplicateHandle",
                    &[
                        ("hSourceProcessHandle", &hSourceProcessHandle),
                        ("hSourceHandle", &hSourceHandle),
                        ("hTargetProcessHandle", &hTargetProcessHandle),
                        ("lpTargetHandle", &lpTargetHandle),
                        ("dwDesiredAccess", &dwDesiredAccess),
                        ("bInheritHandle", &bInheritHandle),
                        ("dwOptions", &dwOptions),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::DuplicateHandle(
                machine,
                hSourceProcessHandle,
                hSourceHandle,
                hTargetProcessHandle,
                lpTargetHandle,
                dwDesiredAccess,
                bInheritHandle,
                dwOptions,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::DuplicateHandle_pos.0,
                    winapi::kernel32::DuplicateHandle_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn EnterCriticalSection(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "EnterCriticalSection",
                    &[("lpCriticalSection", &lpCriticalSection)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::EnterCriticalSection(machine, lpCriticalSection);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::EnterCriticalSection_pos.0,
                    winapi::kernel32::EnterCriticalSection_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn ExitProcess(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uExitCode = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "ExitProcess",
                    &[("uExitCode", &uExitCode)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::ExitProcess(machine, uExitCode);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::ExitProcess_pos.0,
                    winapi::kernel32::ExitProcess_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn ExitThread(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwExitCode = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "ExitThread",
                    &[("dwExitCode", &dwExitCode)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::ExitThread(machine, dwExitCode);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::ExitThread_pos.0,
                    winapi::kernel32::ExitThread_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FileTimeToLocalFileTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 0u32);
            let lpLocalFileTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/time") {
                Some(crate::trace::trace_begin(
                    "kernel32/time",
                    "FileTimeToLocalFileTime",
                    &[
                        ("lpFileTime", &lpFileTime),
                        ("lpLocalFileTime", &lpLocalFileTime),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::kernel32::FileTimeToLocalFileTime(machine, lpFileTime, lpLocalFileTime);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::FileTimeToLocalFileTime_pos.0,
                    winapi::kernel32::FileTimeToLocalFileTime_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FileTimeToSystemTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 0u32);
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/time") {
                Some(crate::trace::trace_begin(
                    "kernel32/time",
                    "FileTimeToSystemTime",
                    &[("lpFileTime", &lpFileTime), ("lpSystemTime", &lpSystemTime)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::FileTimeToSystemTime(machine, lpFileTime, lpSystemTime);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::FileTimeToSystemTime_pos.0,
                    winapi::kernel32::FileTimeToSystemTime_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FindClose(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFindFile = <HFIND>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "FindClose",
                    &[("hFindFile", &hFindFile)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::FindClose(machine, hFindFile);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::FindClose_pos.0,
                    winapi::kernel32::FindClose_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FindFirstFileA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpFindFileData =
                <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "FindFirstFileA",
                    &[
                        ("lpFileName", &lpFileName),
                        ("lpFindFileData", &lpFindFileData),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::FindFirstFileA(machine, lpFileName, lpFindFileData);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::FindFirstFileA_pos.0,
                    winapi::kernel32::FindFirstFileA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FindNextFileA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFindFile = <HFIND>::from_stack(mem, stack_args + 0u32);
            let lpFindFileData =
                <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "FindNextFileA",
                    &[
                        ("hFindFile", &hFindFile),
                        ("lpFindFileData", &lpFindFileData),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::FindNextFileA(machine, hFindFile, lpFindFileData);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::FindNextFileA_pos.0,
                    winapi::kernel32::FindNextFileA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FindResourceA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let lpName = <ResourceKey<&str>>::from_stack(mem, stack_args + 4u32);
            let lpType = <ResourceKey<&str>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/resource") {
                Some(crate::trace::trace_begin(
                    "kernel32/resource",
                    "FindResourceA",
                    &[
                        ("hModule", &hModule),
                        ("lpName", &lpName),
                        ("lpType", &lpType),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::FindResourceA(machine, hModule, lpName, lpType);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::FindResourceA_pos.0,
                    winapi::kernel32::FindResourceA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FindResourceW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let lpName = <ResourceKey<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpType = <ResourceKey<&Str16>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/resource") {
                Some(crate::trace::trace_begin(
                    "kernel32/resource",
                    "FindResourceW",
                    &[
                        ("hModule", &hModule),
                        ("lpName", &lpName),
                        ("lpType", &lpType),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::FindResourceW(machine, hModule, lpName, lpType);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::FindResourceW_pos.0,
                    winapi::kernel32::FindResourceW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FlushFileBuffers(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "FlushFileBuffers",
                    &[("hFile", &hFile)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::FlushFileBuffers(machine, hFile);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::FlushFileBuffers_pos.0,
                    winapi::kernel32::FlushFileBuffers_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FormatMessageA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, stack_args + 0u32);
            let lpSource = <u32>::from_stack(mem, stack_args + 4u32);
            let dwMessageId = <u32>::from_stack(mem, stack_args + 8u32);
            let dwLanguageId = <u32>::from_stack(mem, stack_args + 12u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 16u32);
            let nSize = <u32>::from_stack(mem, stack_args + 20u32);
            let args = <u32>::from_stack(mem, stack_args + 24u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "FormatMessageA",
                    &[
                        ("dwFlags", &dwFlags),
                        ("lpSource", &lpSource),
                        ("dwMessageId", &dwMessageId),
                        ("dwLanguageId", &dwLanguageId),
                        ("lpBuffer", &lpBuffer),
                        ("nSize", &nSize),
                        ("args", &args),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::FormatMessageA(
                machine,
                dwFlags,
                lpSource,
                dwMessageId,
                dwLanguageId,
                lpBuffer,
                nSize,
                args,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::FormatMessageA_pos.0,
                    winapi::kernel32::FormatMessageA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FormatMessageW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <Result<FormatMessageFlags, u32>>::from_stack(mem, stack_args + 0u32);
            let lpSource = <u32>::from_stack(mem, stack_args + 4u32);
            let dwMessageId = <u32>::from_stack(mem, stack_args + 8u32);
            let dwLanguageId = <u32>::from_stack(mem, stack_args + 12u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 16u32);
            let nSize = <u32>::from_stack(mem, stack_args + 20u32);
            let args = <u32>::from_stack(mem, stack_args + 24u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "FormatMessageW",
                    &[
                        ("dwFlags", &dwFlags),
                        ("lpSource", &lpSource),
                        ("dwMessageId", &dwMessageId),
                        ("dwLanguageId", &dwLanguageId),
                        ("lpBuffer", &lpBuffer),
                        ("nSize", &nSize),
                        ("args", &args),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::FormatMessageW(
                machine,
                dwFlags,
                lpSource,
                dwMessageId,
                dwLanguageId,
                lpBuffer,
                nSize,
                args,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::FormatMessageW_pos.0,
                    winapi::kernel32::FormatMessageW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FreeEnvironmentStringsA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _penv = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/env") {
                Some(crate::trace::trace_begin(
                    "kernel32/env",
                    "FreeEnvironmentStringsA",
                    &[("penv", &_penv)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::FreeEnvironmentStringsA(machine, _penv);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::FreeEnvironmentStringsA_pos.0,
                    winapi::kernel32::FreeEnvironmentStringsA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FreeEnvironmentStringsW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/env") {
                Some(crate::trace::trace_begin(
                    "kernel32/env",
                    "FreeEnvironmentStringsW",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::FreeEnvironmentStringsW(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::FreeEnvironmentStringsW_pos.0,
                    winapi::kernel32::FreeEnvironmentStringsW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FreeLibrary(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hLibModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/dll") {
                Some(crate::trace::trace_begin(
                    "kernel32/dll",
                    "FreeLibrary",
                    &[("hLibModule", &hLibModule)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::FreeLibrary(machine, hLibModule);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::FreeLibrary_pos.0,
                    winapi::kernel32::FreeLibrary_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetACP(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/nls") {
                Some(crate::trace::trace_begin("kernel32/nls", "GetACP", &[]))
            } else {
                None
            };
            let result = winapi::kernel32::GetACP(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetACP_pos.0,
                    winapi::kernel32::GetACP_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetCPInfo(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _CodePage = <u32>::from_stack(mem, stack_args + 0u32);
            let _lpCPInfo = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/nls") {
                Some(crate::trace::trace_begin(
                    "kernel32/nls",
                    "GetCPInfo",
                    &[("CodePage", &_CodePage), ("lpCPInfo", &_lpCPInfo)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetCPInfo_pos.0,
                    winapi::kernel32::GetCPInfo_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetCommandLineA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/init") {
                Some(crate::trace::trace_begin(
                    "kernel32/init",
                    "GetCommandLineA",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetCommandLineA(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetCommandLineA_pos.0,
                    winapi::kernel32::GetCommandLineA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetCommandLineW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/init") {
                Some(crate::trace::trace_begin(
                    "kernel32/init",
                    "GetCommandLineW",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetCommandLineW(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetCommandLineW_pos.0,
                    winapi::kernel32::GetCommandLineW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetConsoleMode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleHandle = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpMode = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "GetConsoleMode",
                    &[("hConsoleHandle", &hConsoleHandle), ("lpMode", &lpMode)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetConsoleMode(machine, hConsoleHandle, lpMode);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetConsoleMode_pos.0,
                    winapi::kernel32::GetConsoleMode_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetConsoleScreenBufferInfo(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _hConsoleOutput = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let lpConsoleScreenBufferInfo =
                <Option<&mut CONSOLE_SCREEN_BUFFER_INFO>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/console") {
                Some(crate::trace::trace_begin(
                    "kernel32/console",
                    "GetConsoleScreenBufferInfo",
                    &[
                        ("hConsoleOutput", &_hConsoleOutput),
                        ("lpConsoleScreenBufferInfo", &lpConsoleScreenBufferInfo),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetConsoleScreenBufferInfo(
                machine,
                _hConsoleOutput,
                lpConsoleScreenBufferInfo,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetConsoleScreenBufferInfo_pos.0,
                    winapi::kernel32::GetConsoleScreenBufferInfo_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetCurrentDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nBufferLength = <u32>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "GetCurrentDirectoryA",
                    &[("nBufferLength", &nBufferLength), ("lpBuffer", &lpBuffer)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetCurrentDirectoryA(machine, nBufferLength, lpBuffer);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetCurrentDirectoryA_pos.0,
                    winapi::kernel32::GetCurrentDirectoryA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetCurrentProcess(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "GetCurrentProcess",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetCurrentProcess(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetCurrentProcess_pos.0,
                    winapi::kernel32::GetCurrentProcess_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetCurrentProcessId(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "GetCurrentProcessId",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetCurrentProcessId(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetCurrentProcessId_pos.0,
                    winapi::kernel32::GetCurrentProcessId_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetCurrentThread(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "GetCurrentThread",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetCurrentThread(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetCurrentThread_pos.0,
                    winapi::kernel32::GetCurrentThread_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetCurrentThreadId(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "GetCurrentThreadId",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetCurrentThreadId(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetCurrentThreadId_pos.0,
                    winapi::kernel32::GetCurrentThreadId_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetDiskFreeSpaceA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpRootPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpSectorsPerCluster = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let lpBytesPerSector = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let lpNumberOfFreeClusters = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpTotalNumberOfClusters = <Option<&mut u32>>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "GetDiskFreeSpaceA",
                    &[
                        ("lpRootPathName", &lpRootPathName),
                        ("lpSectorsPerCluster", &lpSectorsPerCluster),
                        ("lpBytesPerSector", &lpBytesPerSector),
                        ("lpNumberOfFreeClusters", &lpNumberOfFreeClusters),
                        ("lpTotalNumberOfClusters", &lpTotalNumberOfClusters),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetDiskFreeSpaceA(
                machine,
                lpRootPathName,
                lpSectorsPerCluster,
                lpBytesPerSector,
                lpNumberOfFreeClusters,
                lpTotalNumberOfClusters,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetDiskFreeSpaceA_pos.0,
                    winapi::kernel32::GetDiskFreeSpaceA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetDriveTypeA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpRootPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "GetDriveTypeA",
                    &[("lpRootPathName", &lpRootPathName)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetDriveTypeA(machine, lpRootPathName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetDriveTypeA_pos.0,
                    winapi::kernel32::GetDriveTypeA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetEnvironmentStrings(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/env") {
                Some(crate::trace::trace_begin(
                    "kernel32/env",
                    "GetEnvironmentStrings",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetEnvironmentStrings(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetEnvironmentStrings_pos.0,
                    winapi::kernel32::GetEnvironmentStrings_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetEnvironmentStringsW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/env") {
                Some(crate::trace::trace_begin(
                    "kernel32/env",
                    "GetEnvironmentStringsW",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetEnvironmentStringsW(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetEnvironmentStringsW_pos.0,
                    winapi::kernel32::GetEnvironmentStringsW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetEnvironmentVariableA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let buf = <ArrayWithSize<u8>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/env") {
                Some(crate::trace::trace_begin(
                    "kernel32/env",
                    "GetEnvironmentVariableA",
                    &[("name", &name), ("buf", &buf)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetEnvironmentVariableA(machine, name, buf);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetEnvironmentVariableA_pos.0,
                    winapi::kernel32::GetEnvironmentVariableA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetEnvironmentVariableW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let buf = <ArrayWithSize<u16>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/env") {
                Some(crate::trace::trace_begin(
                    "kernel32/env",
                    "GetEnvironmentVariableW",
                    &[("name", &name), ("buf", &buf)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetEnvironmentVariableW(machine, name, buf);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetEnvironmentVariableW_pos.0,
                    winapi::kernel32::GetEnvironmentVariableW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetExitCodeProcess(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hProcess = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let lpExitCode = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/process") {
                Some(crate::trace::trace_begin(
                    "kernel32/process",
                    "GetExitCodeProcess",
                    &[("hProcess", &hProcess), ("lpExitCode", &lpExitCode)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetExitCodeProcess(machine, hProcess, lpExitCode);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetExitCodeProcess_pos.0,
                    winapi::kernel32::GetExitCodeProcess_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetFileAttributesA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "GetFileAttributesA",
                    &[("lpFileName", &lpFileName)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetFileAttributesA(machine, lpFileName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetFileAttributesA_pos.0,
                    winapi::kernel32::GetFileAttributesA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetFileInformationByHandle(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpFileInformation =
                <Option<&mut BY_HANDLE_FILE_INFORMATION>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "GetFileInformationByHandle",
                    &[("hFile", &hFile), ("lpFileInformation", &lpFileInformation)],
                ))
            } else {
                None
            };
            let result =
                winapi::kernel32::GetFileInformationByHandle(machine, hFile, lpFileInformation);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetFileInformationByHandle_pos.0,
                    winapi::kernel32::GetFileInformationByHandle_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetFileSize(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpFileSizeHigh = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "GetFileSize",
                    &[("hFile", &hFile), ("lpFileSizeHigh", &lpFileSizeHigh)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetFileSize(machine, hFile, lpFileSizeHigh);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetFileSize_pos.0,
                    winapi::kernel32::GetFileSize_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetFileTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpCreationTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 4u32);
            let lpLastAccessTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 8u32);
            let lpLastWriteTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "GetFileTime",
                    &[
                        ("hFile", &hFile),
                        ("lpCreationTime", &lpCreationTime),
                        ("lpLastAccessTime", &lpLastAccessTime),
                        ("lpLastWriteTime", &lpLastWriteTime),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetFileTime(
                machine,
                hFile,
                lpCreationTime,
                lpLastAccessTime,
                lpLastWriteTime,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetFileTime_pos.0,
                    winapi::kernel32::GetFileTime_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetFileType(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "GetFileType",
                    &[("hFile", &hFile)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetFileType(machine, hFile);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetFileType_pos.0,
                    winapi::kernel32::GetFileType_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetFullPathNameA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let nBufferLength = <u32>::from_stack(mem, stack_args + 4u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
            let lpFilePart = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "GetFullPathNameA",
                    &[
                        ("lpFileName", &lpFileName),
                        ("nBufferLength", &nBufferLength),
                        ("lpBuffer", &lpBuffer),
                        ("lpFilePart", &lpFilePart),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetFullPathNameA(
                machine,
                lpFileName,
                nBufferLength,
                lpBuffer,
                lpFilePart,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetFullPathNameA_pos.0,
                    winapi::kernel32::GetFullPathNameA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetFullPathNameW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let nBufferLength = <u32>::from_stack(mem, stack_args + 4u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
            let lpFilePart = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "GetFullPathNameW",
                    &[
                        ("lpFileName", &lpFileName),
                        ("nBufferLength", &nBufferLength),
                        ("lpBuffer", &lpBuffer),
                        ("lpFilePart", &lpFilePart),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetFullPathNameW(
                machine,
                lpFileName,
                nBufferLength,
                lpBuffer,
                lpFilePart,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetFullPathNameW_pos.0,
                    winapi::kernel32::GetFullPathNameW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetLastError(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "GetLastError",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetLastError(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetLastError_pos.0,
                    winapi::kernel32::GetLastError_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetLocalTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/time") {
                Some(crate::trace::trace_begin(
                    "kernel32/time",
                    "GetLocalTime",
                    &[("lpSystemTime", &lpSystemTime)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetLocalTime(machine, lpSystemTime);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetLocalTime_pos.0,
                    winapi::kernel32::GetLocalTime_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetLogicalDrives(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "GetLogicalDrives",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetLogicalDrives(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetLogicalDrives_pos.0,
                    winapi::kernel32::GetLogicalDrives_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetModuleFileNameA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let filename = <ArrayWithSizeMut<u8>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/dll") {
                Some(crate::trace::trace_begin(
                    "kernel32/dll",
                    "GetModuleFileNameA",
                    &[("hModule", &hModule), ("filename", &filename)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetModuleFileNameA(machine, hModule, filename);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetModuleFileNameA_pos.0,
                    winapi::kernel32::GetModuleFileNameA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetModuleFileNameW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let _lpFilename = <u32>::from_stack(mem, stack_args + 4u32);
            let _nSize = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/dll") {
                Some(crate::trace::trace_begin(
                    "kernel32/dll",
                    "GetModuleFileNameW",
                    &[
                        ("hModule", &hModule),
                        ("lpFilename", &_lpFilename),
                        ("nSize", &_nSize),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetModuleFileNameW_pos.0,
                    winapi::kernel32::GetModuleFileNameW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetModuleHandleA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpModuleName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/dll") {
                Some(crate::trace::trace_begin(
                    "kernel32/dll",
                    "GetModuleHandleA",
                    &[("lpModuleName", &lpModuleName)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetModuleHandleA(machine, lpModuleName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetModuleHandleA_pos.0,
                    winapi::kernel32::GetModuleHandleA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetModuleHandleExW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, stack_args + 0u32);
            let lpModuleName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let hModule = <Option<&mut HMODULE>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/dll") {
                Some(crate::trace::trace_begin(
                    "kernel32/dll",
                    "GetModuleHandleExW",
                    &[
                        ("dwFlags", &dwFlags),
                        ("lpModuleName", &lpModuleName),
                        ("hModule", &hModule),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetModuleHandleExW_pos.0,
                    winapi::kernel32::GetModuleHandleExW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetModuleHandleW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpModuleName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/dll") {
                Some(crate::trace::trace_begin(
                    "kernel32/dll",
                    "GetModuleHandleW",
                    &[("lpModuleName", &lpModuleName)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetModuleHandleW(machine, lpModuleName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetModuleHandleW_pos.0,
                    winapi::kernel32::GetModuleHandleW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetOEMCP(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/nls") {
                Some(crate::trace::trace_begin("kernel32/nls", "GetOEMCP", &[]))
            } else {
                None
            };
            let result = winapi::kernel32::GetOEMCP(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetOEMCP_pos.0,
                    winapi::kernel32::GetOEMCP_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetPrivateProfileIntW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let nDefault = <u32>::from_stack(mem, stack_args + 8u32);
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/ini") {
                Some(crate::trace::trace_begin(
                    "kernel32/ini",
                    "GetPrivateProfileIntW",
                    &[
                        ("lpAppName", &lpAppName),
                        ("lpKeyName", &lpKeyName),
                        ("nDefault", &nDefault),
                        ("lpFileName", &lpFileName),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetPrivateProfileIntW(
                machine, lpAppName, lpKeyName, nDefault, lpFileName,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetPrivateProfileIntW_pos.0,
                    winapi::kernel32::GetPrivateProfileIntW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetPrivateProfileStringA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let lpDefault = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let lpReturnedString = <Option<&str>>::from_stack(mem, stack_args + 12u32);
            let nSize = <u32>::from_stack(mem, stack_args + 16u32);
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("kernel32/ini") {
                Some(crate::trace::trace_begin(
                    "kernel32/ini",
                    "GetPrivateProfileStringA",
                    &[
                        ("lpAppName", &lpAppName),
                        ("lpKeyName", &lpKeyName),
                        ("lpDefault", &lpDefault),
                        ("lpReturnedString", &lpReturnedString),
                        ("nSize", &nSize),
                        ("lpFileName", &lpFileName),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetPrivateProfileStringA(
                machine,
                lpAppName,
                lpKeyName,
                lpDefault,
                lpReturnedString,
                nSize,
                lpFileName,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetPrivateProfileStringA_pos.0,
                    winapi::kernel32::GetPrivateProfileStringA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetPrivateProfileStringW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpDefault = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let lpReturnedString = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 12u32);
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("kernel32/ini") {
                Some(crate::trace::trace_begin(
                    "kernel32/ini",
                    "GetPrivateProfileStringW",
                    &[
                        ("lpAppName", &lpAppName),
                        ("lpKeyName", &lpKeyName),
                        ("lpDefault", &lpDefault),
                        ("lpReturnedString", &lpReturnedString),
                        ("lpFileName", &lpFileName),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetPrivateProfileStringW(
                machine,
                lpAppName,
                lpKeyName,
                lpDefault,
                lpReturnedString,
                lpFileName,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetPrivateProfileStringW_pos.0,
                    winapi::kernel32::GetPrivateProfileStringW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetProcAddress(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let lpProcName = <GetProcAddressArg>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/dll") {
                Some(crate::trace::trace_begin(
                    "kernel32/dll",
                    "GetProcAddress",
                    &[("hModule", &hModule), ("lpProcName", &lpProcName)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetProcAddress(machine, hModule, lpProcName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetProcAddress_pos.0,
                    winapi::kernel32::GetProcAddress_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetProcessHeap(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "GetProcessHeap",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetProcessHeap(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetProcessHeap_pos.0,
                    winapi::kernel32::GetProcessHeap_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetProfileIntW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let nDefault = <i32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/ini") {
                Some(crate::trace::trace_begin(
                    "kernel32/ini",
                    "GetProfileIntW",
                    &[
                        ("lpAppName", &lpAppName),
                        ("lpKeyName", &lpKeyName),
                        ("nDefault", &nDefault),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetProfileIntW(machine, lpAppName, lpKeyName, nDefault);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetProfileIntW_pos.0,
                    winapi::kernel32::GetProfileIntW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetProfileStringW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpDefault = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let lpReturnedString = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/ini") {
                Some(crate::trace::trace_begin(
                    "kernel32/ini",
                    "GetProfileStringW",
                    &[
                        ("lpAppName", &lpAppName),
                        ("lpKeyName", &lpKeyName),
                        ("lpDefault", &lpDefault),
                        ("lpReturnedString", &lpReturnedString),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetProfileStringW(
                machine,
                lpAppName,
                lpKeyName,
                lpDefault,
                lpReturnedString,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetProfileStringW_pos.0,
                    winapi::kernel32::GetProfileStringW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetStartupInfoA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/dll") {
                Some(crate::trace::trace_begin(
                    "kernel32/dll",
                    "GetStartupInfoA",
                    &[("lpStartupInfo", &lpStartupInfo)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetStartupInfoA_pos.0,
                    winapi::kernel32::GetStartupInfoA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetStartupInfoW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/dll") {
                Some(crate::trace::trace_begin(
                    "kernel32/dll",
                    "GetStartupInfoW",
                    &[("lpStartupInfo", &lpStartupInfo)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetStartupInfoW_pos.0,
                    winapi::kernel32::GetStartupInfoW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetStdHandle(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nStdHandle = <Result<STD, u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "GetStdHandle",
                    &[("nStdHandle", &nStdHandle)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetStdHandle(machine, nStdHandle);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetStdHandle_pos.0,
                    winapi::kernel32::GetStdHandle_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetStringTypeA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let Locale = <LCID>::from_stack(mem, stack_args + 0u32);
            let dwInfoType = <u32>::from_stack(mem, stack_args + 4u32);
            let lpSrcStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cchSrc = <i32>::from_stack(mem, stack_args + 12u32);
            let lpCharType = <Option<&mut u32>>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("kernel32/nls") {
                Some(crate::trace::trace_begin(
                    "kernel32/nls",
                    "GetStringTypeA",
                    &[
                        ("Locale", &Locale),
                        ("dwInfoType", &dwInfoType),
                        ("lpSrcStr", &lpSrcStr),
                        ("cchSrc", &cchSrc),
                        ("lpCharType", &lpCharType),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetStringTypeA(
                machine, Locale, dwInfoType, lpSrcStr, cchSrc, lpCharType,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetStringTypeA_pos.0,
                    winapi::kernel32::GetStringTypeA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetStringTypeW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwInfoType = <u32>::from_stack(mem, stack_args + 0u32);
            let lpSrcStr = <u32>::from_stack(mem, stack_args + 4u32);
            let cchSrc = <i32>::from_stack(mem, stack_args + 8u32);
            let lpCharType = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/nls") {
                Some(crate::trace::trace_begin(
                    "kernel32/nls",
                    "GetStringTypeW",
                    &[
                        ("dwInfoType", &dwInfoType),
                        ("lpSrcStr", &lpSrcStr),
                        ("cchSrc", &cchSrc),
                        ("lpCharType", &lpCharType),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::kernel32::GetStringTypeW(machine, dwInfoType, lpSrcStr, cchSrc, lpCharType);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetStringTypeW_pos.0,
                    winapi::kernel32::GetStringTypeW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetSystemDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpBuffer = <u32>::from_stack(mem, stack_args + 0u32);
            let uSize = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "GetSystemDirectoryA",
                    &[("lpBuffer", &lpBuffer), ("uSize", &uSize)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetSystemDirectoryA(machine, lpBuffer, uSize);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetSystemDirectoryA_pos.0,
                    winapi::kernel32::GetSystemDirectoryA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetSystemTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/time") {
                Some(crate::trace::trace_begin(
                    "kernel32/time",
                    "GetSystemTime",
                    &[("lpSystemTime", &lpSystemTime)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetSystemTime(machine, lpSystemTime);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetSystemTime_pos.0,
                    winapi::kernel32::GetSystemTime_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetSystemTimeAsFileTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTimeAsFileTime =
                <Option<&mut FILETIME>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/time") {
                Some(crate::trace::trace_begin(
                    "kernel32/time",
                    "GetSystemTimeAsFileTime",
                    &[("lpSystemTimeAsFileTime", &lpSystemTimeAsFileTime)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetSystemTimeAsFileTime(machine, lpSystemTimeAsFileTime);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetSystemTimeAsFileTime_pos.0,
                    winapi::kernel32::GetSystemTimeAsFileTime_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetThreadPriority(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "GetThreadPriority",
                    &[("hThread", &hThread)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetThreadPriority(machine, hThread);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetThreadPriority_pos.0,
                    winapi::kernel32::GetThreadPriority_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetTickCount(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/time") {
                Some(crate::trace::trace_begin(
                    "kernel32/time",
                    "GetTickCount",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetTickCount(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetTickCount_pos.0,
                    winapi::kernel32::GetTickCount_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetTimeZoneInformation(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpTimeZoneInformation =
                <Option<&mut TIME_ZONE_INFORMATION>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/time") {
                Some(crate::trace::trace_begin(
                    "kernel32/time",
                    "GetTimeZoneInformation",
                    &[("lpTimeZoneInformation", &lpTimeZoneInformation)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetTimeZoneInformation(machine, lpTimeZoneInformation);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetTimeZoneInformation_pos.0,
                    winapi::kernel32::GetTimeZoneInformation_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetVersion(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "GetVersion",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetVersion(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetVersion_pos.0,
                    winapi::kernel32::GetVersion_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetVersionExA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpVersionInformation =
                <Option<&mut OSVERSIONINFO>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "GetVersionExA",
                    &[("lpVersionInformation", &lpVersionInformation)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetVersionExA(machine, lpVersionInformation);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetVersionExA_pos.0,
                    winapi::kernel32::GetVersionExA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetWindowsDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpBuffer = <u32>::from_stack(mem, stack_args + 0u32);
            let uSize = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "GetWindowsDirectoryA",
                    &[("lpBuffer", &lpBuffer), ("uSize", &uSize)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GetWindowsDirectoryA(machine, lpBuffer, uSize);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GetWindowsDirectoryA_pos.0,
                    winapi::kernel32::GetWindowsDirectoryA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GlobalAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uFlags = <GMEM>::from_stack(mem, stack_args + 0u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "GlobalAlloc",
                    &[("uFlags", &uFlags), ("dwBytes", &dwBytes)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GlobalAlloc(machine, uFlags, dwBytes);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GlobalAlloc_pos.0,
                    winapi::kernel32::GlobalAlloc_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GlobalFlags(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "GlobalFlags",
                    &[("hMem", &hMem)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GlobalFlags(machine, hMem);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GlobalFlags_pos.0,
                    winapi::kernel32::GlobalFlags_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GlobalFree(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "GlobalFree",
                    &[("hMem", &hMem)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GlobalFree(machine, hMem);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GlobalFree_pos.0,
                    winapi::kernel32::GlobalFree_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GlobalReAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, stack_args + 0u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 4u32);
            let uFlags = <GMEM>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "GlobalReAlloc",
                    &[("hMem", &hMem), ("dwBytes", &dwBytes), ("uFlags", &uFlags)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::GlobalReAlloc(machine, hMem, dwBytes, uFlags);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::GlobalReAlloc_pos.0,
                    winapi::kernel32::GlobalReAlloc_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn HeapAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <Result<HeapAllocFlags, u32>>::from_stack(mem, stack_args + 4u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "HeapAlloc",
                    &[
                        ("hHeap", &hHeap),
                        ("dwFlags", &dwFlags),
                        ("dwBytes", &dwBytes),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::HeapAlloc_pos.0,
                    winapi::kernel32::HeapAlloc_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn HeapCreate(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let flOptions = <Result<HeapCreateFlags, u32>>::from_stack(mem, stack_args + 0u32);
            let dwInitialSize = <u32>::from_stack(mem, stack_args + 4u32);
            let dwMaximumSize = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "HeapCreate",
                    &[
                        ("flOptions", &flOptions),
                        ("dwInitialSize", &dwInitialSize),
                        ("dwMaximumSize", &dwMaximumSize),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, dwMaximumSize);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::HeapCreate_pos.0,
                    winapi::kernel32::HeapCreate_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn HeapDestroy(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "HeapDestroy",
                    &[("hHeap", &hHeap)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::HeapDestroy(machine, hHeap);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::HeapDestroy_pos.0,
                    winapi::kernel32::HeapDestroy_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn HeapFree(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "HeapFree",
                    &[("hHeap", &hHeap), ("dwFlags", &dwFlags), ("lpMem", &lpMem)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::HeapFree_pos.0,
                    winapi::kernel32::HeapFree_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn HeapReAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "HeapReAlloc",
                    &[
                        ("hHeap", &hHeap),
                        ("dwFlags", &dwFlags),
                        ("lpMem", &lpMem),
                        ("dwBytes", &dwBytes),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::HeapReAlloc_pos.0,
                    winapi::kernel32::HeapReAlloc_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn HeapSetInformation(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let HeapHandle = <u32>::from_stack(mem, stack_args + 0u32);
            let HeapInformationClass = <u32>::from_stack(mem, stack_args + 4u32);
            let HeapInformation = <u32>::from_stack(mem, stack_args + 8u32);
            let HeapInformationLength = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "HeapSetInformation",
                    &[
                        ("HeapHandle", &HeapHandle),
                        ("HeapInformationClass", &HeapInformationClass),
                        ("HeapInformation", &HeapInformation),
                        ("HeapInformationLength", &HeapInformationLength),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::HeapSetInformation(
                machine,
                HeapHandle,
                HeapInformationClass,
                HeapInformation,
                HeapInformationLength,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::HeapSetInformation_pos.0,
                    winapi::kernel32::HeapSetInformation_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn HeapSize(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "HeapSize",
                    &[("hHeap", &hHeap), ("dwFlags", &dwFlags), ("lpMem", &lpMem)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::HeapSize_pos.0,
                    winapi::kernel32::HeapSize_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn HeapValidate(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "HeapValidate",
                    &[("hHeap", &hHeap), ("dwFlags", &dwFlags), ("lpMem", &lpMem)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::HeapValidate(machine, hHeap, dwFlags, lpMem);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::HeapValidate_pos.0,
                    winapi::kernel32::HeapValidate_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn InitOnceBeginInitialize(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let fPending = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let lpContext = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "InitOnceBeginInitialize",
                    &[
                        ("lpInitOnce", &lpInitOnce),
                        ("dwFlags", &dwFlags),
                        ("fPending", &fPending),
                        ("lpContext", &lpContext),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::InitOnceBeginInitialize(
                machine, lpInitOnce, dwFlags, fPending, lpContext,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::InitOnceBeginInitialize_pos.0,
                    winapi::kernel32::InitOnceBeginInitialize_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn InitOnceComplete(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpContext = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "InitOnceComplete",
                    &[
                        ("lpInitOnce", &lpInitOnce),
                        ("dwFlags", &dwFlags),
                        ("lpContext", &lpContext),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::kernel32::InitOnceComplete(machine, lpInitOnce, dwFlags, lpContext);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::InitOnceComplete_pos.0,
                    winapi::kernel32::InitOnceComplete_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn InitializeCriticalSection(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "InitializeCriticalSection",
                    &[("lpCriticalSection", &lpCriticalSection)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::InitializeCriticalSection(machine, lpCriticalSection);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::InitializeCriticalSection_pos.0,
                    winapi::kernel32::InitializeCriticalSection_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn InitializeCriticalSectionAndSpinCount(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSpinCount = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "InitializeCriticalSectionAndSpinCount",
                    &[
                        ("lpCriticalSection", &lpCriticalSection),
                        ("dwSpinCount", &dwSpinCount),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::InitializeCriticalSectionAndSpinCount(
                machine,
                lpCriticalSection,
                dwSpinCount,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::InitializeCriticalSectionAndSpinCount_pos.0,
                    winapi::kernel32::InitializeCriticalSectionAndSpinCount_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn InitializeCriticalSectionEx(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSpinCount = <u32>::from_stack(mem, stack_args + 4u32);
            let flags = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "InitializeCriticalSectionEx",
                    &[
                        ("lpCriticalSection", &lpCriticalSection),
                        ("dwSpinCount", &dwSpinCount),
                        ("flags", &flags),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::InitializeCriticalSectionEx(
                machine,
                lpCriticalSection,
                dwSpinCount,
                flags,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::InitializeCriticalSectionEx_pos.0,
                    winapi::kernel32::InitializeCriticalSectionEx_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn InitializeSListHead(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let ListHead = <Option<&mut SLIST_HEADER>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "InitializeSListHead",
                    &[("ListHead", &ListHead)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::InitializeSListHead(machine, ListHead);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::InitializeSListHead_pos.0,
                    winapi::kernel32::InitializeSListHead_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn InterlockedDecrement(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let addend = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "InterlockedDecrement",
                    &[("addend", &addend)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::InterlockedDecrement(machine, addend);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::InterlockedDecrement_pos.0,
                    winapi::kernel32::InterlockedDecrement_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn InterlockedIncrement(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let addend = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "InterlockedIncrement",
                    &[("addend", &addend)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::InterlockedIncrement(machine, addend);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::InterlockedIncrement_pos.0,
                    winapi::kernel32::InterlockedIncrement_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IsBadCodePtr(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpfn = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "IsBadCodePtr",
                    &[("lpfn", &lpfn)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::IsBadCodePtr(machine, lpfn);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::IsBadCodePtr_pos.0,
                    winapi::kernel32::IsBadCodePtr_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IsBadReadPtr(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lp = <u32>::from_stack(mem, stack_args + 0u32);
            let ucb = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "IsBadReadPtr",
                    &[("lp", &lp), ("ucb", &ucb)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::IsBadReadPtr(machine, lp, ucb);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::IsBadReadPtr_pos.0,
                    winapi::kernel32::IsBadReadPtr_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IsBadWritePtr(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lp = <u32>::from_stack(mem, stack_args + 0u32);
            let ucb = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "IsBadWritePtr",
                    &[("lp", &lp), ("ucb", &ucb)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::IsBadWritePtr(machine, lp, ucb);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::IsBadWritePtr_pos.0,
                    winapi::kernel32::IsBadWritePtr_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IsDBCSLeadByte(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _TestChar = <u8>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/nls") {
                Some(crate::trace::trace_begin(
                    "kernel32/nls",
                    "IsDBCSLeadByte",
                    &[("TestChar", &_TestChar)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::IsDBCSLeadByte(machine, _TestChar);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::IsDBCSLeadByte_pos.0,
                    winapi::kernel32::IsDBCSLeadByte_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IsDBCSLeadByteEx(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _TestChar = <u8>::from_stack(mem, stack_args + 0u32);
            let _CodePage = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/nls") {
                Some(crate::trace::trace_begin(
                    "kernel32/nls",
                    "IsDBCSLeadByteEx",
                    &[("TestChar", &_TestChar), ("CodePage", &_CodePage)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::IsDBCSLeadByteEx(machine, _TestChar, _CodePage);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::IsDBCSLeadByteEx_pos.0,
                    winapi::kernel32::IsDBCSLeadByteEx_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IsDebuggerPresent(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "IsDebuggerPresent",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::IsDebuggerPresent(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::IsDebuggerPresent_pos.0,
                    winapi::kernel32::IsDebuggerPresent_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IsProcessorFeaturePresent(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let feature = <Result<ProcessorFeature, u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "IsProcessorFeaturePresent",
                    &[("feature", &feature)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::IsProcessorFeaturePresent(machine, feature);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::IsProcessorFeaturePresent_pos.0,
                    winapi::kernel32::IsProcessorFeaturePresent_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IsValidCodePage(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/nls") {
                Some(crate::trace::trace_begin(
                    "kernel32/nls",
                    "IsValidCodePage",
                    &[("CodePage", &CodePage)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::IsValidCodePage(machine, CodePage);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::IsValidCodePage_pos.0,
                    winapi::kernel32::IsValidCodePage_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LCMapStringA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let locale = <LCID>::from_stack(mem, stack_args + 0u32);
            let dwMapFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpSrcStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cchSrc = <i32>::from_stack(mem, stack_args + 12u32);
            let lpDestStr = <ArrayWithSizeMut<u8>>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("kernel32/nls") {
                Some(crate::trace::trace_begin(
                    "kernel32/nls",
                    "LCMapStringA",
                    &[
                        ("locale", &locale),
                        ("dwMapFlags", &dwMapFlags),
                        ("lpSrcStr", &lpSrcStr),
                        ("cchSrc", &cchSrc),
                        ("lpDestStr", &lpDestStr),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::LCMapStringA(
                machine, locale, dwMapFlags, lpSrcStr, cchSrc, lpDestStr,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::LCMapStringA_pos.0,
                    winapi::kernel32::LCMapStringA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LCMapStringW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let locale = <LCID>::from_stack(mem, stack_args + 0u32);
            let dwMapFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpSrcStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cchSrc = <i32>::from_stack(mem, stack_args + 12u32);
            let lpDestStr = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("kernel32/nls") {
                Some(crate::trace::trace_begin(
                    "kernel32/nls",
                    "LCMapStringW",
                    &[
                        ("locale", &locale),
                        ("dwMapFlags", &dwMapFlags),
                        ("lpSrcStr", &lpSrcStr),
                        ("cchSrc", &cchSrc),
                        ("lpDestStr", &lpDestStr),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::LCMapStringW(
                machine, locale, dwMapFlags, lpSrcStr, cchSrc, lpDestStr,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::LCMapStringW_pos.0,
                    winapi::kernel32::LCMapStringW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LeaveCriticalSection(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "LeaveCriticalSection",
                    &[("lpCriticalSection", &lpCriticalSection)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::LeaveCriticalSection(machine, lpCriticalSection);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::LeaveCriticalSection_pos.0,
                    winapi::kernel32::LeaveCriticalSection_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadLibraryA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let filename = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/dll") {
                Some(crate::trace::trace_begin(
                    "kernel32/dll",
                    "LoadLibraryA",
                    &[("filename", &filename)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::LoadLibraryA(machine, filename);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::LoadLibraryA_pos.0,
                    winapi::kernel32::LoadLibraryA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadLibraryExW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpLibFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let hFile = <HFILE>::from_stack(mem, stack_args + 4u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/dll") {
                Some(crate::trace::trace_begin(
                    "kernel32/dll",
                    "LoadLibraryExW",
                    &[
                        ("lpLibFileName", &lpLibFileName),
                        ("hFile", &hFile),
                        ("dwFlags", &dwFlags),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::LoadLibraryExW_pos.0,
                    winapi::kernel32::LoadLibraryExW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadResource(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let hResInfo = <HRSRC>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/resource") {
                Some(crate::trace::trace_begin(
                    "kernel32/resource",
                    "LoadResource",
                    &[("hModule", &hModule), ("hResInfo", &hResInfo)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::LoadResource(machine, hModule, hResInfo);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::LoadResource_pos.0,
                    winapi::kernel32::LoadResource_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LocalAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uFlags = <GMEM>::from_stack(mem, stack_args + 0u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "LocalAlloc",
                    &[("uFlags", &uFlags), ("dwBytes", &dwBytes)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::LocalAlloc(machine, uFlags, dwBytes);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::LocalAlloc_pos.0,
                    winapi::kernel32::LocalAlloc_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LocalFree(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "LocalFree",
                    &[("hMem", &hMem)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::LocalFree(machine, hMem);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::LocalFree_pos.0,
                    winapi::kernel32::LocalFree_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LockResource(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hResData = <HRSRC>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/resource") {
                Some(crate::trace::trace_begin(
                    "kernel32/resource",
                    "LockResource",
                    &[("hResData", &hResData)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::LockResource(machine, hResData);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::LockResource_pos.0,
                    winapi::kernel32::LockResource_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn MoveFileA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpExistingFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpNewFileName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "MoveFileA",
                    &[
                        ("lpExistingFileName", &lpExistingFileName),
                        ("lpNewFileName", &lpNewFileName),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::MoveFileA(machine, lpExistingFileName, lpNewFileName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::MoveFileA_pos.0,
                    winapi::kernel32::MoveFileA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn MulDiv(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nNumber = <i32>::from_stack(mem, stack_args + 0u32);
            let nNumerator = <i32>::from_stack(mem, stack_args + 4u32);
            let nDenominator = <i32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "MulDiv",
                    &[
                        ("nNumber", &nNumber),
                        ("nNumerator", &nNumerator),
                        ("nDenominator", &nDenominator),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::MulDiv(machine, nNumber, nNumerator, nDenominator);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::MulDiv_pos.0,
                    winapi::kernel32::MulDiv_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn MultiByteToWideChar(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <Result<CP, u32>>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <Result<MB, u32>>::from_stack(mem, stack_args + 4u32);
            let lpMultiByteStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cbMultiByte = <i32>::from_stack(mem, stack_args + 12u32);
            let lpWideCharStr = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("kernel32/nls") {
                Some(crate::trace::trace_begin(
                    "kernel32/nls",
                    "MultiByteToWideChar",
                    &[
                        ("CodePage", &CodePage),
                        ("dwFlags", &dwFlags),
                        ("lpMultiByteStr", &lpMultiByteStr),
                        ("cbMultiByte", &cbMultiByte),
                        ("lpWideCharStr", &lpWideCharStr),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::MultiByteToWideChar(
                machine,
                CodePage,
                dwFlags,
                lpMultiByteStr,
                cbMultiByte,
                lpWideCharStr,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::MultiByteToWideChar_pos.0,
                    winapi::kernel32::MultiByteToWideChar_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn NtCurrentTeb(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "NtCurrentTeb",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::NtCurrentTeb(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::NtCurrentTeb_pos.0,
                    winapi::kernel32::NtCurrentTeb_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn OutputDebugStringA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let msg = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "OutputDebugStringA",
                    &[("msg", &msg)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::OutputDebugStringA(machine, msg);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::OutputDebugStringA_pos.0,
                    winapi::kernel32::OutputDebugStringA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn PulseEvent(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hEvent = <HEVENT>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/sync") {
                Some(crate::trace::trace_begin(
                    "kernel32/sync",
                    "PulseEvent",
                    &[("hEvent", &hEvent)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::PulseEvent(machine, hEvent);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::PulseEvent_pos.0,
                    winapi::kernel32::PulseEvent_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn QueryPerformanceCounter(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPerformanceCount =
                <Option<&mut LARGE_INTEGER>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/time") {
                Some(crate::trace::trace_begin(
                    "kernel32/time",
                    "QueryPerformanceCounter",
                    &[("lpPerformanceCount", &lpPerformanceCount)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::QueryPerformanceCounter(machine, lpPerformanceCount);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::QueryPerformanceCounter_pos.0,
                    winapi::kernel32::QueryPerformanceCounter_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn QueryPerformanceFrequency(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFrequency = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/time") {
                Some(crate::trace::trace_begin(
                    "kernel32/time",
                    "QueryPerformanceFrequency",
                    &[("lpFrequency", &lpFrequency)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::QueryPerformanceFrequency(machine, lpFrequency);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::QueryPerformanceFrequency_pos.0,
                    winapi::kernel32::QueryPerformanceFrequency_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RaiseException(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwExceptionCode = <u32>::from_stack(mem, stack_args + 0u32);
            let dwExceptionFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let nNumberOfArguments = <u32>::from_stack(mem, stack_args + 8u32);
            let lpArguments = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "RaiseException",
                    &[
                        ("dwExceptionCode", &dwExceptionCode),
                        ("dwExceptionFlags", &dwExceptionFlags),
                        ("nNumberOfArguments", &nNumberOfArguments),
                        ("lpArguments", &lpArguments),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::RaiseException(
                machine,
                dwExceptionCode,
                dwExceptionFlags,
                nNumberOfArguments,
                lpArguments,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::RaiseException_pos.0,
                    winapi::kernel32::RaiseException_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn ReadFile(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <ArrayWithSizeMut<u8>>::from_stack(mem, stack_args + 4u32);
            let lpNumberOfBytesRead = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpOverlapped = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "ReadFile",
                    &[
                        ("hFile", &hFile),
                        ("lpBuffer", &lpBuffer),
                        ("lpNumberOfBytesRead", &lpNumberOfBytesRead),
                        ("lpOverlapped", &lpOverlapped),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::ReadFile(
                machine,
                hFile,
                lpBuffer,
                lpNumberOfBytesRead,
                lpOverlapped,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::ReadFile_pos.0,
                    winapi::kernel32::ReadFile_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn ReleaseSRWLockExclusive(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "ReleaseSRWLockExclusive",
                    &[("SRWLock", &SRWLock)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::ReleaseSRWLockExclusive(machine, SRWLock);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::ReleaseSRWLockExclusive_pos.0,
                    winapi::kernel32::ReleaseSRWLockExclusive_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn ReleaseSRWLockShared(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "ReleaseSRWLockShared",
                    &[("SRWLock", &SRWLock)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::ReleaseSRWLockShared(machine, SRWLock);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::ReleaseSRWLockShared_pos.0,
                    winapi::kernel32::ReleaseSRWLockShared_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RemoveDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "RemoveDirectoryA",
                    &[("lpPathName", &lpPathName)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::RemoveDirectoryA(machine, lpPathName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::RemoveDirectoryA_pos.0,
                    winapi::kernel32::RemoveDirectoryA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn ResetEvent(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hEvent = <HEVENT>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/sync") {
                Some(crate::trace::trace_begin(
                    "kernel32/sync",
                    "ResetEvent",
                    &[("hEvent", &hEvent)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::ResetEvent(machine, hEvent);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::ResetEvent_pos.0,
                    winapi::kernel32::ResetEvent_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn ResumeThread(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "ResumeThread",
                    &[("hThread", &hThread)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::ResumeThread(machine, hThread);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::ResumeThread_pos.0,
                    winapi::kernel32::ResumeThread_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RtlUnwind(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let TargetFrame = <u32>::from_stack(mem, stack_args + 0u32);
            let TargetIp = <u32>::from_stack(mem, stack_args + 4u32);
            let ExceptionRecord = <u32>::from_stack(mem, stack_args + 8u32);
            let ReturnValue = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "RtlUnwind",
                    &[
                        ("TargetFrame", &TargetFrame),
                        ("TargetIp", &TargetIp),
                        ("ExceptionRecord", &ExceptionRecord),
                        ("ReturnValue", &ReturnValue),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::RtlUnwind(
                machine,
                TargetFrame,
                TargetIp,
                ExceptionRecord,
                ReturnValue,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::RtlUnwind_pos.0,
                    winapi::kernel32::RtlUnwind_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetConsoleCtrlHandler(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _handlerRoutine = <DWORD>::from_stack(mem, stack_args + 0u32);
            let _add = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/console") {
                Some(crate::trace::trace_begin(
                    "kernel32/console",
                    "SetConsoleCtrlHandler",
                    &[("handlerRoutine", &_handlerRoutine), ("add", &_add)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SetConsoleCtrlHandler(machine, _handlerRoutine, _add);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetConsoleCtrlHandler_pos.0,
                    winapi::kernel32::SetConsoleCtrlHandler_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetCurrentDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "SetCurrentDirectoryA",
                    &[("lpPathName", &lpPathName)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SetCurrentDirectoryA(machine, lpPathName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetCurrentDirectoryA_pos.0,
                    winapi::kernel32::SetCurrentDirectoryA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetEndOfFile(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "SetEndOfFile",
                    &[("hFile", &hFile)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SetEndOfFile(machine, hFile);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetEndOfFile_pos.0,
                    winapi::kernel32::SetEndOfFile_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetEnvironmentVariableA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let value = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/env") {
                Some(crate::trace::trace_begin(
                    "kernel32/env",
                    "SetEnvironmentVariableA",
                    &[("name", &name), ("value", &value)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SetEnvironmentVariableA(machine, name, value);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetEnvironmentVariableA_pos.0,
                    winapi::kernel32::SetEnvironmentVariableA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetEvent(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hEvent = <HEVENT>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/sync") {
                Some(crate::trace::trace_begin(
                    "kernel32/sync",
                    "SetEvent",
                    &[("hEvent", &hEvent)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SetEvent(machine, hEvent);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetEvent_pos.0,
                    winapi::kernel32::SetEvent_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetFileAttributesA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let dwFileAttributes = <Result<FileAttribute, u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "SetFileAttributesA",
                    &[
                        ("lpFileName", &lpFileName),
                        ("dwFileAttributes", &dwFileAttributes),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::kernel32::SetFileAttributesA(machine, lpFileName, dwFileAttributes);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetFileAttributesA_pos.0,
                    winapi::kernel32::SetFileAttributesA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetFilePointer(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lDistanceToMove = <i32>::from_stack(mem, stack_args + 4u32);
            let lpDistanceToMoveHigh = <Option<&mut i32>>::from_stack(mem, stack_args + 8u32);
            let dwMoveMethod = <Result<FILE, u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "SetFilePointer",
                    &[
                        ("hFile", &hFile),
                        ("lDistanceToMove", &lDistanceToMove),
                        ("lpDistanceToMoveHigh", &lpDistanceToMoveHigh),
                        ("dwMoveMethod", &dwMoveMethod),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SetFilePointer(
                machine,
                hFile,
                lDistanceToMove,
                lpDistanceToMoveHigh,
                dwMoveMethod,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetFilePointer_pos.0,
                    winapi::kernel32::SetFilePointer_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetFileTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpCreationTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 4u32);
            let lpLastAccessTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 8u32);
            let lpLastWriteTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "SetFileTime",
                    &[
                        ("hFile", &hFile),
                        ("lpCreationTime", &lpCreationTime),
                        ("lpLastAccessTime", &lpLastAccessTime),
                        ("lpLastWriteTime", &lpLastWriteTime),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SetFileTime(
                machine,
                hFile,
                lpCreationTime,
                lpLastAccessTime,
                lpLastWriteTime,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetFileTime_pos.0,
                    winapi::kernel32::SetFileTime_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetHandleCount(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uNumber = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "SetHandleCount",
                    &[("uNumber", &uNumber)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SetHandleCount(machine, uNumber);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetHandleCount_pos.0,
                    winapi::kernel32::SetHandleCount_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetLastError(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwErrCode = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "SetLastError",
                    &[("dwErrCode", &dwErrCode)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SetLastError(machine, dwErrCode);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetLastError_pos.0,
                    winapi::kernel32::SetLastError_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetPriorityClass(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hProcess = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let dwPriorityClass = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "SetPriorityClass",
                    &[
                        ("hProcess", &hProcess),
                        ("dwPriorityClass", &dwPriorityClass),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SetPriorityClass(machine, hProcess, dwPriorityClass);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetPriorityClass_pos.0,
                    winapi::kernel32::SetPriorityClass_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetStdHandle(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nStdHandle = <Result<STD, u32>>::from_stack(mem, stack_args + 0u32);
            let hHandle = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "SetStdHandle",
                    &[("nStdHandle", &nStdHandle), ("hHandle", &hHandle)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SetStdHandle(machine, nStdHandle, hHandle);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetStdHandle_pos.0,
                    winapi::kernel32::SetStdHandle_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetThreadDescription(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
            let lpThreadDescription = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "SetThreadDescription",
                    &[
                        ("hThread", &hThread),
                        ("lpThreadDescription", &lpThreadDescription),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::kernel32::SetThreadDescription(machine, hThread, lpThreadDescription);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetThreadDescription_pos.0,
                    winapi::kernel32::SetThreadDescription_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetThreadPriority(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
            let nPriority = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "SetThreadPriority",
                    &[("hThread", &hThread), ("nPriority", &nPriority)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SetThreadPriority(machine, hThread, nPriority);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetThreadPriority_pos.0,
                    winapi::kernel32::SetThreadPriority_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetThreadStackGuarantee(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let StackSizeInBytes = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "SetThreadStackGuarantee",
                    &[("StackSizeInBytes", &StackSizeInBytes)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SetThreadStackGuarantee(machine, StackSizeInBytes);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetThreadStackGuarantee_pos.0,
                    winapi::kernel32::SetThreadStackGuarantee_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetUnhandledExceptionFilter(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _lpTopLevelExceptionFilter = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "SetUnhandledExceptionFilter",
                    &[("lpTopLevelExceptionFilter", &_lpTopLevelExceptionFilter)],
                ))
            } else {
                None
            };
            let result =
                winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SetUnhandledExceptionFilter_pos.0,
                    winapi::kernel32::SetUnhandledExceptionFilter_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SizeofResource(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let hResInfo = <HRSRC>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/resource") {
                Some(crate::trace::trace_begin(
                    "kernel32/resource",
                    "SizeofResource",
                    &[("hModule", &hModule), ("hResInfo", &hResInfo)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SizeofResource(machine, hModule, hResInfo);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SizeofResource_pos.0,
                    winapi::kernel32::SizeofResource_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn Sleep(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let dwMilliseconds = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/time") {
                Some(crate::trace::trace_begin(
                    "kernel32/time",
                    "Sleep",
                    &[("dwMilliseconds", &dwMilliseconds)],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::kernel32::Sleep(machine, dwMilliseconds).await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::kernel32::Sleep_pos.0,
                        winapi::kernel32::Sleep_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn SystemTimeToFileTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&SYSTEMTIME>>::from_stack(mem, stack_args + 0u32);
            let lpFileTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/time") {
                Some(crate::trace::trace_begin(
                    "kernel32/time",
                    "SystemTimeToFileTime",
                    &[("lpSystemTime", &lpSystemTime), ("lpFileTime", &lpFileTime)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::SystemTimeToFileTime(machine, lpSystemTime, lpFileTime);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::SystemTimeToFileTime_pos.0,
                    winapi::kernel32::SystemTimeToFileTime_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn TerminateProcess(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hProcess = <u32>::from_stack(mem, stack_args + 0u32);
            let uExitCode = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "TerminateProcess",
                    &[("hProcess", &hProcess), ("uExitCode", &uExitCode)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::TerminateProcess(machine, hProcess, uExitCode);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::TerminateProcess_pos.0,
                    winapi::kernel32::TerminateProcess_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn TlsAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "TlsAlloc",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::TlsAlloc(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::TlsAlloc_pos.0,
                    winapi::kernel32::TlsAlloc_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn TlsFree(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "TlsFree",
                    &[("dwTlsIndex", &dwTlsIndex)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::TlsFree(machine, dwTlsIndex);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::TlsFree_pos.0,
                    winapi::kernel32::TlsFree_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn TlsGetValue(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "TlsGetValue",
                    &[("dwTlsIndex", &dwTlsIndex)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::TlsGetValue(machine, dwTlsIndex);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::TlsGetValue_pos.0,
                    winapi::kernel32::TlsGetValue_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn TlsSetValue(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, stack_args + 0u32);
            let lpTlsValue = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "TlsSetValue",
                    &[("dwTlsIndex", &dwTlsIndex), ("lpTlsValue", &lpTlsValue)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::TlsSetValue_pos.0,
                    winapi::kernel32::TlsSetValue_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn TryAcquireSRWLockExclusive(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/thread") {
                Some(crate::trace::trace_begin(
                    "kernel32/thread",
                    "TryAcquireSRWLockExclusive",
                    &[("SRWLock", &SRWLock)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::TryAcquireSRWLockExclusive(machine, SRWLock);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::TryAcquireSRWLockExclusive_pos.0,
                    winapi::kernel32::TryAcquireSRWLockExclusive_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn UnhandledExceptionFilter(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _exceptionInfo = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/misc") {
                Some(crate::trace::trace_begin(
                    "kernel32/misc",
                    "UnhandledExceptionFilter",
                    &[("exceptionInfo", &_exceptionInfo)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::UnhandledExceptionFilter_pos.0,
                    winapi::kernel32::UnhandledExceptionFilter_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn VirtualAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSize = <u32>::from_stack(mem, stack_args + 4u32);
            let flAllocationType = <Result<MEM, u32>>::from_stack(mem, stack_args + 8u32);
            let flProtec = <Result<PAGE, u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "VirtualAlloc",
                    &[
                        ("lpAddress", &lpAddress),
                        ("dwSize", &dwSize),
                        ("flAllocationType", &flAllocationType),
                        ("flProtec", &flProtec),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::VirtualAlloc(
                machine,
                lpAddress,
                dwSize,
                flAllocationType,
                flProtec,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::VirtualAlloc_pos.0,
                    winapi::kernel32::VirtualAlloc_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn VirtualFree(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSize = <u32>::from_stack(mem, stack_args + 4u32);
            let dwFreeType = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "VirtualFree",
                    &[
                        ("lpAddress", &lpAddress),
                        ("dwSize", &dwSize),
                        ("dwFreeType", &dwFreeType),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::VirtualFree_pos.0,
                    winapi::kernel32::VirtualFree_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn VirtualProtect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSize = <u32>::from_stack(mem, stack_args + 4u32);
            let flNewProtect = <u32>::from_stack(mem, stack_args + 8u32);
            let lpflOldProtect = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "VirtualProtect",
                    &[
                        ("lpAddress", &lpAddress),
                        ("dwSize", &dwSize),
                        ("flNewProtect", &flNewProtect),
                        ("lpflOldProtect", &lpflOldProtect),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::VirtualProtect(
                machine,
                lpAddress,
                dwSize,
                flNewProtect,
                lpflOldProtect,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::VirtualProtect_pos.0,
                    winapi::kernel32::VirtualProtect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn VirtualQuery(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
            let lpBuffer =
                <Option<&mut MEMORY_BASIC_INFORMATION>>::from_stack(mem, stack_args + 4u32);
            let dwLength = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/memory") {
                Some(crate::trace::trace_begin(
                    "kernel32/memory",
                    "VirtualQuery",
                    &[
                        ("lpAddress", &lpAddress),
                        ("lpBuffer", &lpBuffer),
                        ("dwLength", &dwLength),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::VirtualQuery(machine, lpAddress, lpBuffer, dwLength);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::VirtualQuery_pos.0,
                    winapi::kernel32::VirtualQuery_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn WaitForMultipleObjects(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nCount = <u32>::from_stack(mem, stack_args + 0u32);
            let lpHandles = <Option<&mut HANDLE<()>>>::from_stack(mem, stack_args + 4u32);
            let bWaitAll = <bool>::from_stack(mem, stack_args + 8u32);
            let dwMilliseconds = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/sync") {
                Some(crate::trace::trace_begin(
                    "kernel32/sync",
                    "WaitForMultipleObjects",
                    &[
                        ("nCount", &nCount),
                        ("lpHandles", &lpHandles),
                        ("bWaitAll", &bWaitAll),
                        ("dwMilliseconds", &dwMilliseconds),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::WaitForMultipleObjects(
                machine,
                nCount,
                lpHandles,
                bWaitAll,
                dwMilliseconds,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::WaitForMultipleObjects_pos.0,
                    winapi::kernel32::WaitForMultipleObjects_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn WaitForSingleObject(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHandle = <HEVENT>::from_stack(mem, stack_args + 0u32);
            let dwMilliseconds = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/sync") {
                Some(crate::trace::trace_begin(
                    "kernel32/sync",
                    "WaitForSingleObject",
                    &[("hHandle", &hHandle), ("dwMilliseconds", &dwMilliseconds)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::WaitForSingleObject(machine, hHandle, dwMilliseconds);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::WaitForSingleObject_pos.0,
                    winapi::kernel32::WaitForSingleObject_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn WideCharToMultiByte(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <Result<CP, u32>>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <Result<WC, u32>>::from_stack(mem, stack_args + 4u32);
            let lpWideCharStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cchWideChar = <i32>::from_stack(mem, stack_args + 12u32);
            let lpMultiByteStr = <u32>::from_stack(mem, stack_args + 16u32);
            let cbMultiByte = <i32>::from_stack(mem, stack_args + 20u32);
            let lpUsedDefaultChar = <Option<&mut u32>>::from_stack(mem, stack_args + 24u32);
            let __trace_context = if crate::trace::enabled("kernel32/nls") {
                Some(crate::trace::trace_begin(
                    "kernel32/nls",
                    "WideCharToMultiByte",
                    &[
                        ("CodePage", &CodePage),
                        ("dwFlags", &dwFlags),
                        ("lpWideCharStr", &lpWideCharStr),
                        ("cchWideChar", &cchWideChar),
                        ("lpMultiByteStr", &lpMultiByteStr),
                        ("cbMultiByte", &cbMultiByte),
                        ("lpUsedDefaultChar", &lpUsedDefaultChar),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::WideCharToMultiByte(
                machine,
                CodePage,
                dwFlags,
                lpWideCharStr,
                cchWideChar,
                lpMultiByteStr,
                cbMultiByte,
                lpUsedDefaultChar,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::WideCharToMultiByte_pos.0,
                    winapi::kernel32::WideCharToMultiByte_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn WriteConsoleA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleOutput = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, stack_args + 4u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpReserved = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("kernel32/console") {
                Some(crate::trace::trace_begin(
                    "kernel32/console",
                    "WriteConsoleA",
                    &[
                        ("hConsoleOutput", &hConsoleOutput),
                        ("lpBuffer", &lpBuffer),
                        ("lpNumberOfCharsWritten", &lpNumberOfCharsWritten),
                        ("lpReserved", &lpReserved),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::WriteConsoleA(
                machine,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                lpReserved,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::WriteConsoleA_pos.0,
                    winapi::kernel32::WriteConsoleA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn WriteConsoleW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleOutput = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <ArrayWithSize<u16>>::from_stack(mem, stack_args + 4u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let _lpReserved = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("kernel32/console") {
                Some(crate::trace::trace_begin(
                    "kernel32/console",
                    "WriteConsoleW",
                    &[
                        ("hConsoleOutput", &hConsoleOutput),
                        ("lpBuffer", &lpBuffer),
                        ("lpNumberOfCharsWritten", &lpNumberOfCharsWritten),
                        ("lpReserved", &_lpReserved),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::WriteConsoleW(
                machine,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                _lpReserved,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::WriteConsoleW_pos.0,
                    winapi::kernel32::WriteConsoleW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn WriteFile(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, stack_args + 4u32);
            let lpNumberOfBytesWritten = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpOverlapped = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("kernel32/file") {
                Some(crate::trace::trace_begin(
                    "kernel32/file",
                    "WriteFile",
                    &[
                        ("hFile", &hFile),
                        ("lpBuffer", &lpBuffer),
                        ("lpNumberOfBytesWritten", &lpNumberOfBytesWritten),
                        ("lpOverlapped", &lpOverlapped),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::WriteFile(
                machine,
                hFile,
                lpBuffer,
                lpNumberOfBytesWritten,
                lpOverlapped,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::WriteFile_pos.0,
                    winapi::kernel32::WriteFile_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn WritePrivateProfileStringA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("kernel32/ini") {
                Some(crate::trace::trace_begin(
                    "kernel32/ini",
                    "WritePrivateProfileStringA",
                    &[
                        ("lpAppName", &lpAppName),
                        ("lpKeyName", &lpKeyName),
                        ("lpString", &lpString),
                        ("lpFileName", &lpFileName),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::WritePrivateProfileStringA(
                machine, lpAppName, lpKeyName, lpString, lpFileName,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::WritePrivateProfileStringA_pos.0,
                    winapi::kernel32::WritePrivateProfileStringA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn WriteProfileStringW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/ini") {
                Some(crate::trace::trace_begin(
                    "kernel32/ini",
                    "WriteProfileStringW",
                    &[
                        ("lpAppName", &lpAppName),
                        ("lpKeyName", &lpKeyName),
                        ("lpString", &lpString),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::kernel32::WriteProfileStringW(machine, lpAppName, lpKeyName, lpString);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::WriteProfileStringW_pos.0,
                    winapi::kernel32::WriteProfileStringW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn _lclose(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/file16") {
                Some(crate::trace::trace_begin(
                    "kernel32/file16",
                    "_lclose",
                    &[("hFile", &hFile)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::_lclose(machine, hFile);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::_lclose_pos.0,
                    winapi::kernel32::_lclose_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn _llseek(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lOffset = <i32>::from_stack(mem, stack_args + 4u32);
            let iOrigin = <i32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("kernel32/file16") {
                Some(crate::trace::trace_begin(
                    "kernel32/file16",
                    "_llseek",
                    &[
                        ("hFile", &hFile),
                        ("lOffset", &lOffset),
                        ("iOrigin", &iOrigin),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::_llseek(machine, hFile, lOffset, iOrigin);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::_llseek_pos.0,
                    winapi::kernel32::_llseek_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn _lopen(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let iReadWrite = <i32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/file16") {
                Some(crate::trace::trace_begin(
                    "kernel32/file16",
                    "_lopen",
                    &[("lpPathName", &lpPathName), ("iReadWrite", &iReadWrite)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::_lopen(machine, lpPathName, iReadWrite);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::_lopen_pos.0,
                    winapi::kernel32::_lopen_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn _lread(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/file16") {
                Some(crate::trace::trace_begin(
                    "kernel32/file16",
                    "_lread",
                    &[("hFile", &hFile), ("lpBuffer", &lpBuffer)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::_lread(machine, hFile, lpBuffer);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::_lread_pos.0,
                    winapi::kernel32::_lread_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn lstrcmpiA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpString2 = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/libc") {
                Some(crate::trace::trace_begin(
                    "kernel32/libc",
                    "lstrcmpiA",
                    &[("lpString1", &lpString1), ("lpString2", &lpString2)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::lstrcmpiA(machine, lpString1, lpString2);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::lstrcmpiA_pos.0,
                    winapi::kernel32::lstrcmpiA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn lstrcpyA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <u32>::from_stack(mem, stack_args + 0u32);
            let lpString2 = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/libc") {
                Some(crate::trace::trace_begin(
                    "kernel32/libc",
                    "lstrcpyA",
                    &[("lpString1", &lpString1), ("lpString2", &lpString2)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::lstrcpyA(machine, lpString1, lpString2);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::lstrcpyA_pos.0,
                    winapi::kernel32::lstrcpyA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn lstrcpyW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <u32>::from_stack(mem, stack_args + 0u32);
            let lpString2 = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/libc") {
                Some(crate::trace::trace_begin(
                    "kernel32/libc",
                    "lstrcpyW",
                    &[("lpString1", &lpString1), ("lpString2", &lpString2)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::lstrcpyW(machine, lpString1, lpString2);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::lstrcpyW_pos.0,
                    winapi::kernel32::lstrcpyW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn lstrlenA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/libc") {
                Some(crate::trace::trace_begin(
                    "kernel32/libc",
                    "lstrlenA",
                    &[("lpString", &lpString)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::lstrlenA(machine, lpString);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::lstrlenA_pos.0,
                    winapi::kernel32::lstrlenA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn lstrlenW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/libc") {
                Some(crate::trace::trace_begin(
                    "kernel32/libc",
                    "lstrlenW",
                    &[("lpString", &lpString)],
                ))
            } else {
                None
            };
            let result = winapi::kernel32::lstrlenW(machine, lpString);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::lstrlenW_pos.0,
                    winapi::kernel32::lstrlenW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn retrowin32_main(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let entry_point = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("kernel32/init") {
                Some(crate::trace::trace_begin(
                    "kernel32/init",
                    "retrowin32_main",
                    &[("entry_point", &entry_point)],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::kernel32::retrowin32_main(machine, entry_point).await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::kernel32::retrowin32_main_pos.0,
                        winapi::kernel32::retrowin32_main_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn retrowin32_thread_main(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let entry_point = <u32>::from_stack(mem, stack_args + 0u32);
            let param = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("kernel32/init") {
                Some(crate::trace::trace_begin(
                    "kernel32/init",
                    "retrowin32_thread_main",
                    &[("entry_point", &entry_point), ("param", &param)],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result =
                    winapi::kernel32::retrowin32_thread_main(machine, entry_point, param).await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::kernel32::retrowin32_thread_main_pos.0,
                        winapi::kernel32::retrowin32_thread_main_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
    }
    const SHIMS: [Shim; 190usize] = [
        Shim {
            name: "AcquireSRWLockExclusive",
            func: Handler::Sync(wrappers::AcquireSRWLockExclusive),
        },
        Shim {
            name: "AcquireSRWLockShared",
            func: Handler::Sync(wrappers::AcquireSRWLockShared),
        },
        Shim {
            name: "AddVectoredExceptionHandler",
            func: Handler::Sync(wrappers::AddVectoredExceptionHandler),
        },
        Shim {
            name: "CloseHandle",
            func: Handler::Sync(wrappers::CloseHandle),
        },
        Shim {
            name: "CompareStringA",
            func: Handler::Sync(wrappers::CompareStringA),
        },
        Shim {
            name: "CompareStringW",
            func: Handler::Sync(wrappers::CompareStringW),
        },
        Shim {
            name: "CreateDirectoryA",
            func: Handler::Sync(wrappers::CreateDirectoryA),
        },
        Shim {
            name: "CreateEventA",
            func: Handler::Sync(wrappers::CreateEventA),
        },
        Shim {
            name: "CreateFileA",
            func: Handler::Sync(wrappers::CreateFileA),
        },
        Shim {
            name: "CreateFileW",
            func: Handler::Sync(wrappers::CreateFileW),
        },
        Shim {
            name: "CreateProcessA",
            func: Handler::Sync(wrappers::CreateProcessA),
        },
        Shim {
            name: "CreateThread",
            func: Handler::Async(wrappers::CreateThread),
        },
        Shim {
            name: "DebugBreak",
            func: Handler::Sync(wrappers::DebugBreak),
        },
        Shim {
            name: "DeleteCriticalSection",
            func: Handler::Sync(wrappers::DeleteCriticalSection),
        },
        Shim {
            name: "DeleteFileA",
            func: Handler::Sync(wrappers::DeleteFileA),
        },
        Shim {
            name: "DisableThreadLibraryCalls",
            func: Handler::Sync(wrappers::DisableThreadLibraryCalls),
        },
        Shim {
            name: "DuplicateHandle",
            func: Handler::Sync(wrappers::DuplicateHandle),
        },
        Shim {
            name: "EnterCriticalSection",
            func: Handler::Sync(wrappers::EnterCriticalSection),
        },
        Shim {
            name: "ExitProcess",
            func: Handler::Sync(wrappers::ExitProcess),
        },
        Shim {
            name: "ExitThread",
            func: Handler::Sync(wrappers::ExitThread),
        },
        Shim {
            name: "FileTimeToLocalFileTime",
            func: Handler::Sync(wrappers::FileTimeToLocalFileTime),
        },
        Shim {
            name: "FileTimeToSystemTime",
            func: Handler::Sync(wrappers::FileTimeToSystemTime),
        },
        Shim {
            name: "FindClose",
            func: Handler::Sync(wrappers::FindClose),
        },
        Shim {
            name: "FindFirstFileA",
            func: Handler::Sync(wrappers::FindFirstFileA),
        },
        Shim {
            name: "FindNextFileA",
            func: Handler::Sync(wrappers::FindNextFileA),
        },
        Shim {
            name: "FindResourceA",
            func: Handler::Sync(wrappers::FindResourceA),
        },
        Shim {
            name: "FindResourceW",
            func: Handler::Sync(wrappers::FindResourceW),
        },
        Shim {
            name: "FlushFileBuffers",
            func: Handler::Sync(wrappers::FlushFileBuffers),
        },
        Shim {
            name: "FormatMessageA",
            func: Handler::Sync(wrappers::FormatMessageA),
        },
        Shim {
            name: "FormatMessageW",
            func: Handler::Sync(wrappers::FormatMessageW),
        },
        Shim {
            name: "FreeEnvironmentStringsA",
            func: Handler::Sync(wrappers::FreeEnvironmentStringsA),
        },
        Shim {
            name: "FreeEnvironmentStringsW",
            func: Handler::Sync(wrappers::FreeEnvironmentStringsW),
        },
        Shim {
            name: "FreeLibrary",
            func: Handler::Sync(wrappers::FreeLibrary),
        },
        Shim {
            name: "GetACP",
            func: Handler::Sync(wrappers::GetACP),
        },
        Shim {
            name: "GetCPInfo",
            func: Handler::Sync(wrappers::GetCPInfo),
        },
        Shim {
            name: "GetCommandLineA",
            func: Handler::Sync(wrappers::GetCommandLineA),
        },
        Shim {
            name: "GetCommandLineW",
            func: Handler::Sync(wrappers::GetCommandLineW),
        },
        Shim {
            name: "GetConsoleMode",
            func: Handler::Sync(wrappers::GetConsoleMode),
        },
        Shim {
            name: "GetConsoleScreenBufferInfo",
            func: Handler::Sync(wrappers::GetConsoleScreenBufferInfo),
        },
        Shim {
            name: "GetCurrentDirectoryA",
            func: Handler::Sync(wrappers::GetCurrentDirectoryA),
        },
        Shim {
            name: "GetCurrentProcess",
            func: Handler::Sync(wrappers::GetCurrentProcess),
        },
        Shim {
            name: "GetCurrentProcessId",
            func: Handler::Sync(wrappers::GetCurrentProcessId),
        },
        Shim {
            name: "GetCurrentThread",
            func: Handler::Sync(wrappers::GetCurrentThread),
        },
        Shim {
            name: "GetCurrentThreadId",
            func: Handler::Sync(wrappers::GetCurrentThreadId),
        },
        Shim {
            name: "GetDiskFreeSpaceA",
            func: Handler::Sync(wrappers::GetDiskFreeSpaceA),
        },
        Shim {
            name: "GetDriveTypeA",
            func: Handler::Sync(wrappers::GetDriveTypeA),
        },
        Shim {
            name: "GetEnvironmentStrings",
            func: Handler::Sync(wrappers::GetEnvironmentStrings),
        },
        Shim {
            name: "GetEnvironmentStringsW",
            func: Handler::Sync(wrappers::GetEnvironmentStringsW),
        },
        Shim {
            name: "GetEnvironmentVariableA",
            func: Handler::Sync(wrappers::GetEnvironmentVariableA),
        },
        Shim {
            name: "GetEnvironmentVariableW",
            func: Handler::Sync(wrappers::GetEnvironmentVariableW),
        },
        Shim {
            name: "GetExitCodeProcess",
            func: Handler::Sync(wrappers::GetExitCodeProcess),
        },
        Shim {
            name: "GetFileAttributesA",
            func: Handler::Sync(wrappers::GetFileAttributesA),
        },
        Shim {
            name: "GetFileInformationByHandle",
            func: Handler::Sync(wrappers::GetFileInformationByHandle),
        },
        Shim {
            name: "GetFileSize",
            func: Handler::Sync(wrappers::GetFileSize),
        },
        Shim {
            name: "GetFileTime",
            func: Handler::Sync(wrappers::GetFileTime),
        },
        Shim {
            name: "GetFileType",
            func: Handler::Sync(wrappers::GetFileType),
        },
        Shim {
            name: "GetFullPathNameA",
            func: Handler::Sync(wrappers::GetFullPathNameA),
        },
        Shim {
            name: "GetFullPathNameW",
            func: Handler::Sync(wrappers::GetFullPathNameW),
        },
        Shim {
            name: "GetLastError",
            func: Handler::Sync(wrappers::GetLastError),
        },
        Shim {
            name: "GetLocalTime",
            func: Handler::Sync(wrappers::GetLocalTime),
        },
        Shim {
            name: "GetLogicalDrives",
            func: Handler::Sync(wrappers::GetLogicalDrives),
        },
        Shim {
            name: "GetModuleFileNameA",
            func: Handler::Sync(wrappers::GetModuleFileNameA),
        },
        Shim {
            name: "GetModuleFileNameW",
            func: Handler::Sync(wrappers::GetModuleFileNameW),
        },
        Shim {
            name: "GetModuleHandleA",
            func: Handler::Sync(wrappers::GetModuleHandleA),
        },
        Shim {
            name: "GetModuleHandleExW",
            func: Handler::Sync(wrappers::GetModuleHandleExW),
        },
        Shim {
            name: "GetModuleHandleW",
            func: Handler::Sync(wrappers::GetModuleHandleW),
        },
        Shim {
            name: "GetOEMCP",
            func: Handler::Sync(wrappers::GetOEMCP),
        },
        Shim {
            name: "GetPrivateProfileIntW",
            func: Handler::Sync(wrappers::GetPrivateProfileIntW),
        },
        Shim {
            name: "GetPrivateProfileStringA",
            func: Handler::Sync(wrappers::GetPrivateProfileStringA),
        },
        Shim {
            name: "GetPrivateProfileStringW",
            func: Handler::Sync(wrappers::GetPrivateProfileStringW),
        },
        Shim {
            name: "GetProcAddress",
            func: Handler::Sync(wrappers::GetProcAddress),
        },
        Shim {
            name: "GetProcessHeap",
            func: Handler::Sync(wrappers::GetProcessHeap),
        },
        Shim {
            name: "GetProfileIntW",
            func: Handler::Sync(wrappers::GetProfileIntW),
        },
        Shim {
            name: "GetProfileStringW",
            func: Handler::Sync(wrappers::GetProfileStringW),
        },
        Shim {
            name: "GetStartupInfoA",
            func: Handler::Sync(wrappers::GetStartupInfoA),
        },
        Shim {
            name: "GetStartupInfoW",
            func: Handler::Sync(wrappers::GetStartupInfoW),
        },
        Shim {
            name: "GetStdHandle",
            func: Handler::Sync(wrappers::GetStdHandle),
        },
        Shim {
            name: "GetStringTypeA",
            func: Handler::Sync(wrappers::GetStringTypeA),
        },
        Shim {
            name: "GetStringTypeW",
            func: Handler::Sync(wrappers::GetStringTypeW),
        },
        Shim {
            name: "GetSystemDirectoryA",
            func: Handler::Sync(wrappers::GetSystemDirectoryA),
        },
        Shim {
            name: "GetSystemTime",
            func: Handler::Sync(wrappers::GetSystemTime),
        },
        Shim {
            name: "GetSystemTimeAsFileTime",
            func: Handler::Sync(wrappers::GetSystemTimeAsFileTime),
        },
        Shim {
            name: "GetThreadPriority",
            func: Handler::Sync(wrappers::GetThreadPriority),
        },
        Shim {
            name: "GetTickCount",
            func: Handler::Sync(wrappers::GetTickCount),
        },
        Shim {
            name: "GetTimeZoneInformation",
            func: Handler::Sync(wrappers::GetTimeZoneInformation),
        },
        Shim {
            name: "GetVersion",
            func: Handler::Sync(wrappers::GetVersion),
        },
        Shim {
            name: "GetVersionExA",
            func: Handler::Sync(wrappers::GetVersionExA),
        },
        Shim {
            name: "GetWindowsDirectoryA",
            func: Handler::Sync(wrappers::GetWindowsDirectoryA),
        },
        Shim {
            name: "GlobalAlloc",
            func: Handler::Sync(wrappers::GlobalAlloc),
        },
        Shim {
            name: "GlobalFlags",
            func: Handler::Sync(wrappers::GlobalFlags),
        },
        Shim {
            name: "GlobalFree",
            func: Handler::Sync(wrappers::GlobalFree),
        },
        Shim {
            name: "GlobalReAlloc",
            func: Handler::Sync(wrappers::GlobalReAlloc),
        },
        Shim {
            name: "HeapAlloc",
            func: Handler::Sync(wrappers::HeapAlloc),
        },
        Shim {
            name: "HeapCreate",
            func: Handler::Sync(wrappers::HeapCreate),
        },
        Shim {
            name: "HeapDestroy",
            func: Handler::Sync(wrappers::HeapDestroy),
        },
        Shim {
            name: "HeapFree",
            func: Handler::Sync(wrappers::HeapFree),
        },
        Shim {
            name: "HeapReAlloc",
            func: Handler::Sync(wrappers::HeapReAlloc),
        },
        Shim {
            name: "HeapSetInformation",
            func: Handler::Sync(wrappers::HeapSetInformation),
        },
        Shim {
            name: "HeapSize",
            func: Handler::Sync(wrappers::HeapSize),
        },
        Shim {
            name: "HeapValidate",
            func: Handler::Sync(wrappers::HeapValidate),
        },
        Shim {
            name: "InitOnceBeginInitialize",
            func: Handler::Sync(wrappers::InitOnceBeginInitialize),
        },
        Shim {
            name: "InitOnceComplete",
            func: Handler::Sync(wrappers::InitOnceComplete),
        },
        Shim {
            name: "InitializeCriticalSection",
            func: Handler::Sync(wrappers::InitializeCriticalSection),
        },
        Shim {
            name: "InitializeCriticalSectionAndSpinCount",
            func: Handler::Sync(wrappers::InitializeCriticalSectionAndSpinCount),
        },
        Shim {
            name: "InitializeCriticalSectionEx",
            func: Handler::Sync(wrappers::InitializeCriticalSectionEx),
        },
        Shim {
            name: "InitializeSListHead",
            func: Handler::Sync(wrappers::InitializeSListHead),
        },
        Shim {
            name: "InterlockedDecrement",
            func: Handler::Sync(wrappers::InterlockedDecrement),
        },
        Shim {
            name: "InterlockedIncrement",
            func: Handler::Sync(wrappers::InterlockedIncrement),
        },
        Shim {
            name: "IsBadCodePtr",
            func: Handler::Sync(wrappers::IsBadCodePtr),
        },
        Shim {
            name: "IsBadReadPtr",
            func: Handler::Sync(wrappers::IsBadReadPtr),
        },
        Shim {
            name: "IsBadWritePtr",
            func: Handler::Sync(wrappers::IsBadWritePtr),
        },
        Shim {
            name: "IsDBCSLeadByte",
            func: Handler::Sync(wrappers::IsDBCSLeadByte),
        },
        Shim {
            name: "IsDBCSLeadByteEx",
            func: Handler::Sync(wrappers::IsDBCSLeadByteEx),
        },
        Shim {
            name: "IsDebuggerPresent",
            func: Handler::Sync(wrappers::IsDebuggerPresent),
        },
        Shim {
            name: "IsProcessorFeaturePresent",
            func: Handler::Sync(wrappers::IsProcessorFeaturePresent),
        },
        Shim {
            name: "IsValidCodePage",
            func: Handler::Sync(wrappers::IsValidCodePage),
        },
        Shim {
            name: "LCMapStringA",
            func: Handler::Sync(wrappers::LCMapStringA),
        },
        Shim {
            name: "LCMapStringW",
            func: Handler::Sync(wrappers::LCMapStringW),
        },
        Shim {
            name: "LeaveCriticalSection",
            func: Handler::Sync(wrappers::LeaveCriticalSection),
        },
        Shim {
            name: "LoadLibraryA",
            func: Handler::Sync(wrappers::LoadLibraryA),
        },
        Shim {
            name: "LoadLibraryExW",
            func: Handler::Sync(wrappers::LoadLibraryExW),
        },
        Shim {
            name: "LoadResource",
            func: Handler::Sync(wrappers::LoadResource),
        },
        Shim {
            name: "LocalAlloc",
            func: Handler::Sync(wrappers::LocalAlloc),
        },
        Shim {
            name: "LocalFree",
            func: Handler::Sync(wrappers::LocalFree),
        },
        Shim {
            name: "LockResource",
            func: Handler::Sync(wrappers::LockResource),
        },
        Shim {
            name: "MoveFileA",
            func: Handler::Sync(wrappers::MoveFileA),
        },
        Shim {
            name: "MulDiv",
            func: Handler::Sync(wrappers::MulDiv),
        },
        Shim {
            name: "MultiByteToWideChar",
            func: Handler::Sync(wrappers::MultiByteToWideChar),
        },
        Shim {
            name: "NtCurrentTeb",
            func: Handler::Sync(wrappers::NtCurrentTeb),
        },
        Shim {
            name: "OutputDebugStringA",
            func: Handler::Sync(wrappers::OutputDebugStringA),
        },
        Shim {
            name: "PulseEvent",
            func: Handler::Sync(wrappers::PulseEvent),
        },
        Shim {
            name: "QueryPerformanceCounter",
            func: Handler::Sync(wrappers::QueryPerformanceCounter),
        },
        Shim {
            name: "QueryPerformanceFrequency",
            func: Handler::Sync(wrappers::QueryPerformanceFrequency),
        },
        Shim {
            name: "RaiseException",
            func: Handler::Sync(wrappers::RaiseException),
        },
        Shim {
            name: "ReadFile",
            func: Handler::Sync(wrappers::ReadFile),
        },
        Shim {
            name: "ReleaseSRWLockExclusive",
            func: Handler::Sync(wrappers::ReleaseSRWLockExclusive),
        },
        Shim {
            name: "ReleaseSRWLockShared",
            func: Handler::Sync(wrappers::ReleaseSRWLockShared),
        },
        Shim {
            name: "RemoveDirectoryA",
            func: Handler::Sync(wrappers::RemoveDirectoryA),
        },
        Shim {
            name: "ResetEvent",
            func: Handler::Sync(wrappers::ResetEvent),
        },
        Shim {
            name: "ResumeThread",
            func: Handler::Sync(wrappers::ResumeThread),
        },
        Shim {
            name: "RtlUnwind",
            func: Handler::Sync(wrappers::RtlUnwind),
        },
        Shim {
            name: "SetConsoleCtrlHandler",
            func: Handler::Sync(wrappers::SetConsoleCtrlHandler),
        },
        Shim {
            name: "SetCurrentDirectoryA",
            func: Handler::Sync(wrappers::SetCurrentDirectoryA),
        },
        Shim {
            name: "SetEndOfFile",
            func: Handler::Sync(wrappers::SetEndOfFile),
        },
        Shim {
            name: "SetEnvironmentVariableA",
            func: Handler::Sync(wrappers::SetEnvironmentVariableA),
        },
        Shim {
            name: "SetEvent",
            func: Handler::Sync(wrappers::SetEvent),
        },
        Shim {
            name: "SetFileAttributesA",
            func: Handler::Sync(wrappers::SetFileAttributesA),
        },
        Shim {
            name: "SetFilePointer",
            func: Handler::Sync(wrappers::SetFilePointer),
        },
        Shim {
            name: "SetFileTime",
            func: Handler::Sync(wrappers::SetFileTime),
        },
        Shim {
            name: "SetHandleCount",
            func: Handler::Sync(wrappers::SetHandleCount),
        },
        Shim {
            name: "SetLastError",
            func: Handler::Sync(wrappers::SetLastError),
        },
        Shim {
            name: "SetPriorityClass",
            func: Handler::Sync(wrappers::SetPriorityClass),
        },
        Shim {
            name: "SetStdHandle",
            func: Handler::Sync(wrappers::SetStdHandle),
        },
        Shim {
            name: "SetThreadDescription",
            func: Handler::Sync(wrappers::SetThreadDescription),
        },
        Shim {
            name: "SetThreadPriority",
            func: Handler::Sync(wrappers::SetThreadPriority),
        },
        Shim {
            name: "SetThreadStackGuarantee",
            func: Handler::Sync(wrappers::SetThreadStackGuarantee),
        },
        Shim {
            name: "SetUnhandledExceptionFilter",
            func: Handler::Sync(wrappers::SetUnhandledExceptionFilter),
        },
        Shim {
            name: "SizeofResource",
            func: Handler::Sync(wrappers::SizeofResource),
        },
        Shim {
            name: "Sleep",
            func: Handler::Async(wrappers::Sleep),
        },
        Shim {
            name: "SystemTimeToFileTime",
            func: Handler::Sync(wrappers::SystemTimeToFileTime),
        },
        Shim {
            name: "TerminateProcess",
            func: Handler::Sync(wrappers::TerminateProcess),
        },
        Shim {
            name: "TlsAlloc",
            func: Handler::Sync(wrappers::TlsAlloc),
        },
        Shim {
            name: "TlsFree",
            func: Handler::Sync(wrappers::TlsFree),
        },
        Shim {
            name: "TlsGetValue",
            func: Handler::Sync(wrappers::TlsGetValue),
        },
        Shim {
            name: "TlsSetValue",
            func: Handler::Sync(wrappers::TlsSetValue),
        },
        Shim {
            name: "TryAcquireSRWLockExclusive",
            func: Handler::Sync(wrappers::TryAcquireSRWLockExclusive),
        },
        Shim {
            name: "UnhandledExceptionFilter",
            func: Handler::Sync(wrappers::UnhandledExceptionFilter),
        },
        Shim {
            name: "VirtualAlloc",
            func: Handler::Sync(wrappers::VirtualAlloc),
        },
        Shim {
            name: "VirtualFree",
            func: Handler::Sync(wrappers::VirtualFree),
        },
        Shim {
            name: "VirtualProtect",
            func: Handler::Sync(wrappers::VirtualProtect),
        },
        Shim {
            name: "VirtualQuery",
            func: Handler::Sync(wrappers::VirtualQuery),
        },
        Shim {
            name: "WaitForMultipleObjects",
            func: Handler::Sync(wrappers::WaitForMultipleObjects),
        },
        Shim {
            name: "WaitForSingleObject",
            func: Handler::Sync(wrappers::WaitForSingleObject),
        },
        Shim {
            name: "WideCharToMultiByte",
            func: Handler::Sync(wrappers::WideCharToMultiByte),
        },
        Shim {
            name: "WriteConsoleA",
            func: Handler::Sync(wrappers::WriteConsoleA),
        },
        Shim {
            name: "WriteConsoleW",
            func: Handler::Sync(wrappers::WriteConsoleW),
        },
        Shim {
            name: "WriteFile",
            func: Handler::Sync(wrappers::WriteFile),
        },
        Shim {
            name: "WritePrivateProfileStringA",
            func: Handler::Sync(wrappers::WritePrivateProfileStringA),
        },
        Shim {
            name: "WriteProfileStringW",
            func: Handler::Sync(wrappers::WriteProfileStringW),
        },
        Shim {
            name: "_lclose",
            func: Handler::Sync(wrappers::_lclose),
        },
        Shim {
            name: "_llseek",
            func: Handler::Sync(wrappers::_llseek),
        },
        Shim {
            name: "_lopen",
            func: Handler::Sync(wrappers::_lopen),
        },
        Shim {
            name: "_lread",
            func: Handler::Sync(wrappers::_lread),
        },
        Shim {
            name: "lstrcmpiA",
            func: Handler::Sync(wrappers::lstrcmpiA),
        },
        Shim {
            name: "lstrcpyA",
            func: Handler::Sync(wrappers::lstrcpyA),
        },
        Shim {
            name: "lstrcpyW",
            func: Handler::Sync(wrappers::lstrcpyW),
        },
        Shim {
            name: "lstrlenA",
            func: Handler::Sync(wrappers::lstrlenA),
        },
        Shim {
            name: "lstrlenW",
            func: Handler::Sync(wrappers::lstrlenW),
        },
        Shim {
            name: "retrowin32_main",
            func: Handler::Async(wrappers::retrowin32_main),
        },
        Shim {
            name: "retrowin32_thread_main",
            func: Handler::Async(wrappers::retrowin32_thread_main),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "kernel32.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/kernel32.dll"),
    };
}
pub mod ntdll {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ntdll::*;
        pub unsafe fn NtReadFile(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let FileHandle = <HFILE>::from_stack(mem, stack_args + 0u32);
            let Event = <u32>::from_stack(mem, stack_args + 4u32);
            let ApcRoutine = <u32>::from_stack(mem, stack_args + 8u32);
            let ApcContext = <u32>::from_stack(mem, stack_args + 12u32);
            let IoStatusBlock = <Option<&mut IO_STATUS_BLOCK>>::from_stack(mem, stack_args + 16u32);
            let Buffer = <ArrayWithSizeMut<u8>>::from_stack(mem, stack_args + 20u32);
            let ByteOffset = <Option<&mut u64>>::from_stack(mem, stack_args + 28u32);
            let Key = <u32>::from_stack(mem, stack_args + 32u32);
            let __trace_context = if crate::trace::enabled("ntdll") {
                Some(crate::trace::trace_begin(
                    "ntdll",
                    "NtReadFile",
                    &[
                        ("FileHandle", &FileHandle),
                        ("Event", &Event),
                        ("ApcRoutine", &ApcRoutine),
                        ("ApcContext", &ApcContext),
                        ("IoStatusBlock", &IoStatusBlock),
                        ("Buffer", &Buffer),
                        ("ByteOffset", &ByteOffset),
                        ("Key", &Key),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ntdll::NtReadFile(
                machine,
                FileHandle,
                Event,
                ApcRoutine,
                ApcContext,
                IoStatusBlock,
                Buffer,
                ByteOffset,
                Key,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ntdll::NtReadFile_pos.0,
                    winapi::ntdll::NtReadFile_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RtlExitUserProcess(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let exit_code = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("ntdll") {
                Some(crate::trace::trace_begin(
                    "ntdll",
                    "RtlExitUserProcess",
                    &[("exit_code", &exit_code)],
                ))
            } else {
                None
            };
            let result = winapi::ntdll::RtlExitUserProcess(machine, exit_code);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ntdll::RtlExitUserProcess_pos.0,
                    winapi::ntdll::RtlExitUserProcess_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
    }
    const SHIMS: [Shim; 2usize] = [
        Shim {
            name: "NtReadFile",
            func: Handler::Sync(wrappers::NtReadFile),
        },
        Shim {
            name: "RtlExitUserProcess",
            func: Handler::Sync(wrappers::RtlExitUserProcess),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ntdll.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/ntdll.dll"),
    };
}
pub mod ole32 {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ole32::*;
        pub unsafe fn CoCreateInstance(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let rclsid = <u32>::from_stack(mem, stack_args + 0u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 4u32);
            let dwClsContext = <u32>::from_stack(mem, stack_args + 8u32);
            let riid = <u32>::from_stack(mem, stack_args + 12u32);
            let ppv = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("ole32") {
                Some(crate::trace::trace_begin(
                    "ole32",
                    "CoCreateInstance",
                    &[
                        ("rclsid", &rclsid),
                        ("pUnkOuter", &pUnkOuter),
                        ("dwClsContext", &dwClsContext),
                        ("riid", &riid),
                        ("ppv", &ppv),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::ole32::CoCreateInstance(
                machine,
                rclsid,
                pUnkOuter,
                dwClsContext,
                riid,
                ppv,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ole32::CoCreateInstance_pos.0,
                    winapi::ole32::CoCreateInstance_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CoInitialize(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let pvReserved = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("ole32") {
                Some(crate::trace::trace_begin(
                    "ole32",
                    "CoInitialize",
                    &[("pvReserved", &pvReserved)],
                ))
            } else {
                None
            };
            let result = winapi::ole32::CoInitialize(machine, pvReserved);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ole32::CoInitialize_pos.0,
                    winapi::ole32::CoInitialize_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CoUninitialize(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("ole32") {
                Some(crate::trace::trace_begin("ole32", "CoUninitialize", &[]))
            } else {
                None
            };
            let result = winapi::ole32::CoUninitialize(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ole32::CoUninitialize_pos.0,
                    winapi::ole32::CoUninitialize_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn OleInitialize(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _pvReserved = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("ole32") {
                Some(crate::trace::trace_begin(
                    "ole32",
                    "OleInitialize",
                    &[("pvReserved", &_pvReserved)],
                ))
            } else {
                None
            };
            let result = winapi::ole32::OleInitialize(machine, _pvReserved);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::ole32::OleInitialize_pos.0,
                    winapi::ole32::OleInitialize_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
    }
    const SHIMS: [Shim; 4usize] = [
        Shim {
            name: "CoCreateInstance",
            func: Handler::Sync(wrappers::CoCreateInstance),
        },
        Shim {
            name: "CoInitialize",
            func: Handler::Sync(wrappers::CoInitialize),
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
        raw: std::include_bytes!("../../dll/ole32.dll"),
    };
}
pub mod oleaut32 {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::oleaut32::*;
    }
    const SHIMS: [Shim; 0usize] = [];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "oleaut32.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/oleaut32.dll"),
    };
}
pub mod retrowin32_test {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::retrowin32_test::*;
        pub unsafe fn retrowin32_test_callback1(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let func = <u32>::from_stack(mem, stack_args + 0u32);
            let data = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("retrowin32_test") {
                Some(crate::trace::trace_begin(
                    "retrowin32_test",
                    "retrowin32_test_callback1",
                    &[("func", &func), ("data", &data)],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result =
                    winapi::retrowin32_test::retrowin32_test_callback1(machine, func, data).await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::retrowin32_test::retrowin32_test_callback1_pos.0,
                        winapi::retrowin32_test::retrowin32_test_callback1_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
    }
    const SHIMS: [Shim; 1usize] = [Shim {
        name: "retrowin32_test_callback1",
        func: Handler::Async(wrappers::retrowin32_test_callback1),
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "retrowin32_test.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/retrowin32_test.dll"),
    };
}
pub mod ucrtbase {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
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
            let result =
                winapi::ucrtbase::_controlfp_s(machine, _currentControl, _newControl, _mask);
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
        pub unsafe fn _get_initial_narrow_environment(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
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
        pub unsafe fn _initialize_narrow_environment(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
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
        raw: std::include_bytes!("../../dll/ucrtbase.dll"),
    };
}
pub mod vcruntime140 {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
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
        raw: std::include_bytes!("../../dll/vcruntime140.dll"),
    };
}
pub mod version {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::version::*;
        pub unsafe fn GetFileVersionInfoSizeA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lptstrFilename = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpdwHandle = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("version") {
                Some(crate::trace::trace_begin(
                    "version",
                    "GetFileVersionInfoSizeA",
                    &[
                        ("lptstrFilename", &lptstrFilename),
                        ("lpdwHandle", &lpdwHandle),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::version::GetFileVersionInfoSizeA(machine, lptstrFilename, lpdwHandle);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::version::GetFileVersionInfoSizeA_pos.0,
                    winapi::version::GetFileVersionInfoSizeA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
    }
    const SHIMS: [Shim; 1usize] = [Shim {
        name: "GetFileVersionInfoSizeA",
        func: Handler::Sync(wrappers::GetFileVersionInfoSizeA),
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "version.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/version.dll"),
    };
}
pub mod user32 {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::user32::*;
        pub unsafe fn AdjustWindowRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, stack_args + 4u32);
            let bMenu = <bool>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "AdjustWindowRect",
                    &[
                        ("lpRect", &lpRect),
                        ("dwStyle", &dwStyle),
                        ("bMenu", &bMenu),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::AdjustWindowRect(machine, lpRect, dwStyle, bMenu);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::AdjustWindowRect_pos.0,
                    winapi::user32::AdjustWindowRect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn AdjustWindowRectEx(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, stack_args + 4u32);
            let bMenu = <bool>::from_stack(mem, stack_args + 8u32);
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "AdjustWindowRectEx",
                    &[
                        ("lpRect", &lpRect),
                        ("dwStyle", &dwStyle),
                        ("bMenu", &bMenu),
                        ("dwExStyle", &dwExStyle),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::user32::AdjustWindowRectEx(machine, lpRect, dwStyle, bMenu, dwExStyle);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::AdjustWindowRectEx_pos.0,
                    winapi::user32::AdjustWindowRectEx_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn AppendMenuA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let uFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let uIDNewItem = <u32>::from_stack(mem, stack_args + 8u32);
            let lpNewItem = <Option<&str>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/menu") {
                Some(crate::trace::trace_begin(
                    "user32/menu",
                    "AppendMenuA",
                    &[
                        ("hMenu", &hMenu),
                        ("uFlags", &uFlags),
                        ("uIDNewItem", &uIDNewItem),
                        ("lpNewItem", &lpNewItem),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::AppendMenuA(machine, hMenu, uFlags, uIDNewItem, lpNewItem);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::AppendMenuA_pos.0,
                    winapi::user32::AppendMenuA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn BeginPaint(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpPaint = <Option<&mut PAINTSTRUCT>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/paint") {
                Some(crate::trace::trace_begin(
                    "user32/paint",
                    "BeginPaint",
                    &[("hWnd", &hWnd), ("lpPaint", &lpPaint)],
                ))
            } else {
                None
            };
            let result = winapi::user32::BeginPaint(machine, hWnd, lpPaint);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::BeginPaint_pos.0,
                    winapi::user32::BeginPaint_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CheckDlgButton(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDButton = <i32>::from_stack(mem, stack_args + 4u32);
            let uCheck = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "CheckDlgButton",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDButton", &nIDButton),
                        ("uCheck", &uCheck),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::CheckDlgButton(machine, hDlg, nIDButton, uCheck);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::CheckDlgButton_pos.0,
                    winapi::user32::CheckDlgButton_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CheckMenuItem(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let uIDCheckItem = <u32>::from_stack(mem, stack_args + 4u32);
            let uCheck = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/menu") {
                Some(crate::trace::trace_begin(
                    "user32/menu",
                    "CheckMenuItem",
                    &[
                        ("hMenu", &hMenu),
                        ("uIDCheckItem", &uIDCheckItem),
                        ("uCheck", &uCheck),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::CheckMenuItem(machine, hMenu, uIDCheckItem, uCheck);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::CheckMenuItem_pos.0,
                    winapi::user32::CheckMenuItem_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CheckRadioButton(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDFirstButton = <i32>::from_stack(mem, stack_args + 4u32);
            let nIDLastButton = <i32>::from_stack(mem, stack_args + 8u32);
            let nIDCheckButton = <i32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "CheckRadioButton",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDFirstButton", &nIDFirstButton),
                        ("nIDLastButton", &nIDLastButton),
                        ("nIDCheckButton", &nIDCheckButton),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::CheckRadioButton(
                machine,
                hDlg,
                nIDFirstButton,
                nIDLastButton,
                nIDCheckButton,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::CheckRadioButton_pos.0,
                    winapi::user32::CheckRadioButton_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn ClientToScreen(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpPoint = <Option<&mut POINT>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "ClientToScreen",
                    &[("hWnd", &hWnd), ("lpPoint", &lpPoint)],
                ))
            } else {
                None
            };
            let result = winapi::user32::ClientToScreen(machine, hWnd, lpPoint);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::ClientToScreen_pos.0,
                    winapi::user32::ClientToScreen_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CopyRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprcDst = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let lprcSrc = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/rect") {
                Some(crate::trace::trace_begin(
                    "user32/rect",
                    "CopyRect",
                    &[("lprcDst", &lprcDst), ("lprcSrc", &lprcSrc)],
                ))
            } else {
                None
            };
            let result = winapi::user32::CopyRect(machine, lprcDst, lprcSrc);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::CopyRect_pos.0,
                    winapi::user32::CopyRect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateCursor(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInst = <u32>::from_stack(mem, stack_args + 0u32);
            let xHotSpot = <u32>::from_stack(mem, stack_args + 4u32);
            let yHotSpot = <u32>::from_stack(mem, stack_args + 8u32);
            let nWidth = <u32>::from_stack(mem, stack_args + 12u32);
            let nHeight = <u32>::from_stack(mem, stack_args + 16u32);
            let pvANDPlane = <u32>::from_stack(mem, stack_args + 20u32);
            let pvXORPlane = <u32>::from_stack(mem, stack_args + 24u32);
            let __trace_context = if crate::trace::enabled("user32/resource") {
                Some(crate::trace::trace_begin(
                    "user32/resource",
                    "CreateCursor",
                    &[
                        ("hInst", &hInst),
                        ("xHotSpot", &xHotSpot),
                        ("yHotSpot", &yHotSpot),
                        ("nWidth", &nWidth),
                        ("nHeight", &nHeight),
                        ("pvANDPlane", &pvANDPlane),
                        ("pvXORPlane", &pvXORPlane),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::CreateCursor(
                machine, hInst, xHotSpot, yHotSpot, nWidth, nHeight, pvANDPlane, pvXORPlane,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::CreateCursor_pos.0,
                    winapi::user32::CreateCursor_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreatePopupMenu(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("user32/menu") {
                Some(crate::trace::trace_begin(
                    "user32/menu",
                    "CreatePopupMenu",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::user32::CreatePopupMenu(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::CreatePopupMenu_pos.0,
                    winapi::user32::CreatePopupMenu_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn CreateWindowExA(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, stack_args + 0u32);
            let lpClassName = <CreateWindowClassName<'_, str>>::from_stack(mem, stack_args + 4u32);
            let lpWindowName = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, stack_args + 12u32);
            let X = <u32>::from_stack(mem, stack_args + 16u32);
            let Y = <u32>::from_stack(mem, stack_args + 20u32);
            let nWidth = <u32>::from_stack(mem, stack_args + 24u32);
            let nHeight = <u32>::from_stack(mem, stack_args + 28u32);
            let hWndParent = <HWND>::from_stack(mem, stack_args + 32u32);
            let hMenu = <u32>::from_stack(mem, stack_args + 36u32);
            let hInstance = <u32>::from_stack(mem, stack_args + 40u32);
            let lpParam = <u32>::from_stack(mem, stack_args + 44u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "CreateWindowExA",
                    &[
                        ("dwExStyle", &dwExStyle),
                        ("lpClassName", &lpClassName),
                        ("lpWindowName", &lpWindowName),
                        ("dwStyle", &dwStyle),
                        ("X", &X),
                        ("Y", &Y),
                        ("nWidth", &nWidth),
                        ("nHeight", &nHeight),
                        ("hWndParent", &hWndParent),
                        ("hMenu", &hMenu),
                        ("hInstance", &hInstance),
                        ("lpParam", &lpParam),
                    ],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::user32::CreateWindowExA(
                    machine,
                    dwExStyle,
                    lpClassName,
                    lpWindowName,
                    dwStyle,
                    X,
                    Y,
                    nWidth,
                    nHeight,
                    hWndParent,
                    hMenu,
                    hInstance,
                    lpParam,
                )
                .await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::user32::CreateWindowExA_pos.0,
                        winapi::user32::CreateWindowExA_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn CreateWindowExW(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, stack_args + 0u32);
            let lpClassName =
                <CreateWindowClassName<'_, Str16>>::from_stack(mem, stack_args + 4u32);
            let lpWindowName = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, stack_args + 12u32);
            let X = <u32>::from_stack(mem, stack_args + 16u32);
            let Y = <u32>::from_stack(mem, stack_args + 20u32);
            let nWidth = <u32>::from_stack(mem, stack_args + 24u32);
            let nHeight = <u32>::from_stack(mem, stack_args + 28u32);
            let hWndParent = <HWND>::from_stack(mem, stack_args + 32u32);
            let hMenu = <u32>::from_stack(mem, stack_args + 36u32);
            let hInstance = <u32>::from_stack(mem, stack_args + 40u32);
            let lpParam = <u32>::from_stack(mem, stack_args + 44u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "CreateWindowExW",
                    &[
                        ("dwExStyle", &dwExStyle),
                        ("lpClassName", &lpClassName),
                        ("lpWindowName", &lpWindowName),
                        ("dwStyle", &dwStyle),
                        ("X", &X),
                        ("Y", &Y),
                        ("nWidth", &nWidth),
                        ("nHeight", &nHeight),
                        ("hWndParent", &hWndParent),
                        ("hMenu", &hMenu),
                        ("hInstance", &hInstance),
                        ("lpParam", &lpParam),
                    ],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::user32::CreateWindowExW(
                    machine,
                    dwExStyle,
                    lpClassName,
                    lpWindowName,
                    dwStyle,
                    X,
                    Y,
                    nWidth,
                    nHeight,
                    hWndParent,
                    hMenu,
                    hInstance,
                    lpParam,
                )
                .await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::user32::CreateWindowExW_pos.0,
                        winapi::user32::CreateWindowExW_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn DefWindowProcA(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let msg = <Result<WM, u32>>::from_stack(mem, stack_args + 4u32);
            let wParam = <u32>::from_stack(mem, stack_args + 8u32);
            let lParam = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "DefWindowProcA",
                    &[
                        ("hWnd", &hWnd),
                        ("msg", &msg),
                        ("wParam", &wParam),
                        ("lParam", &lParam),
                    ],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result =
                    winapi::user32::DefWindowProcA(machine, hWnd, msg, wParam, lParam).await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::user32::DefWindowProcA_pos.0,
                        winapi::user32::DefWindowProcA_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn DefWindowProcW(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let msg = <Result<WM, u32>>::from_stack(mem, stack_args + 4u32);
            let wParam = <u32>::from_stack(mem, stack_args + 8u32);
            let lParam = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "DefWindowProcW",
                    &[
                        ("hWnd", &hWnd),
                        ("msg", &msg),
                        ("wParam", &wParam),
                        ("lParam", &lParam),
                    ],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result =
                    winapi::user32::DefWindowProcW(machine, hWnd, msg, wParam, lParam).await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::user32::DefWindowProcW_pos.0,
                        winapi::user32::DefWindowProcW_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn DestroyWindow(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "DestroyWindow",
                    &[("hWnd", &hWnd)],
                ))
            } else {
                None
            };
            let result = winapi::user32::DestroyWindow(machine, hWnd);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::DestroyWindow_pos.0,
                    winapi::user32::DestroyWindow_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn DialogBoxIndirectParamA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let hDialogTemplate = <u32>::from_stack(mem, stack_args + 4u32);
            let hWndParent = <HWND>::from_stack(mem, stack_args + 8u32);
            let lpDialogFunc = <u32>::from_stack(mem, stack_args + 12u32);
            let dwInitParam = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "DialogBoxIndirectParamA",
                    &[
                        ("hInstance", &hInstance),
                        ("hDialogTemplate", &hDialogTemplate),
                        ("hWndParent", &hWndParent),
                        ("lpDialogFunc", &lpDialogFunc),
                        ("dwInitParam", &dwInitParam),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::DialogBoxIndirectParamA(
                machine,
                hInstance,
                hDialogTemplate,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::DialogBoxIndirectParamA_pos.0,
                    winapi::user32::DialogBoxIndirectParamA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn DialogBoxParamA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpTemplateName = <u32>::from_stack(mem, stack_args + 4u32);
            let hWndParent = <HWND>::from_stack(mem, stack_args + 8u32);
            let lpDialogFunc = <u32>::from_stack(mem, stack_args + 12u32);
            let dwInitParam = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "DialogBoxParamA",
                    &[
                        ("hInstance", &hInstance),
                        ("lpTemplateName", &lpTemplateName),
                        ("hWndParent", &hWndParent),
                        ("lpDialogFunc", &lpDialogFunc),
                        ("dwInitParam", &dwInitParam),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::DialogBoxParamA(
                machine,
                hInstance,
                lpTemplateName,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::DialogBoxParamA_pos.0,
                    winapi::user32::DialogBoxParamA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn DialogBoxParamW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpTemplateName = <u32>::from_stack(mem, stack_args + 4u32);
            let hWndParent = <HWND>::from_stack(mem, stack_args + 8u32);
            let lpDialogFunc = <u32>::from_stack(mem, stack_args + 12u32);
            let dwInitParam = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "DialogBoxParamW",
                    &[
                        ("hInstance", &hInstance),
                        ("lpTemplateName", &lpTemplateName),
                        ("hWndParent", &hWndParent),
                        ("lpDialogFunc", &lpDialogFunc),
                        ("dwInitParam", &dwInitParam),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::DialogBoxParamW(
                machine,
                hInstance,
                lpTemplateName,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::DialogBoxParamW_pos.0,
                    winapi::user32::DialogBoxParamW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn DispatchMessageA(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/message") {
                Some(crate::trace::trace_begin(
                    "user32/message",
                    "DispatchMessageA",
                    &[("lpMsg", &lpMsg)],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::user32::DispatchMessageA(machine, lpMsg).await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::user32::DispatchMessageA_pos.0,
                        winapi::user32::DispatchMessageA_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn DispatchMessageW(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/message") {
                Some(crate::trace::trace_begin(
                    "user32/message",
                    "DispatchMessageW",
                    &[("lpMsg", &lpMsg)],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::user32::DispatchMessageW(machine, lpMsg).await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::user32::DispatchMessageW_pos.0,
                        winapi::user32::DispatchMessageW_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn DrawTextW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let nCount = <i32>::from_stack(mem, stack_args + 8u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 12u32);
            let uFormat = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("user32/paint") {
                Some(crate::trace::trace_begin(
                    "user32/paint",
                    "DrawTextW",
                    &[
                        ("hDC", &hDC),
                        ("lpString", &lpString),
                        ("nCount", &nCount),
                        ("lpRect", &lpRect),
                        ("uFormat", &uFormat),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::DrawTextW(machine, hDC, lpString, nCount, lpRect, uFormat);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::DrawTextW_pos.0,
                    winapi::user32::DrawTextW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn EnableMenuItem(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let uIDEnableItem = <u32>::from_stack(mem, stack_args + 4u32);
            let uEnable = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/menu") {
                Some(crate::trace::trace_begin(
                    "user32/menu",
                    "EnableMenuItem",
                    &[
                        ("hMenu", &hMenu),
                        ("uIDEnableItem", &uIDEnableItem),
                        ("uEnable", &uEnable),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::EnableMenuItem(machine, hMenu, uIDEnableItem, uEnable);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::EnableMenuItem_pos.0,
                    winapi::user32::EnableMenuItem_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn EnableWindow(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let bEnable = <bool>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "EnableWindow",
                    &[("hWnd", &hWnd), ("bEnable", &bEnable)],
                ))
            } else {
                None
            };
            let result = winapi::user32::EnableWindow(machine, hWnd, bEnable);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::EnableWindow_pos.0,
                    winapi::user32::EnableWindow_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn EndDialog(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nResult = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "EndDialog",
                    &[("hDlg", &hDlg), ("nResult", &nResult)],
                ))
            } else {
                None
            };
            let result = winapi::user32::EndDialog(machine, hDlg, nResult);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::EndDialog_pos.0,
                    winapi::user32::EndDialog_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn EndPaint(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpPaint = <Option<&PAINTSTRUCT>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/paint") {
                Some(crate::trace::trace_begin(
                    "user32/paint",
                    "EndPaint",
                    &[("hWnd", &hWnd), ("lpPaint", &lpPaint)],
                ))
            } else {
                None
            };
            let result = winapi::user32::EndPaint(machine, hWnd, lpPaint);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::EndPaint_pos.0,
                    winapi::user32::EndPaint_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FillRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
            let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let hbr = <BrushOrColor>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/paint") {
                Some(crate::trace::trace_begin(
                    "user32/paint",
                    "FillRect",
                    &[("hDC", &hDC), ("lprc", &lprc), ("hbr", &hbr)],
                ))
            } else {
                None
            };
            let result = winapi::user32::FillRect(machine, hDC, lprc, hbr);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::FillRect_pos.0,
                    winapi::user32::FillRect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FindWindowA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpClassName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpWindowName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "FindWindowA",
                    &[
                        ("lpClassName", &lpClassName),
                        ("lpWindowName", &lpWindowName),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::FindWindowA(machine, lpClassName, lpWindowName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::FindWindowA_pos.0,
                    winapi::user32::FindWindowA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn FrameRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
            let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let hbr = <HBRUSH>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/paint") {
                Some(crate::trace::trace_begin(
                    "user32/paint",
                    "FrameRect",
                    &[("hDC", &hDC), ("lprc", &lprc), ("hbr", &hbr)],
                ))
            } else {
                None
            };
            let result = winapi::user32::FrameRect(machine, hDC, lprc, hbr);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::FrameRect_pos.0,
                    winapi::user32::FrameRect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetActiveWindow(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "GetActiveWindow",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetActiveWindow(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetActiveWindow_pos.0,
                    winapi::user32::GetActiveWindow_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetCapture(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "GetCapture",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetCapture(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetCapture_pos.0,
                    winapi::user32::GetCapture_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetClientRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "GetClientRect",
                    &[("hWnd", &hWnd), ("lpRect", &lpRect)],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetClientRect(machine, hWnd, lpRect);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetClientRect_pos.0,
                    winapi::user32::GetClientRect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetDC(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "GetDC",
                    &[("hWnd", &hWnd)],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetDC(machine, hWnd);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetDC_pos.0,
                    winapi::user32::GetDC_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetDesktopWindow(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "GetDesktopWindow",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetDesktopWindow(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetDesktopWindow_pos.0,
                    winapi::user32::GetDesktopWindow_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetDlgItem(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "GetDlgItem",
                    &[("hDlg", &hDlg), ("nIDDlgItem", &nIDDlgItem)],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetDlgItem(machine, hDlg, nIDDlgItem);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetDlgItem_pos.0,
                    winapi::user32::GetDlgItem_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetDlgItemInt(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let lpTranslated = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let bSigned = <bool>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "GetDlgItemInt",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDDlgItem", &nIDDlgItem),
                        ("lpTranslated", &lpTranslated),
                        ("bSigned", &bSigned),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::user32::GetDlgItemInt(machine, hDlg, nIDDlgItem, lpTranslated, bSigned);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetDlgItemInt_pos.0,
                    winapi::user32::GetDlgItemInt_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetDlgItemTextW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let lpString = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "GetDlgItemTextW",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDDlgItem", &nIDDlgItem),
                        ("lpString", &lpString),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetDlgItemTextW(machine, hDlg, nIDDlgItem, lpString);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetDlgItemTextW_pos.0,
                    winapi::user32::GetDlgItemTextW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetFocus(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin("user32/window", "GetFocus", &[]))
            } else {
                None
            };
            let result = winapi::user32::GetFocus(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetFocus_pos.0,
                    winapi::user32::GetFocus_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetForegroundWindow(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "GetForegroundWindow",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetForegroundWindow(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetForegroundWindow_pos.0,
                    winapi::user32::GetForegroundWindow_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetKeyState(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nVirtKey = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/misc") {
                Some(crate::trace::trace_begin(
                    "user32/misc",
                    "GetKeyState",
                    &[("nVirtKey", &nVirtKey)],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetKeyState(machine, nVirtKey);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetKeyState_pos.0,
                    winapi::user32::GetKeyState_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetLastActivePopup(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "GetLastActivePopup",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetLastActivePopup(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetLastActivePopup_pos.0,
                    winapi::user32::GetLastActivePopup_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetMenu(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/menu") {
                Some(crate::trace::trace_begin(
                    "user32/menu",
                    "GetMenu",
                    &[("hWnd", &hWnd)],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetMenu(machine, hWnd);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetMenu_pos.0,
                    winapi::user32::GetMenu_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetMenuItemRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hMenu = <HMENU>::from_stack(mem, stack_args + 4u32);
            let uItem = <u32>::from_stack(mem, stack_args + 8u32);
            let lprcItem = <Option<&mut RECT>>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/menu") {
                Some(crate::trace::trace_begin(
                    "user32/menu",
                    "GetMenuItemRect",
                    &[
                        ("hWnd", &hWnd),
                        ("hMenu", &hMenu),
                        ("uItem", &uItem),
                        ("lprcItem", &lprcItem),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetMenuItemRect(machine, hWnd, hMenu, uItem, lprcItem);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetMenuItemRect_pos.0,
                    winapi::user32::GetMenuItemRect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetMessageA(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, stack_args + 0u32);
            let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let wMsgFilterMin = <u32>::from_stack(mem, stack_args + 8u32);
            let wMsgFilterMax = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/message") {
                Some(crate::trace::trace_begin(
                    "user32/message",
                    "GetMessageA",
                    &[
                        ("lpMsg", &lpMsg),
                        ("hWnd", &hWnd),
                        ("wMsgFilterMin", &wMsgFilterMin),
                        ("wMsgFilterMax", &wMsgFilterMax),
                    ],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result =
                    winapi::user32::GetMessageA(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax)
                        .await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::user32::GetMessageA_pos.0,
                        winapi::user32::GetMessageA_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn GetMessageW(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, stack_args + 0u32);
            let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let wMsgFilterMin = <u32>::from_stack(mem, stack_args + 8u32);
            let wMsgFilterMax = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/message") {
                Some(crate::trace::trace_begin(
                    "user32/message",
                    "GetMessageW",
                    &[
                        ("lpMsg", &lpMsg),
                        ("hWnd", &hWnd),
                        ("wMsgFilterMin", &wMsgFilterMin),
                        ("wMsgFilterMax", &wMsgFilterMax),
                    ],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result =
                    winapi::user32::GetMessageW(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax)
                        .await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::user32::GetMessageW_pos.0,
                        winapi::user32::GetMessageW_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn GetSubMenu(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let nPos = <i32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/menu") {
                Some(crate::trace::trace_begin(
                    "user32/menu",
                    "GetSubMenu",
                    &[("hMenu", &hMenu), ("nPos", &nPos)],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetSubMenu(machine, hMenu, nPos);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetSubMenu_pos.0,
                    winapi::user32::GetSubMenu_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetSysColor(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nIndex = <i32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/misc") {
                Some(crate::trace::trace_begin(
                    "user32/misc",
                    "GetSysColor",
                    &[("nIndex", &nIndex)],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetSysColor(machine, nIndex);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetSysColor_pos.0,
                    winapi::user32::GetSysColor_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetSystemMenu(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let bRevert = <bool>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/menu") {
                Some(crate::trace::trace_begin(
                    "user32/menu",
                    "GetSystemMenu",
                    &[("hWnd", &hWnd), ("bRevert", &bRevert)],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetSystemMenu(machine, hWnd, bRevert);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetSystemMenu_pos.0,
                    winapi::user32::GetSystemMenu_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetSystemMetrics(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nIndex = <Result<SystemMetric, u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/misc") {
                Some(crate::trace::trace_begin(
                    "user32/misc",
                    "GetSystemMetrics",
                    &[("nIndex", &nIndex)],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetSystemMetrics(machine, nIndex);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetSystemMetrics_pos.0,
                    winapi::user32::GetSystemMetrics_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetWindowDC(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "GetWindowDC",
                    &[("hWnd", &hWnd)],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetWindowDC(machine, hWnd);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetWindowDC_pos.0,
                    winapi::user32::GetWindowDC_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetWindowLongA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIndex = <i32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "GetWindowLongA",
                    &[("hWnd", &hWnd), ("nIndex", &nIndex)],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetWindowLongA(machine, hWnd, nIndex);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetWindowLongA_pos.0,
                    winapi::user32::GetWindowLongA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetWindowPlacement(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpwndpl = <Option<&mut WINDOWPLACEMENT>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "GetWindowPlacement",
                    &[("hWnd", &hWnd), ("lpwndpl", &lpwndpl)],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetWindowPlacement(machine, hWnd, lpwndpl);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetWindowPlacement_pos.0,
                    winapi::user32::GetWindowPlacement_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn GetWindowRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "GetWindowRect",
                    &[("hWnd", &hWnd), ("lpRect", &lpRect)],
                ))
            } else {
                None
            };
            let result = winapi::user32::GetWindowRect(machine, hWnd, lpRect);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::GetWindowRect_pos.0,
                    winapi::user32::GetWindowRect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn InflateRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let dx = <i32>::from_stack(mem, stack_args + 4u32);
            let dy = <i32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/rect") {
                Some(crate::trace::trace_begin(
                    "user32/rect",
                    "InflateRect",
                    &[("lprc", &lprc), ("dx", &dx), ("dy", &dy)],
                ))
            } else {
                None
            };
            let result = winapi::user32::InflateRect(machine, lprc, dx, dy);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::InflateRect_pos.0,
                    winapi::user32::InflateRect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IntersectRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprcDst = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let lprcSrc1 = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let lprcSrc2 = <Option<&RECT>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/rect") {
                Some(crate::trace::trace_begin(
                    "user32/rect",
                    "IntersectRect",
                    &[
                        ("lprcDst", &lprcDst),
                        ("lprcSrc1", &lprcSrc1),
                        ("lprcSrc2", &lprcSrc2),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::IntersectRect(machine, lprcDst, lprcSrc1, lprcSrc2);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::IntersectRect_pos.0,
                    winapi::user32::IntersectRect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn InvalidateRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let bErase = <bool>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/paint") {
                Some(crate::trace::trace_begin(
                    "user32/paint",
                    "InvalidateRect",
                    &[("hWnd", &hWnd), ("lpRect", &lpRect), ("bErase", &bErase)],
                ))
            } else {
                None
            };
            let result = winapi::user32::InvalidateRect(machine, hWnd, lpRect, bErase);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::InvalidateRect_pos.0,
                    winapi::user32::InvalidateRect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn InvalidateRgn(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hRgn = <HRGN>::from_stack(mem, stack_args + 4u32);
            let bErase = <bool>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/paint") {
                Some(crate::trace::trace_begin(
                    "user32/paint",
                    "InvalidateRgn",
                    &[("hWnd", &hWnd), ("hRgn", &hRgn), ("bErase", &bErase)],
                ))
            } else {
                None
            };
            let result = winapi::user32::InvalidateRgn(machine, hWnd, hRgn, bErase);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::InvalidateRgn_pos.0,
                    winapi::user32::InvalidateRgn_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn InvertRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpr = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/paint") {
                Some(crate::trace::trace_begin(
                    "user32/paint",
                    "InvertRect",
                    &[("hDC", &hDC), ("lpr", &lpr)],
                ))
            } else {
                None
            };
            let result = winapi::user32::InvertRect(machine, hDC, lpr);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::InvertRect_pos.0,
                    winapi::user32::InvertRect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IsDlgButtonChecked(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDButton = <i32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "IsDlgButtonChecked",
                    &[("hDlg", &hDlg), ("nIDButton", &nIDButton)],
                ))
            } else {
                None
            };
            let result = winapi::user32::IsDlgButtonChecked(machine, hDlg, nIDButton);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::IsDlgButtonChecked_pos.0,
                    winapi::user32::IsDlgButtonChecked_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IsIconic(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/misc") {
                Some(crate::trace::trace_begin(
                    "user32/misc",
                    "IsIconic",
                    &[("hwnd", &hwnd)],
                ))
            } else {
                None
            };
            let result = winapi::user32::IsIconic(machine, hwnd);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::IsIconic_pos.0,
                    winapi::user32::IsIconic_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn IsRectEmpty(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/rect") {
                Some(crate::trace::trace_begin(
                    "user32/rect",
                    "IsRectEmpty",
                    &[("lprc", &lprc)],
                ))
            } else {
                None
            };
            let result = winapi::user32::IsRectEmpty(machine, lprc);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::IsRectEmpty_pos.0,
                    winapi::user32::IsRectEmpty_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn KillTimer(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let uIDEvent = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/timer") {
                Some(crate::trace::trace_begin(
                    "user32/timer",
                    "KillTimer",
                    &[("hWnd", &hWnd), ("uIDEvent", &uIDEvent)],
                ))
            } else {
                None
            };
            let result = winapi::user32::KillTimer(machine, hWnd, uIDEvent);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::KillTimer_pos.0,
                    winapi::user32::KillTimer_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadAcceleratorsW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpTableName = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/resource") {
                Some(crate::trace::trace_begin(
                    "user32/resource",
                    "LoadAcceleratorsW",
                    &[("hInstance", &hInstance), ("lpTableName", &lpTableName)],
                ))
            } else {
                None
            };
            let result = winapi::user32::LoadAcceleratorsW(machine, hInstance, lpTableName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::LoadAcceleratorsW_pos.0,
                    winapi::user32::LoadAcceleratorsW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadBitmapA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <HINSTANCE>::from_stack(mem, stack_args + 0u32);
            let lpBitmapName = <ResourceKey<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/resource") {
                Some(crate::trace::trace_begin(
                    "user32/resource",
                    "LoadBitmapA",
                    &[("hInstance", &hInstance), ("lpBitmapName", &lpBitmapName)],
                ))
            } else {
                None
            };
            let result = winapi::user32::LoadBitmapA(machine, hInstance, lpBitmapName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::LoadBitmapA_pos.0,
                    winapi::user32::LoadBitmapA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadCursorA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpCursorName = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/resource") {
                Some(crate::trace::trace_begin(
                    "user32/resource",
                    "LoadCursorA",
                    &[("hInstance", &hInstance), ("lpCursorName", &lpCursorName)],
                ))
            } else {
                None
            };
            let result = winapi::user32::LoadCursorA(machine, hInstance, lpCursorName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::LoadCursorA_pos.0,
                    winapi::user32::LoadCursorA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadCursorW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpCursorName = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/resource") {
                Some(crate::trace::trace_begin(
                    "user32/resource",
                    "LoadCursorW",
                    &[("hInstance", &hInstance), ("lpCursorName", &lpCursorName)],
                ))
            } else {
                None
            };
            let result = winapi::user32::LoadCursorW(machine, hInstance, lpCursorName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::LoadCursorW_pos.0,
                    winapi::user32::LoadCursorW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadIconA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpIconName = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/resource") {
                Some(crate::trace::trace_begin(
                    "user32/resource",
                    "LoadIconA",
                    &[("hInstance", &hInstance), ("lpIconName", &lpIconName)],
                ))
            } else {
                None
            };
            let result = winapi::user32::LoadIconA(machine, hInstance, lpIconName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::LoadIconA_pos.0,
                    winapi::user32::LoadIconA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadIconW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpIconName = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/resource") {
                Some(crate::trace::trace_begin(
                    "user32/resource",
                    "LoadIconW",
                    &[("hInstance", &hInstance), ("lpIconName", &lpIconName)],
                ))
            } else {
                None
            };
            let result = winapi::user32::LoadIconW(machine, hInstance, lpIconName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::LoadIconW_pos.0,
                    winapi::user32::LoadIconW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadImageA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let name = <ResourceKey<&str>>::from_stack(mem, stack_args + 4u32);
            let typ = <u32>::from_stack(mem, stack_args + 8u32);
            let cx = <u32>::from_stack(mem, stack_args + 12u32);
            let cy = <u32>::from_stack(mem, stack_args + 16u32);
            let fuLoad = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("user32/resource") {
                Some(crate::trace::trace_begin(
                    "user32/resource",
                    "LoadImageA",
                    &[
                        ("hInstance", &hInstance),
                        ("name", &name),
                        ("typ", &typ),
                        ("cx", &cx),
                        ("cy", &cy),
                        ("fuLoad", &fuLoad),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::LoadImageA(machine, hInstance, name, typ, cx, cy, fuLoad);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::LoadImageA_pos.0,
                    winapi::user32::LoadImageA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadImageW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let name = <ResourceKey<&Str16>>::from_stack(mem, stack_args + 4u32);
            let typ = <u32>::from_stack(mem, stack_args + 8u32);
            let cx = <u32>::from_stack(mem, stack_args + 12u32);
            let cy = <u32>::from_stack(mem, stack_args + 16u32);
            let fuLoad = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("user32/resource") {
                Some(crate::trace::trace_begin(
                    "user32/resource",
                    "LoadImageW",
                    &[
                        ("hInstance", &hInstance),
                        ("name", &name),
                        ("typ", &typ),
                        ("cx", &cx),
                        ("cy", &cy),
                        ("fuLoad", &fuLoad),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::LoadImageW(machine, hInstance, name, typ, cx, cy, fuLoad);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::LoadImageW_pos.0,
                    winapi::user32::LoadImageW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadMenuA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpMenuName = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/menu") {
                Some(crate::trace::trace_begin(
                    "user32/menu",
                    "LoadMenuA",
                    &[("hInstance", &hInstance), ("lpMenuName", &lpMenuName)],
                ))
            } else {
                None
            };
            let result = winapi::user32::LoadMenuA(machine, hInstance, lpMenuName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::LoadMenuA_pos.0,
                    winapi::user32::LoadMenuA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadMenuW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpMenuName = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/resource") {
                Some(crate::trace::trace_begin(
                    "user32/resource",
                    "LoadMenuW",
                    &[("hInstance", &hInstance), ("lpMenuName", &lpMenuName)],
                ))
            } else {
                None
            };
            let result = winapi::user32::LoadMenuW(machine, hInstance, lpMenuName);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::LoadMenuW_pos.0,
                    winapi::user32::LoadMenuW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadStringA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let uID = <u32>::from_stack(mem, stack_args + 4u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
            let cchBufferMax = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/resource") {
                Some(crate::trace::trace_begin(
                    "user32/resource",
                    "LoadStringA",
                    &[
                        ("hInstance", &hInstance),
                        ("uID", &uID),
                        ("lpBuffer", &lpBuffer),
                        ("cchBufferMax", &cchBufferMax),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::user32::LoadStringA(machine, hInstance, uID, lpBuffer, cchBufferMax);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::LoadStringA_pos.0,
                    winapi::user32::LoadStringA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn LoadStringW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let uID = <u32>::from_stack(mem, stack_args + 4u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
            let cchBufferMax = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/resource") {
                Some(crate::trace::trace_begin(
                    "user32/resource",
                    "LoadStringW",
                    &[
                        ("hInstance", &hInstance),
                        ("uID", &uID),
                        ("lpBuffer", &lpBuffer),
                        ("cchBufferMax", &cchBufferMax),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::user32::LoadStringW(machine, hInstance, uID, lpBuffer, cchBufferMax);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::LoadStringW_pos.0,
                    winapi::user32::LoadStringW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn MapWindowPoints(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWndFrom = <HWND>::from_stack(mem, stack_args + 0u32);
            let hWndTo = <HWND>::from_stack(mem, stack_args + 4u32);
            let lpPoints = <ArrayWithSize<POINT>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "MapWindowPoints",
                    &[
                        ("hWndFrom", &hWndFrom),
                        ("hWndTo", &hWndTo),
                        ("lpPoints", &lpPoints),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::MapWindowPoints(machine, hWndFrom, hWndTo, lpPoints);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::MapWindowPoints_pos.0,
                    winapi::user32::MapWindowPoints_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn MessageBoxA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpText = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let lpCaption = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let uType = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "MessageBoxA",
                    &[
                        ("hWnd", &hWnd),
                        ("lpText", &lpText),
                        ("lpCaption", &lpCaption),
                        ("uType", &uType),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::MessageBoxA(machine, hWnd, lpText, lpCaption, uType);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::MessageBoxA_pos.0,
                    winapi::user32::MessageBoxA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn MessageBoxW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpText = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpCaption = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let uType = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "MessageBoxW",
                    &[
                        ("hWnd", &hWnd),
                        ("lpText", &lpText),
                        ("lpCaption", &lpCaption),
                        ("uType", &uType),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::MessageBoxW(machine, hWnd, lpText, lpCaption, uType);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::MessageBoxW_pos.0,
                    winapi::user32::MessageBoxW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn MoveWindow(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let X = <u32>::from_stack(mem, stack_args + 4u32);
            let Y = <u32>::from_stack(mem, stack_args + 8u32);
            let nWidth = <u32>::from_stack(mem, stack_args + 12u32);
            let nHeight = <u32>::from_stack(mem, stack_args + 16u32);
            let bRepaint = <bool>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "MoveWindow",
                    &[
                        ("hWnd", &hWnd),
                        ("X", &X),
                        ("Y", &Y),
                        ("nWidth", &nWidth),
                        ("nHeight", &nHeight),
                        ("bRepaint", &bRepaint),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::MoveWindow(machine, hWnd, X, Y, nWidth, nHeight, bRepaint);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::MoveWindow_pos.0,
                    winapi::user32::MoveWindow_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn MsgWaitForMultipleObjects(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nCount = <u32>::from_stack(mem, stack_args + 0u32);
            let pHandles = <u32>::from_stack(mem, stack_args + 4u32);
            let fWaitAll = <bool>::from_stack(mem, stack_args + 8u32);
            let dwMilliseconds = <u32>::from_stack(mem, stack_args + 12u32);
            let dwWakeMask = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("user32/message") {
                Some(crate::trace::trace_begin(
                    "user32/message",
                    "MsgWaitForMultipleObjects",
                    &[
                        ("nCount", &nCount),
                        ("pHandles", &pHandles),
                        ("fWaitAll", &fWaitAll),
                        ("dwMilliseconds", &dwMilliseconds),
                        ("dwWakeMask", &dwWakeMask),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::MsgWaitForMultipleObjects(
                machine,
                nCount,
                pHandles,
                fWaitAll,
                dwMilliseconds,
                dwWakeMask,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::MsgWaitForMultipleObjects_pos.0,
                    winapi::user32::MsgWaitForMultipleObjects_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn PeekMessageA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, stack_args + 0u32);
            let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let wMsgFilterMin = <u32>::from_stack(mem, stack_args + 8u32);
            let wMsgFilterMax = <u32>::from_stack(mem, stack_args + 12u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("user32/message") {
                Some(crate::trace::trace_begin(
                    "user32/message",
                    "PeekMessageA",
                    &[
                        ("lpMsg", &lpMsg),
                        ("hWnd", &hWnd),
                        ("wMsgFilterMin", &wMsgFilterMin),
                        ("wMsgFilterMax", &wMsgFilterMax),
                        ("wRemoveMsg", &wRemoveMsg),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::PeekMessageA(
                machine,
                lpMsg,
                hWnd,
                wMsgFilterMin,
                wMsgFilterMax,
                wRemoveMsg,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::PeekMessageA_pos.0,
                    winapi::user32::PeekMessageA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn PeekMessageW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, stack_args + 0u32);
            let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let wMsgFilterMin = <u32>::from_stack(mem, stack_args + 8u32);
            let wMsgFilterMax = <u32>::from_stack(mem, stack_args + 12u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("user32/message") {
                Some(crate::trace::trace_begin(
                    "user32/message",
                    "PeekMessageW",
                    &[
                        ("lpMsg", &lpMsg),
                        ("hWnd", &hWnd),
                        ("wMsgFilterMin", &wMsgFilterMin),
                        ("wMsgFilterMax", &wMsgFilterMax),
                        ("wRemoveMsg", &wRemoveMsg),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::PeekMessageW(
                machine,
                lpMsg,
                hWnd,
                wMsgFilterMin,
                wMsgFilterMax,
                wRemoveMsg,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::PeekMessageW_pos.0,
                    winapi::user32::PeekMessageW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn PostMessageW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let Msg = <u32>::from_stack(mem, stack_args + 4u32);
            let wParam = <u32>::from_stack(mem, stack_args + 8u32);
            let lParam = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/message") {
                Some(crate::trace::trace_begin(
                    "user32/message",
                    "PostMessageW",
                    &[
                        ("hWnd", &hWnd),
                        ("Msg", &Msg),
                        ("wParam", &wParam),
                        ("lParam", &lParam),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::PostMessageW(machine, hWnd, Msg, wParam, lParam);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::PostMessageW_pos.0,
                    winapi::user32::PostMessageW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn PostQuitMessage(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nExitCode = <i32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/message") {
                Some(crate::trace::trace_begin(
                    "user32/message",
                    "PostQuitMessage",
                    &[("nExitCode", &nExitCode)],
                ))
            } else {
                None
            };
            let result = winapi::user32::PostQuitMessage(machine, nExitCode);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::PostQuitMessage_pos.0,
                    winapi::user32::PostQuitMessage_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn PtInRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 0u32);
            let pt = <POINT>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/rect") {
                Some(crate::trace::trace_begin(
                    "user32/rect",
                    "PtInRect",
                    &[("lprc", &lprc), ("pt", &pt)],
                ))
            } else {
                None
            };
            let result = winapi::user32::PtInRect(machine, lprc, pt);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::PtInRect_pos.0,
                    winapi::user32::PtInRect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RegisterClassA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "RegisterClassA",
                    &[("lpWndClass", &lpWndClass)],
                ))
            } else {
                None
            };
            let result = winapi::user32::RegisterClassA(machine, lpWndClass);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::RegisterClassA_pos.0,
                    winapi::user32::RegisterClassA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RegisterClassExA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClassEx = <Option<&WNDCLASSEXA>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "RegisterClassExA",
                    &[("lpWndClassEx", &lpWndClassEx)],
                ))
            } else {
                None
            };
            let result = winapi::user32::RegisterClassExA(machine, lpWndClassEx);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::RegisterClassExA_pos.0,
                    winapi::user32::RegisterClassExA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RegisterClassExW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClassEx = <Option<&WNDCLASSEXW>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "RegisterClassExW",
                    &[("lpWndClassEx", &lpWndClassEx)],
                ))
            } else {
                None
            };
            let result = winapi::user32::RegisterClassExW(machine, lpWndClassEx);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::RegisterClassExW_pos.0,
                    winapi::user32::RegisterClassExW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RegisterClassW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "RegisterClassW",
                    &[("lpWndClass", &lpWndClass)],
                ))
            } else {
                None
            };
            let result = winapi::user32::RegisterClassW(machine, lpWndClass);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::RegisterClassW_pos.0,
                    winapi::user32::RegisterClassW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn RegisterWindowMessageW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "RegisterWindowMessageW",
                    &[("lpString", &lpString)],
                ))
            } else {
                None
            };
            let result = winapi::user32::RegisterWindowMessageW(machine, lpString);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::RegisterWindowMessageW_pos.0,
                    winapi::user32::RegisterWindowMessageW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn ReleaseCapture(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "ReleaseCapture",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::user32::ReleaseCapture(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::ReleaseCapture_pos.0,
                    winapi::user32::ReleaseCapture_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn ReleaseDC(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hdc = <HDC>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "ReleaseDC",
                    &[("hwnd", &hwnd), ("hdc", &hdc)],
                ))
            } else {
                None
            };
            let result = winapi::user32::ReleaseDC(machine, hwnd, hdc);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::ReleaseDC_pos.0,
                    winapi::user32::ReleaseDC_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SendMessageA(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let Msg = <Result<WM, u32>>::from_stack(mem, stack_args + 4u32);
            let wParam = <u32>::from_stack(mem, stack_args + 8u32);
            let lParam = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/message") {
                Some(crate::trace::trace_begin(
                    "user32/message",
                    "SendMessageA",
                    &[
                        ("hWnd", &hWnd),
                        ("Msg", &Msg),
                        ("wParam", &wParam),
                        ("lParam", &lParam),
                    ],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::user32::SendMessageA(machine, hWnd, Msg, wParam, lParam).await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::user32::SendMessageA_pos.0,
                        winapi::user32::SendMessageA_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn SendMessageW(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let Msg = <Result<WM, u32>>::from_stack(mem, stack_args + 4u32);
            let wParam = <u32>::from_stack(mem, stack_args + 8u32);
            let lParam = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/message") {
                Some(crate::trace::trace_begin(
                    "user32/message",
                    "SendMessageW",
                    &[
                        ("hWnd", &hWnd),
                        ("Msg", &Msg),
                        ("wParam", &wParam),
                        ("lParam", &lParam),
                    ],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::user32::SendMessageW(machine, hWnd, Msg, wParam, lParam).await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::user32::SendMessageW_pos.0,
                        winapi::user32::SendMessageW_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn SetCapture(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "SetCapture",
                    &[("hwnd", &hwnd)],
                ))
            } else {
                None
            };
            let result = winapi::user32::SetCapture(machine, hwnd);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::SetCapture_pos.0,
                    winapi::user32::SetCapture_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetCursor(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hCursor = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/resource") {
                Some(crate::trace::trace_begin(
                    "user32/resource",
                    "SetCursor",
                    &[("hCursor", &hCursor)],
                ))
            } else {
                None
            };
            let result = winapi::user32::SetCursor(machine, hCursor);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::SetCursor_pos.0,
                    winapi::user32::SetCursor_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetCursorPos(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let x = <i32>::from_stack(mem, stack_args + 0u32);
            let y = <i32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/misc") {
                Some(crate::trace::trace_begin(
                    "user32/misc",
                    "SetCursorPos",
                    &[("x", &x), ("y", &y)],
                ))
            } else {
                None
            };
            let result = winapi::user32::SetCursorPos(machine, x, y);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::SetCursorPos_pos.0,
                    winapi::user32::SetCursorPos_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetDlgItemInt(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let uValue = <u32>::from_stack(mem, stack_args + 8u32);
            let _bSigned = <bool>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "SetDlgItemInt",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDDlgItem", &nIDDlgItem),
                        ("uValue", &uValue),
                        ("bSigned", &_bSigned),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::SetDlgItemInt(machine, hDlg, nIDDlgItem, uValue, _bSigned);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::SetDlgItemInt_pos.0,
                    winapi::user32::SetDlgItemInt_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetDlgItemTextA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "SetDlgItemTextA",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDDlgItem", &nIDDlgItem),
                        ("lpString", &lpString),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::SetDlgItemTextA(machine, hDlg, nIDDlgItem, lpString);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::SetDlgItemTextA_pos.0,
                    winapi::user32::SetDlgItemTextA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetDlgItemTextW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/dialog") {
                Some(crate::trace::trace_begin(
                    "user32/dialog",
                    "SetDlgItemTextW",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDDlgItem", &nIDDlgItem),
                        ("lpString", &lpString),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::SetDlgItemTextW(machine, hDlg, nIDDlgItem, lpString);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::SetDlgItemTextW_pos.0,
                    winapi::user32::SetDlgItemTextW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetFocus(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "SetFocus",
                    &[("hWnd", &hWnd)],
                ))
            } else {
                None
            };
            let result = winapi::user32::SetFocus(machine, hWnd);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::SetFocus_pos.0,
                    winapi::user32::SetFocus_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetForegroundWindow(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "SetForegroundWindow",
                    &[("hWnd", &hWnd)],
                ))
            } else {
                None
            };
            let result = winapi::user32::SetForegroundWindow(machine, hWnd);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::SetForegroundWindow_pos.0,
                    winapi::user32::SetForegroundWindow_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetMenu(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hMenu = <HMENU>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/menu") {
                Some(crate::trace::trace_begin(
                    "user32/menu",
                    "SetMenu",
                    &[("hWnd", &hWnd), ("hMenu", &hMenu)],
                ))
            } else {
                None
            };
            let result = winapi::user32::SetMenu(machine, hWnd, hMenu);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::SetMenu_pos.0,
                    winapi::user32::SetMenu_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetMenuItemInfoA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let item = <u32>::from_stack(mem, stack_args + 4u32);
            let fByPosition = <bool>::from_stack(mem, stack_args + 8u32);
            let lpmii = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/menu") {
                Some(crate::trace::trace_begin(
                    "user32/menu",
                    "SetMenuItemInfoA",
                    &[
                        ("hMenu", &hMenu),
                        ("item", &item),
                        ("fByPosition", &fByPosition),
                        ("lpmii", &lpmii),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::SetMenuItemInfoA(machine, hMenu, item, fByPosition, lpmii);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::SetMenuItemInfoA_pos.0,
                    winapi::user32::SetMenuItemInfoA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let xLeft = <i32>::from_stack(mem, stack_args + 4u32);
            let yTop = <i32>::from_stack(mem, stack_args + 8u32);
            let xRight = <i32>::from_stack(mem, stack_args + 12u32);
            let yBottom = <i32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("user32/rect") {
                Some(crate::trace::trace_begin(
                    "user32/rect",
                    "SetRect",
                    &[
                        ("lprc", &lprc),
                        ("xLeft", &xLeft),
                        ("yTop", &yTop),
                        ("xRight", &xRight),
                        ("yBottom", &yBottom),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::SetRect(machine, lprc, xLeft, yTop, xRight, yBottom);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::SetRect_pos.0,
                    winapi::user32::SetRect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetRectEmpty(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/rect") {
                Some(crate::trace::trace_begin(
                    "user32/rect",
                    "SetRectEmpty",
                    &[("lprc", &lprc)],
                ))
            } else {
                None
            };
            let result = winapi::user32::SetRectEmpty(machine, lprc);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::SetRectEmpty_pos.0,
                    winapi::user32::SetRectEmpty_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetTimer(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDEvent = <u32>::from_stack(mem, stack_args + 4u32);
            let uElapse = <u32>::from_stack(mem, stack_args + 8u32);
            let lpTimerFunc = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/timer") {
                Some(crate::trace::trace_begin(
                    "user32/timer",
                    "SetTimer",
                    &[
                        ("hWnd", &hWnd),
                        ("nIDEvent", &nIDEvent),
                        ("uElapse", &uElapse),
                        ("lpTimerFunc", &lpTimerFunc),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::SetTimer(machine, hWnd, nIDEvent, uElapse, lpTimerFunc);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::SetTimer_pos.0,
                    winapi::user32::SetTimer_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn SetWindowPos(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hWndInsertAfter = <HWND>::from_stack(mem, stack_args + 4u32);
            let X = <i32>::from_stack(mem, stack_args + 8u32);
            let Y = <i32>::from_stack(mem, stack_args + 12u32);
            let cx = <i32>::from_stack(mem, stack_args + 16u32);
            let cy = <i32>::from_stack(mem, stack_args + 20u32);
            let uFlags = <Result<SWP, u32>>::from_stack(mem, stack_args + 24u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "SetWindowPos",
                    &[
                        ("hWnd", &hWnd),
                        ("hWndInsertAfter", &hWndInsertAfter),
                        ("X", &X),
                        ("Y", &Y),
                        ("cx", &cx),
                        ("cy", &cy),
                        ("uFlags", &uFlags),
                    ],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::user32::SetWindowPos(
                    machine,
                    hWnd,
                    hWndInsertAfter,
                    X,
                    Y,
                    cx,
                    cy,
                    uFlags,
                )
                .await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::user32::SetWindowPos_pos.0,
                        winapi::user32::SetWindowPos_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn SetWindowTextA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "SetWindowTextA",
                    &[("hWnd", &hWnd), ("lpString", &lpString)],
                ))
            } else {
                None
            };
            let result = winapi::user32::SetWindowTextA(machine, hWnd, lpString);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::SetWindowTextA_pos.0,
                    winapi::user32::SetWindowTextA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn ShowCursor(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let bShow = <bool>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/resource") {
                Some(crate::trace::trace_begin(
                    "user32/resource",
                    "ShowCursor",
                    &[("bShow", &bShow)],
                ))
            } else {
                None
            };
            let result = winapi::user32::ShowCursor(machine, bShow);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::ShowCursor_pos.0,
                    winapi::user32::ShowCursor_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn ShowWindow(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let nCmdShow = <Result<SW, u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "ShowWindow",
                    &[("hWnd", &hWnd), ("nCmdShow", &nCmdShow)],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::user32::ShowWindow(machine, hWnd, nCmdShow).await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::user32::ShowWindow_pos.0,
                        winapi::user32::ShowWindow_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn TranslateAcceleratorW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hAccTable = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/message") {
                Some(crate::trace::trace_begin(
                    "user32/message",
                    "TranslateAcceleratorW",
                    &[
                        ("hWnd", &hWnd),
                        ("hAccTable", &hAccTable),
                        ("lpMsg", &lpMsg),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::TranslateAcceleratorW(machine, hWnd, hAccTable, lpMsg);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::TranslateAcceleratorW_pos.0,
                    winapi::user32::TranslateAcceleratorW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn TranslateMessage(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/message") {
                Some(crate::trace::trace_begin(
                    "user32/message",
                    "TranslateMessage",
                    &[("lpMsg", &lpMsg)],
                ))
            } else {
                None
            };
            let result = winapi::user32::TranslateMessage(machine, lpMsg);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::TranslateMessage_pos.0,
                    winapi::user32::TranslateMessage_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn UpdateWindow(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("user32/window") {
                Some(crate::trace::trace_begin(
                    "user32/window",
                    "UpdateWindow",
                    &[("hWnd", &hWnd)],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::user32::UpdateWindow(machine, hWnd).await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::user32::UpdateWindow_pos.0,
                        winapi::user32::UpdateWindow_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn ValidateRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("user32/paint") {
                Some(crate::trace::trace_begin(
                    "user32/paint",
                    "ValidateRect",
                    &[("hWnd", &hWnd), ("lpRect", &lpRect)],
                ))
            } else {
                None
            };
            let result = winapi::user32::ValidateRect(machine, hWnd, lpRect);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::ValidateRect_pos.0,
                    winapi::user32::ValidateRect_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn WaitMessage(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("user32/message") {
                Some(crate::trace::trace_begin(
                    "user32/message",
                    "WaitMessage",
                    &[],
                ))
            } else {
                None
            };
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let result = winapi::user32::WaitMessage(machine).await;
                if let Some(__trace_context) = __trace_context {
                    crate::trace::trace_return(
                        &__trace_context,
                        winapi::user32::WaitMessage_pos.0,
                        winapi::user32::WaitMessage_pos.1,
                        &result,
                    );
                }
                result.to_raw()
            })
        }
        pub unsafe fn WinHelpW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWndMain = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpszHelp = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let uCommand = <u32>::from_stack(mem, stack_args + 8u32);
            let dwData = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_context = if crate::trace::enabled("user32/misc") {
                Some(crate::trace::trace_begin(
                    "user32/misc",
                    "WinHelpW",
                    &[
                        ("hWndMain", &hWndMain),
                        ("lpszHelp", &lpszHelp),
                        ("uCommand", &uCommand),
                        ("dwData", &dwData),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::user32::WinHelpW(machine, hWndMain, lpszHelp, uCommand, dwData);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::WinHelpW_pos.0,
                    winapi::user32::WinHelpW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn wsprintfA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let buf = <u32>::from_stack(mem, stack_args + 0u32);
            let fmt = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let args = <VarArgs>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/misc") {
                Some(crate::trace::trace_begin(
                    "user32/misc",
                    "wsprintfA",
                    &[("buf", &buf), ("fmt", &fmt), ("args", &args)],
                ))
            } else {
                None
            };
            let result = winapi::user32::wsprintfA(machine, buf, fmt, args);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::wsprintfA_pos.0,
                    winapi::user32::wsprintfA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn wsprintfW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let buf = <u32>::from_stack(mem, stack_args + 0u32);
            let fmt = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let args = <VarArgs>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("user32/misc") {
                Some(crate::trace::trace_begin(
                    "user32/misc",
                    "wsprintfW",
                    &[("buf", &buf), ("fmt", &fmt), ("args", &args)],
                ))
            } else {
                None
            };
            let result = winapi::user32::wsprintfW(machine, buf, fmt, args);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::user32::wsprintfW_pos.0,
                    winapi::user32::wsprintfW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
    }
    const SHIMS: [Shim; 118usize] = [
        Shim {
            name: "AdjustWindowRect",
            func: Handler::Sync(wrappers::AdjustWindowRect),
        },
        Shim {
            name: "AdjustWindowRectEx",
            func: Handler::Sync(wrappers::AdjustWindowRectEx),
        },
        Shim {
            name: "AppendMenuA",
            func: Handler::Sync(wrappers::AppendMenuA),
        },
        Shim {
            name: "BeginPaint",
            func: Handler::Sync(wrappers::BeginPaint),
        },
        Shim {
            name: "CheckDlgButton",
            func: Handler::Sync(wrappers::CheckDlgButton),
        },
        Shim {
            name: "CheckMenuItem",
            func: Handler::Sync(wrappers::CheckMenuItem),
        },
        Shim {
            name: "CheckRadioButton",
            func: Handler::Sync(wrappers::CheckRadioButton),
        },
        Shim {
            name: "ClientToScreen",
            func: Handler::Sync(wrappers::ClientToScreen),
        },
        Shim {
            name: "CopyRect",
            func: Handler::Sync(wrappers::CopyRect),
        },
        Shim {
            name: "CreateCursor",
            func: Handler::Sync(wrappers::CreateCursor),
        },
        Shim {
            name: "CreatePopupMenu",
            func: Handler::Sync(wrappers::CreatePopupMenu),
        },
        Shim {
            name: "CreateWindowExA",
            func: Handler::Async(wrappers::CreateWindowExA),
        },
        Shim {
            name: "CreateWindowExW",
            func: Handler::Async(wrappers::CreateWindowExW),
        },
        Shim {
            name: "DefWindowProcA",
            func: Handler::Async(wrappers::DefWindowProcA),
        },
        Shim {
            name: "DefWindowProcW",
            func: Handler::Async(wrappers::DefWindowProcW),
        },
        Shim {
            name: "DestroyWindow",
            func: Handler::Sync(wrappers::DestroyWindow),
        },
        Shim {
            name: "DialogBoxIndirectParamA",
            func: Handler::Sync(wrappers::DialogBoxIndirectParamA),
        },
        Shim {
            name: "DialogBoxParamA",
            func: Handler::Sync(wrappers::DialogBoxParamA),
        },
        Shim {
            name: "DialogBoxParamW",
            func: Handler::Sync(wrappers::DialogBoxParamW),
        },
        Shim {
            name: "DispatchMessageA",
            func: Handler::Async(wrappers::DispatchMessageA),
        },
        Shim {
            name: "DispatchMessageW",
            func: Handler::Async(wrappers::DispatchMessageW),
        },
        Shim {
            name: "DrawTextW",
            func: Handler::Sync(wrappers::DrawTextW),
        },
        Shim {
            name: "EnableMenuItem",
            func: Handler::Sync(wrappers::EnableMenuItem),
        },
        Shim {
            name: "EnableWindow",
            func: Handler::Sync(wrappers::EnableWindow),
        },
        Shim {
            name: "EndDialog",
            func: Handler::Sync(wrappers::EndDialog),
        },
        Shim {
            name: "EndPaint",
            func: Handler::Sync(wrappers::EndPaint),
        },
        Shim {
            name: "FillRect",
            func: Handler::Sync(wrappers::FillRect),
        },
        Shim {
            name: "FindWindowA",
            func: Handler::Sync(wrappers::FindWindowA),
        },
        Shim {
            name: "FrameRect",
            func: Handler::Sync(wrappers::FrameRect),
        },
        Shim {
            name: "GetActiveWindow",
            func: Handler::Sync(wrappers::GetActiveWindow),
        },
        Shim {
            name: "GetCapture",
            func: Handler::Sync(wrappers::GetCapture),
        },
        Shim {
            name: "GetClientRect",
            func: Handler::Sync(wrappers::GetClientRect),
        },
        Shim {
            name: "GetDC",
            func: Handler::Sync(wrappers::GetDC),
        },
        Shim {
            name: "GetDesktopWindow",
            func: Handler::Sync(wrappers::GetDesktopWindow),
        },
        Shim {
            name: "GetDlgItem",
            func: Handler::Sync(wrappers::GetDlgItem),
        },
        Shim {
            name: "GetDlgItemInt",
            func: Handler::Sync(wrappers::GetDlgItemInt),
        },
        Shim {
            name: "GetDlgItemTextW",
            func: Handler::Sync(wrappers::GetDlgItemTextW),
        },
        Shim {
            name: "GetFocus",
            func: Handler::Sync(wrappers::GetFocus),
        },
        Shim {
            name: "GetForegroundWindow",
            func: Handler::Sync(wrappers::GetForegroundWindow),
        },
        Shim {
            name: "GetKeyState",
            func: Handler::Sync(wrappers::GetKeyState),
        },
        Shim {
            name: "GetLastActivePopup",
            func: Handler::Sync(wrappers::GetLastActivePopup),
        },
        Shim {
            name: "GetMenu",
            func: Handler::Sync(wrappers::GetMenu),
        },
        Shim {
            name: "GetMenuItemRect",
            func: Handler::Sync(wrappers::GetMenuItemRect),
        },
        Shim {
            name: "GetMessageA",
            func: Handler::Async(wrappers::GetMessageA),
        },
        Shim {
            name: "GetMessageW",
            func: Handler::Async(wrappers::GetMessageW),
        },
        Shim {
            name: "GetSubMenu",
            func: Handler::Sync(wrappers::GetSubMenu),
        },
        Shim {
            name: "GetSysColor",
            func: Handler::Sync(wrappers::GetSysColor),
        },
        Shim {
            name: "GetSystemMenu",
            func: Handler::Sync(wrappers::GetSystemMenu),
        },
        Shim {
            name: "GetSystemMetrics",
            func: Handler::Sync(wrappers::GetSystemMetrics),
        },
        Shim {
            name: "GetWindowDC",
            func: Handler::Sync(wrappers::GetWindowDC),
        },
        Shim {
            name: "GetWindowLongA",
            func: Handler::Sync(wrappers::GetWindowLongA),
        },
        Shim {
            name: "GetWindowPlacement",
            func: Handler::Sync(wrappers::GetWindowPlacement),
        },
        Shim {
            name: "GetWindowRect",
            func: Handler::Sync(wrappers::GetWindowRect),
        },
        Shim {
            name: "InflateRect",
            func: Handler::Sync(wrappers::InflateRect),
        },
        Shim {
            name: "IntersectRect",
            func: Handler::Sync(wrappers::IntersectRect),
        },
        Shim {
            name: "InvalidateRect",
            func: Handler::Sync(wrappers::InvalidateRect),
        },
        Shim {
            name: "InvalidateRgn",
            func: Handler::Sync(wrappers::InvalidateRgn),
        },
        Shim {
            name: "InvertRect",
            func: Handler::Sync(wrappers::InvertRect),
        },
        Shim {
            name: "IsDlgButtonChecked",
            func: Handler::Sync(wrappers::IsDlgButtonChecked),
        },
        Shim {
            name: "IsIconic",
            func: Handler::Sync(wrappers::IsIconic),
        },
        Shim {
            name: "IsRectEmpty",
            func: Handler::Sync(wrappers::IsRectEmpty),
        },
        Shim {
            name: "KillTimer",
            func: Handler::Sync(wrappers::KillTimer),
        },
        Shim {
            name: "LoadAcceleratorsW",
            func: Handler::Sync(wrappers::LoadAcceleratorsW),
        },
        Shim {
            name: "LoadBitmapA",
            func: Handler::Sync(wrappers::LoadBitmapA),
        },
        Shim {
            name: "LoadCursorA",
            func: Handler::Sync(wrappers::LoadCursorA),
        },
        Shim {
            name: "LoadCursorW",
            func: Handler::Sync(wrappers::LoadCursorW),
        },
        Shim {
            name: "LoadIconA",
            func: Handler::Sync(wrappers::LoadIconA),
        },
        Shim {
            name: "LoadIconW",
            func: Handler::Sync(wrappers::LoadIconW),
        },
        Shim {
            name: "LoadImageA",
            func: Handler::Sync(wrappers::LoadImageA),
        },
        Shim {
            name: "LoadImageW",
            func: Handler::Sync(wrappers::LoadImageW),
        },
        Shim {
            name: "LoadMenuA",
            func: Handler::Sync(wrappers::LoadMenuA),
        },
        Shim {
            name: "LoadMenuW",
            func: Handler::Sync(wrappers::LoadMenuW),
        },
        Shim {
            name: "LoadStringA",
            func: Handler::Sync(wrappers::LoadStringA),
        },
        Shim {
            name: "LoadStringW",
            func: Handler::Sync(wrappers::LoadStringW),
        },
        Shim {
            name: "MapWindowPoints",
            func: Handler::Sync(wrappers::MapWindowPoints),
        },
        Shim {
            name: "MessageBoxA",
            func: Handler::Sync(wrappers::MessageBoxA),
        },
        Shim {
            name: "MessageBoxW",
            func: Handler::Sync(wrappers::MessageBoxW),
        },
        Shim {
            name: "MoveWindow",
            func: Handler::Sync(wrappers::MoveWindow),
        },
        Shim {
            name: "MsgWaitForMultipleObjects",
            func: Handler::Sync(wrappers::MsgWaitForMultipleObjects),
        },
        Shim {
            name: "PeekMessageA",
            func: Handler::Sync(wrappers::PeekMessageA),
        },
        Shim {
            name: "PeekMessageW",
            func: Handler::Sync(wrappers::PeekMessageW),
        },
        Shim {
            name: "PostMessageW",
            func: Handler::Sync(wrappers::PostMessageW),
        },
        Shim {
            name: "PostQuitMessage",
            func: Handler::Sync(wrappers::PostQuitMessage),
        },
        Shim {
            name: "PtInRect",
            func: Handler::Sync(wrappers::PtInRect),
        },
        Shim {
            name: "RegisterClassA",
            func: Handler::Sync(wrappers::RegisterClassA),
        },
        Shim {
            name: "RegisterClassExA",
            func: Handler::Sync(wrappers::RegisterClassExA),
        },
        Shim {
            name: "RegisterClassExW",
            func: Handler::Sync(wrappers::RegisterClassExW),
        },
        Shim {
            name: "RegisterClassW",
            func: Handler::Sync(wrappers::RegisterClassW),
        },
        Shim {
            name: "RegisterWindowMessageW",
            func: Handler::Sync(wrappers::RegisterWindowMessageW),
        },
        Shim {
            name: "ReleaseCapture",
            func: Handler::Sync(wrappers::ReleaseCapture),
        },
        Shim {
            name: "ReleaseDC",
            func: Handler::Sync(wrappers::ReleaseDC),
        },
        Shim {
            name: "SendMessageA",
            func: Handler::Async(wrappers::SendMessageA),
        },
        Shim {
            name: "SendMessageW",
            func: Handler::Async(wrappers::SendMessageW),
        },
        Shim {
            name: "SetCapture",
            func: Handler::Sync(wrappers::SetCapture),
        },
        Shim {
            name: "SetCursor",
            func: Handler::Sync(wrappers::SetCursor),
        },
        Shim {
            name: "SetCursorPos",
            func: Handler::Sync(wrappers::SetCursorPos),
        },
        Shim {
            name: "SetDlgItemInt",
            func: Handler::Sync(wrappers::SetDlgItemInt),
        },
        Shim {
            name: "SetDlgItemTextA",
            func: Handler::Sync(wrappers::SetDlgItemTextA),
        },
        Shim {
            name: "SetDlgItemTextW",
            func: Handler::Sync(wrappers::SetDlgItemTextW),
        },
        Shim {
            name: "SetFocus",
            func: Handler::Sync(wrappers::SetFocus),
        },
        Shim {
            name: "SetForegroundWindow",
            func: Handler::Sync(wrappers::SetForegroundWindow),
        },
        Shim {
            name: "SetMenu",
            func: Handler::Sync(wrappers::SetMenu),
        },
        Shim {
            name: "SetMenuItemInfoA",
            func: Handler::Sync(wrappers::SetMenuItemInfoA),
        },
        Shim {
            name: "SetRect",
            func: Handler::Sync(wrappers::SetRect),
        },
        Shim {
            name: "SetRectEmpty",
            func: Handler::Sync(wrappers::SetRectEmpty),
        },
        Shim {
            name: "SetTimer",
            func: Handler::Sync(wrappers::SetTimer),
        },
        Shim {
            name: "SetWindowPos",
            func: Handler::Async(wrappers::SetWindowPos),
        },
        Shim {
            name: "SetWindowTextA",
            func: Handler::Sync(wrappers::SetWindowTextA),
        },
        Shim {
            name: "ShowCursor",
            func: Handler::Sync(wrappers::ShowCursor),
        },
        Shim {
            name: "ShowWindow",
            func: Handler::Async(wrappers::ShowWindow),
        },
        Shim {
            name: "TranslateAcceleratorW",
            func: Handler::Sync(wrappers::TranslateAcceleratorW),
        },
        Shim {
            name: "TranslateMessage",
            func: Handler::Sync(wrappers::TranslateMessage),
        },
        Shim {
            name: "UpdateWindow",
            func: Handler::Async(wrappers::UpdateWindow),
        },
        Shim {
            name: "ValidateRect",
            func: Handler::Sync(wrappers::ValidateRect),
        },
        Shim {
            name: "WaitMessage",
            func: Handler::Async(wrappers::WaitMessage),
        },
        Shim {
            name: "WinHelpW",
            func: Handler::Sync(wrappers::WinHelpW),
        },
        Shim {
            name: "wsprintfA",
            func: Handler::Sync(wrappers::wsprintfA),
        },
        Shim {
            name: "wsprintfW",
            func: Handler::Sync(wrappers::wsprintfW),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "user32.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/user32.dll"),
    };
}
pub mod wininet {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::wininet::*;
        pub unsafe fn InternetOpenA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpszAgent = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let dwAccessType = <u32>::from_stack(mem, stack_args + 4u32);
            let lpszProxy = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let lpszProxyBypass = <Option<&str>>::from_stack(mem, stack_args + 12u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("wininet") {
                Some(crate::trace::trace_begin(
                    "wininet",
                    "InternetOpenA",
                    &[
                        ("lpszAgent", &lpszAgent),
                        ("dwAccessType", &dwAccessType),
                        ("lpszProxy", &lpszProxy),
                        ("lpszProxyBypass", &lpszProxyBypass),
                        ("dwFlags", &dwFlags),
                    ],
                ))
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
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::wininet::InternetOpenA_pos.0,
                    winapi::wininet::InternetOpenA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
    }
    const SHIMS: [Shim; 1usize] = [Shim {
        name: "InternetOpenA",
        func: Handler::Sync(wrappers::InternetOpenA),
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "wininet.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/wininet.dll"),
    };
}
pub mod winmm {
    use super::*;
    mod wrappers {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::winmm::*;
        pub unsafe fn PlaySoundW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let pszSound = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let hmod = <HMODULE>::from_stack(mem, stack_args + 4u32);
            let fdwSound = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("winmm/misc") {
                Some(crate::trace::trace_begin(
                    "winmm/misc",
                    "PlaySoundW",
                    &[
                        ("pszSound", &pszSound),
                        ("hmod", &hmod),
                        ("fdwSound", &fdwSound),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::winmm::PlaySoundW(machine, pszSound, hmod, fdwSound);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::PlaySoundW_pos.0,
                    winapi::winmm::PlaySoundW_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn mciSendCommandA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("winmm/misc") {
                Some(crate::trace::trace_begin(
                    "winmm/misc",
                    "mciSendCommandA",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::winmm::mciSendCommandA(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::mciSendCommandA_pos.0,
                    winapi::winmm::mciSendCommandA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn mixerClose(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hmx = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("winmm/mixer") {
                Some(crate::trace::trace_begin(
                    "winmm/mixer",
                    "mixerClose",
                    &[("hmx", &hmx)],
                ))
            } else {
                None
            };
            let result = winapi::winmm::mixerClose(machine, hmx);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::mixerClose_pos.0,
                    winapi::winmm::mixerClose_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn mixerGetControlDetailsA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("winmm/mixer") {
                Some(crate::trace::trace_begin(
                    "winmm/mixer",
                    "mixerGetControlDetailsA",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::winmm::mixerGetControlDetailsA(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::mixerGetControlDetailsA_pos.0,
                    winapi::winmm::mixerGetControlDetailsA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn mixerGetLineControlsA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hmxobj = <u32>::from_stack(mem, stack_args + 0u32);
            let pmxlc = <u32>::from_stack(mem, stack_args + 4u32);
            let fdwControls = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("winmm/mixer") {
                Some(crate::trace::trace_begin(
                    "winmm/mixer",
                    "mixerGetLineControlsA",
                    &[
                        ("hmxobj", &hmxobj),
                        ("pmxlc", &pmxlc),
                        ("fdwControls", &fdwControls),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::winmm::mixerGetLineControlsA(machine, hmxobj, pmxlc, fdwControls);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::mixerGetLineControlsA_pos.0,
                    winapi::winmm::mixerGetLineControlsA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn mixerGetLineInfoA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hmxobj = <HMIXEROBJ>::from_stack(mem, stack_args + 0u32);
            let pmxl = <u32>::from_stack(mem, stack_args + 4u32);
            let fdwInfo = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("winmm/mixer") {
                Some(crate::trace::trace_begin(
                    "winmm/mixer",
                    "mixerGetLineInfoA",
                    &[("hmxobj", &hmxobj), ("pmxl", &pmxl), ("fdwInfo", &fdwInfo)],
                ))
            } else {
                None
            };
            let result = winapi::winmm::mixerGetLineInfoA(machine, hmxobj, pmxl, fdwInfo);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::mixerGetLineInfoA_pos.0,
                    winapi::winmm::mixerGetLineInfoA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn mixerOpen(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let phmx = <u32>::from_stack(mem, stack_args + 0u32);
            let uMxId = <u32>::from_stack(mem, stack_args + 4u32);
            let dwCallback = <u32>::from_stack(mem, stack_args + 8u32);
            let dwInstance = <u32>::from_stack(mem, stack_args + 12u32);
            let fdwOpen = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("winmm/mixer") {
                Some(crate::trace::trace_begin(
                    "winmm/mixer",
                    "mixerOpen",
                    &[
                        ("phmx", &phmx),
                        ("uMxId", &uMxId),
                        ("dwCallback", &dwCallback),
                        ("dwInstance", &dwInstance),
                        ("fdwOpen", &fdwOpen),
                    ],
                ))
            } else {
                None
            };
            let result =
                winapi::winmm::mixerOpen(machine, phmx, uMxId, dwCallback, dwInstance, fdwOpen);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::mixerOpen_pos.0,
                    winapi::winmm::mixerOpen_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn mixerSetControlDetails(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hmxobj = <u32>::from_stack(mem, stack_args + 0u32);
            let pmxcd = <u32>::from_stack(mem, stack_args + 4u32);
            let fdwDetails = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("winmm/mixer") {
                Some(crate::trace::trace_begin(
                    "winmm/mixer",
                    "mixerSetControlDetails",
                    &[
                        ("hmxobj", &hmxobj),
                        ("pmxcd", &pmxcd),
                        ("fdwDetails", &fdwDetails),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::winmm::mixerSetControlDetails(machine, hmxobj, pmxcd, fdwDetails);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::mixerSetControlDetails_pos.0,
                    winapi::winmm::mixerSetControlDetails_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn timeBeginPeriod(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uPeriod = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("winmm/time") {
                Some(crate::trace::trace_begin(
                    "winmm/time",
                    "timeBeginPeriod",
                    &[("uPeriod", &uPeriod)],
                ))
            } else {
                None
            };
            let result = winapi::winmm::timeBeginPeriod(machine, uPeriod);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::timeBeginPeriod_pos.0,
                    winapi::winmm::timeBeginPeriod_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn timeEndPeriod(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uPeriod = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("winmm/time") {
                Some(crate::trace::trace_begin(
                    "winmm/time",
                    "timeEndPeriod",
                    &[("uPeriod", &uPeriod)],
                ))
            } else {
                None
            };
            let result = winapi::winmm::timeEndPeriod(machine, uPeriod);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::timeEndPeriod_pos.0,
                    winapi::winmm::timeEndPeriod_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn timeGetTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("winmm/time") {
                Some(crate::trace::trace_begin("winmm/time", "timeGetTime", &[]))
            } else {
                None
            };
            let result = winapi::winmm::timeGetTime(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::timeGetTime_pos.0,
                    winapi::winmm::timeGetTime_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn timeKillEvent(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uTimerID = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("winmm/time") {
                Some(crate::trace::trace_begin(
                    "winmm/time",
                    "timeKillEvent",
                    &[("uTimerID", &uTimerID)],
                ))
            } else {
                None
            };
            let result = winapi::winmm::timeKillEvent(machine, uTimerID);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::timeKillEvent_pos.0,
                    winapi::winmm::timeKillEvent_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn timeSetEvent(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uDelay = <u32>::from_stack(mem, stack_args + 0u32);
            let uResolution = <u32>::from_stack(mem, stack_args + 4u32);
            let lpTimeProc = <u32>::from_stack(mem, stack_args + 8u32);
            let dwUser = <u32>::from_stack(mem, stack_args + 12u32);
            let fuEvent = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_context = if crate::trace::enabled("winmm/time") {
                Some(crate::trace::trace_begin(
                    "winmm/time",
                    "timeSetEvent",
                    &[
                        ("uDelay", &uDelay),
                        ("uResolution", &uResolution),
                        ("lpTimeProc", &lpTimeProc),
                        ("dwUser", &dwUser),
                        ("fuEvent", &fuEvent),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::winmm::timeSetEvent(
                machine,
                uDelay,
                uResolution,
                lpTimeProc,
                dwUser,
                fuEvent,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::timeSetEvent_pos.0,
                    winapi::winmm::timeSetEvent_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn waveOutClose(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("winmm/wave") {
                Some(crate::trace::trace_begin(
                    "winmm/wave",
                    "waveOutClose",
                    &[("hwo", &hwo)],
                ))
            } else {
                None
            };
            let result = winapi::winmm::waveOutClose(machine, hwo);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::waveOutClose_pos.0,
                    winapi::winmm::waveOutClose_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn waveOutGetDevCapsA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uDeviceID = <u32>::from_stack(mem, stack_args + 0u32);
            let pwoc = <Option<&mut WAVEOUTCAPS>>::from_stack(mem, stack_args + 4u32);
            let cbwoc = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("winmm/wave") {
                Some(crate::trace::trace_begin(
                    "winmm/wave",
                    "waveOutGetDevCapsA",
                    &[
                        ("uDeviceID", &uDeviceID),
                        ("pwoc", &pwoc),
                        ("cbwoc", &cbwoc),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::winmm::waveOutGetDevCapsA(machine, uDeviceID, pwoc, cbwoc);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::waveOutGetDevCapsA_pos.0,
                    winapi::winmm::waveOutGetDevCapsA_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn waveOutGetNumDevs(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let __trace_context = if crate::trace::enabled("winmm/wave") {
                Some(crate::trace::trace_begin(
                    "winmm/wave",
                    "waveOutGetNumDevs",
                    &[],
                ))
            } else {
                None
            };
            let result = winapi::winmm::waveOutGetNumDevs(machine);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::waveOutGetNumDevs_pos.0,
                    winapi::winmm::waveOutGetNumDevs_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn waveOutGetPosition(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let pmmt = <Option<&mut MMTIME>>::from_stack(mem, stack_args + 4u32);
            let cbmmt = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("winmm/wave") {
                Some(crate::trace::trace_begin(
                    "winmm/wave",
                    "waveOutGetPosition",
                    &[("hwo", &hwo), ("pmmt", &pmmt), ("cbmmt", &cbmmt)],
                ))
            } else {
                None
            };
            let result = winapi::winmm::waveOutGetPosition(machine, hwo, pmmt, cbmmt);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::waveOutGetPosition_pos.0,
                    winapi::winmm::waveOutGetPosition_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn waveOutGetVolume(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let pdwVolume = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("winmm/wave") {
                Some(crate::trace::trace_begin(
                    "winmm/wave",
                    "waveOutGetVolume",
                    &[("hwo", &hwo), ("pdwVolume", &pdwVolume)],
                ))
            } else {
                None
            };
            let result = winapi::winmm::waveOutGetVolume(machine, hwo, pdwVolume);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::waveOutGetVolume_pos.0,
                    winapi::winmm::waveOutGetVolume_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn waveOutOpen(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let phwo = <Option<&mut HWAVEOUT>>::from_stack(mem, stack_args + 0u32);
            let uDeviceID = <u32>::from_stack(mem, stack_args + 4u32);
            let pwfx = <Option<&WAVEFORMATEX>>::from_stack(mem, stack_args + 8u32);
            let dwCallback = <u32>::from_stack(mem, stack_args + 12u32);
            let dwInstance = <u32>::from_stack(mem, stack_args + 16u32);
            let fdwOpen = <Result<WaveOutOpenFlags, u32>>::from_stack(mem, stack_args + 20u32);
            let __trace_context = if crate::trace::enabled("winmm/wave") {
                Some(crate::trace::trace_begin(
                    "winmm/wave",
                    "waveOutOpen",
                    &[
                        ("phwo", &phwo),
                        ("uDeviceID", &uDeviceID),
                        ("pwfx", &pwfx),
                        ("dwCallback", &dwCallback),
                        ("dwInstance", &dwInstance),
                        ("fdwOpen", &fdwOpen),
                    ],
                ))
            } else {
                None
            };
            let result = winapi::winmm::waveOutOpen(
                machine, phwo, uDeviceID, pwfx, dwCallback, dwInstance, fdwOpen,
            );
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::waveOutOpen_pos.0,
                    winapi::winmm::waveOutOpen_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn waveOutPrepareHeader(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let pwh = <Option<&WAVEHDR>>::from_stack(mem, stack_args + 4u32);
            let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("winmm/wave") {
                Some(crate::trace::trace_begin(
                    "winmm/wave",
                    "waveOutPrepareHeader",
                    &[("hwo", &hwo), ("pwh", &pwh), ("cbwh", &cbwh)],
                ))
            } else {
                None
            };
            let result = winapi::winmm::waveOutPrepareHeader(machine, hwo, pwh, cbwh);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::waveOutPrepareHeader_pos.0,
                    winapi::winmm::waveOutPrepareHeader_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn waveOutReset(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let __trace_context = if crate::trace::enabled("winmm/wave") {
                Some(crate::trace::trace_begin(
                    "winmm/wave",
                    "waveOutReset",
                    &[("hwo", &hwo)],
                ))
            } else {
                None
            };
            let result = winapi::winmm::waveOutReset(machine, hwo);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::waveOutReset_pos.0,
                    winapi::winmm::waveOutReset_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn waveOutSetVolume(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let dwVolume = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_context = if crate::trace::enabled("winmm/wave") {
                Some(crate::trace::trace_begin(
                    "winmm/wave",
                    "waveOutSetVolume",
                    &[("hwo", &hwo), ("dwVolume", &dwVolume)],
                ))
            } else {
                None
            };
            let result = winapi::winmm::waveOutSetVolume(machine, hwo, dwVolume);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::waveOutSetVolume_pos.0,
                    winapi::winmm::waveOutSetVolume_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn waveOutUnprepareHeader(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let pwh = <Option<&mut WAVEHDR>>::from_stack(mem, stack_args + 4u32);
            let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("winmm/wave") {
                Some(crate::trace::trace_begin(
                    "winmm/wave",
                    "waveOutUnprepareHeader",
                    &[("hwo", &hwo), ("pwh", &pwh), ("cbwh", &cbwh)],
                ))
            } else {
                None
            };
            let result = winapi::winmm::waveOutUnprepareHeader(machine, hwo, pwh, cbwh);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::waveOutUnprepareHeader_pos.0,
                    winapi::winmm::waveOutUnprepareHeader_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
        pub unsafe fn waveOutWrite(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let pwh = <Option<&WAVEHDR>>::from_stack(mem, stack_args + 4u32);
            let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_context = if crate::trace::enabled("winmm/wave") {
                Some(crate::trace::trace_begin(
                    "winmm/wave",
                    "waveOutWrite",
                    &[("hwo", &hwo), ("pwh", &pwh), ("cbwh", &cbwh)],
                ))
            } else {
                None
            };
            let result = winapi::winmm::waveOutWrite(machine, hwo, pwh, cbwh);
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::winmm::waveOutWrite_pos.0,
                    winapi::winmm::waveOutWrite_pos.1,
                    &result,
                );
            }
            result.to_raw()
        }
    }
    const SHIMS: [Shim; 24usize] = [
        Shim {
            name: "PlaySoundW",
            func: Handler::Sync(wrappers::PlaySoundW),
        },
        Shim {
            name: "mciSendCommandA",
            func: Handler::Sync(wrappers::mciSendCommandA),
        },
        Shim {
            name: "mixerClose",
            func: Handler::Sync(wrappers::mixerClose),
        },
        Shim {
            name: "mixerGetControlDetailsA",
            func: Handler::Sync(wrappers::mixerGetControlDetailsA),
        },
        Shim {
            name: "mixerGetLineControlsA",
            func: Handler::Sync(wrappers::mixerGetLineControlsA),
        },
        Shim {
            name: "mixerGetLineInfoA",
            func: Handler::Sync(wrappers::mixerGetLineInfoA),
        },
        Shim {
            name: "mixerOpen",
            func: Handler::Sync(wrappers::mixerOpen),
        },
        Shim {
            name: "mixerSetControlDetails",
            func: Handler::Sync(wrappers::mixerSetControlDetails),
        },
        Shim {
            name: "timeBeginPeriod",
            func: Handler::Sync(wrappers::timeBeginPeriod),
        },
        Shim {
            name: "timeEndPeriod",
            func: Handler::Sync(wrappers::timeEndPeriod),
        },
        Shim {
            name: "timeGetTime",
            func: Handler::Sync(wrappers::timeGetTime),
        },
        Shim {
            name: "timeKillEvent",
            func: Handler::Sync(wrappers::timeKillEvent),
        },
        Shim {
            name: "timeSetEvent",
            func: Handler::Sync(wrappers::timeSetEvent),
        },
        Shim {
            name: "waveOutClose",
            func: Handler::Sync(wrappers::waveOutClose),
        },
        Shim {
            name: "waveOutGetDevCapsA",
            func: Handler::Sync(wrappers::waveOutGetDevCapsA),
        },
        Shim {
            name: "waveOutGetNumDevs",
            func: Handler::Sync(wrappers::waveOutGetNumDevs),
        },
        Shim {
            name: "waveOutGetPosition",
            func: Handler::Sync(wrappers::waveOutGetPosition),
        },
        Shim {
            name: "waveOutGetVolume",
            func: Handler::Sync(wrappers::waveOutGetVolume),
        },
        Shim {
            name: "waveOutOpen",
            func: Handler::Sync(wrappers::waveOutOpen),
        },
        Shim {
            name: "waveOutPrepareHeader",
            func: Handler::Sync(wrappers::waveOutPrepareHeader),
        },
        Shim {
            name: "waveOutReset",
            func: Handler::Sync(wrappers::waveOutReset),
        },
        Shim {
            name: "waveOutSetVolume",
            func: Handler::Sync(wrappers::waveOutSetVolume),
        },
        Shim {
            name: "waveOutUnprepareHeader",
            func: Handler::Sync(wrappers::waveOutUnprepareHeader),
        },
        Shim {
            name: "waveOutWrite",
            func: Handler::Sync(wrappers::waveOutWrite),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "winmm.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/winmm.dll"),
    };
}
