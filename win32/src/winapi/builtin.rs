#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
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
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::advapi32::*;
        pub unsafe fn RegCloseKey(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            winapi::advapi32::RegCloseKey(machine, hKey).to_raw()
        }
        pub unsafe fn RegCreateKeyA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpSubKey = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let phkResult = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            winapi::advapi32::RegCreateKeyA(machine, hKey, lpSubKey, phkResult).to_raw()
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
            winapi::advapi32::RegCreateKeyExW(
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
            )
            .to_raw()
        }
        pub unsafe fn RegOpenKeyExA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpSubKey = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let ulOptions = <u32>::from_stack(mem, stack_args + 8u32);
            let samDesired = <u32>::from_stack(mem, stack_args + 12u32);
            let phkResult = <Option<&mut HKEY>>::from_stack(mem, stack_args + 16u32);
            winapi::advapi32::RegOpenKeyExA(
                machine, hKey, lpSubKey, ulOptions, samDesired, phkResult,
            )
            .to_raw()
        }
        pub unsafe fn RegQueryValueExA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpValueName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let lpReserved = <u32>::from_stack(mem, stack_args + 8u32);
            let lpType = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpData = <u32>::from_stack(mem, stack_args + 16u32);
            let lpcbData = <Option<&mut u32>>::from_stack(mem, stack_args + 20u32);
            winapi::advapi32::RegQueryValueExA(
                machine,
                hKey,
                lpValueName,
                lpReserved,
                lpType,
                lpData,
                lpcbData,
            )
            .to_raw()
        }
        pub unsafe fn RegQueryValueExW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpValueName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpReserved = <u32>::from_stack(mem, stack_args + 8u32);
            let lpType = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpData = <u32>::from_stack(mem, stack_args + 16u32);
            let lpcbData = <Option<&mut u32>>::from_stack(mem, stack_args + 20u32);
            winapi::advapi32::RegQueryValueExW(
                machine,
                hKey,
                lpValueName,
                lpReserved,
                lpType,
                lpData,
                lpcbData,
            )
            .to_raw()
        }
        pub unsafe fn RegSetValueExA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpValueName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let Reserved = <u32>::from_stack(mem, stack_args + 8u32);
            let dwType = <u32>::from_stack(mem, stack_args + 12u32);
            let lpData = <u32>::from_stack(mem, stack_args + 16u32);
            let cbData = <u32>::from_stack(mem, stack_args + 20u32);
            winapi::advapi32::RegSetValueExA(
                machine,
                hKey,
                lpValueName,
                Reserved,
                dwType,
                lpData,
                cbData,
            )
            .to_raw()
        }
        pub unsafe fn RegSetValueExW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, stack_args + 0u32);
            let lpValueName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let Reserved = <u32>::from_stack(mem, stack_args + 8u32);
            let dwType = <u32>::from_stack(mem, stack_args + 12u32);
            let lpData = <u32>::from_stack(mem, stack_args + 16u32);
            let cbData = <u32>::from_stack(mem, stack_args + 20u32);
            winapi::advapi32::RegSetValueExW(
                machine,
                hKey,
                lpValueName,
                Reserved,
                dwType,
                lpData,
                cbData,
            )
            .to_raw()
        }
    }
    const SHIMS: [Shim; 8usize] = [
        Shim {
            name: "RegCloseKey",
            func: Handler::Sync(impls::RegCloseKey),
        },
        Shim {
            name: "RegCreateKeyA",
            func: Handler::Sync(impls::RegCreateKeyA),
        },
        Shim {
            name: "RegCreateKeyExW",
            func: Handler::Sync(impls::RegCreateKeyExW),
        },
        Shim {
            name: "RegOpenKeyExA",
            func: Handler::Sync(impls::RegOpenKeyExA),
        },
        Shim {
            name: "RegQueryValueExA",
            func: Handler::Sync(impls::RegQueryValueExA),
        },
        Shim {
            name: "RegQueryValueExW",
            func: Handler::Sync(impls::RegQueryValueExW),
        },
        Shim {
            name: "RegSetValueExA",
            func: Handler::Sync(impls::RegSetValueExA),
        },
        Shim {
            name: "RegSetValueExW",
            func: Handler::Sync(impls::RegSetValueExW),
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
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::bass::*;
        pub unsafe fn BASS_ChannelGetPosition(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let mode = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::bass::BASS_ChannelGetPosition(machine, mode).to_raw()
        }
        pub unsafe fn BASS_Free(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::bass::BASS_Free(machine, arg1).to_raw()
        }
        pub unsafe fn BASS_Init(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
            let arg2 = <u32>::from_stack(mem, stack_args + 4u32);
            let arg3 = <u32>::from_stack(mem, stack_args + 8u32);
            let arg4 = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::bass::BASS_Init(machine, arg1, arg2, arg3, arg4).to_raw()
        }
        pub unsafe fn BASS_MusicLoad(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
            let arg2 = <u32>::from_stack(mem, stack_args + 4u32);
            let arg3 = <u32>::from_stack(mem, stack_args + 8u32);
            let arg4 = <u32>::from_stack(mem, stack_args + 12u32);
            let arg5 = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::bass::BASS_MusicLoad(machine, arg1, arg2, arg3, arg4, arg5).to_raw()
        }
        pub unsafe fn BASS_MusicPlay(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::bass::BASS_MusicPlay(machine, arg1).to_raw()
        }
        pub unsafe fn BASS_MusicSetPositionScaler(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
            let arg2 = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::bass::BASS_MusicSetPositionScaler(machine, arg1, arg2).to_raw()
        }
        pub unsafe fn BASS_Start(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::bass::BASS_Start(machine).to_raw()
        }
    }
    const SHIMS: [Shim; 7usize] = [
        Shim {
            name: "BASS_ChannelGetPosition",
            func: Handler::Sync(impls::BASS_ChannelGetPosition),
        },
        Shim {
            name: "BASS_Free",
            func: Handler::Sync(impls::BASS_Free),
        },
        Shim {
            name: "BASS_Init",
            func: Handler::Sync(impls::BASS_Init),
        },
        Shim {
            name: "BASS_MusicLoad",
            func: Handler::Sync(impls::BASS_MusicLoad),
        },
        Shim {
            name: "BASS_MusicPlay",
            func: Handler::Sync(impls::BASS_MusicPlay),
        },
        Shim {
            name: "BASS_MusicSetPositionScaler",
            func: Handler::Sync(impls::BASS_MusicSetPositionScaler),
        },
        Shim {
            name: "BASS_Start",
            func: Handler::Sync(impls::BASS_Start),
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
    mod impls {
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
            winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter).to_raw()
        }
        pub unsafe fn DirectDrawCreateClipper(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, stack_args + 0u32);
            let lplpDDClipper = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::ddraw::DirectDrawCreateClipper(machine, dwFlags, lplpDDClipper, pUnkOuter)
                .to_raw()
        }
        pub unsafe fn DirectDrawCreateEx(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, stack_args + 0u32);
            let lplpDD = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let iid = <Option<&GUID>>::from_stack(mem, stack_args + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter).to_raw()
        }
        pub unsafe fn IDirectDraw2_CreateSurface(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let desc = <Option<&DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
            let lplpDDSurface = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::ddraw::IDirectDraw2::CreateSurface(
                machine,
                this,
                desc,
                lplpDDSurface,
                pUnkOuter,
            )
            .to_raw()
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
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::ddraw::IDirectDraw2::EnumDisplayModes(
                    machine,
                    this,
                    dwFlags,
                    lpSurfaceDesc,
                    lpContext,
                    lpEnumCallback,
                )
                .await
                .to_raw()
            })
        }
        pub unsafe fn IDirectDraw2_GetDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSurfaceDesc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDraw2::GetDisplayMode(machine, this, lpDDSurfaceDesc).to_raw()
        }
        pub unsafe fn IDirectDraw2_QueryInterface(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let riid = <Option<&GUID>>::from_stack(mem, stack_args + 4u32);
            let ppvObject = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::ddraw::IDirectDraw2::QueryInterface(machine, this, riid, ppvObject).to_raw()
        }
        pub unsafe fn IDirectDraw2_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ddraw::IDirectDraw2::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDraw2_SetDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let width = <u32>::from_stack(mem, stack_args + 4u32);
            let height = <u32>::from_stack(mem, stack_args + 8u32);
            let bpp = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::ddraw::IDirectDraw2::SetDisplayMode(machine, this, width, height, bpp).to_raw()
        }
        pub unsafe fn IDirectDraw7_CreatePalette(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let flags = <Result<DDPCAPS, u32>>::from_stack(mem, stack_args + 4u32);
            let entries = <u32>::from_stack(mem, stack_args + 8u32);
            let lplpPalette = <u32>::from_stack(mem, stack_args + 12u32);
            let unused = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::ddraw::IDirectDraw7::CreatePalette(
                machine,
                this,
                flags,
                entries,
                lplpPalette,
                unused,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDraw7_CreateSurface(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let desc = <Option<&DDSURFACEDESC2>>::from_stack(mem, stack_args + 4u32);
            let lpDirectDrawSurface7 = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let unused = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::ddraw::IDirectDraw7::CreateSurface(
                machine,
                this,
                desc,
                lpDirectDrawSurface7,
                unused,
            )
            .to_raw()
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
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::ddraw::IDirectDraw7::EnumDisplayModes(
                    machine,
                    this,
                    dwFlags,
                    lpSurfaceDesc,
                    lpContext,
                    lpEnumCallback,
                )
                .await
                .to_raw()
            })
        }
        pub unsafe fn IDirectDraw7_GetDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSurfaceDesc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDraw7::GetDisplayMode(machine, this, lpDDSurfaceDesc).to_raw()
        }
        pub unsafe fn IDirectDraw7_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ddraw::IDirectDraw7::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDraw7_RestoreDisplayMode(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ddraw::IDirectDraw7::RestoreDisplayMode(machine, this).to_raw()
        }
        pub unsafe fn IDirectDraw7_SetCooperativeLevel(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let hwnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let flags = <Result<DDSCL, u32>>::from_stack(mem, stack_args + 8u32);
            winapi::ddraw::IDirectDraw7::SetCooperativeLevel(machine, this, hwnd, flags).to_raw()
        }
        pub unsafe fn IDirectDraw7_SetDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let width = <u32>::from_stack(mem, stack_args + 4u32);
            let height = <u32>::from_stack(mem, stack_args + 8u32);
            let bpp = <u32>::from_stack(mem, stack_args + 12u32);
            let refresh = <u32>::from_stack(mem, stack_args + 16u32);
            let flags = <u32>::from_stack(mem, stack_args + 20u32);
            winapi::ddraw::IDirectDraw7::SetDisplayMode(
                machine, this, width, height, bpp, refresh, flags,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDraw7_WaitForVerticalBlank(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let flags = <u32>::from_stack(mem, stack_args + 4u32);
            let _unused = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::ddraw::IDirectDraw7::WaitForVerticalBlank(machine, this, flags, _unused)
                .to_raw()
        }
        pub unsafe fn IDirectDrawClipper_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ddraw::IDirectDrawClipper::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawClipper_SetHWnd(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let unused = <u32>::from_stack(mem, stack_args + 4u32);
            let hwnd = <HWND>::from_stack(mem, stack_args + 8u32);
            winapi::ddraw::IDirectDrawClipper::SetHWnd(machine, this, unused, hwnd).to_raw()
        }
        pub unsafe fn IDirectDrawPalette_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ddraw::IDirectDrawPalette::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawPalette_SetEntries(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let unused = <u32>::from_stack(mem, stack_args + 4u32);
            let start = <u32>::from_stack(mem, stack_args + 8u32);
            let count = <u32>::from_stack(mem, stack_args + 12u32);
            let entries = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::ddraw::IDirectDrawPalette::SetEntries(
                machine, this, unused, start, count, entries,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_GetAttachedSurface(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCaps = <Option<&DDSCAPS>>::from_stack(mem, stack_args + 4u32);
            let lpDirectDrawSurface = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            winapi::ddraw::IDirectDrawSurface2::GetAttachedSurface(
                machine,
                this,
                lpDDSCaps,
                lpDirectDrawSurface,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_GetCaps(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCAPS = <Option<&mut DDSCAPS>>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDrawSurface2::GetCaps(machine, this, lpDDSCAPS).to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_GetSurfaceDesc(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDrawSurface2::GetSurfaceDesc(machine, this, desc).to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_Lock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let rect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, stack_args + 8u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, stack_args + 12u32);
            let event = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::ddraw::IDirectDrawSurface2::Lock(machine, this, rect, desc, flags, event)
                .to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ddraw::IDirectDrawSurface2::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_Unlock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let ptr = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDrawSurface2::Unlock(machine, this, ptr).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Blt(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDstRect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let lpSrc = <u32>::from_stack(mem, stack_args + 8u32);
            let lpSrcRect = <Option<&RECT>>::from_stack(mem, stack_args + 12u32);
            let flags = <Result<DDBLT, u32>>::from_stack(mem, stack_args + 16u32);
            let lpDDBLTFX = <Option<&DDBLTFX>>::from_stack(mem, stack_args + 20u32);
            winapi::ddraw::IDirectDrawSurface7::Blt(
                machine, this, lpDstRect, lpSrc, lpSrcRect, flags, lpDDBLTFX,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_BltFast(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let lpSrc = <u32>::from_stack(mem, stack_args + 12u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 16u32);
            let flags = <u32>::from_stack(mem, stack_args + 20u32);
            winapi::ddraw::IDirectDrawSurface7::BltFast(machine, this, x, y, lpSrc, lpRect, flags)
                .to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Flip(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpSurf = <u32>::from_stack(mem, stack_args + 4u32);
            let flags = <Result<DDFLIP, u32>>::from_stack(mem, stack_args + 8u32);
            winapi::ddraw::IDirectDrawSurface7::Flip(machine, this, lpSurf, flags).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetAttachedSurface(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCaps2 = <Option<&DDSCAPS2>>::from_stack(mem, stack_args + 4u32);
            let lpDirectDrawSurface7 = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            winapi::ddraw::IDirectDrawSurface7::GetAttachedSurface(
                machine,
                this,
                lpDDSCaps2,
                lpDirectDrawSurface7,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetCaps(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCAPS2 = <Option<&mut DDSCAPS2>>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDrawSurface7::GetCaps(machine, this, lpDDSCAPS2).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetDC(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpHDC = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDrawSurface7::GetDC(machine, this, lpHDC).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetPixelFormat(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let fmt = <Option<&mut DDPIXELFORMAT>>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDrawSurface7::GetPixelFormat(machine, this, fmt).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetSurfaceDesc(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDesc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDrawSurface7::GetSurfaceDesc(machine, this, lpDesc).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Lock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let rect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let desc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, stack_args + 8u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, stack_args + 12u32);
            let unused = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::ddraw::IDirectDrawSurface7::Lock(machine, this, rect, desc, flags, unused)
                .to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ddraw::IDirectDrawSurface7::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_ReleaseDC(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _this = <u32>::from_stack(mem, stack_args + 0u32);
            let _hDC = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDrawSurface7::ReleaseDC(machine, _this, _hDC).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Restore(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _this = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ddraw::IDirectDrawSurface7::Restore(machine, _this).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_SetClipper(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let clipper = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDrawSurface7::SetClipper(machine, this, clipper).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_SetPalette(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let palette = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDrawSurface7::SetPalette(machine, this, palette).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Unlock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let rect = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDrawSurface7::Unlock(machine, this, rect).to_raw()
        }
        pub unsafe fn IDirectDrawSurface_GetAttachedSurface(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCaps = <Option<&DDSCAPS>>::from_stack(mem, stack_args + 4u32);
            let lpDirectDrawSurface = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            winapi::ddraw::IDirectDrawSurface::GetAttachedSurface(
                machine,
                this,
                lpDDSCaps,
                lpDirectDrawSurface,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface_GetCaps(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCAPS = <Option<&mut DDSCAPS>>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDrawSurface::GetCaps(machine, this, lpDDSCAPS).to_raw()
        }
        pub unsafe fn IDirectDrawSurface_Lock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let rect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, stack_args + 8u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, stack_args + 12u32);
            let event = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::ddraw::IDirectDrawSurface::Lock(machine, this, rect, desc, flags, event)
                .to_raw()
        }
        pub unsafe fn IDirectDrawSurface_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ddraw::IDirectDrawSurface::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawSurface_Unlock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let ptr = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::ddraw::IDirectDrawSurface::Unlock(machine, this, ptr).to_raw()
        }
        pub unsafe fn IDirectDraw_CreateSurface(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let desc = <Option<&DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
            let lplpDDSurface = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::ddraw::IDirectDraw::CreateSurface(machine, this, desc, lplpDDSurface, pUnkOuter)
                .to_raw()
        }
        pub unsafe fn IDirectDraw_EnumDisplayModes(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpSurfaceDesc = <Option<&DDSURFACEDESC>>::from_stack(mem, stack_args + 8u32);
            let lpContext = <u32>::from_stack(mem, stack_args + 12u32);
            let lpEnumCallback = <u32>::from_stack(mem, stack_args + 16u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::ddraw::IDirectDraw::EnumDisplayModes(
                    machine,
                    this,
                    dwFlags,
                    lpSurfaceDesc,
                    lpContext,
                    lpEnumCallback,
                )
                .await
                .to_raw()
            })
        }
        pub unsafe fn IDirectDraw_QueryInterface(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let riid = <Option<&GUID>>::from_stack(mem, stack_args + 4u32);
            let ppvObject = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            winapi::ddraw::IDirectDraw::QueryInterface(machine, this, riid, ppvObject).to_raw()
        }
        pub unsafe fn IDirectDraw_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ddraw::IDirectDraw::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDraw_SetDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let width = <u32>::from_stack(mem, stack_args + 4u32);
            let height = <u32>::from_stack(mem, stack_args + 8u32);
            let bpp = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::ddraw::IDirectDraw::SetDisplayMode(machine, this, width, height, bpp).to_raw()
        }
    }
    const SHIMS: [Shim; 53usize] = [
        Shim {
            name: "DirectDrawCreate",
            func: Handler::Sync(impls::DirectDrawCreate),
        },
        Shim {
            name: "DirectDrawCreateClipper",
            func: Handler::Sync(impls::DirectDrawCreateClipper),
        },
        Shim {
            name: "DirectDrawCreateEx",
            func: Handler::Sync(impls::DirectDrawCreateEx),
        },
        Shim {
            name: "IDirectDraw2::CreateSurface",
            func: Handler::Sync(impls::IDirectDraw2_CreateSurface),
        },
        Shim {
            name: "IDirectDraw2::EnumDisplayModes",
            func: Handler::Async(impls::IDirectDraw2_EnumDisplayModes),
        },
        Shim {
            name: "IDirectDraw2::GetDisplayMode",
            func: Handler::Sync(impls::IDirectDraw2_GetDisplayMode),
        },
        Shim {
            name: "IDirectDraw2::QueryInterface",
            func: Handler::Sync(impls::IDirectDraw2_QueryInterface),
        },
        Shim {
            name: "IDirectDraw2::Release",
            func: Handler::Sync(impls::IDirectDraw2_Release),
        },
        Shim {
            name: "IDirectDraw2::SetDisplayMode",
            func: Handler::Sync(impls::IDirectDraw2_SetDisplayMode),
        },
        Shim {
            name: "IDirectDraw7::CreatePalette",
            func: Handler::Sync(impls::IDirectDraw7_CreatePalette),
        },
        Shim {
            name: "IDirectDraw7::CreateSurface",
            func: Handler::Sync(impls::IDirectDraw7_CreateSurface),
        },
        Shim {
            name: "IDirectDraw7::EnumDisplayModes",
            func: Handler::Async(impls::IDirectDraw7_EnumDisplayModes),
        },
        Shim {
            name: "IDirectDraw7::GetDisplayMode",
            func: Handler::Sync(impls::IDirectDraw7_GetDisplayMode),
        },
        Shim {
            name: "IDirectDraw7::Release",
            func: Handler::Sync(impls::IDirectDraw7_Release),
        },
        Shim {
            name: "IDirectDraw7::RestoreDisplayMode",
            func: Handler::Sync(impls::IDirectDraw7_RestoreDisplayMode),
        },
        Shim {
            name: "IDirectDraw7::SetCooperativeLevel",
            func: Handler::Sync(impls::IDirectDraw7_SetCooperativeLevel),
        },
        Shim {
            name: "IDirectDraw7::SetDisplayMode",
            func: Handler::Sync(impls::IDirectDraw7_SetDisplayMode),
        },
        Shim {
            name: "IDirectDraw7::WaitForVerticalBlank",
            func: Handler::Sync(impls::IDirectDraw7_WaitForVerticalBlank),
        },
        Shim {
            name: "IDirectDrawClipper::Release",
            func: Handler::Sync(impls::IDirectDrawClipper_Release),
        },
        Shim {
            name: "IDirectDrawClipper::SetHWnd",
            func: Handler::Sync(impls::IDirectDrawClipper_SetHWnd),
        },
        Shim {
            name: "IDirectDrawPalette::Release",
            func: Handler::Sync(impls::IDirectDrawPalette_Release),
        },
        Shim {
            name: "IDirectDrawPalette::SetEntries",
            func: Handler::Sync(impls::IDirectDrawPalette_SetEntries),
        },
        Shim {
            name: "IDirectDrawSurface2::GetAttachedSurface",
            func: Handler::Sync(impls::IDirectDrawSurface2_GetAttachedSurface),
        },
        Shim {
            name: "IDirectDrawSurface2::GetCaps",
            func: Handler::Sync(impls::IDirectDrawSurface2_GetCaps),
        },
        Shim {
            name: "IDirectDrawSurface2::GetSurfaceDesc",
            func: Handler::Sync(impls::IDirectDrawSurface2_GetSurfaceDesc),
        },
        Shim {
            name: "IDirectDrawSurface2::Lock",
            func: Handler::Sync(impls::IDirectDrawSurface2_Lock),
        },
        Shim {
            name: "IDirectDrawSurface2::Release",
            func: Handler::Sync(impls::IDirectDrawSurface2_Release),
        },
        Shim {
            name: "IDirectDrawSurface2::Unlock",
            func: Handler::Sync(impls::IDirectDrawSurface2_Unlock),
        },
        Shim {
            name: "IDirectDrawSurface7::Blt",
            func: Handler::Sync(impls::IDirectDrawSurface7_Blt),
        },
        Shim {
            name: "IDirectDrawSurface7::BltFast",
            func: Handler::Sync(impls::IDirectDrawSurface7_BltFast),
        },
        Shim {
            name: "IDirectDrawSurface7::Flip",
            func: Handler::Sync(impls::IDirectDrawSurface7_Flip),
        },
        Shim {
            name: "IDirectDrawSurface7::GetAttachedSurface",
            func: Handler::Sync(impls::IDirectDrawSurface7_GetAttachedSurface),
        },
        Shim {
            name: "IDirectDrawSurface7::GetCaps",
            func: Handler::Sync(impls::IDirectDrawSurface7_GetCaps),
        },
        Shim {
            name: "IDirectDrawSurface7::GetDC",
            func: Handler::Sync(impls::IDirectDrawSurface7_GetDC),
        },
        Shim {
            name: "IDirectDrawSurface7::GetPixelFormat",
            func: Handler::Sync(impls::IDirectDrawSurface7_GetPixelFormat),
        },
        Shim {
            name: "IDirectDrawSurface7::GetSurfaceDesc",
            func: Handler::Sync(impls::IDirectDrawSurface7_GetSurfaceDesc),
        },
        Shim {
            name: "IDirectDrawSurface7::Lock",
            func: Handler::Sync(impls::IDirectDrawSurface7_Lock),
        },
        Shim {
            name: "IDirectDrawSurface7::Release",
            func: Handler::Sync(impls::IDirectDrawSurface7_Release),
        },
        Shim {
            name: "IDirectDrawSurface7::ReleaseDC",
            func: Handler::Sync(impls::IDirectDrawSurface7_ReleaseDC),
        },
        Shim {
            name: "IDirectDrawSurface7::Restore",
            func: Handler::Sync(impls::IDirectDrawSurface7_Restore),
        },
        Shim {
            name: "IDirectDrawSurface7::SetClipper",
            func: Handler::Sync(impls::IDirectDrawSurface7_SetClipper),
        },
        Shim {
            name: "IDirectDrawSurface7::SetPalette",
            func: Handler::Sync(impls::IDirectDrawSurface7_SetPalette),
        },
        Shim {
            name: "IDirectDrawSurface7::Unlock",
            func: Handler::Sync(impls::IDirectDrawSurface7_Unlock),
        },
        Shim {
            name: "IDirectDrawSurface::GetAttachedSurface",
            func: Handler::Sync(impls::IDirectDrawSurface_GetAttachedSurface),
        },
        Shim {
            name: "IDirectDrawSurface::GetCaps",
            func: Handler::Sync(impls::IDirectDrawSurface_GetCaps),
        },
        Shim {
            name: "IDirectDrawSurface::Lock",
            func: Handler::Sync(impls::IDirectDrawSurface_Lock),
        },
        Shim {
            name: "IDirectDrawSurface::Release",
            func: Handler::Sync(impls::IDirectDrawSurface_Release),
        },
        Shim {
            name: "IDirectDrawSurface::Unlock",
            func: Handler::Sync(impls::IDirectDrawSurface_Unlock),
        },
        Shim {
            name: "IDirectDraw::CreateSurface",
            func: Handler::Sync(impls::IDirectDraw_CreateSurface),
        },
        Shim {
            name: "IDirectDraw::EnumDisplayModes",
            func: Handler::Async(impls::IDirectDraw_EnumDisplayModes),
        },
        Shim {
            name: "IDirectDraw::QueryInterface",
            func: Handler::Sync(impls::IDirectDraw_QueryInterface),
        },
        Shim {
            name: "IDirectDraw::Release",
            func: Handler::Sync(impls::IDirectDraw_Release),
        },
        Shim {
            name: "IDirectDraw::SetDisplayMode",
            func: Handler::Sync(impls::IDirectDraw_SetDisplayMode),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ddraw.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/ddraw.dll"),
    };
}
pub mod dsound {
    use super::*;
    mod impls {
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
            winapi::dsound::DirectSoundCreate(machine, lpGuid, ppDS, pUnkOuter).to_raw()
        }
        pub unsafe fn DirectSoundEnumerateA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpDSEnumCallback = <u32>::from_stack(mem, stack_args + 0u32);
            let lpContext = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::dsound::DirectSoundEnumerateA(machine, lpDSEnumCallback, lpContext).to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_GetCurrentPosition(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpdwCurrentPlayCursor = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let lpdwCurrentWriteCursor = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            winapi::dsound::IDirectSoundBuffer::GetCurrentPosition(
                machine,
                this,
                lpdwCurrentPlayCursor,
                lpdwCurrentWriteCursor,
            )
            .to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_GetStatus(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpdwStatus = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            winapi::dsound::IDirectSoundBuffer::GetStatus(machine, this, lpdwStatus).to_raw()
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
            winapi::dsound::IDirectSoundBuffer::Lock(
                machine,
                this,
                dwWriteCursor,
                dwWriteBytes,
                lplpvAudioPtr1,
                lpdwAudioBytes1,
                lplpvAudioPtr2,
                lpdwAudioBytes2,
                dwFlags,
            )
            .to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_Play(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let dwReserved1 = <u32>::from_stack(mem, stack_args + 4u32);
            let dwReserved2 = <u32>::from_stack(mem, stack_args + 8u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::dsound::IDirectSoundBuffer::Play(
                machine,
                this,
                dwReserved1,
                dwReserved2,
                dwFlags,
            )
            .to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::dsound::IDirectSoundBuffer::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_SetFormat(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpcfxFormat = <Option<&WAVEFORMATEX>>::from_stack(mem, stack_args + 4u32);
            winapi::dsound::IDirectSoundBuffer::SetFormat(machine, this, lpcfxFormat).to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_Unlock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpvAudioPtr1 = <u32>::from_stack(mem, stack_args + 4u32);
            let dwAudioBytes1 = <u32>::from_stack(mem, stack_args + 8u32);
            let lpvAudioPtr2 = <u32>::from_stack(mem, stack_args + 12u32);
            let dwAudioBytes2 = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::dsound::IDirectSoundBuffer::Unlock(
                machine,
                this,
                lpvAudioPtr1,
                dwAudioBytes1,
                lpvAudioPtr2,
                dwAudioBytes2,
            )
            .to_raw()
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
            winapi::dsound::IDirectSound::CreateSoundBuffer(
                machine,
                this,
                lpcDSBufferDesc,
                lplpDirectSoundBuffer,
                pUnkOuter,
            )
            .to_raw()
        }
        pub unsafe fn IDirectSound_Release(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::dsound::IDirectSound::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectSound_SetCooperativeLevel(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let hwnd = <u32>::from_stack(mem, stack_args + 4u32);
            let dwLevel = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::dsound::IDirectSound::SetCooperativeLevel(machine, this, hwnd, dwLevel).to_raw()
        }
    }
    const SHIMS: [Shim; 12usize] = [
        Shim {
            name: "DirectSoundCreate",
            func: Handler::Sync(impls::DirectSoundCreate),
        },
        Shim {
            name: "DirectSoundEnumerateA",
            func: Handler::Sync(impls::DirectSoundEnumerateA),
        },
        Shim {
            name: "IDirectSoundBuffer::GetCurrentPosition",
            func: Handler::Sync(impls::IDirectSoundBuffer_GetCurrentPosition),
        },
        Shim {
            name: "IDirectSoundBuffer::GetStatus",
            func: Handler::Sync(impls::IDirectSoundBuffer_GetStatus),
        },
        Shim {
            name: "IDirectSoundBuffer::Lock",
            func: Handler::Sync(impls::IDirectSoundBuffer_Lock),
        },
        Shim {
            name: "IDirectSoundBuffer::Play",
            func: Handler::Sync(impls::IDirectSoundBuffer_Play),
        },
        Shim {
            name: "IDirectSoundBuffer::Release",
            func: Handler::Sync(impls::IDirectSoundBuffer_Release),
        },
        Shim {
            name: "IDirectSoundBuffer::SetFormat",
            func: Handler::Sync(impls::IDirectSoundBuffer_SetFormat),
        },
        Shim {
            name: "IDirectSoundBuffer::Unlock",
            func: Handler::Sync(impls::IDirectSoundBuffer_Unlock),
        },
        Shim {
            name: "IDirectSound::CreateSoundBuffer",
            func: Handler::Sync(impls::IDirectSound_CreateSoundBuffer),
        },
        Shim {
            name: "IDirectSound::Release",
            func: Handler::Sync(impls::IDirectSound_Release),
        },
        Shim {
            name: "IDirectSound::SetCooperativeLevel",
            func: Handler::Sync(impls::IDirectSound_SetCooperativeLevel),
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
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::gdi32::*;
        pub unsafe fn BitBlt(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <i32>::from_stack(mem, stack_args + 4u32);
            let y = <i32>::from_stack(mem, stack_args + 8u32);
            let cx = <u32>::from_stack(mem, stack_args + 12u32);
            let cy = <u32>::from_stack(mem, stack_args + 16u32);
            let hdcSrc = <HDC>::from_stack(mem, stack_args + 20u32);
            let x1 = <i32>::from_stack(mem, stack_args + 24u32);
            let y1 = <i32>::from_stack(mem, stack_args + 28u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, stack_args + 32u32);
            winapi::gdi32::BitBlt(machine, hdc, x, y, cx, cy, hdcSrc, x1, y1, rop).to_raw()
        }
        pub unsafe fn CreateBitmap(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nWidth = <u32>::from_stack(mem, stack_args + 0u32);
            let nHeight = <u32>::from_stack(mem, stack_args + 4u32);
            let nPlanes = <u32>::from_stack(mem, stack_args + 8u32);
            let nBitCount = <u32>::from_stack(mem, stack_args + 12u32);
            let lpBits = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::gdi32::CreateBitmap(machine, nWidth, nHeight, nPlanes, nBitCount, lpBits)
                .to_raw()
        }
        pub unsafe fn CreateCompatibleBitmap(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let cx = <u32>::from_stack(mem, stack_args + 4u32);
            let cy = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::gdi32::CreateCompatibleBitmap(machine, hdc, cx, cy).to_raw()
        }
        pub unsafe fn CreateCompatibleDC(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            winapi::gdi32::CreateCompatibleDC(machine, hdc).to_raw()
        }
        pub unsafe fn CreateDIBSection(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let pbmi = <Option<&BITMAPINFOHEADER>>::from_stack(mem, stack_args + 4u32);
            let usage = <u32>::from_stack(mem, stack_args + 8u32);
            let ppvBits = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let hSection = <u32>::from_stack(mem, stack_args + 16u32);
            let offset = <u32>::from_stack(mem, stack_args + 20u32);
            winapi::gdi32::CreateDIBSection(machine, hdc, pbmi, usage, ppvBits, hSection, offset)
                .to_raw()
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
            winapi::gdi32::CreateFontA(
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
            )
            .to_raw()
        }
        pub unsafe fn CreatePalette(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let plpal = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::gdi32::CreatePalette(machine, plpal).to_raw()
        }
        pub unsafe fn CreatePen(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let iStyle = <Result<PS, u32>>::from_stack(mem, stack_args + 0u32);
            let cWidth = <u32>::from_stack(mem, stack_args + 4u32);
            let color = <COLORREF>::from_stack(mem, stack_args + 8u32);
            winapi::gdi32::CreatePen(machine, iStyle, cWidth, color).to_raw()
        }
        pub unsafe fn CreateSolidBrush(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let color = <COLORREF>::from_stack(mem, stack_args + 0u32);
            winapi::gdi32::CreateSolidBrush(machine, color).to_raw()
        }
        pub unsafe fn DeleteDC(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::gdi32::DeleteDC(machine, hdc).to_raw()
        }
        pub unsafe fn DeleteObject(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let handle = <HGDIOBJ>::from_stack(mem, stack_args + 0u32);
            winapi::gdi32::DeleteObject(machine, handle).to_raw()
        }
        pub unsafe fn GetDCOrgEx(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpPoint = <Option<&mut POINT>>::from_stack(mem, stack_args + 4u32);
            winapi::gdi32::GetDCOrgEx(machine, hdc, lpPoint).to_raw()
        }
        pub unsafe fn GetDeviceCaps(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let index = <Result<GetDeviceCapsArg, u32>>::from_stack(mem, stack_args + 4u32);
            winapi::gdi32::GetDeviceCaps(machine, hdc, index).to_raw()
        }
        pub unsafe fn GetLayout(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            winapi::gdi32::GetLayout(machine, hdc).to_raw()
        }
        pub unsafe fn GetObjectA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let handle = <HGDIOBJ>::from_stack(mem, stack_args + 0u32);
            let bytes = <u32>::from_stack(mem, stack_args + 4u32);
            let out = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::gdi32::GetObjectA(machine, handle, bytes, out).to_raw()
        }
        pub unsafe fn GetPixel(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::gdi32::GetPixel(machine, hdc, x, y).to_raw()
        }
        pub unsafe fn GetStockObject(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let i = <Result<GetStockObjectArg, u32>>::from_stack(mem, stack_args + 0u32);
            winapi::gdi32::GetStockObject(machine, i).to_raw()
        }
        pub unsafe fn GetTextExtentPoint32A(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let c = <i32>::from_stack(mem, stack_args + 8u32);
            let psizl = <Option<&mut SIZE>>::from_stack(mem, stack_args + 12u32);
            winapi::gdi32::GetTextExtentPoint32A(machine, hdc, lpString, c, psizl).to_raw()
        }
        pub unsafe fn GetTextExtentPoint32W(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let c = <i32>::from_stack(mem, stack_args + 8u32);
            let psizl = <Option<&mut SIZE>>::from_stack(mem, stack_args + 12u32);
            winapi::gdi32::GetTextExtentPoint32W(machine, hdc, lpString, c, psizl).to_raw()
        }
        pub unsafe fn GetTextMetricsA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lptm = <Option<&mut TEXTMETRICA>>::from_stack(mem, stack_args + 4u32);
            winapi::gdi32::GetTextMetricsA(machine, hdc, lptm).to_raw()
        }
        pub unsafe fn GetTextMetricsW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lptm = <Option<&mut TEXTMETRICW>>::from_stack(mem, stack_args + 4u32);
            winapi::gdi32::GetTextMetricsW(machine, hdc, lptm).to_raw()
        }
        pub unsafe fn LineDDA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let xStart = <i32>::from_stack(mem, stack_args + 0u32);
            let yStart = <i32>::from_stack(mem, stack_args + 4u32);
            let xEnd = <i32>::from_stack(mem, stack_args + 8u32);
            let yEnd = <i32>::from_stack(mem, stack_args + 12u32);
            let lpProc = <u32>::from_stack(mem, stack_args + 16u32);
            let data = <u32>::from_stack(mem, stack_args + 20u32);
            winapi::gdi32::LineDDA(machine, xStart, yStart, xEnd, yEnd, lpProc, data).to_raw()
        }
        pub unsafe fn LineTo(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::gdi32::LineTo(machine, hdc, x, y).to_raw()
        }
        pub unsafe fn MoveToEx(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let lppt = <Option<&mut POINT>>::from_stack(mem, stack_args + 12u32);
            winapi::gdi32::MoveToEx(machine, hdc, x, y, lppt).to_raw()
        }
        pub unsafe fn PatBlt(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <i32>::from_stack(mem, stack_args + 4u32);
            let y = <i32>::from_stack(mem, stack_args + 8u32);
            let w = <i32>::from_stack(mem, stack_args + 12u32);
            let h = <i32>::from_stack(mem, stack_args + 16u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, stack_args + 20u32);
            winapi::gdi32::PatBlt(machine, hdc, x, y, w, h, rop).to_raw()
        }
        pub unsafe fn PtVisible(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <i32>::from_stack(mem, stack_args + 4u32);
            let y = <i32>::from_stack(mem, stack_args + 8u32);
            winapi::gdi32::PtVisible(machine, hdc, x, y).to_raw()
        }
        pub unsafe fn SelectObject(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let hGdiObj = <HGDIOBJ>::from_stack(mem, stack_args + 4u32);
            winapi::gdi32::SelectObject(machine, hdc, hGdiObj).to_raw()
        }
        pub unsafe fn SetBkColor(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let color = <COLORREF>::from_stack(mem, stack_args + 4u32);
            winapi::gdi32::SetBkColor(machine, hdc, color).to_raw()
        }
        pub unsafe fn SetBkMode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let mode = <i32>::from_stack(mem, stack_args + 4u32);
            winapi::gdi32::SetBkMode(machine, hdc, mode).to_raw()
        }
        pub unsafe fn SetBrushOrgEx(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <i32>::from_stack(mem, stack_args + 4u32);
            let y = <i32>::from_stack(mem, stack_args + 8u32);
            let lppt = <Option<&mut POINT>>::from_stack(mem, stack_args + 12u32);
            winapi::gdi32::SetBrushOrgEx(machine, hdc, x, y, lppt).to_raw()
        }
        pub unsafe fn SetDIBitsToDevice(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let xDest = <u32>::from_stack(mem, stack_args + 4u32);
            let yDest = <u32>::from_stack(mem, stack_args + 8u32);
            let w = <u32>::from_stack(mem, stack_args + 12u32);
            let h = <u32>::from_stack(mem, stack_args + 16u32);
            let xSrc = <u32>::from_stack(mem, stack_args + 20u32);
            let ySrc = <u32>::from_stack(mem, stack_args + 24u32);
            let StartScan = <u32>::from_stack(mem, stack_args + 28u32);
            let cLines = <u32>::from_stack(mem, stack_args + 32u32);
            let lpvBits = <u32>::from_stack(mem, stack_args + 36u32);
            let lpbmi = <u32>::from_stack(mem, stack_args + 40u32);
            let ColorUse = <u32>::from_stack(mem, stack_args + 44u32);
            winapi::gdi32::SetDIBitsToDevice(
                machine, hdc, xDest, yDest, w, h, xSrc, ySrc, StartScan, cLines, lpvBits, lpbmi,
                ColorUse,
            )
            .to_raw()
        }
        pub unsafe fn SetLayout(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let l = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::gdi32::SetLayout(machine, hdc, l).to_raw()
        }
        pub unsafe fn SetPixel(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let color = <COLORREF>::from_stack(mem, stack_args + 12u32);
            winapi::gdi32::SetPixel(machine, hdc, x, y, color).to_raw()
        }
        pub unsafe fn SetROP2(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let rop2 = <Result<R2, u32>>::from_stack(mem, stack_args + 4u32);
            winapi::gdi32::SetROP2(machine, hdc, rop2).to_raw()
        }
        pub unsafe fn SetTextAlign(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let fMode = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::gdi32::SetTextAlign(machine, hdc, fMode).to_raw()
        }
        pub unsafe fn SetTextColor(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let color = <COLORREF>::from_stack(mem, stack_args + 4u32);
            winapi::gdi32::SetTextColor(machine, hdc, color).to_raw()
        }
        pub unsafe fn StretchBlt(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdcDest = <HDC>::from_stack(mem, stack_args + 0u32);
            let xDest = <i32>::from_stack(mem, stack_args + 4u32);
            let yDest = <i32>::from_stack(mem, stack_args + 8u32);
            let wDest = <u32>::from_stack(mem, stack_args + 12u32);
            let hDest = <u32>::from_stack(mem, stack_args + 16u32);
            let hdcSrc = <HDC>::from_stack(mem, stack_args + 20u32);
            let xSrc = <i32>::from_stack(mem, stack_args + 24u32);
            let ySrc = <i32>::from_stack(mem, stack_args + 28u32);
            let wSrc = <u32>::from_stack(mem, stack_args + 32u32);
            let hSrc = <u32>::from_stack(mem, stack_args + 36u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, stack_args + 40u32);
            winapi::gdi32::StretchBlt(
                machine, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
            )
            .to_raw()
        }
        pub unsafe fn StretchDIBits(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let xDest = <u32>::from_stack(mem, stack_args + 4u32);
            let yDest = <u32>::from_stack(mem, stack_args + 8u32);
            let DestWidth = <u32>::from_stack(mem, stack_args + 12u32);
            let DestHeight = <u32>::from_stack(mem, stack_args + 16u32);
            let xSrc = <u32>::from_stack(mem, stack_args + 20u32);
            let ySrc = <u32>::from_stack(mem, stack_args + 24u32);
            let SrcWidth = <u32>::from_stack(mem, stack_args + 28u32);
            let SrcHeight = <u32>::from_stack(mem, stack_args + 32u32);
            let lpBits = <u32>::from_stack(mem, stack_args + 36u32);
            let lpbmi = <u32>::from_stack(mem, stack_args + 40u32);
            let iUsage = <u32>::from_stack(mem, stack_args + 44u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, stack_args + 48u32);
            winapi::gdi32::StretchDIBits(
                machine, hdc, xDest, yDest, DestWidth, DestHeight, xSrc, ySrc, SrcWidth, SrcHeight,
                lpBits, lpbmi, iUsage, rop,
            )
            .to_raw()
        }
        pub unsafe fn TextOutA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let lpString = <ArrayWithSize<u8>>::from_stack(mem, stack_args + 12u32);
            winapi::gdi32::TextOutA(machine, hdc, x, y, lpString).to_raw()
        }
        pub unsafe fn TextOutW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let lpString = <ArrayWithSize<u16>>::from_stack(mem, stack_args + 12u32);
            winapi::gdi32::TextOutW(machine, hdc, x, y, lpString).to_raw()
        }
    }
    const SHIMS: [Shim; 40usize] = [
        Shim {
            name: "BitBlt",
            func: Handler::Sync(impls::BitBlt),
        },
        Shim {
            name: "CreateBitmap",
            func: Handler::Sync(impls::CreateBitmap),
        },
        Shim {
            name: "CreateCompatibleBitmap",
            func: Handler::Sync(impls::CreateCompatibleBitmap),
        },
        Shim {
            name: "CreateCompatibleDC",
            func: Handler::Sync(impls::CreateCompatibleDC),
        },
        Shim {
            name: "CreateDIBSection",
            func: Handler::Sync(impls::CreateDIBSection),
        },
        Shim {
            name: "CreateFontA",
            func: Handler::Sync(impls::CreateFontA),
        },
        Shim {
            name: "CreatePalette",
            func: Handler::Sync(impls::CreatePalette),
        },
        Shim {
            name: "CreatePen",
            func: Handler::Sync(impls::CreatePen),
        },
        Shim {
            name: "CreateSolidBrush",
            func: Handler::Sync(impls::CreateSolidBrush),
        },
        Shim {
            name: "DeleteDC",
            func: Handler::Sync(impls::DeleteDC),
        },
        Shim {
            name: "DeleteObject",
            func: Handler::Sync(impls::DeleteObject),
        },
        Shim {
            name: "GetDCOrgEx",
            func: Handler::Sync(impls::GetDCOrgEx),
        },
        Shim {
            name: "GetDeviceCaps",
            func: Handler::Sync(impls::GetDeviceCaps),
        },
        Shim {
            name: "GetLayout",
            func: Handler::Sync(impls::GetLayout),
        },
        Shim {
            name: "GetObjectA",
            func: Handler::Sync(impls::GetObjectA),
        },
        Shim {
            name: "GetPixel",
            func: Handler::Sync(impls::GetPixel),
        },
        Shim {
            name: "GetStockObject",
            func: Handler::Sync(impls::GetStockObject),
        },
        Shim {
            name: "GetTextExtentPoint32A",
            func: Handler::Sync(impls::GetTextExtentPoint32A),
        },
        Shim {
            name: "GetTextExtentPoint32W",
            func: Handler::Sync(impls::GetTextExtentPoint32W),
        },
        Shim {
            name: "GetTextMetricsA",
            func: Handler::Sync(impls::GetTextMetricsA),
        },
        Shim {
            name: "GetTextMetricsW",
            func: Handler::Sync(impls::GetTextMetricsW),
        },
        Shim {
            name: "LineDDA",
            func: Handler::Sync(impls::LineDDA),
        },
        Shim {
            name: "LineTo",
            func: Handler::Sync(impls::LineTo),
        },
        Shim {
            name: "MoveToEx",
            func: Handler::Sync(impls::MoveToEx),
        },
        Shim {
            name: "PatBlt",
            func: Handler::Sync(impls::PatBlt),
        },
        Shim {
            name: "PtVisible",
            func: Handler::Sync(impls::PtVisible),
        },
        Shim {
            name: "SelectObject",
            func: Handler::Sync(impls::SelectObject),
        },
        Shim {
            name: "SetBkColor",
            func: Handler::Sync(impls::SetBkColor),
        },
        Shim {
            name: "SetBkMode",
            func: Handler::Sync(impls::SetBkMode),
        },
        Shim {
            name: "SetBrushOrgEx",
            func: Handler::Sync(impls::SetBrushOrgEx),
        },
        Shim {
            name: "SetDIBitsToDevice",
            func: Handler::Sync(impls::SetDIBitsToDevice),
        },
        Shim {
            name: "SetLayout",
            func: Handler::Sync(impls::SetLayout),
        },
        Shim {
            name: "SetPixel",
            func: Handler::Sync(impls::SetPixel),
        },
        Shim {
            name: "SetROP2",
            func: Handler::Sync(impls::SetROP2),
        },
        Shim {
            name: "SetTextAlign",
            func: Handler::Sync(impls::SetTextAlign),
        },
        Shim {
            name: "SetTextColor",
            func: Handler::Sync(impls::SetTextColor),
        },
        Shim {
            name: "StretchBlt",
            func: Handler::Sync(impls::StretchBlt),
        },
        Shim {
            name: "StretchDIBits",
            func: Handler::Sync(impls::StretchDIBits),
        },
        Shim {
            name: "TextOutA",
            func: Handler::Sync(impls::TextOutA),
        },
        Shim {
            name: "TextOutW",
            func: Handler::Sync(impls::TextOutW),
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
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::kernel32::*;
        pub unsafe fn AcquireSRWLockExclusive(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::AcquireSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn AcquireSRWLockShared(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::AcquireSRWLockShared(machine, SRWLock).to_raw()
        }
        pub unsafe fn AddVectoredExceptionHandler(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let first = <u32>::from_stack(mem, stack_args + 0u32);
            let handler = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::AddVectoredExceptionHandler(machine, first, handler).to_raw()
        }
        pub unsafe fn CloseHandle(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hObject = <HFILE>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::CloseHandle(machine, hObject).to_raw()
        }
        pub unsafe fn CreateDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::CreateDirectoryA(machine, lpPathName, lpSecurityAttributes).to_raw()
        }
        pub unsafe fn CreateEventA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpEventAttributes = <u32>::from_stack(mem, stack_args + 0u32);
            let bManualReset = <bool>::from_stack(mem, stack_args + 4u32);
            let bInitialState = <bool>::from_stack(mem, stack_args + 8u32);
            let lpName = <Option<&str>>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::CreateEventA(
                machine,
                lpEventAttributes,
                bManualReset,
                bInitialState,
                lpName,
            )
            .to_raw()
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
            winapi::kernel32::CreateFileA(
                machine,
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            )
            .to_raw()
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
            winapi::kernel32::CreateFileW(
                machine,
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            )
            .to_raw()
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
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::kernel32::CreateThread(
                    machine,
                    lpThreadAttributes,
                    dwStackSize,
                    lpStartAddress,
                    lpParameter,
                    dwCreationFlags,
                    lpThreadId,
                )
                .await
                .to_raw()
            })
        }
        pub unsafe fn DebugBreak(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::DebugBreak(machine).to_raw()
        }
        pub unsafe fn DeleteCriticalSection(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::DeleteCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn DeleteFileA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::DeleteFileA(machine, lpFileName).to_raw()
        }
        pub unsafe fn DisableThreadLibraryCalls(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hLibModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::DisableThreadLibraryCalls(machine, hLibModule).to_raw()
        }
        pub unsafe fn EnterCriticalSection(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::EnterCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn ExitProcess(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uExitCode = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::ExitProcess(machine, uExitCode).to_raw()
        }
        pub unsafe fn FileTimeToSystemTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 0u32);
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::FileTimeToSystemTime(machine, lpFileTime, lpSystemTime).to_raw()
        }
        pub unsafe fn FindClose(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFindFile = <HFIND>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::FindClose(machine, hFindFile).to_raw()
        }
        pub unsafe fn FindFirstFileA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpFindFileData =
                <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::FindFirstFileA(machine, lpFileName, lpFindFileData).to_raw()
        }
        pub unsafe fn FindNextFileA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFindFile = <HFIND>::from_stack(mem, stack_args + 0u32);
            let lpFindFileData =
                <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::FindNextFileA(machine, hFindFile, lpFindFileData).to_raw()
        }
        pub unsafe fn FindResourceA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let lpName = <ResourceKey<&str>>::from_stack(mem, stack_args + 4u32);
            let lpType = <ResourceKey<&str>>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::FindResourceA(machine, hModule, lpName, lpType).to_raw()
        }
        pub unsafe fn FindResourceW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let lpName = <ResourceKey<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpType = <ResourceKey<&Str16>>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::FindResourceW(machine, hModule, lpName, lpType).to_raw()
        }
        pub unsafe fn FlushFileBuffers(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::FlushFileBuffers(machine, hFile).to_raw()
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
            winapi::kernel32::FormatMessageA(
                machine,
                dwFlags,
                lpSource,
                dwMessageId,
                dwLanguageId,
                lpBuffer,
                nSize,
                args,
            )
            .to_raw()
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
            winapi::kernel32::FormatMessageW(
                machine,
                dwFlags,
                lpSource,
                dwMessageId,
                dwLanguageId,
                lpBuffer,
                nSize,
                args,
            )
            .to_raw()
        }
        pub unsafe fn FreeEnvironmentStringsA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _penv = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::FreeEnvironmentStringsA(machine, _penv).to_raw()
        }
        pub unsafe fn FreeEnvironmentStringsW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::FreeEnvironmentStringsW(machine).to_raw()
        }
        pub unsafe fn FreeLibrary(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hLibModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::FreeLibrary(machine, hLibModule).to_raw()
        }
        pub unsafe fn GetACP(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetACP(machine).to_raw()
        }
        pub unsafe fn GetCPInfo(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _CodePage = <u32>::from_stack(mem, stack_args + 0u32);
            let _lpCPInfo = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo).to_raw()
        }
        pub unsafe fn GetCommandLineA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCommandLineA(machine).to_raw()
        }
        pub unsafe fn GetCommandLineW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCommandLineW(machine).to_raw()
        }
        pub unsafe fn GetConsoleMode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleHandle = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpMode = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::GetConsoleMode(machine, hConsoleHandle, lpMode).to_raw()
        }
        pub unsafe fn GetConsoleScreenBufferInfo(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _hConsoleOutput = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let lpConsoleScreenBufferInfo =
                <Option<&mut CONSOLE_SCREEN_BUFFER_INFO>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::GetConsoleScreenBufferInfo(
                machine,
                _hConsoleOutput,
                lpConsoleScreenBufferInfo,
            )
            .to_raw()
        }
        pub unsafe fn GetCurrentDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nBufferLength = <u32>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::GetCurrentDirectoryA(machine, nBufferLength, lpBuffer).to_raw()
        }
        pub unsafe fn GetCurrentProcess(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentProcess(machine).to_raw()
        }
        pub unsafe fn GetCurrentProcessId(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentProcessId(machine).to_raw()
        }
        pub unsafe fn GetCurrentThread(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentThread(machine).to_raw()
        }
        pub unsafe fn GetCurrentThreadId(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentThreadId(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentStrings(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetEnvironmentStrings(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentStringsW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetEnvironmentStringsW(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentVariableA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let buf = <ArrayWithSize<u8>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::GetEnvironmentVariableA(machine, name, buf).to_raw()
        }
        pub unsafe fn GetEnvironmentVariableW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let buf = <ArrayWithSize<u16>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::GetEnvironmentVariableW(machine, name, buf).to_raw()
        }
        pub unsafe fn GetFileAttributesA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::GetFileAttributesA(machine, lpFileName).to_raw()
        }
        pub unsafe fn GetFileInformationByHandle(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpFileInformation =
                <Option<&mut BY_HANDLE_FILE_INFORMATION>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::GetFileInformationByHandle(machine, hFile, lpFileInformation).to_raw()
        }
        pub unsafe fn GetFileSize(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpFileSizeHigh = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::GetFileSize(machine, hFile, lpFileSizeHigh).to_raw()
        }
        pub unsafe fn GetFileTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpCreationTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 4u32);
            let lpLastAccessTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 8u32);
            let lpLastWriteTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::GetFileTime(
                machine,
                hFile,
                lpCreationTime,
                lpLastAccessTime,
                lpLastWriteTime,
            )
            .to_raw()
        }
        pub unsafe fn GetFileType(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::GetFileType(machine, hFile).to_raw()
        }
        pub unsafe fn GetFullPathNameA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let nBufferLength = <u32>::from_stack(mem, stack_args + 4u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
            let lpFilePart = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::GetFullPathNameA(
                machine,
                lpFileName,
                nBufferLength,
                lpBuffer,
                lpFilePart,
            )
            .to_raw()
        }
        pub unsafe fn GetFullPathNameW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let nBufferLength = <u32>::from_stack(mem, stack_args + 4u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
            let lpFilePart = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::GetFullPathNameW(
                machine,
                lpFileName,
                nBufferLength,
                lpBuffer,
                lpFilePart,
            )
            .to_raw()
        }
        pub unsafe fn GetLastError(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetLastError(machine).to_raw()
        }
        pub unsafe fn GetLocalTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::GetLocalTime(machine, lpSystemTime).to_raw()
        }
        pub unsafe fn GetModuleFileNameA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let filename = <ArrayWithSizeMut<u8>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::GetModuleFileNameA(machine, hModule, filename).to_raw()
        }
        pub unsafe fn GetModuleFileNameW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let _lpFilename = <u32>::from_stack(mem, stack_args + 4u32);
            let _nSize = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize).to_raw()
        }
        pub unsafe fn GetModuleHandleA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpModuleName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::GetModuleHandleA(machine, lpModuleName).to_raw()
        }
        pub unsafe fn GetModuleHandleExW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, stack_args + 0u32);
            let lpModuleName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let hModule = <Option<&mut HMODULE>>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule).to_raw()
        }
        pub unsafe fn GetModuleHandleW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpModuleName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::GetModuleHandleW(machine, lpModuleName).to_raw()
        }
        pub unsafe fn GetOEMCP(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetOEMCP(machine).to_raw()
        }
        pub unsafe fn GetPrivateProfileIntW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let nDefault = <u32>::from_stack(mem, stack_args + 8u32);
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::GetPrivateProfileIntW(
                machine, lpAppName, lpKeyName, nDefault, lpFileName,
            )
            .to_raw()
        }
        pub unsafe fn GetPrivateProfileStringW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpDefault = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let lpReturnedString = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 12u32);
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 20u32);
            winapi::kernel32::GetPrivateProfileStringW(
                machine,
                lpAppName,
                lpKeyName,
                lpDefault,
                lpReturnedString,
                lpFileName,
            )
            .to_raw()
        }
        pub unsafe fn GetProcAddress(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let lpProcName = <GetProcAddressArg>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::GetProcAddress(machine, hModule, lpProcName).to_raw()
        }
        pub unsafe fn GetProcessHeap(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetProcessHeap(machine).to_raw()
        }
        pub unsafe fn GetProfileIntW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let nDefault = <i32>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::GetProfileIntW(machine, lpAppName, lpKeyName, nDefault).to_raw()
        }
        pub unsafe fn GetProfileStringW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpDefault = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let lpReturnedString = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::GetProfileStringW(
                machine,
                lpAppName,
                lpKeyName,
                lpDefault,
                lpReturnedString,
            )
            .to_raw()
        }
        pub unsafe fn GetStartupInfoA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo).to_raw()
        }
        pub unsafe fn GetStartupInfoW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo).to_raw()
        }
        pub unsafe fn GetStdHandle(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nStdHandle = <Result<STD, u32>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::GetStdHandle(machine, nStdHandle).to_raw()
        }
        pub unsafe fn GetStringTypeA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let Locale = <LCID>::from_stack(mem, stack_args + 0u32);
            let dwInfoType = <u32>::from_stack(mem, stack_args + 4u32);
            let lpSrcStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cchSrc = <i32>::from_stack(mem, stack_args + 12u32);
            let lpCharType = <Option<&mut u32>>::from_stack(mem, stack_args + 16u32);
            winapi::kernel32::GetStringTypeA(
                machine, Locale, dwInfoType, lpSrcStr, cchSrc, lpCharType,
            )
            .to_raw()
        }
        pub unsafe fn GetStringTypeW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwInfoType = <u32>::from_stack(mem, stack_args + 0u32);
            let lpSrcStr = <u32>::from_stack(mem, stack_args + 4u32);
            let cchSrc = <i32>::from_stack(mem, stack_args + 8u32);
            let lpCharType = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::GetStringTypeW(machine, dwInfoType, lpSrcStr, cchSrc, lpCharType)
                .to_raw()
        }
        pub unsafe fn GetSystemDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpBuffer = <u32>::from_stack(mem, stack_args + 0u32);
            let uSize = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::GetSystemDirectoryA(machine, lpBuffer, uSize).to_raw()
        }
        pub unsafe fn GetSystemTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::GetSystemTime(machine, lpSystemTime).to_raw()
        }
        pub unsafe fn GetSystemTimeAsFileTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTimeAsFileTime =
                <Option<&mut FILETIME>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::GetSystemTimeAsFileTime(machine, lpSystemTimeAsFileTime).to_raw()
        }
        pub unsafe fn GetTickCount(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetTickCount(machine).to_raw()
        }
        pub unsafe fn GetTimeZoneInformation(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpTimeZoneInformation =
                <Option<&mut TIME_ZONE_INFORMATION>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::GetTimeZoneInformation(machine, lpTimeZoneInformation).to_raw()
        }
        pub unsafe fn GetVersion(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetVersion(machine).to_raw()
        }
        pub unsafe fn GetVersionExA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpVersionInformation =
                <Option<&mut OSVERSIONINFO>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::GetVersionExA(machine, lpVersionInformation).to_raw()
        }
        pub unsafe fn GetWindowsDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpBuffer = <u32>::from_stack(mem, stack_args + 0u32);
            let uSize = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::GetWindowsDirectoryA(machine, lpBuffer, uSize).to_raw()
        }
        pub unsafe fn GlobalAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uFlags = <GMEM>::from_stack(mem, stack_args + 0u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::GlobalAlloc(machine, uFlags, dwBytes).to_raw()
        }
        pub unsafe fn GlobalFlags(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::GlobalFlags(machine, hMem).to_raw()
        }
        pub unsafe fn GlobalFree(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::GlobalFree(machine, hMem).to_raw()
        }
        pub unsafe fn GlobalReAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, stack_args + 0u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 4u32);
            let uFlags = <GMEM>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::GlobalReAlloc(machine, hMem, dwBytes, uFlags).to_raw()
        }
        pub unsafe fn HeapAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <Result<HeapAllocFlags, u32>>::from_stack(mem, stack_args + 4u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes).to_raw()
        }
        pub unsafe fn HeapCreate(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let flOptions = <Result<HeapCreateFlags, u32>>::from_stack(mem, stack_args + 0u32);
            let dwInitialSize = <u32>::from_stack(mem, stack_args + 4u32);
            let dwMaximumSize = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, dwMaximumSize).to_raw()
        }
        pub unsafe fn HeapDestroy(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::HeapDestroy(machine, hHeap).to_raw()
        }
        pub unsafe fn HeapFree(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem).to_raw()
        }
        pub unsafe fn HeapReAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes).to_raw()
        }
        pub unsafe fn HeapSetInformation(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let HeapHandle = <u32>::from_stack(mem, stack_args + 0u32);
            let HeapInformationClass = <u32>::from_stack(mem, stack_args + 4u32);
            let HeapInformation = <u32>::from_stack(mem, stack_args + 8u32);
            let HeapInformationLength = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::HeapSetInformation(
                machine,
                HeapHandle,
                HeapInformationClass,
                HeapInformation,
                HeapInformationLength,
            )
            .to_raw()
        }
        pub unsafe fn HeapSize(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem).to_raw()
        }
        pub unsafe fn HeapValidate(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::HeapValidate(machine, hHeap, dwFlags, lpMem).to_raw()
        }
        pub unsafe fn InitOnceBeginInitialize(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let fPending = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let lpContext = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::InitOnceBeginInitialize(
                machine, lpInitOnce, dwFlags, fPending, lpContext,
            )
            .to_raw()
        }
        pub unsafe fn InitOnceComplete(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpContext = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::InitOnceComplete(machine, lpInitOnce, dwFlags, lpContext).to_raw()
        }
        pub unsafe fn InitializeCriticalSection(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::InitializeCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn InitializeCriticalSectionAndSpinCount(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSpinCount = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::InitializeCriticalSectionAndSpinCount(
                machine,
                lpCriticalSection,
                dwSpinCount,
            )
            .to_raw()
        }
        pub unsafe fn InitializeCriticalSectionEx(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSpinCount = <u32>::from_stack(mem, stack_args + 4u32);
            let flags = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::InitializeCriticalSectionEx(
                machine,
                lpCriticalSection,
                dwSpinCount,
                flags,
            )
            .to_raw()
        }
        pub unsafe fn InitializeSListHead(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let ListHead = <Option<&mut SLIST_HEADER>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::InitializeSListHead(machine, ListHead).to_raw()
        }
        pub unsafe fn InterlockedDecrement(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let addend = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::InterlockedDecrement(machine, addend).to_raw()
        }
        pub unsafe fn InterlockedIncrement(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let addend = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::InterlockedIncrement(machine, addend).to_raw()
        }
        pub unsafe fn IsBadCodePtr(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpfn = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::IsBadCodePtr(machine, lpfn).to_raw()
        }
        pub unsafe fn IsBadReadPtr(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lp = <u32>::from_stack(mem, stack_args + 0u32);
            let ucb = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::IsBadReadPtr(machine, lp, ucb).to_raw()
        }
        pub unsafe fn IsBadWritePtr(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lp = <u32>::from_stack(mem, stack_args + 0u32);
            let ucb = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::IsBadWritePtr(machine, lp, ucb).to_raw()
        }
        pub unsafe fn IsDBCSLeadByte(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _TestChar = <u8>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::IsDBCSLeadByte(machine, _TestChar).to_raw()
        }
        pub unsafe fn IsDBCSLeadByteEx(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _TestChar = <u8>::from_stack(mem, stack_args + 0u32);
            let _CodePage = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::IsDBCSLeadByteEx(machine, _TestChar, _CodePage).to_raw()
        }
        pub unsafe fn IsDebuggerPresent(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::IsDebuggerPresent(machine).to_raw()
        }
        pub unsafe fn IsProcessorFeaturePresent(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let feature = <Result<ProcessorFeature, u32>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::IsProcessorFeaturePresent(machine, feature).to_raw()
        }
        pub unsafe fn IsValidCodePage(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::IsValidCodePage(machine, CodePage).to_raw()
        }
        pub unsafe fn LCMapStringA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let locale = <LCID>::from_stack(mem, stack_args + 0u32);
            let dwMapFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpSrcStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cchSrc = <i32>::from_stack(mem, stack_args + 12u32);
            let lpDestStr = <ArrayWithSizeMut<u8>>::from_stack(mem, stack_args + 16u32);
            winapi::kernel32::LCMapStringA(machine, locale, dwMapFlags, lpSrcStr, cchSrc, lpDestStr)
                .to_raw()
        }
        pub unsafe fn LCMapStringW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let locale = <LCID>::from_stack(mem, stack_args + 0u32);
            let dwMapFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpSrcStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cchSrc = <i32>::from_stack(mem, stack_args + 12u32);
            let lpDestStr = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 16u32);
            winapi::kernel32::LCMapStringW(machine, locale, dwMapFlags, lpSrcStr, cchSrc, lpDestStr)
                .to_raw()
        }
        pub unsafe fn LeaveCriticalSection(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::LeaveCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn LoadLibraryA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let filename = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::LoadLibraryA(machine, filename).to_raw()
        }
        pub unsafe fn LoadLibraryExW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpLibFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let hFile = <HFILE>::from_stack(mem, stack_args + 4u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags).to_raw()
        }
        pub unsafe fn LoadResource(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let hResInfo = <HRSRC>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::LoadResource(machine, hModule, hResInfo).to_raw()
        }
        pub unsafe fn LocalAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uFlags = <GMEM>::from_stack(mem, stack_args + 0u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::LocalAlloc(machine, uFlags, dwBytes).to_raw()
        }
        pub unsafe fn LocalFree(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::LocalFree(machine, hMem).to_raw()
        }
        pub unsafe fn LockResource(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hResData = <HRSRC>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::LockResource(machine, hResData).to_raw()
        }
        pub unsafe fn MulDiv(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nNumber = <i32>::from_stack(mem, stack_args + 0u32);
            let nNumerator = <i32>::from_stack(mem, stack_args + 4u32);
            let nDenominator = <i32>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::MulDiv(machine, nNumber, nNumerator, nDenominator).to_raw()
        }
        pub unsafe fn MultiByteToWideChar(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <Result<CP, u32>>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMultiByteStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cbMultiByte = <i32>::from_stack(mem, stack_args + 12u32);
            let lpWideCharStr = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 16u32);
            winapi::kernel32::MultiByteToWideChar(
                machine,
                CodePage,
                dwFlags,
                lpMultiByteStr,
                cbMultiByte,
                lpWideCharStr,
            )
            .to_raw()
        }
        pub unsafe fn NtCurrentTeb(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::NtCurrentTeb(machine).to_raw()
        }
        pub unsafe fn OutputDebugStringA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let msg = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::OutputDebugStringA(machine, msg).to_raw()
        }
        pub unsafe fn QueryPerformanceCounter(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPerformanceCount =
                <Option<&mut LARGE_INTEGER>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::QueryPerformanceCounter(machine, lpPerformanceCount).to_raw()
        }
        pub unsafe fn QueryPerformanceFrequency(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFrequency = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::QueryPerformanceFrequency(machine, lpFrequency).to_raw()
        }
        pub unsafe fn RaiseException(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwExceptionCode = <u32>::from_stack(mem, stack_args + 0u32);
            let dwExceptionFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let nNumberOfArguments = <u32>::from_stack(mem, stack_args + 8u32);
            let lpArguments = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::RaiseException(
                machine,
                dwExceptionCode,
                dwExceptionFlags,
                nNumberOfArguments,
                lpArguments,
            )
            .to_raw()
        }
        pub unsafe fn ReadFile(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <ArrayWithSizeMut<u8>>::from_stack(mem, stack_args + 4u32);
            let lpNumberOfBytesRead = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpOverlapped = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::kernel32::ReadFile(machine, hFile, lpBuffer, lpNumberOfBytesRead, lpOverlapped)
                .to_raw()
        }
        pub unsafe fn ReleaseSRWLockExclusive(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::ReleaseSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn ReleaseSRWLockShared(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::ReleaseSRWLockShared(machine, SRWLock).to_raw()
        }
        pub unsafe fn RemoveDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::RemoveDirectoryA(machine, lpPathName).to_raw()
        }
        pub unsafe fn ResumeThread(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::ResumeThread(machine, hThread).to_raw()
        }
        pub unsafe fn RtlUnwind(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let TargetFrame = <u32>::from_stack(mem, stack_args + 0u32);
            let TargetIp = <u32>::from_stack(mem, stack_args + 4u32);
            let ExceptionRecord = <u32>::from_stack(mem, stack_args + 8u32);
            let ReturnValue = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::RtlUnwind(
                machine,
                TargetFrame,
                TargetIp,
                ExceptionRecord,
                ReturnValue,
            )
            .to_raw()
        }
        pub unsafe fn SetConsoleCtrlHandler(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _handlerRoutine = <DWORD>::from_stack(mem, stack_args + 0u32);
            let _add = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::SetConsoleCtrlHandler(machine, _handlerRoutine, _add).to_raw()
        }
        pub unsafe fn SetEndOfFile(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::SetEndOfFile(machine, hFile).to_raw()
        }
        pub unsafe fn SetEnvironmentVariableA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let value = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::SetEnvironmentVariableA(machine, name, value).to_raw()
        }
        pub unsafe fn SetEvent(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hEvent = <HEVENT>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::SetEvent(machine, hEvent).to_raw()
        }
        pub unsafe fn SetFileAttributesA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let dwFileAttributes = <Result<FileAttribute, u32>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::SetFileAttributesA(machine, lpFileName, dwFileAttributes).to_raw()
        }
        pub unsafe fn SetFilePointer(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lDistanceToMove = <i32>::from_stack(mem, stack_args + 4u32);
            let lpDistanceToMoveHigh = <Option<&mut i32>>::from_stack(mem, stack_args + 8u32);
            let dwMoveMethod = <Result<FILE, u32>>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::SetFilePointer(
                machine,
                hFile,
                lDistanceToMove,
                lpDistanceToMoveHigh,
                dwMoveMethod,
            )
            .to_raw()
        }
        pub unsafe fn SetFileTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpCreationTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 4u32);
            let lpLastAccessTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 8u32);
            let lpLastWriteTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::SetFileTime(
                machine,
                hFile,
                lpCreationTime,
                lpLastAccessTime,
                lpLastWriteTime,
            )
            .to_raw()
        }
        pub unsafe fn SetHandleCount(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uNumber = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::SetHandleCount(machine, uNumber).to_raw()
        }
        pub unsafe fn SetLastError(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwErrCode = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::SetLastError(machine, dwErrCode).to_raw()
        }
        pub unsafe fn SetPriorityClass(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hProcess = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let dwPriorityClass = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::SetPriorityClass(machine, hProcess, dwPriorityClass).to_raw()
        }
        pub unsafe fn SetStdHandle(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nStdHandle = <Result<STD, u32>>::from_stack(mem, stack_args + 0u32);
            let hHandle = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::SetStdHandle(machine, nStdHandle, hHandle).to_raw()
        }
        pub unsafe fn SetThreadDescription(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
            let lpThreadDescription = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::SetThreadDescription(machine, hThread, lpThreadDescription).to_raw()
        }
        pub unsafe fn SetThreadPriority(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
            let nPriority = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::SetThreadPriority(machine, hThread, nPriority).to_raw()
        }
        pub unsafe fn SetThreadStackGuarantee(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let StackSizeInBytes = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::SetThreadStackGuarantee(machine, StackSizeInBytes).to_raw()
        }
        pub unsafe fn SetUnhandledExceptionFilter(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _lpTopLevelExceptionFilter = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter)
                .to_raw()
        }
        pub unsafe fn SizeofResource(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let hResInfo = <HRSRC>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::SizeofResource(machine, hModule, hResInfo).to_raw()
        }
        pub unsafe fn Sleep(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let dwMilliseconds = <u32>::from_stack(mem, stack_args + 0u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::kernel32::Sleep(machine, dwMilliseconds)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn SystemTimeToFileTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&SYSTEMTIME>>::from_stack(mem, stack_args + 0u32);
            let lpFileTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::SystemTimeToFileTime(machine, lpSystemTime, lpFileTime).to_raw()
        }
        pub unsafe fn TerminateProcess(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hProcess = <u32>::from_stack(mem, stack_args + 0u32);
            let uExitCode = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::TerminateProcess(machine, hProcess, uExitCode).to_raw()
        }
        pub unsafe fn TlsAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::TlsAlloc(machine).to_raw()
        }
        pub unsafe fn TlsFree(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::TlsFree(machine, dwTlsIndex).to_raw()
        }
        pub unsafe fn TlsGetValue(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::TlsGetValue(machine, dwTlsIndex).to_raw()
        }
        pub unsafe fn TlsSetValue(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, stack_args + 0u32);
            let lpTlsValue = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue).to_raw()
        }
        pub unsafe fn TryAcquireSRWLockExclusive(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::TryAcquireSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn UnhandledExceptionFilter(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _exceptionInfo = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo).to_raw()
        }
        pub unsafe fn VirtualAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSize = <u32>::from_stack(mem, stack_args + 4u32);
            let flAllocationType = <Result<MEM, u32>>::from_stack(mem, stack_args + 8u32);
            let flProtec = <Result<PAGE, u32>>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::VirtualAlloc(machine, lpAddress, dwSize, flAllocationType, flProtec)
                .to_raw()
        }
        pub unsafe fn VirtualFree(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSize = <u32>::from_stack(mem, stack_args + 4u32);
            let dwFreeType = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType).to_raw()
        }
        pub unsafe fn VirtualProtect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSize = <u32>::from_stack(mem, stack_args + 4u32);
            let flNewProtect = <u32>::from_stack(mem, stack_args + 8u32);
            let lpflOldProtect = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            winapi::kernel32::VirtualProtect(
                machine,
                lpAddress,
                dwSize,
                flNewProtect,
                lpflOldProtect,
            )
            .to_raw()
        }
        pub unsafe fn VirtualQuery(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
            let lpBuffer =
                <Option<&mut MEMORY_BASIC_INFORMATION>>::from_stack(mem, stack_args + 4u32);
            let dwLength = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::VirtualQuery(machine, lpAddress, lpBuffer, dwLength).to_raw()
        }
        pub unsafe fn WaitForSingleObject(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHandle = <HEVENT>::from_stack(mem, stack_args + 0u32);
            let dwMilliseconds = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::WaitForSingleObject(machine, hHandle, dwMilliseconds).to_raw()
        }
        pub unsafe fn WideCharToMultiByte(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <Result<CP, u32>>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpWideCharStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cchWideChar = <i32>::from_stack(mem, stack_args + 12u32);
            let lpMultiByteStr = <u32>::from_stack(mem, stack_args + 16u32);
            let cbMultiByte = <i32>::from_stack(mem, stack_args + 20u32);
            let lpUsedDefaultChar = <Option<&mut u32>>::from_stack(mem, stack_args + 24u32);
            winapi::kernel32::WideCharToMultiByte(
                machine,
                CodePage,
                dwFlags,
                lpWideCharStr,
                cchWideChar,
                lpMultiByteStr,
                cbMultiByte,
                lpUsedDefaultChar,
            )
            .to_raw()
        }
        pub unsafe fn WriteConsoleA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleOutput = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, stack_args + 4u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpReserved = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::kernel32::WriteConsoleA(
                machine,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                lpReserved,
            )
            .to_raw()
        }
        pub unsafe fn WriteConsoleW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleOutput = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <ArrayWithSize<u16>>::from_stack(mem, stack_args + 4u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let _lpReserved = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::kernel32::WriteConsoleW(
                machine,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                _lpReserved,
            )
            .to_raw()
        }
        pub unsafe fn WriteFile(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, stack_args + 4u32);
            let lpNumberOfBytesWritten = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpOverlapped = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::kernel32::WriteFile(
                machine,
                hFile,
                lpBuffer,
                lpNumberOfBytesWritten,
                lpOverlapped,
            )
            .to_raw()
        }
        pub unsafe fn WriteProfileStringW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            winapi::kernel32::WriteProfileStringW(machine, lpAppName, lpKeyName, lpString).to_raw()
        }
        pub unsafe fn lstrcmpiA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpString2 = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::lstrcmpiA(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrcpyA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <u32>::from_stack(mem, stack_args + 0u32);
            let lpString2 = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::lstrcpyA(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrcpyW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <u32>::from_stack(mem, stack_args + 0u32);
            let lpString2 = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            winapi::kernel32::lstrcpyW(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrlenA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::lstrlenA(machine, lpString).to_raw()
        }
        pub unsafe fn lstrlenW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            winapi::kernel32::lstrlenW(machine, lpString).to_raw()
        }
        pub unsafe fn retrowin32_main(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let entry_point = <u32>::from_stack(mem, stack_args + 0u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::kernel32::retrowin32_main(machine, entry_point)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn retrowin32_thread_main(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let entry_point = <u32>::from_stack(mem, stack_args + 0u32);
            let param = <u32>::from_stack(mem, stack_args + 4u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::kernel32::retrowin32_thread_main(machine, entry_point, param)
                    .await
                    .to_raw()
            })
        }
    }
    const SHIMS: [Shim; 168usize] = [
        Shim {
            name: "AcquireSRWLockExclusive",
            func: Handler::Sync(impls::AcquireSRWLockExclusive),
        },
        Shim {
            name: "AcquireSRWLockShared",
            func: Handler::Sync(impls::AcquireSRWLockShared),
        },
        Shim {
            name: "AddVectoredExceptionHandler",
            func: Handler::Sync(impls::AddVectoredExceptionHandler),
        },
        Shim {
            name: "CloseHandle",
            func: Handler::Sync(impls::CloseHandle),
        },
        Shim {
            name: "CreateDirectoryA",
            func: Handler::Sync(impls::CreateDirectoryA),
        },
        Shim {
            name: "CreateEventA",
            func: Handler::Sync(impls::CreateEventA),
        },
        Shim {
            name: "CreateFileA",
            func: Handler::Sync(impls::CreateFileA),
        },
        Shim {
            name: "CreateFileW",
            func: Handler::Sync(impls::CreateFileW),
        },
        Shim {
            name: "CreateThread",
            func: Handler::Async(impls::CreateThread),
        },
        Shim {
            name: "DebugBreak",
            func: Handler::Sync(impls::DebugBreak),
        },
        Shim {
            name: "DeleteCriticalSection",
            func: Handler::Sync(impls::DeleteCriticalSection),
        },
        Shim {
            name: "DeleteFileA",
            func: Handler::Sync(impls::DeleteFileA),
        },
        Shim {
            name: "DisableThreadLibraryCalls",
            func: Handler::Sync(impls::DisableThreadLibraryCalls),
        },
        Shim {
            name: "EnterCriticalSection",
            func: Handler::Sync(impls::EnterCriticalSection),
        },
        Shim {
            name: "ExitProcess",
            func: Handler::Sync(impls::ExitProcess),
        },
        Shim {
            name: "FileTimeToSystemTime",
            func: Handler::Sync(impls::FileTimeToSystemTime),
        },
        Shim {
            name: "FindClose",
            func: Handler::Sync(impls::FindClose),
        },
        Shim {
            name: "FindFirstFileA",
            func: Handler::Sync(impls::FindFirstFileA),
        },
        Shim {
            name: "FindNextFileA",
            func: Handler::Sync(impls::FindNextFileA),
        },
        Shim {
            name: "FindResourceA",
            func: Handler::Sync(impls::FindResourceA),
        },
        Shim {
            name: "FindResourceW",
            func: Handler::Sync(impls::FindResourceW),
        },
        Shim {
            name: "FlushFileBuffers",
            func: Handler::Sync(impls::FlushFileBuffers),
        },
        Shim {
            name: "FormatMessageA",
            func: Handler::Sync(impls::FormatMessageA),
        },
        Shim {
            name: "FormatMessageW",
            func: Handler::Sync(impls::FormatMessageW),
        },
        Shim {
            name: "FreeEnvironmentStringsA",
            func: Handler::Sync(impls::FreeEnvironmentStringsA),
        },
        Shim {
            name: "FreeEnvironmentStringsW",
            func: Handler::Sync(impls::FreeEnvironmentStringsW),
        },
        Shim {
            name: "FreeLibrary",
            func: Handler::Sync(impls::FreeLibrary),
        },
        Shim {
            name: "GetACP",
            func: Handler::Sync(impls::GetACP),
        },
        Shim {
            name: "GetCPInfo",
            func: Handler::Sync(impls::GetCPInfo),
        },
        Shim {
            name: "GetCommandLineA",
            func: Handler::Sync(impls::GetCommandLineA),
        },
        Shim {
            name: "GetCommandLineW",
            func: Handler::Sync(impls::GetCommandLineW),
        },
        Shim {
            name: "GetConsoleMode",
            func: Handler::Sync(impls::GetConsoleMode),
        },
        Shim {
            name: "GetConsoleScreenBufferInfo",
            func: Handler::Sync(impls::GetConsoleScreenBufferInfo),
        },
        Shim {
            name: "GetCurrentDirectoryA",
            func: Handler::Sync(impls::GetCurrentDirectoryA),
        },
        Shim {
            name: "GetCurrentProcess",
            func: Handler::Sync(impls::GetCurrentProcess),
        },
        Shim {
            name: "GetCurrentProcessId",
            func: Handler::Sync(impls::GetCurrentProcessId),
        },
        Shim {
            name: "GetCurrentThread",
            func: Handler::Sync(impls::GetCurrentThread),
        },
        Shim {
            name: "GetCurrentThreadId",
            func: Handler::Sync(impls::GetCurrentThreadId),
        },
        Shim {
            name: "GetEnvironmentStrings",
            func: Handler::Sync(impls::GetEnvironmentStrings),
        },
        Shim {
            name: "GetEnvironmentStringsW",
            func: Handler::Sync(impls::GetEnvironmentStringsW),
        },
        Shim {
            name: "GetEnvironmentVariableA",
            func: Handler::Sync(impls::GetEnvironmentVariableA),
        },
        Shim {
            name: "GetEnvironmentVariableW",
            func: Handler::Sync(impls::GetEnvironmentVariableW),
        },
        Shim {
            name: "GetFileAttributesA",
            func: Handler::Sync(impls::GetFileAttributesA),
        },
        Shim {
            name: "GetFileInformationByHandle",
            func: Handler::Sync(impls::GetFileInformationByHandle),
        },
        Shim {
            name: "GetFileSize",
            func: Handler::Sync(impls::GetFileSize),
        },
        Shim {
            name: "GetFileTime",
            func: Handler::Sync(impls::GetFileTime),
        },
        Shim {
            name: "GetFileType",
            func: Handler::Sync(impls::GetFileType),
        },
        Shim {
            name: "GetFullPathNameA",
            func: Handler::Sync(impls::GetFullPathNameA),
        },
        Shim {
            name: "GetFullPathNameW",
            func: Handler::Sync(impls::GetFullPathNameW),
        },
        Shim {
            name: "GetLastError",
            func: Handler::Sync(impls::GetLastError),
        },
        Shim {
            name: "GetLocalTime",
            func: Handler::Sync(impls::GetLocalTime),
        },
        Shim {
            name: "GetModuleFileNameA",
            func: Handler::Sync(impls::GetModuleFileNameA),
        },
        Shim {
            name: "GetModuleFileNameW",
            func: Handler::Sync(impls::GetModuleFileNameW),
        },
        Shim {
            name: "GetModuleHandleA",
            func: Handler::Sync(impls::GetModuleHandleA),
        },
        Shim {
            name: "GetModuleHandleExW",
            func: Handler::Sync(impls::GetModuleHandleExW),
        },
        Shim {
            name: "GetModuleHandleW",
            func: Handler::Sync(impls::GetModuleHandleW),
        },
        Shim {
            name: "GetOEMCP",
            func: Handler::Sync(impls::GetOEMCP),
        },
        Shim {
            name: "GetPrivateProfileIntW",
            func: Handler::Sync(impls::GetPrivateProfileIntW),
        },
        Shim {
            name: "GetPrivateProfileStringW",
            func: Handler::Sync(impls::GetPrivateProfileStringW),
        },
        Shim {
            name: "GetProcAddress",
            func: Handler::Sync(impls::GetProcAddress),
        },
        Shim {
            name: "GetProcessHeap",
            func: Handler::Sync(impls::GetProcessHeap),
        },
        Shim {
            name: "GetProfileIntW",
            func: Handler::Sync(impls::GetProfileIntW),
        },
        Shim {
            name: "GetProfileStringW",
            func: Handler::Sync(impls::GetProfileStringW),
        },
        Shim {
            name: "GetStartupInfoA",
            func: Handler::Sync(impls::GetStartupInfoA),
        },
        Shim {
            name: "GetStartupInfoW",
            func: Handler::Sync(impls::GetStartupInfoW),
        },
        Shim {
            name: "GetStdHandle",
            func: Handler::Sync(impls::GetStdHandle),
        },
        Shim {
            name: "GetStringTypeA",
            func: Handler::Sync(impls::GetStringTypeA),
        },
        Shim {
            name: "GetStringTypeW",
            func: Handler::Sync(impls::GetStringTypeW),
        },
        Shim {
            name: "GetSystemDirectoryA",
            func: Handler::Sync(impls::GetSystemDirectoryA),
        },
        Shim {
            name: "GetSystemTime",
            func: Handler::Sync(impls::GetSystemTime),
        },
        Shim {
            name: "GetSystemTimeAsFileTime",
            func: Handler::Sync(impls::GetSystemTimeAsFileTime),
        },
        Shim {
            name: "GetTickCount",
            func: Handler::Sync(impls::GetTickCount),
        },
        Shim {
            name: "GetTimeZoneInformation",
            func: Handler::Sync(impls::GetTimeZoneInformation),
        },
        Shim {
            name: "GetVersion",
            func: Handler::Sync(impls::GetVersion),
        },
        Shim {
            name: "GetVersionExA",
            func: Handler::Sync(impls::GetVersionExA),
        },
        Shim {
            name: "GetWindowsDirectoryA",
            func: Handler::Sync(impls::GetWindowsDirectoryA),
        },
        Shim {
            name: "GlobalAlloc",
            func: Handler::Sync(impls::GlobalAlloc),
        },
        Shim {
            name: "GlobalFlags",
            func: Handler::Sync(impls::GlobalFlags),
        },
        Shim {
            name: "GlobalFree",
            func: Handler::Sync(impls::GlobalFree),
        },
        Shim {
            name: "GlobalReAlloc",
            func: Handler::Sync(impls::GlobalReAlloc),
        },
        Shim {
            name: "HeapAlloc",
            func: Handler::Sync(impls::HeapAlloc),
        },
        Shim {
            name: "HeapCreate",
            func: Handler::Sync(impls::HeapCreate),
        },
        Shim {
            name: "HeapDestroy",
            func: Handler::Sync(impls::HeapDestroy),
        },
        Shim {
            name: "HeapFree",
            func: Handler::Sync(impls::HeapFree),
        },
        Shim {
            name: "HeapReAlloc",
            func: Handler::Sync(impls::HeapReAlloc),
        },
        Shim {
            name: "HeapSetInformation",
            func: Handler::Sync(impls::HeapSetInformation),
        },
        Shim {
            name: "HeapSize",
            func: Handler::Sync(impls::HeapSize),
        },
        Shim {
            name: "HeapValidate",
            func: Handler::Sync(impls::HeapValidate),
        },
        Shim {
            name: "InitOnceBeginInitialize",
            func: Handler::Sync(impls::InitOnceBeginInitialize),
        },
        Shim {
            name: "InitOnceComplete",
            func: Handler::Sync(impls::InitOnceComplete),
        },
        Shim {
            name: "InitializeCriticalSection",
            func: Handler::Sync(impls::InitializeCriticalSection),
        },
        Shim {
            name: "InitializeCriticalSectionAndSpinCount",
            func: Handler::Sync(impls::InitializeCriticalSectionAndSpinCount),
        },
        Shim {
            name: "InitializeCriticalSectionEx",
            func: Handler::Sync(impls::InitializeCriticalSectionEx),
        },
        Shim {
            name: "InitializeSListHead",
            func: Handler::Sync(impls::InitializeSListHead),
        },
        Shim {
            name: "InterlockedDecrement",
            func: Handler::Sync(impls::InterlockedDecrement),
        },
        Shim {
            name: "InterlockedIncrement",
            func: Handler::Sync(impls::InterlockedIncrement),
        },
        Shim {
            name: "IsBadCodePtr",
            func: Handler::Sync(impls::IsBadCodePtr),
        },
        Shim {
            name: "IsBadReadPtr",
            func: Handler::Sync(impls::IsBadReadPtr),
        },
        Shim {
            name: "IsBadWritePtr",
            func: Handler::Sync(impls::IsBadWritePtr),
        },
        Shim {
            name: "IsDBCSLeadByte",
            func: Handler::Sync(impls::IsDBCSLeadByte),
        },
        Shim {
            name: "IsDBCSLeadByteEx",
            func: Handler::Sync(impls::IsDBCSLeadByteEx),
        },
        Shim {
            name: "IsDebuggerPresent",
            func: Handler::Sync(impls::IsDebuggerPresent),
        },
        Shim {
            name: "IsProcessorFeaturePresent",
            func: Handler::Sync(impls::IsProcessorFeaturePresent),
        },
        Shim {
            name: "IsValidCodePage",
            func: Handler::Sync(impls::IsValidCodePage),
        },
        Shim {
            name: "LCMapStringA",
            func: Handler::Sync(impls::LCMapStringA),
        },
        Shim {
            name: "LCMapStringW",
            func: Handler::Sync(impls::LCMapStringW),
        },
        Shim {
            name: "LeaveCriticalSection",
            func: Handler::Sync(impls::LeaveCriticalSection),
        },
        Shim {
            name: "LoadLibraryA",
            func: Handler::Sync(impls::LoadLibraryA),
        },
        Shim {
            name: "LoadLibraryExW",
            func: Handler::Sync(impls::LoadLibraryExW),
        },
        Shim {
            name: "LoadResource",
            func: Handler::Sync(impls::LoadResource),
        },
        Shim {
            name: "LocalAlloc",
            func: Handler::Sync(impls::LocalAlloc),
        },
        Shim {
            name: "LocalFree",
            func: Handler::Sync(impls::LocalFree),
        },
        Shim {
            name: "LockResource",
            func: Handler::Sync(impls::LockResource),
        },
        Shim {
            name: "MulDiv",
            func: Handler::Sync(impls::MulDiv),
        },
        Shim {
            name: "MultiByteToWideChar",
            func: Handler::Sync(impls::MultiByteToWideChar),
        },
        Shim {
            name: "NtCurrentTeb",
            func: Handler::Sync(impls::NtCurrentTeb),
        },
        Shim {
            name: "OutputDebugStringA",
            func: Handler::Sync(impls::OutputDebugStringA),
        },
        Shim {
            name: "QueryPerformanceCounter",
            func: Handler::Sync(impls::QueryPerformanceCounter),
        },
        Shim {
            name: "QueryPerformanceFrequency",
            func: Handler::Sync(impls::QueryPerformanceFrequency),
        },
        Shim {
            name: "RaiseException",
            func: Handler::Sync(impls::RaiseException),
        },
        Shim {
            name: "ReadFile",
            func: Handler::Sync(impls::ReadFile),
        },
        Shim {
            name: "ReleaseSRWLockExclusive",
            func: Handler::Sync(impls::ReleaseSRWLockExclusive),
        },
        Shim {
            name: "ReleaseSRWLockShared",
            func: Handler::Sync(impls::ReleaseSRWLockShared),
        },
        Shim {
            name: "RemoveDirectoryA",
            func: Handler::Sync(impls::RemoveDirectoryA),
        },
        Shim {
            name: "ResumeThread",
            func: Handler::Sync(impls::ResumeThread),
        },
        Shim {
            name: "RtlUnwind",
            func: Handler::Sync(impls::RtlUnwind),
        },
        Shim {
            name: "SetConsoleCtrlHandler",
            func: Handler::Sync(impls::SetConsoleCtrlHandler),
        },
        Shim {
            name: "SetEndOfFile",
            func: Handler::Sync(impls::SetEndOfFile),
        },
        Shim {
            name: "SetEnvironmentVariableA",
            func: Handler::Sync(impls::SetEnvironmentVariableA),
        },
        Shim {
            name: "SetEvent",
            func: Handler::Sync(impls::SetEvent),
        },
        Shim {
            name: "SetFileAttributesA",
            func: Handler::Sync(impls::SetFileAttributesA),
        },
        Shim {
            name: "SetFilePointer",
            func: Handler::Sync(impls::SetFilePointer),
        },
        Shim {
            name: "SetFileTime",
            func: Handler::Sync(impls::SetFileTime),
        },
        Shim {
            name: "SetHandleCount",
            func: Handler::Sync(impls::SetHandleCount),
        },
        Shim {
            name: "SetLastError",
            func: Handler::Sync(impls::SetLastError),
        },
        Shim {
            name: "SetPriorityClass",
            func: Handler::Sync(impls::SetPriorityClass),
        },
        Shim {
            name: "SetStdHandle",
            func: Handler::Sync(impls::SetStdHandle),
        },
        Shim {
            name: "SetThreadDescription",
            func: Handler::Sync(impls::SetThreadDescription),
        },
        Shim {
            name: "SetThreadPriority",
            func: Handler::Sync(impls::SetThreadPriority),
        },
        Shim {
            name: "SetThreadStackGuarantee",
            func: Handler::Sync(impls::SetThreadStackGuarantee),
        },
        Shim {
            name: "SetUnhandledExceptionFilter",
            func: Handler::Sync(impls::SetUnhandledExceptionFilter),
        },
        Shim {
            name: "SizeofResource",
            func: Handler::Sync(impls::SizeofResource),
        },
        Shim {
            name: "Sleep",
            func: Handler::Async(impls::Sleep),
        },
        Shim {
            name: "SystemTimeToFileTime",
            func: Handler::Sync(impls::SystemTimeToFileTime),
        },
        Shim {
            name: "TerminateProcess",
            func: Handler::Sync(impls::TerminateProcess),
        },
        Shim {
            name: "TlsAlloc",
            func: Handler::Sync(impls::TlsAlloc),
        },
        Shim {
            name: "TlsFree",
            func: Handler::Sync(impls::TlsFree),
        },
        Shim {
            name: "TlsGetValue",
            func: Handler::Sync(impls::TlsGetValue),
        },
        Shim {
            name: "TlsSetValue",
            func: Handler::Sync(impls::TlsSetValue),
        },
        Shim {
            name: "TryAcquireSRWLockExclusive",
            func: Handler::Sync(impls::TryAcquireSRWLockExclusive),
        },
        Shim {
            name: "UnhandledExceptionFilter",
            func: Handler::Sync(impls::UnhandledExceptionFilter),
        },
        Shim {
            name: "VirtualAlloc",
            func: Handler::Sync(impls::VirtualAlloc),
        },
        Shim {
            name: "VirtualFree",
            func: Handler::Sync(impls::VirtualFree),
        },
        Shim {
            name: "VirtualProtect",
            func: Handler::Sync(impls::VirtualProtect),
        },
        Shim {
            name: "VirtualQuery",
            func: Handler::Sync(impls::VirtualQuery),
        },
        Shim {
            name: "WaitForSingleObject",
            func: Handler::Sync(impls::WaitForSingleObject),
        },
        Shim {
            name: "WideCharToMultiByte",
            func: Handler::Sync(impls::WideCharToMultiByte),
        },
        Shim {
            name: "WriteConsoleA",
            func: Handler::Sync(impls::WriteConsoleA),
        },
        Shim {
            name: "WriteConsoleW",
            func: Handler::Sync(impls::WriteConsoleW),
        },
        Shim {
            name: "WriteFile",
            func: Handler::Sync(impls::WriteFile),
        },
        Shim {
            name: "WriteProfileStringW",
            func: Handler::Sync(impls::WriteProfileStringW),
        },
        Shim {
            name: "lstrcmpiA",
            func: Handler::Sync(impls::lstrcmpiA),
        },
        Shim {
            name: "lstrcpyA",
            func: Handler::Sync(impls::lstrcpyA),
        },
        Shim {
            name: "lstrcpyW",
            func: Handler::Sync(impls::lstrcpyW),
        },
        Shim {
            name: "lstrlenA",
            func: Handler::Sync(impls::lstrlenA),
        },
        Shim {
            name: "lstrlenW",
            func: Handler::Sync(impls::lstrlenW),
        },
        Shim {
            name: "retrowin32_main",
            func: Handler::Async(impls::retrowin32_main),
        },
        Shim {
            name: "retrowin32_thread_main",
            func: Handler::Async(impls::retrowin32_thread_main),
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
    mod impls {
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
            winapi::ntdll::NtReadFile(
                machine,
                FileHandle,
                Event,
                ApcRoutine,
                ApcContext,
                IoStatusBlock,
                Buffer,
                ByteOffset,
                Key,
            )
            .to_raw()
        }
        pub unsafe fn RtlExitUserProcess(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let exit_code = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ntdll::RtlExitUserProcess(machine, exit_code).to_raw()
        }
    }
    const SHIMS: [Shim; 2usize] = [
        Shim {
            name: "NtReadFile",
            func: Handler::Sync(impls::NtReadFile),
        },
        Shim {
            name: "RtlExitUserProcess",
            func: Handler::Sync(impls::RtlExitUserProcess),
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
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ole32::*;
        pub unsafe fn OleInitialize(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _pvReserved = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ole32::OleInitialize(machine, _pvReserved).to_raw()
        }
    }
    const SHIMS: [Shim; 1usize] = [Shim {
        name: "OleInitialize",
        func: Handler::Sync(impls::OleInitialize),
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ole32.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/ole32.dll"),
    };
}
pub mod oleaut32 {
    use super::*;
    mod impls {
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
    mod impls {
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
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::retrowin32_test::retrowin32_test_callback1(machine, func, data)
                    .await
                    .to_raw()
            })
        }
    }
    const SHIMS: [Shim; 1usize] = [Shim {
        name: "retrowin32_test_callback1",
        func: Handler::Async(impls::retrowin32_test_callback1),
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "retrowin32_test.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/retrowin32_test.dll"),
    };
}
pub mod ucrtbase {
    use super::*;
    mod impls {
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
            winapi::ucrtbase::_XcptFilter(machine, xcptnum, pxcptinfoptrs).to_raw()
        }
        pub unsafe fn __dllonexit(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let func = <u32>::from_stack(mem, stack_args + 0u32);
            let d = <u32>::from_stack(mem, stack_args + 4u32);
            let f = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::ucrtbase::__dllonexit(machine, func, d, f).to_raw()
        }
        pub unsafe fn __getmainargs(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let argc = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
            let argv = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let env = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let doWildCard = <u32>::from_stack(mem, stack_args + 12u32);
            let startInfo = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::ucrtbase::__getmainargs(machine, argc, argv, env, doWildCard, startInfo)
                .to_raw()
        }
        pub unsafe fn __p___argc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p___argc(machine).to_raw()
        }
        pub unsafe fn __p___argv(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p___argv(machine).to_raw()
        }
        pub unsafe fn __p__commode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p__commode(machine).to_raw()
        }
        pub unsafe fn __p__fmode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p__fmode(machine).to_raw()
        }
        pub unsafe fn __set_app_type(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _app_type = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::__set_app_type(machine, _app_type).to_raw()
        }
        pub unsafe fn __setusermatherr(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let pf = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::__setusermatherr(machine, pf).to_raw()
        }
        pub unsafe fn _configthreadlocale(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let per_thread_locale_type = <i32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::_configthreadlocale(machine, per_thread_locale_type).to_raw()
        }
        pub unsafe fn _configure_narrow_argv(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _mode = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::_configure_narrow_argv(machine, _mode).to_raw()
        }
        pub unsafe fn _controlfp(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _new = <u32>::from_stack(mem, stack_args + 0u32);
            let _mask = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::ucrtbase::_controlfp(machine, _new, _mask).to_raw()
        }
        pub unsafe fn _controlfp_s(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _currentControl = <u32>::from_stack(mem, stack_args + 0u32);
            let _newControl = <u32>::from_stack(mem, stack_args + 4u32);
            let _mask = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::ucrtbase::_controlfp_s(machine, _currentControl, _newControl, _mask).to_raw()
        }
        pub unsafe fn _crt_atexit(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _function = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::_crt_atexit(machine, _function).to_raw()
        }
        pub unsafe fn _except_handler3(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let exception_record = <u32>::from_stack(mem, stack_args + 0u32);
            let registration = <u32>::from_stack(mem, stack_args + 4u32);
            let context = <u32>::from_stack(mem, stack_args + 8u32);
            let dispatcher = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::ucrtbase::_except_handler3(
                machine,
                exception_record,
                registration,
                context,
                dispatcher,
            )
            .to_raw()
        }
        pub unsafe fn _exit(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let status = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::_exit(machine, status).to_raw()
        }
        pub unsafe fn _get_initial_narrow_environment(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::_get_initial_narrow_environment(machine).to_raw()
        }
        pub unsafe fn _initialize_narrow_environment(
            machine: &mut Machine,
            stack_args: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::_initialize_narrow_environment(machine).to_raw()
        }
        pub unsafe fn _initterm(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let start = <u32>::from_stack(mem, stack_args + 0u32);
            let end = <u32>::from_stack(mem, stack_args + 4u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::ucrtbase::_initterm(machine, start, end)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn _initterm_e(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let start = <u32>::from_stack(mem, stack_args + 0u32);
            let end = <u32>::from_stack(mem, stack_args + 4u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::ucrtbase::_initterm_e(machine, start, end)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn _lock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let locknum = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::_lock(machine, locknum).to_raw()
        }
        pub unsafe fn _set_app_type(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _app_type = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::_set_app_type(machine, _app_type).to_raw()
        }
        pub unsafe fn _set_fmode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let _mode = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::_set_fmode(machine, _mode).to_raw()
        }
        pub unsafe fn _set_new_mode(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let newhandlermode = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::_set_new_mode(machine, newhandlermode).to_raw()
        }
        pub unsafe fn _time64(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let destTime = <Option<&mut u64>>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::_time64(machine, destTime).to_raw()
        }
        pub unsafe fn _unlock(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let locknum = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::_unlock(machine, locknum).to_raw()
        }
        pub unsafe fn exit(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let status = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::exit(machine, status).to_raw()
        }
        pub unsafe fn free(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let ptr = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::free(machine, ptr).to_raw()
        }
        pub unsafe fn malloc(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let size = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::malloc(machine, size).to_raw()
        }
        pub unsafe fn rand(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::rand(machine).to_raw()
        }
        pub unsafe fn srand(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let seed = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::srand(machine, seed).to_raw()
        }
        pub unsafe fn time(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let destTime = <Option<&mut u64>>::from_stack(mem, stack_args + 0u32);
            winapi::ucrtbase::time(machine, destTime).to_raw()
        }
    }
    const SHIMS: [Shim; 32usize] = [
        Shim {
            name: "_XcptFilter",
            func: Handler::Sync(impls::_XcptFilter),
        },
        Shim {
            name: "__dllonexit",
            func: Handler::Sync(impls::__dllonexit),
        },
        Shim {
            name: "__getmainargs",
            func: Handler::Sync(impls::__getmainargs),
        },
        Shim {
            name: "__p___argc",
            func: Handler::Sync(impls::__p___argc),
        },
        Shim {
            name: "__p___argv",
            func: Handler::Sync(impls::__p___argv),
        },
        Shim {
            name: "__p__commode",
            func: Handler::Sync(impls::__p__commode),
        },
        Shim {
            name: "__p__fmode",
            func: Handler::Sync(impls::__p__fmode),
        },
        Shim {
            name: "__set_app_type",
            func: Handler::Sync(impls::__set_app_type),
        },
        Shim {
            name: "__setusermatherr",
            func: Handler::Sync(impls::__setusermatherr),
        },
        Shim {
            name: "_configthreadlocale",
            func: Handler::Sync(impls::_configthreadlocale),
        },
        Shim {
            name: "_configure_narrow_argv",
            func: Handler::Sync(impls::_configure_narrow_argv),
        },
        Shim {
            name: "_controlfp",
            func: Handler::Sync(impls::_controlfp),
        },
        Shim {
            name: "_controlfp_s",
            func: Handler::Sync(impls::_controlfp_s),
        },
        Shim {
            name: "_crt_atexit",
            func: Handler::Sync(impls::_crt_atexit),
        },
        Shim {
            name: "_except_handler3",
            func: Handler::Sync(impls::_except_handler3),
        },
        Shim {
            name: "_exit",
            func: Handler::Sync(impls::_exit),
        },
        Shim {
            name: "_get_initial_narrow_environment",
            func: Handler::Sync(impls::_get_initial_narrow_environment),
        },
        Shim {
            name: "_initialize_narrow_environment",
            func: Handler::Sync(impls::_initialize_narrow_environment),
        },
        Shim {
            name: "_initterm",
            func: Handler::Async(impls::_initterm),
        },
        Shim {
            name: "_initterm_e",
            func: Handler::Async(impls::_initterm_e),
        },
        Shim {
            name: "_lock",
            func: Handler::Sync(impls::_lock),
        },
        Shim {
            name: "_set_app_type",
            func: Handler::Sync(impls::_set_app_type),
        },
        Shim {
            name: "_set_fmode",
            func: Handler::Sync(impls::_set_fmode),
        },
        Shim {
            name: "_set_new_mode",
            func: Handler::Sync(impls::_set_new_mode),
        },
        Shim {
            name: "_time64",
            func: Handler::Sync(impls::_time64),
        },
        Shim {
            name: "_unlock",
            func: Handler::Sync(impls::_unlock),
        },
        Shim {
            name: "exit",
            func: Handler::Sync(impls::exit),
        },
        Shim {
            name: "free",
            func: Handler::Sync(impls::free),
        },
        Shim {
            name: "malloc",
            func: Handler::Sync(impls::malloc),
        },
        Shim {
            name: "rand",
            func: Handler::Sync(impls::rand),
        },
        Shim {
            name: "srand",
            func: Handler::Sync(impls::srand),
        },
        Shim {
            name: "time",
            func: Handler::Sync(impls::time),
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
    mod impls {
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
            winapi::vcruntime140::_CxxThrowException(machine, pExceptionObject, pThrowInfo).to_raw()
        }
        pub unsafe fn memcmp(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lhs = <u32>::from_stack(mem, stack_args + 0u32);
            let rhs = <u32>::from_stack(mem, stack_args + 4u32);
            let len = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::vcruntime140::memcmp(machine, lhs, rhs, len).to_raw()
        }
        pub unsafe fn memcpy(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dst = <u32>::from_stack(mem, stack_args + 0u32);
            let src = <u32>::from_stack(mem, stack_args + 4u32);
            let len = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::vcruntime140::memcpy(machine, dst, src, len).to_raw()
        }
        pub unsafe fn memset(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let dst = <u32>::from_stack(mem, stack_args + 0u32);
            let val = <u32>::from_stack(mem, stack_args + 4u32);
            let len = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::vcruntime140::memset(machine, dst, val, len).to_raw()
        }
    }
    const SHIMS: [Shim; 4usize] = [
        Shim {
            name: "_CxxThrowException",
            func: Handler::Sync(impls::_CxxThrowException),
        },
        Shim {
            name: "memcmp",
            func: Handler::Sync(impls::memcmp),
        },
        Shim {
            name: "memcpy",
            func: Handler::Sync(impls::memcpy),
        },
        Shim {
            name: "memset",
            func: Handler::Sync(impls::memset),
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
    mod impls {
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
            winapi::version::GetFileVersionInfoSizeA(machine, lptstrFilename, lpdwHandle).to_raw()
        }
    }
    const SHIMS: [Shim; 1usize] = [Shim {
        name: "GetFileVersionInfoSizeA",
        func: Handler::Sync(impls::GetFileVersionInfoSizeA),
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "version.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/version.dll"),
    };
}
pub mod user32 {
    use super::*;
    mod impls {
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
            winapi::user32::AdjustWindowRect(machine, lpRect, dwStyle, bMenu).to_raw()
        }
        pub unsafe fn AdjustWindowRectEx(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, stack_args + 4u32);
            let bMenu = <bool>::from_stack(mem, stack_args + 8u32);
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, stack_args + 12u32);
            winapi::user32::AdjustWindowRectEx(machine, lpRect, dwStyle, bMenu, dwExStyle).to_raw()
        }
        pub unsafe fn AppendMenuA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let uFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let uIDNewItem = <u32>::from_stack(mem, stack_args + 8u32);
            let lpNewItem = <Option<&str>>::from_stack(mem, stack_args + 12u32);
            winapi::user32::AppendMenuA(machine, hMenu, uFlags, uIDNewItem, lpNewItem).to_raw()
        }
        pub unsafe fn BeginPaint(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpPaint = <Option<&mut PAINTSTRUCT>>::from_stack(mem, stack_args + 4u32);
            winapi::user32::BeginPaint(machine, hWnd, lpPaint).to_raw()
        }
        pub unsafe fn CheckDlgButton(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDButton = <i32>::from_stack(mem, stack_args + 4u32);
            let uCheck = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::user32::CheckDlgButton(machine, hDlg, nIDButton, uCheck).to_raw()
        }
        pub unsafe fn CheckMenuItem(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let uIDCheckItem = <u32>::from_stack(mem, stack_args + 4u32);
            let uCheck = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::user32::CheckMenuItem(machine, hMenu, uIDCheckItem, uCheck).to_raw()
        }
        pub unsafe fn CheckRadioButton(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDFirstButton = <i32>::from_stack(mem, stack_args + 4u32);
            let nIDLastButton = <i32>::from_stack(mem, stack_args + 8u32);
            let nIDCheckButton = <i32>::from_stack(mem, stack_args + 12u32);
            winapi::user32::CheckRadioButton(
                machine,
                hDlg,
                nIDFirstButton,
                nIDLastButton,
                nIDCheckButton,
            )
            .to_raw()
        }
        pub unsafe fn ClientToScreen(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpPoint = <Option<&mut POINT>>::from_stack(mem, stack_args + 4u32);
            winapi::user32::ClientToScreen(machine, hWnd, lpPoint).to_raw()
        }
        pub unsafe fn CopyRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprcDst = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let lprcSrc = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            winapi::user32::CopyRect(machine, lprcDst, lprcSrc).to_raw()
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
            winapi::user32::CreateCursor(
                machine, hInst, xHotSpot, yHotSpot, nWidth, nHeight, pvANDPlane, pvXORPlane,
            )
            .to_raw()
        }
        pub unsafe fn CreatePopupMenu(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::CreatePopupMenu(machine).to_raw()
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
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::CreateWindowExA(
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
                .await
                .to_raw()
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
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::CreateWindowExW(
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
                .await
                .to_raw()
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
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::DefWindowProcA(machine, hWnd, msg, wParam, lParam)
                    .await
                    .to_raw()
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
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::DefWindowProcW(machine, hWnd, msg, wParam, lParam)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn DestroyWindow(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            winapi::user32::DestroyWindow(machine, hWnd).to_raw()
        }
        pub unsafe fn DialogBoxIndirectParamA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let hDialogTemplate = <u32>::from_stack(mem, stack_args + 4u32);
            let hWndParent = <HWND>::from_stack(mem, stack_args + 8u32);
            let lpDialogFunc = <u32>::from_stack(mem, stack_args + 12u32);
            let dwInitParam = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::user32::DialogBoxIndirectParamA(
                machine,
                hInstance,
                hDialogTemplate,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            )
            .to_raw()
        }
        pub unsafe fn DialogBoxParamA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpTemplateName = <u32>::from_stack(mem, stack_args + 4u32);
            let hWndParent = <HWND>::from_stack(mem, stack_args + 8u32);
            let lpDialogFunc = <u32>::from_stack(mem, stack_args + 12u32);
            let dwInitParam = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::user32::DialogBoxParamA(
                machine,
                hInstance,
                lpTemplateName,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            )
            .to_raw()
        }
        pub unsafe fn DialogBoxParamW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpTemplateName = <u32>::from_stack(mem, stack_args + 4u32);
            let hWndParent = <HWND>::from_stack(mem, stack_args + 8u32);
            let lpDialogFunc = <u32>::from_stack(mem, stack_args + 12u32);
            let dwInitParam = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::user32::DialogBoxParamW(
                machine,
                hInstance,
                lpTemplateName,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            )
            .to_raw()
        }
        pub unsafe fn DispatchMessageA(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 0u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::DispatchMessageA(machine, lpMsg)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn DispatchMessageW(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 0u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::DispatchMessageW(machine, lpMsg)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn DrawTextW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let nCount = <i32>::from_stack(mem, stack_args + 8u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 12u32);
            let uFormat = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::user32::DrawTextW(machine, hDC, lpString, nCount, lpRect, uFormat).to_raw()
        }
        pub unsafe fn EnableMenuItem(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let uIDEnableItem = <u32>::from_stack(mem, stack_args + 4u32);
            let uEnable = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::user32::EnableMenuItem(machine, hMenu, uIDEnableItem, uEnable).to_raw()
        }
        pub unsafe fn EnableWindow(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let bEnable = <bool>::from_stack(mem, stack_args + 4u32);
            winapi::user32::EnableWindow(machine, hWnd, bEnable).to_raw()
        }
        pub unsafe fn EndDialog(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nResult = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            winapi::user32::EndDialog(machine, hDlg, nResult).to_raw()
        }
        pub unsafe fn EndPaint(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpPaint = <Option<&PAINTSTRUCT>>::from_stack(mem, stack_args + 4u32);
            winapi::user32::EndPaint(machine, hWnd, lpPaint).to_raw()
        }
        pub unsafe fn FillRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
            let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let hbr = <BrushOrColor>::from_stack(mem, stack_args + 8u32);
            winapi::user32::FillRect(machine, hDC, lprc, hbr).to_raw()
        }
        pub unsafe fn FindWindowA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpClassName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpWindowName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            winapi::user32::FindWindowA(machine, lpClassName, lpWindowName).to_raw()
        }
        pub unsafe fn FrameRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
            let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let hbr = <HBRUSH>::from_stack(mem, stack_args + 8u32);
            winapi::user32::FrameRect(machine, hDC, lprc, hbr).to_raw()
        }
        pub unsafe fn GetActiveWindow(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetActiveWindow(machine).to_raw()
        }
        pub unsafe fn GetCapture(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetCapture(machine).to_raw()
        }
        pub unsafe fn GetClientRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
            winapi::user32::GetClientRect(machine, hWnd, lpRect).to_raw()
        }
        pub unsafe fn GetDC(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            winapi::user32::GetDC(machine, hWnd).to_raw()
        }
        pub unsafe fn GetDesktopWindow(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetDesktopWindow(machine).to_raw()
        }
        pub unsafe fn GetDlgItem(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            winapi::user32::GetDlgItem(machine, hDlg, nIDDlgItem).to_raw()
        }
        pub unsafe fn GetDlgItemInt(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let lpTranslated = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let bSigned = <bool>::from_stack(mem, stack_args + 12u32);
            winapi::user32::GetDlgItemInt(machine, hDlg, nIDDlgItem, lpTranslated, bSigned).to_raw()
        }
        pub unsafe fn GetDlgItemTextW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let lpString = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 8u32);
            winapi::user32::GetDlgItemTextW(machine, hDlg, nIDDlgItem, lpString).to_raw()
        }
        pub unsafe fn GetFocus(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetFocus(machine).to_raw()
        }
        pub unsafe fn GetForegroundWindow(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetForegroundWindow(machine).to_raw()
        }
        pub unsafe fn GetKeyState(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nVirtKey = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::user32::GetKeyState(machine, nVirtKey).to_raw()
        }
        pub unsafe fn GetLastActivePopup(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetLastActivePopup(machine).to_raw()
        }
        pub unsafe fn GetMenu(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            winapi::user32::GetMenu(machine, hWnd).to_raw()
        }
        pub unsafe fn GetMenuItemRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hMenu = <HMENU>::from_stack(mem, stack_args + 4u32);
            let uItem = <u32>::from_stack(mem, stack_args + 8u32);
            let lprcItem = <Option<&mut RECT>>::from_stack(mem, stack_args + 12u32);
            winapi::user32::GetMenuItemRect(machine, hWnd, hMenu, uItem, lprcItem).to_raw()
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
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::GetMessageA(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax)
                    .await
                    .to_raw()
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
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::GetMessageW(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn GetSubMenu(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let nPos = <i32>::from_stack(mem, stack_args + 4u32);
            winapi::user32::GetSubMenu(machine, hMenu, nPos).to_raw()
        }
        pub unsafe fn GetSysColor(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nIndex = <i32>::from_stack(mem, stack_args + 0u32);
            winapi::user32::GetSysColor(machine, nIndex).to_raw()
        }
        pub unsafe fn GetSystemMenu(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let bRevert = <bool>::from_stack(mem, stack_args + 4u32);
            winapi::user32::GetSystemMenu(machine, hWnd, bRevert).to_raw()
        }
        pub unsafe fn GetSystemMetrics(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nIndex = <Result<SystemMetric, u32>>::from_stack(mem, stack_args + 0u32);
            winapi::user32::GetSystemMetrics(machine, nIndex).to_raw()
        }
        pub unsafe fn GetWindowDC(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            winapi::user32::GetWindowDC(machine, hWnd).to_raw()
        }
        pub unsafe fn GetWindowLongA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIndex = <i32>::from_stack(mem, stack_args + 4u32);
            winapi::user32::GetWindowLongA(machine, hWnd, nIndex).to_raw()
        }
        pub unsafe fn GetWindowPlacement(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpwndpl = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            winapi::user32::GetWindowPlacement(machine, hWnd, lpwndpl).to_raw()
        }
        pub unsafe fn GetWindowRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
            winapi::user32::GetWindowRect(machine, hWnd, lpRect).to_raw()
        }
        pub unsafe fn InflateRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let dx = <i32>::from_stack(mem, stack_args + 4u32);
            let dy = <i32>::from_stack(mem, stack_args + 8u32);
            winapi::user32::InflateRect(machine, lprc, dx, dy).to_raw()
        }
        pub unsafe fn IntersectRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprcDst = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let lprcSrc1 = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let lprcSrc2 = <Option<&RECT>>::from_stack(mem, stack_args + 8u32);
            winapi::user32::IntersectRect(machine, lprcDst, lprcSrc1, lprcSrc2).to_raw()
        }
        pub unsafe fn InvalidateRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let bErase = <bool>::from_stack(mem, stack_args + 8u32);
            winapi::user32::InvalidateRect(machine, hWnd, lpRect, bErase).to_raw()
        }
        pub unsafe fn InvalidateRgn(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hRgn = <HRGN>::from_stack(mem, stack_args + 4u32);
            let bErase = <bool>::from_stack(mem, stack_args + 8u32);
            winapi::user32::InvalidateRgn(machine, hWnd, hRgn, bErase).to_raw()
        }
        pub unsafe fn InvertRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpr = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            winapi::user32::InvertRect(machine, hDC, lpr).to_raw()
        }
        pub unsafe fn IsDlgButtonChecked(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDButton = <i32>::from_stack(mem, stack_args + 4u32);
            winapi::user32::IsDlgButtonChecked(machine, hDlg, nIDButton).to_raw()
        }
        pub unsafe fn IsIconic(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, stack_args + 0u32);
            winapi::user32::IsIconic(machine, hwnd).to_raw()
        }
        pub unsafe fn IsRectEmpty(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 0u32);
            winapi::user32::IsRectEmpty(machine, lprc).to_raw()
        }
        pub unsafe fn KillTimer(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let uIDEvent = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::user32::KillTimer(machine, hWnd, uIDEvent).to_raw()
        }
        pub unsafe fn LoadAcceleratorsW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpTableName = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::user32::LoadAcceleratorsW(machine, hInstance, lpTableName).to_raw()
        }
        pub unsafe fn LoadBitmapA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <HINSTANCE>::from_stack(mem, stack_args + 0u32);
            let lpBitmapName = <ResourceKey<&str>>::from_stack(mem, stack_args + 4u32);
            winapi::user32::LoadBitmapA(machine, hInstance, lpBitmapName).to_raw()
        }
        pub unsafe fn LoadCursorA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpCursorName = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::user32::LoadCursorA(machine, hInstance, lpCursorName).to_raw()
        }
        pub unsafe fn LoadCursorW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpCursorName = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::user32::LoadCursorW(machine, hInstance, lpCursorName).to_raw()
        }
        pub unsafe fn LoadIconA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpIconName = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::user32::LoadIconA(machine, hInstance, lpIconName).to_raw()
        }
        pub unsafe fn LoadIconW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpIconName = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::user32::LoadIconW(machine, hInstance, lpIconName).to_raw()
        }
        pub unsafe fn LoadImageA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let name = <ResourceKey<&str>>::from_stack(mem, stack_args + 4u32);
            let typ = <u32>::from_stack(mem, stack_args + 8u32);
            let cx = <u32>::from_stack(mem, stack_args + 12u32);
            let cy = <u32>::from_stack(mem, stack_args + 16u32);
            let fuLoad = <u32>::from_stack(mem, stack_args + 20u32);
            winapi::user32::LoadImageA(machine, hInstance, name, typ, cx, cy, fuLoad).to_raw()
        }
        pub unsafe fn LoadImageW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let name = <ResourceKey<&Str16>>::from_stack(mem, stack_args + 4u32);
            let typ = <u32>::from_stack(mem, stack_args + 8u32);
            let cx = <u32>::from_stack(mem, stack_args + 12u32);
            let cy = <u32>::from_stack(mem, stack_args + 16u32);
            let fuLoad = <u32>::from_stack(mem, stack_args + 20u32);
            winapi::user32::LoadImageW(machine, hInstance, name, typ, cx, cy, fuLoad).to_raw()
        }
        pub unsafe fn LoadMenuA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpMenuName = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::user32::LoadMenuA(machine, hInstance, lpMenuName).to_raw()
        }
        pub unsafe fn LoadMenuW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpMenuName = <u32>::from_stack(mem, stack_args + 4u32);
            winapi::user32::LoadMenuW(machine, hInstance, lpMenuName).to_raw()
        }
        pub unsafe fn LoadStringA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let uID = <u32>::from_stack(mem, stack_args + 4u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
            let cchBufferMax = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::user32::LoadStringA(machine, hInstance, uID, lpBuffer, cchBufferMax).to_raw()
        }
        pub unsafe fn LoadStringW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let uID = <u32>::from_stack(mem, stack_args + 4u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
            let cchBufferMax = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::user32::LoadStringW(machine, hInstance, uID, lpBuffer, cchBufferMax).to_raw()
        }
        pub unsafe fn MapWindowPoints(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWndFrom = <HWND>::from_stack(mem, stack_args + 0u32);
            let hWndTo = <HWND>::from_stack(mem, stack_args + 4u32);
            let lpPoints = <ArrayWithSize<POINT>>::from_stack(mem, stack_args + 8u32);
            winapi::user32::MapWindowPoints(machine, hWndFrom, hWndTo, lpPoints).to_raw()
        }
        pub unsafe fn MessageBoxA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpText = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let lpCaption = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let uType = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::user32::MessageBoxA(machine, hWnd, lpText, lpCaption, uType).to_raw()
        }
        pub unsafe fn MessageBoxW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpText = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpCaption = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let uType = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::user32::MessageBoxW(machine, hWnd, lpText, lpCaption, uType).to_raw()
        }
        pub unsafe fn MoveWindow(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let X = <u32>::from_stack(mem, stack_args + 4u32);
            let Y = <u32>::from_stack(mem, stack_args + 8u32);
            let nWidth = <u32>::from_stack(mem, stack_args + 12u32);
            let nHeight = <u32>::from_stack(mem, stack_args + 16u32);
            let bRepaint = <bool>::from_stack(mem, stack_args + 20u32);
            winapi::user32::MoveWindow(machine, hWnd, X, Y, nWidth, nHeight, bRepaint).to_raw()
        }
        pub unsafe fn MsgWaitForMultipleObjects(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nCount = <u32>::from_stack(mem, stack_args + 0u32);
            let pHandles = <u32>::from_stack(mem, stack_args + 4u32);
            let fWaitAll = <bool>::from_stack(mem, stack_args + 8u32);
            let dwMilliseconds = <u32>::from_stack(mem, stack_args + 12u32);
            let dwWakeMask = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::user32::MsgWaitForMultipleObjects(
                machine,
                nCount,
                pHandles,
                fWaitAll,
                dwMilliseconds,
                dwWakeMask,
            )
            .to_raw()
        }
        pub unsafe fn PeekMessageA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, stack_args + 0u32);
            let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let wMsgFilterMin = <u32>::from_stack(mem, stack_args + 8u32);
            let wMsgFilterMax = <u32>::from_stack(mem, stack_args + 12u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, stack_args + 16u32);
            winapi::user32::PeekMessageA(
                machine,
                lpMsg,
                hWnd,
                wMsgFilterMin,
                wMsgFilterMax,
                wRemoveMsg,
            )
            .to_raw()
        }
        pub unsafe fn PeekMessageW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, stack_args + 0u32);
            let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let wMsgFilterMin = <u32>::from_stack(mem, stack_args + 8u32);
            let wMsgFilterMax = <u32>::from_stack(mem, stack_args + 12u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, stack_args + 16u32);
            winapi::user32::PeekMessageW(
                machine,
                lpMsg,
                hWnd,
                wMsgFilterMin,
                wMsgFilterMax,
                wRemoveMsg,
            )
            .to_raw()
        }
        pub unsafe fn PostMessageW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let Msg = <u32>::from_stack(mem, stack_args + 4u32);
            let wParam = <u32>::from_stack(mem, stack_args + 8u32);
            let lParam = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::user32::PostMessageW(machine, hWnd, Msg, wParam, lParam).to_raw()
        }
        pub unsafe fn PostQuitMessage(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let nExitCode = <i32>::from_stack(mem, stack_args + 0u32);
            winapi::user32::PostQuitMessage(machine, nExitCode).to_raw()
        }
        pub unsafe fn PtInRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 0u32);
            let pt = <POINT>::from_stack(mem, stack_args + 4u32);
            winapi::user32::PtInRect(machine, lprc, pt).to_raw()
        }
        pub unsafe fn RegisterClassA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, stack_args + 0u32);
            winapi::user32::RegisterClassA(machine, lpWndClass).to_raw()
        }
        pub unsafe fn RegisterClassExA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClassEx = <Option<&WNDCLASSEXA>>::from_stack(mem, stack_args + 0u32);
            winapi::user32::RegisterClassExA(machine, lpWndClassEx).to_raw()
        }
        pub unsafe fn RegisterClassExW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClassEx = <Option<&WNDCLASSEXW>>::from_stack(mem, stack_args + 0u32);
            winapi::user32::RegisterClassExW(machine, lpWndClassEx).to_raw()
        }
        pub unsafe fn RegisterClassW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, stack_args + 0u32);
            winapi::user32::RegisterClassW(machine, lpWndClass).to_raw()
        }
        pub unsafe fn RegisterWindowMessageW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            winapi::user32::RegisterWindowMessageW(machine, lpString).to_raw()
        }
        pub unsafe fn ReleaseCapture(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::ReleaseCapture(machine).to_raw()
        }
        pub unsafe fn ReleaseDC(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hdc = <HDC>::from_stack(mem, stack_args + 4u32);
            winapi::user32::ReleaseDC(machine, hwnd, hdc).to_raw()
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
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::SendMessageA(machine, hWnd, Msg, wParam, lParam)
                    .await
                    .to_raw()
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
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::SendMessageW(machine, hWnd, Msg, wParam, lParam)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn SetCapture(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, stack_args + 0u32);
            winapi::user32::SetCapture(machine, hwnd).to_raw()
        }
        pub unsafe fn SetCursor(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hCursor = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::user32::SetCursor(machine, hCursor).to_raw()
        }
        pub unsafe fn SetCursorPos(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let x = <i32>::from_stack(mem, stack_args + 0u32);
            let y = <i32>::from_stack(mem, stack_args + 4u32);
            winapi::user32::SetCursorPos(machine, x, y).to_raw()
        }
        pub unsafe fn SetDlgItemInt(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let uValue = <u32>::from_stack(mem, stack_args + 8u32);
            let _bSigned = <bool>::from_stack(mem, stack_args + 12u32);
            winapi::user32::SetDlgItemInt(machine, hDlg, nIDDlgItem, uValue, _bSigned).to_raw()
        }
        pub unsafe fn SetDlgItemTextA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            winapi::user32::SetDlgItemTextA(machine, hDlg, nIDDlgItem, lpString).to_raw()
        }
        pub unsafe fn SetDlgItemTextW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            winapi::user32::SetDlgItemTextW(machine, hDlg, nIDDlgItem, lpString).to_raw()
        }
        pub unsafe fn SetFocus(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            winapi::user32::SetFocus(machine, hWnd).to_raw()
        }
        pub unsafe fn SetForegroundWindow(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            winapi::user32::SetForegroundWindow(machine, hWnd).to_raw()
        }
        pub unsafe fn SetMenu(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hMenu = <HMENU>::from_stack(mem, stack_args + 4u32);
            winapi::user32::SetMenu(machine, hWnd, hMenu).to_raw()
        }
        pub unsafe fn SetMenuItemInfoA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let item = <u32>::from_stack(mem, stack_args + 4u32);
            let fByPosition = <bool>::from_stack(mem, stack_args + 8u32);
            let lpmii = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::user32::SetMenuItemInfoA(machine, hMenu, item, fByPosition, lpmii).to_raw()
        }
        pub unsafe fn SetRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let xLeft = <i32>::from_stack(mem, stack_args + 4u32);
            let yTop = <i32>::from_stack(mem, stack_args + 8u32);
            let xRight = <i32>::from_stack(mem, stack_args + 12u32);
            let yBottom = <i32>::from_stack(mem, stack_args + 16u32);
            winapi::user32::SetRect(machine, lprc, xLeft, yTop, xRight, yBottom).to_raw()
        }
        pub unsafe fn SetRectEmpty(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            winapi::user32::SetRectEmpty(machine, lprc).to_raw()
        }
        pub unsafe fn SetTimer(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDEvent = <u32>::from_stack(mem, stack_args + 4u32);
            let uElapse = <u32>::from_stack(mem, stack_args + 8u32);
            let lpTimerFunc = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::user32::SetTimer(machine, hWnd, nIDEvent, uElapse, lpTimerFunc).to_raw()
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
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::SetWindowPos(machine, hWnd, hWndInsertAfter, X, Y, cx, cy, uFlags)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn SetWindowTextA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            winapi::user32::SetWindowTextA(machine, hWnd, lpString).to_raw()
        }
        pub unsafe fn ShowCursor(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let bShow = <bool>::from_stack(mem, stack_args + 0u32);
            winapi::user32::ShowCursor(machine, bShow).to_raw()
        }
        pub unsafe fn ShowWindow(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let nCmdShow = <Result<SW, u32>>::from_stack(mem, stack_args + 4u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::ShowWindow(machine, hWnd, nCmdShow)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn TranslateAcceleratorW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hAccTable = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 8u32);
            winapi::user32::TranslateAcceleratorW(machine, hWnd, hAccTable, lpMsg).to_raw()
        }
        pub unsafe fn TranslateMessage(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 0u32);
            winapi::user32::TranslateMessage(machine, lpMsg).to_raw()
        }
        pub unsafe fn UpdateWindow(
            machine: &mut Machine,
            stack_args: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::UpdateWindow(machine, hWnd).await.to_raw()
            })
        }
        pub unsafe fn ValidateRect(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            winapi::user32::ValidateRect(machine, hWnd, lpRect).to_raw()
        }
        pub unsafe fn WaitMessage(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::WaitMessage(machine).to_raw()
        }
        pub unsafe fn WinHelpW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWndMain = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpszHelp = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let uCommand = <u32>::from_stack(mem, stack_args + 8u32);
            let dwData = <u32>::from_stack(mem, stack_args + 12u32);
            winapi::user32::WinHelpW(machine, hWndMain, lpszHelp, uCommand, dwData).to_raw()
        }
        pub unsafe fn wsprintfA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let buf = <u32>::from_stack(mem, stack_args + 0u32);
            let fmt = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let args = <VarArgs>::from_stack(mem, stack_args + 8u32);
            winapi::user32::wsprintfA(machine, buf, fmt, args).to_raw()
        }
        pub unsafe fn wsprintfW(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let buf = <u32>::from_stack(mem, stack_args + 0u32);
            let fmt = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let args = <VarArgs>::from_stack(mem, stack_args + 8u32);
            winapi::user32::wsprintfW(machine, buf, fmt, args).to_raw()
        }
    }
    const SHIMS: [Shim; 118usize] = [
        Shim {
            name: "AdjustWindowRect",
            func: Handler::Sync(impls::AdjustWindowRect),
        },
        Shim {
            name: "AdjustWindowRectEx",
            func: Handler::Sync(impls::AdjustWindowRectEx),
        },
        Shim {
            name: "AppendMenuA",
            func: Handler::Sync(impls::AppendMenuA),
        },
        Shim {
            name: "BeginPaint",
            func: Handler::Sync(impls::BeginPaint),
        },
        Shim {
            name: "CheckDlgButton",
            func: Handler::Sync(impls::CheckDlgButton),
        },
        Shim {
            name: "CheckMenuItem",
            func: Handler::Sync(impls::CheckMenuItem),
        },
        Shim {
            name: "CheckRadioButton",
            func: Handler::Sync(impls::CheckRadioButton),
        },
        Shim {
            name: "ClientToScreen",
            func: Handler::Sync(impls::ClientToScreen),
        },
        Shim {
            name: "CopyRect",
            func: Handler::Sync(impls::CopyRect),
        },
        Shim {
            name: "CreateCursor",
            func: Handler::Sync(impls::CreateCursor),
        },
        Shim {
            name: "CreatePopupMenu",
            func: Handler::Sync(impls::CreatePopupMenu),
        },
        Shim {
            name: "CreateWindowExA",
            func: Handler::Async(impls::CreateWindowExA),
        },
        Shim {
            name: "CreateWindowExW",
            func: Handler::Async(impls::CreateWindowExW),
        },
        Shim {
            name: "DefWindowProcA",
            func: Handler::Async(impls::DefWindowProcA),
        },
        Shim {
            name: "DefWindowProcW",
            func: Handler::Async(impls::DefWindowProcW),
        },
        Shim {
            name: "DestroyWindow",
            func: Handler::Sync(impls::DestroyWindow),
        },
        Shim {
            name: "DialogBoxIndirectParamA",
            func: Handler::Sync(impls::DialogBoxIndirectParamA),
        },
        Shim {
            name: "DialogBoxParamA",
            func: Handler::Sync(impls::DialogBoxParamA),
        },
        Shim {
            name: "DialogBoxParamW",
            func: Handler::Sync(impls::DialogBoxParamW),
        },
        Shim {
            name: "DispatchMessageA",
            func: Handler::Async(impls::DispatchMessageA),
        },
        Shim {
            name: "DispatchMessageW",
            func: Handler::Async(impls::DispatchMessageW),
        },
        Shim {
            name: "DrawTextW",
            func: Handler::Sync(impls::DrawTextW),
        },
        Shim {
            name: "EnableMenuItem",
            func: Handler::Sync(impls::EnableMenuItem),
        },
        Shim {
            name: "EnableWindow",
            func: Handler::Sync(impls::EnableWindow),
        },
        Shim {
            name: "EndDialog",
            func: Handler::Sync(impls::EndDialog),
        },
        Shim {
            name: "EndPaint",
            func: Handler::Sync(impls::EndPaint),
        },
        Shim {
            name: "FillRect",
            func: Handler::Sync(impls::FillRect),
        },
        Shim {
            name: "FindWindowA",
            func: Handler::Sync(impls::FindWindowA),
        },
        Shim {
            name: "FrameRect",
            func: Handler::Sync(impls::FrameRect),
        },
        Shim {
            name: "GetActiveWindow",
            func: Handler::Sync(impls::GetActiveWindow),
        },
        Shim {
            name: "GetCapture",
            func: Handler::Sync(impls::GetCapture),
        },
        Shim {
            name: "GetClientRect",
            func: Handler::Sync(impls::GetClientRect),
        },
        Shim {
            name: "GetDC",
            func: Handler::Sync(impls::GetDC),
        },
        Shim {
            name: "GetDesktopWindow",
            func: Handler::Sync(impls::GetDesktopWindow),
        },
        Shim {
            name: "GetDlgItem",
            func: Handler::Sync(impls::GetDlgItem),
        },
        Shim {
            name: "GetDlgItemInt",
            func: Handler::Sync(impls::GetDlgItemInt),
        },
        Shim {
            name: "GetDlgItemTextW",
            func: Handler::Sync(impls::GetDlgItemTextW),
        },
        Shim {
            name: "GetFocus",
            func: Handler::Sync(impls::GetFocus),
        },
        Shim {
            name: "GetForegroundWindow",
            func: Handler::Sync(impls::GetForegroundWindow),
        },
        Shim {
            name: "GetKeyState",
            func: Handler::Sync(impls::GetKeyState),
        },
        Shim {
            name: "GetLastActivePopup",
            func: Handler::Sync(impls::GetLastActivePopup),
        },
        Shim {
            name: "GetMenu",
            func: Handler::Sync(impls::GetMenu),
        },
        Shim {
            name: "GetMenuItemRect",
            func: Handler::Sync(impls::GetMenuItemRect),
        },
        Shim {
            name: "GetMessageA",
            func: Handler::Async(impls::GetMessageA),
        },
        Shim {
            name: "GetMessageW",
            func: Handler::Async(impls::GetMessageW),
        },
        Shim {
            name: "GetSubMenu",
            func: Handler::Sync(impls::GetSubMenu),
        },
        Shim {
            name: "GetSysColor",
            func: Handler::Sync(impls::GetSysColor),
        },
        Shim {
            name: "GetSystemMenu",
            func: Handler::Sync(impls::GetSystemMenu),
        },
        Shim {
            name: "GetSystemMetrics",
            func: Handler::Sync(impls::GetSystemMetrics),
        },
        Shim {
            name: "GetWindowDC",
            func: Handler::Sync(impls::GetWindowDC),
        },
        Shim {
            name: "GetWindowLongA",
            func: Handler::Sync(impls::GetWindowLongA),
        },
        Shim {
            name: "GetWindowPlacement",
            func: Handler::Sync(impls::GetWindowPlacement),
        },
        Shim {
            name: "GetWindowRect",
            func: Handler::Sync(impls::GetWindowRect),
        },
        Shim {
            name: "InflateRect",
            func: Handler::Sync(impls::InflateRect),
        },
        Shim {
            name: "IntersectRect",
            func: Handler::Sync(impls::IntersectRect),
        },
        Shim {
            name: "InvalidateRect",
            func: Handler::Sync(impls::InvalidateRect),
        },
        Shim {
            name: "InvalidateRgn",
            func: Handler::Sync(impls::InvalidateRgn),
        },
        Shim {
            name: "InvertRect",
            func: Handler::Sync(impls::InvertRect),
        },
        Shim {
            name: "IsDlgButtonChecked",
            func: Handler::Sync(impls::IsDlgButtonChecked),
        },
        Shim {
            name: "IsIconic",
            func: Handler::Sync(impls::IsIconic),
        },
        Shim {
            name: "IsRectEmpty",
            func: Handler::Sync(impls::IsRectEmpty),
        },
        Shim {
            name: "KillTimer",
            func: Handler::Sync(impls::KillTimer),
        },
        Shim {
            name: "LoadAcceleratorsW",
            func: Handler::Sync(impls::LoadAcceleratorsW),
        },
        Shim {
            name: "LoadBitmapA",
            func: Handler::Sync(impls::LoadBitmapA),
        },
        Shim {
            name: "LoadCursorA",
            func: Handler::Sync(impls::LoadCursorA),
        },
        Shim {
            name: "LoadCursorW",
            func: Handler::Sync(impls::LoadCursorW),
        },
        Shim {
            name: "LoadIconA",
            func: Handler::Sync(impls::LoadIconA),
        },
        Shim {
            name: "LoadIconW",
            func: Handler::Sync(impls::LoadIconW),
        },
        Shim {
            name: "LoadImageA",
            func: Handler::Sync(impls::LoadImageA),
        },
        Shim {
            name: "LoadImageW",
            func: Handler::Sync(impls::LoadImageW),
        },
        Shim {
            name: "LoadMenuA",
            func: Handler::Sync(impls::LoadMenuA),
        },
        Shim {
            name: "LoadMenuW",
            func: Handler::Sync(impls::LoadMenuW),
        },
        Shim {
            name: "LoadStringA",
            func: Handler::Sync(impls::LoadStringA),
        },
        Shim {
            name: "LoadStringW",
            func: Handler::Sync(impls::LoadStringW),
        },
        Shim {
            name: "MapWindowPoints",
            func: Handler::Sync(impls::MapWindowPoints),
        },
        Shim {
            name: "MessageBoxA",
            func: Handler::Sync(impls::MessageBoxA),
        },
        Shim {
            name: "MessageBoxW",
            func: Handler::Sync(impls::MessageBoxW),
        },
        Shim {
            name: "MoveWindow",
            func: Handler::Sync(impls::MoveWindow),
        },
        Shim {
            name: "MsgWaitForMultipleObjects",
            func: Handler::Sync(impls::MsgWaitForMultipleObjects),
        },
        Shim {
            name: "PeekMessageA",
            func: Handler::Sync(impls::PeekMessageA),
        },
        Shim {
            name: "PeekMessageW",
            func: Handler::Sync(impls::PeekMessageW),
        },
        Shim {
            name: "PostMessageW",
            func: Handler::Sync(impls::PostMessageW),
        },
        Shim {
            name: "PostQuitMessage",
            func: Handler::Sync(impls::PostQuitMessage),
        },
        Shim {
            name: "PtInRect",
            func: Handler::Sync(impls::PtInRect),
        },
        Shim {
            name: "RegisterClassA",
            func: Handler::Sync(impls::RegisterClassA),
        },
        Shim {
            name: "RegisterClassExA",
            func: Handler::Sync(impls::RegisterClassExA),
        },
        Shim {
            name: "RegisterClassExW",
            func: Handler::Sync(impls::RegisterClassExW),
        },
        Shim {
            name: "RegisterClassW",
            func: Handler::Sync(impls::RegisterClassW),
        },
        Shim {
            name: "RegisterWindowMessageW",
            func: Handler::Sync(impls::RegisterWindowMessageW),
        },
        Shim {
            name: "ReleaseCapture",
            func: Handler::Sync(impls::ReleaseCapture),
        },
        Shim {
            name: "ReleaseDC",
            func: Handler::Sync(impls::ReleaseDC),
        },
        Shim {
            name: "SendMessageA",
            func: Handler::Async(impls::SendMessageA),
        },
        Shim {
            name: "SendMessageW",
            func: Handler::Async(impls::SendMessageW),
        },
        Shim {
            name: "SetCapture",
            func: Handler::Sync(impls::SetCapture),
        },
        Shim {
            name: "SetCursor",
            func: Handler::Sync(impls::SetCursor),
        },
        Shim {
            name: "SetCursorPos",
            func: Handler::Sync(impls::SetCursorPos),
        },
        Shim {
            name: "SetDlgItemInt",
            func: Handler::Sync(impls::SetDlgItemInt),
        },
        Shim {
            name: "SetDlgItemTextA",
            func: Handler::Sync(impls::SetDlgItemTextA),
        },
        Shim {
            name: "SetDlgItemTextW",
            func: Handler::Sync(impls::SetDlgItemTextW),
        },
        Shim {
            name: "SetFocus",
            func: Handler::Sync(impls::SetFocus),
        },
        Shim {
            name: "SetForegroundWindow",
            func: Handler::Sync(impls::SetForegroundWindow),
        },
        Shim {
            name: "SetMenu",
            func: Handler::Sync(impls::SetMenu),
        },
        Shim {
            name: "SetMenuItemInfoA",
            func: Handler::Sync(impls::SetMenuItemInfoA),
        },
        Shim {
            name: "SetRect",
            func: Handler::Sync(impls::SetRect),
        },
        Shim {
            name: "SetRectEmpty",
            func: Handler::Sync(impls::SetRectEmpty),
        },
        Shim {
            name: "SetTimer",
            func: Handler::Sync(impls::SetTimer),
        },
        Shim {
            name: "SetWindowPos",
            func: Handler::Async(impls::SetWindowPos),
        },
        Shim {
            name: "SetWindowTextA",
            func: Handler::Sync(impls::SetWindowTextA),
        },
        Shim {
            name: "ShowCursor",
            func: Handler::Sync(impls::ShowCursor),
        },
        Shim {
            name: "ShowWindow",
            func: Handler::Async(impls::ShowWindow),
        },
        Shim {
            name: "TranslateAcceleratorW",
            func: Handler::Sync(impls::TranslateAcceleratorW),
        },
        Shim {
            name: "TranslateMessage",
            func: Handler::Sync(impls::TranslateMessage),
        },
        Shim {
            name: "UpdateWindow",
            func: Handler::Async(impls::UpdateWindow),
        },
        Shim {
            name: "ValidateRect",
            func: Handler::Sync(impls::ValidateRect),
        },
        Shim {
            name: "WaitMessage",
            func: Handler::Sync(impls::WaitMessage),
        },
        Shim {
            name: "WinHelpW",
            func: Handler::Sync(impls::WinHelpW),
        },
        Shim {
            name: "wsprintfA",
            func: Handler::Sync(impls::wsprintfA),
        },
        Shim {
            name: "wsprintfW",
            func: Handler::Sync(impls::wsprintfW),
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
    mod impls {
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
            winapi::wininet::InternetOpenA(
                machine,
                lpszAgent,
                dwAccessType,
                lpszProxy,
                lpszProxyBypass,
                dwFlags,
            )
            .to_raw()
        }
    }
    const SHIMS: [Shim; 1usize] = [Shim {
        name: "InternetOpenA",
        func: Handler::Sync(impls::InternetOpenA),
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "wininet.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/wininet.dll"),
    };
}
pub mod winmm {
    use super::*;
    mod impls {
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
            winapi::winmm::PlaySoundW(machine, pszSound, hmod, fdwSound).to_raw()
        }
        pub unsafe fn timeBeginPeriod(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uPeriod = <u32>::from_stack(mem, stack_args + 0u32);
            winapi::winmm::timeBeginPeriod(machine, uPeriod).to_raw()
        }
        pub unsafe fn timeGetTime(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::winmm::timeGetTime(machine).to_raw()
        }
        pub unsafe fn timeSetEvent(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uDelay = <u32>::from_stack(mem, stack_args + 0u32);
            let uResolution = <u32>::from_stack(mem, stack_args + 4u32);
            let lpTimeProc = <u32>::from_stack(mem, stack_args + 8u32);
            let dwUser = <u32>::from_stack(mem, stack_args + 12u32);
            let fuEvent = <u32>::from_stack(mem, stack_args + 16u32);
            winapi::winmm::timeSetEvent(machine, uDelay, uResolution, lpTimeProc, dwUser, fuEvent)
                .to_raw()
        }
        pub unsafe fn waveOutClose(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            winapi::winmm::waveOutClose(machine, hwo).to_raw()
        }
        pub unsafe fn waveOutGetDevCapsA(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let uDeviceID = <u32>::from_stack(mem, stack_args + 0u32);
            let pwoc = <Option<&mut WAVEOUTCAPS>>::from_stack(mem, stack_args + 4u32);
            let cbwoc = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::winmm::waveOutGetDevCapsA(machine, uDeviceID, pwoc, cbwoc).to_raw()
        }
        pub unsafe fn waveOutGetNumDevs(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::winmm::waveOutGetNumDevs(machine).to_raw()
        }
        pub unsafe fn waveOutGetPosition(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let pmmt = <Option<&mut MMTIME>>::from_stack(mem, stack_args + 4u32);
            let cbmmt = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::winmm::waveOutGetPosition(machine, hwo, pmmt, cbmmt).to_raw()
        }
        pub unsafe fn waveOutOpen(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let phwo = <Option<&mut HWAVEOUT>>::from_stack(mem, stack_args + 0u32);
            let uDeviceID = <u32>::from_stack(mem, stack_args + 4u32);
            let pwfx = <Option<&WAVEFORMATEX>>::from_stack(mem, stack_args + 8u32);
            let dwCallback = <u32>::from_stack(mem, stack_args + 12u32);
            let dwInstance = <u32>::from_stack(mem, stack_args + 16u32);
            let fdwOpen = <Result<WaveOutOpenFlags, u32>>::from_stack(mem, stack_args + 20u32);
            winapi::winmm::waveOutOpen(
                machine, phwo, uDeviceID, pwfx, dwCallback, dwInstance, fdwOpen,
            )
            .to_raw()
        }
        pub unsafe fn waveOutPrepareHeader(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let pwh = <Option<&WAVEHDR>>::from_stack(mem, stack_args + 4u32);
            let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::winmm::waveOutPrepareHeader(machine, hwo, pwh, cbwh).to_raw()
        }
        pub unsafe fn waveOutReset(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            winapi::winmm::waveOutReset(machine, hwo).to_raw()
        }
        pub unsafe fn waveOutUnprepareHeader(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let pwh = <Option<&mut WAVEHDR>>::from_stack(mem, stack_args + 4u32);
            let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::winmm::waveOutUnprepareHeader(machine, hwo, pwh, cbwh).to_raw()
        }
        pub unsafe fn waveOutWrite(machine: &mut Machine, stack_args: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let pwh = <Option<&WAVEHDR>>::from_stack(mem, stack_args + 4u32);
            let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
            winapi::winmm::waveOutWrite(machine, hwo, pwh, cbwh).to_raw()
        }
    }
    const SHIMS: [Shim; 13usize] = [
        Shim {
            name: "PlaySoundW",
            func: Handler::Sync(impls::PlaySoundW),
        },
        Shim {
            name: "timeBeginPeriod",
            func: Handler::Sync(impls::timeBeginPeriod),
        },
        Shim {
            name: "timeGetTime",
            func: Handler::Sync(impls::timeGetTime),
        },
        Shim {
            name: "timeSetEvent",
            func: Handler::Sync(impls::timeSetEvent),
        },
        Shim {
            name: "waveOutClose",
            func: Handler::Sync(impls::waveOutClose),
        },
        Shim {
            name: "waveOutGetDevCapsA",
            func: Handler::Sync(impls::waveOutGetDevCapsA),
        },
        Shim {
            name: "waveOutGetNumDevs",
            func: Handler::Sync(impls::waveOutGetNumDevs),
        },
        Shim {
            name: "waveOutGetPosition",
            func: Handler::Sync(impls::waveOutGetPosition),
        },
        Shim {
            name: "waveOutOpen",
            func: Handler::Sync(impls::waveOutOpen),
        },
        Shim {
            name: "waveOutPrepareHeader",
            func: Handler::Sync(impls::waveOutPrepareHeader),
        },
        Shim {
            name: "waveOutReset",
            func: Handler::Sync(impls::waveOutReset),
        },
        Shim {
            name: "waveOutUnprepareHeader",
            func: Handler::Sync(impls::waveOutUnprepareHeader),
        },
        Shim {
            name: "waveOutWrite",
            func: Handler::Sync(impls::waveOutWrite),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "winmm.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/winmm.dll"),
    };
}
