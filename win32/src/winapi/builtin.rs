#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#[doc = r" Generated code, do not edit."]
use crate::shims::Shim;
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
        pub unsafe fn RegCloseKey(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            winapi::advapi32::RegCloseKey(machine, hKey).to_raw()
        }
        pub unsafe fn RegCreateKeyA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpSubKey = <Option<&str>>::from_stack(mem, esp + 8u32);
            let phkResult = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            winapi::advapi32::RegCreateKeyA(machine, hKey, lpSubKey, phkResult).to_raw()
        }
        pub unsafe fn RegCreateKeyExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpSubKey = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let Reserved = <u32>::from_stack(mem, esp + 12u32);
            let lpClass = <Option<&Str16>>::from_stack(mem, esp + 16u32);
            let dwOptions = <u32>::from_stack(mem, esp + 20u32);
            let samDesired = <u32>::from_stack(mem, esp + 24u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 28u32);
            let phkResult = <Option<&mut u32>>::from_stack(mem, esp + 32u32);
            let lpdwDisposition = <Option<&mut u32>>::from_stack(mem, esp + 36u32);
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
        pub unsafe fn RegQueryValueExA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpValueName = <Option<&str>>::from_stack(mem, esp + 8u32);
            let lpReserved = <u32>::from_stack(mem, esp + 12u32);
            let lpType = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpData = <u32>::from_stack(mem, esp + 20u32);
            let lpcbData = <Option<&mut u32>>::from_stack(mem, esp + 24u32);
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
        pub unsafe fn RegQueryValueExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpValueName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpReserved = <u32>::from_stack(mem, esp + 12u32);
            let lpType = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpData = <u32>::from_stack(mem, esp + 20u32);
            let lpcbData = <Option<&mut u32>>::from_stack(mem, esp + 24u32);
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
        pub unsafe fn RegSetValueExA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpValueName = <Option<&str>>::from_stack(mem, esp + 8u32);
            let Reserved = <u32>::from_stack(mem, esp + 12u32);
            let dwType = <u32>::from_stack(mem, esp + 16u32);
            let lpData = <u32>::from_stack(mem, esp + 20u32);
            let cbData = <u32>::from_stack(mem, esp + 24u32);
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
        pub unsafe fn RegSetValueExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpValueName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let Reserved = <u32>::from_stack(mem, esp + 12u32);
            let dwType = <u32>::from_stack(mem, esp + 16u32);
            let lpData = <u32>::from_stack(mem, esp + 20u32);
            let cbData = <u32>::from_stack(mem, esp + 24u32);
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
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const RegCloseKey: Shim = Shim {
            name: "RegCloseKey",
            func: crate::shims::Handler::Sync(impls::RegCloseKey),
        };
        pub const RegCreateKeyA: Shim = Shim {
            name: "RegCreateKeyA",
            func: crate::shims::Handler::Sync(impls::RegCreateKeyA),
        };
        pub const RegCreateKeyExW: Shim = Shim {
            name: "RegCreateKeyExW",
            func: crate::shims::Handler::Sync(impls::RegCreateKeyExW),
        };
        pub const RegQueryValueExA: Shim = Shim {
            name: "RegQueryValueExA",
            func: crate::shims::Handler::Sync(impls::RegQueryValueExA),
        };
        pub const RegQueryValueExW: Shim = Shim {
            name: "RegQueryValueExW",
            func: crate::shims::Handler::Sync(impls::RegQueryValueExW),
        };
        pub const RegSetValueExA: Shim = Shim {
            name: "RegSetValueExA",
            func: crate::shims::Handler::Sync(impls::RegSetValueExA),
        };
        pub const RegSetValueExW: Shim = Shim {
            name: "RegSetValueExW",
            func: crate::shims::Handler::Sync(impls::RegSetValueExW),
        };
    }
    const SHIMS: [Shim; 7usize] = [
        shims::RegCloseKey,
        shims::RegCreateKeyA,
        shims::RegCreateKeyExW,
        shims::RegQueryValueExA,
        shims::RegQueryValueExW,
        shims::RegSetValueExA,
        shims::RegSetValueExW,
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
        pub unsafe fn BASS_ChannelGetPosition(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let mode = <u32>::from_stack(mem, esp + 4u32);
            winapi::bass::BASS_ChannelGetPosition(machine, mode).to_raw()
        }
        pub unsafe fn BASS_Free(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            winapi::bass::BASS_Free(machine, arg1).to_raw()
        }
        pub unsafe fn BASS_Init(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            let arg2 = <u32>::from_stack(mem, esp + 8u32);
            let arg3 = <u32>::from_stack(mem, esp + 12u32);
            let arg4 = <u32>::from_stack(mem, esp + 16u32);
            winapi::bass::BASS_Init(machine, arg1, arg2, arg3, arg4).to_raw()
        }
        pub unsafe fn BASS_MusicLoad(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            let arg2 = <u32>::from_stack(mem, esp + 8u32);
            let arg3 = <u32>::from_stack(mem, esp + 12u32);
            let arg4 = <u32>::from_stack(mem, esp + 16u32);
            let arg5 = <u32>::from_stack(mem, esp + 20u32);
            winapi::bass::BASS_MusicLoad(machine, arg1, arg2, arg3, arg4, arg5).to_raw()
        }
        pub unsafe fn BASS_MusicPlay(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            winapi::bass::BASS_MusicPlay(machine, arg1).to_raw()
        }
        pub unsafe fn BASS_MusicSetPositionScaler(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            let arg2 = <u32>::from_stack(mem, esp + 8u32);
            winapi::bass::BASS_MusicSetPositionScaler(machine, arg1, arg2).to_raw()
        }
        pub unsafe fn BASS_Start(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::bass::BASS_Start(machine).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const BASS_ChannelGetPosition: Shim = Shim {
            name: "BASS_ChannelGetPosition",
            func: crate::shims::Handler::Sync(impls::BASS_ChannelGetPosition),
        };
        pub const BASS_Free: Shim = Shim {
            name: "BASS_Free",
            func: crate::shims::Handler::Sync(impls::BASS_Free),
        };
        pub const BASS_Init: Shim = Shim {
            name: "BASS_Init",
            func: crate::shims::Handler::Sync(impls::BASS_Init),
        };
        pub const BASS_MusicLoad: Shim = Shim {
            name: "BASS_MusicLoad",
            func: crate::shims::Handler::Sync(impls::BASS_MusicLoad),
        };
        pub const BASS_MusicPlay: Shim = Shim {
            name: "BASS_MusicPlay",
            func: crate::shims::Handler::Sync(impls::BASS_MusicPlay),
        };
        pub const BASS_MusicSetPositionScaler: Shim = Shim {
            name: "BASS_MusicSetPositionScaler",
            func: crate::shims::Handler::Sync(impls::BASS_MusicSetPositionScaler),
        };
        pub const BASS_Start: Shim = Shim {
            name: "BASS_Start",
            func: crate::shims::Handler::Sync(impls::BASS_Start),
        };
    }
    const SHIMS: [Shim; 7usize] = [
        shims::BASS_ChannelGetPosition,
        shims::BASS_Free,
        shims::BASS_Init,
        shims::BASS_MusicLoad,
        shims::BASS_MusicPlay,
        shims::BASS_MusicSetPositionScaler,
        shims::BASS_Start,
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
        pub unsafe fn DirectDrawCreate(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, esp + 4u32);
            let lplpDD = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 12u32);
            winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter).to_raw()
        }
        pub unsafe fn DirectDrawCreateClipper(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, esp + 4u32);
            let lplpDDClipper = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 12u32);
            winapi::ddraw::DirectDrawCreateClipper(machine, dwFlags, lplpDDClipper, pUnkOuter)
                .to_raw()
        }
        pub unsafe fn DirectDrawCreateEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, esp + 4u32);
            let lplpDD = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let iid = <Option<&GUID>>::from_stack(mem, esp + 12u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 16u32);
            winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter).to_raw()
        }
        pub unsafe fn IDirectDraw2_CreateSurface(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let desc = <Option<&DDSURFACEDESC>>::from_stack(mem, esp + 8u32);
            let lplpDDSurface = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 16u32);
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
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpSurfaceDesc = <Option<&DDSURFACEDESC>>::from_stack(mem, esp + 12u32);
            let lpContext = <u32>::from_stack(mem, esp + 16u32);
            let lpEnumCallback = <u32>::from_stack(mem, esp + 20u32);
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
        pub unsafe fn IDirectDraw2_GetDisplayMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSurfaceDesc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDraw2::GetDisplayMode(machine, this, lpDDSurfaceDesc).to_raw()
        }
        pub unsafe fn IDirectDraw2_QueryInterface(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let riid = <Option<&GUID>>::from_stack(mem, esp + 8u32);
            let ppvObject = <u32>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDraw2::QueryInterface(machine, this, riid, ppvObject).to_raw()
        }
        pub unsafe fn IDirectDraw2_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDraw2::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDraw2_SetDisplayMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let width = <u32>::from_stack(mem, esp + 8u32);
            let height = <u32>::from_stack(mem, esp + 12u32);
            let bpp = <u32>::from_stack(mem, esp + 16u32);
            winapi::ddraw::IDirectDraw2::SetDisplayMode(machine, this, width, height, bpp).to_raw()
        }
        pub unsafe fn IDirectDraw7_CreatePalette(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let flags = <Result<DDPCAPS, u32>>::from_stack(mem, esp + 8u32);
            let entries = <u32>::from_stack(mem, esp + 12u32);
            let lplpPalette = <u32>::from_stack(mem, esp + 16u32);
            let unused = <u32>::from_stack(mem, esp + 20u32);
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
        pub unsafe fn IDirectDraw7_CreateSurface(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let desc = <Option<&DDSURFACEDESC2>>::from_stack(mem, esp + 8u32);
            let lpDirectDrawSurface7 = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            let unused = <u32>::from_stack(mem, esp + 16u32);
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
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpSurfaceDesc = <Option<&DDSURFACEDESC2>>::from_stack(mem, esp + 12u32);
            let lpContext = <u32>::from_stack(mem, esp + 16u32);
            let lpEnumCallback = <u32>::from_stack(mem, esp + 20u32);
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
        pub unsafe fn IDirectDraw7_GetDisplayMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSurfaceDesc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDraw7::GetDisplayMode(machine, this, lpDDSurfaceDesc).to_raw()
        }
        pub unsafe fn IDirectDraw7_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDraw7::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDraw7_RestoreDisplayMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDraw7::RestoreDisplayMode(machine, this).to_raw()
        }
        pub unsafe fn IDirectDraw7_SetCooperativeLevel(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let hwnd = <HWND>::from_stack(mem, esp + 8u32);
            let flags = <Result<DDSCL, u32>>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDraw7::SetCooperativeLevel(machine, this, hwnd, flags).to_raw()
        }
        pub unsafe fn IDirectDraw7_SetDisplayMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let width = <u32>::from_stack(mem, esp + 8u32);
            let height = <u32>::from_stack(mem, esp + 12u32);
            let bpp = <u32>::from_stack(mem, esp + 16u32);
            let refresh = <u32>::from_stack(mem, esp + 20u32);
            let flags = <u32>::from_stack(mem, esp + 24u32);
            winapi::ddraw::IDirectDraw7::SetDisplayMode(
                machine, this, width, height, bpp, refresh, flags,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDraw7_WaitForVerticalBlank(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let flags = <u32>::from_stack(mem, esp + 8u32);
            let _unused = <u32>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDraw7::WaitForVerticalBlank(machine, this, flags, _unused)
                .to_raw()
        }
        pub unsafe fn IDirectDrawClipper_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDrawClipper::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawClipper_SetHWnd(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let unused = <u32>::from_stack(mem, esp + 8u32);
            let hwnd = <HWND>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDrawClipper::SetHWnd(machine, this, unused, hwnd).to_raw()
        }
        pub unsafe fn IDirectDrawPalette_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDrawPalette::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawPalette_SetEntries(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let unused = <u32>::from_stack(mem, esp + 8u32);
            let start = <u32>::from_stack(mem, esp + 12u32);
            let count = <u32>::from_stack(mem, esp + 16u32);
            let entries = <u32>::from_stack(mem, esp + 20u32);
            winapi::ddraw::IDirectDrawPalette::SetEntries(
                machine, this, unused, start, count, entries,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_GetAttachedSurface(
            machine: &mut Machine,
            esp: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSCaps = <Option<&DDSCAPS>>::from_stack(mem, esp + 8u32);
            let lpDirectDrawSurface = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDrawSurface2::GetAttachedSurface(
                machine,
                this,
                lpDDSCaps,
                lpDirectDrawSurface,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_GetCaps(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSCAPS = <Option<&mut DDSCAPS>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface2::GetCaps(machine, this, lpDDSCAPS).to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_GetSurfaceDesc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface2::GetSurfaceDesc(machine, this, desc).to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_Lock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let rect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, esp + 12u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, esp + 16u32);
            let event = <u32>::from_stack(mem, esp + 20u32);
            winapi::ddraw::IDirectDrawSurface2::Lock(machine, this, rect, desc, flags, event)
                .to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDrawSurface2::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_Unlock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let ptr = <u32>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface2::Unlock(machine, this, ptr).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Blt(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDstRect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let lpSrc = <u32>::from_stack(mem, esp + 12u32);
            let lpSrcRect = <Option<&RECT>>::from_stack(mem, esp + 16u32);
            let flags = <Result<DDBLT, u32>>::from_stack(mem, esp + 20u32);
            let lpDDBLTFX = <Option<&DDBLTFX>>::from_stack(mem, esp + 24u32);
            winapi::ddraw::IDirectDrawSurface7::Blt(
                machine, this, lpDstRect, lpSrc, lpSrcRect, flags, lpDDBLTFX,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_BltFast(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let lpSrc = <u32>::from_stack(mem, esp + 16u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, esp + 20u32);
            let flags = <u32>::from_stack(mem, esp + 24u32);
            winapi::ddraw::IDirectDrawSurface7::BltFast(machine, this, x, y, lpSrc, lpRect, flags)
                .to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Flip(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpSurf = <u32>::from_stack(mem, esp + 8u32);
            let flags = <Result<DDFLIP, u32>>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDrawSurface7::Flip(machine, this, lpSurf, flags).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetAttachedSurface(
            machine: &mut Machine,
            esp: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSCaps2 = <Option<&DDSCAPS2>>::from_stack(mem, esp + 8u32);
            let lpDirectDrawSurface7 = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDrawSurface7::GetAttachedSurface(
                machine,
                this,
                lpDDSCaps2,
                lpDirectDrawSurface7,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetCaps(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSCAPS2 = <Option<&mut DDSCAPS2>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::GetCaps(machine, this, lpDDSCAPS2).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpHDC = <u32>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::GetDC(machine, this, lpHDC).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetPixelFormat(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let fmt = <Option<&mut DDPIXELFORMAT>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::GetPixelFormat(machine, this, fmt).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetSurfaceDesc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDesc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::GetSurfaceDesc(machine, this, lpDesc).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Lock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let rect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let desc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, esp + 12u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, esp + 16u32);
            let unused = <u32>::from_stack(mem, esp + 20u32);
            winapi::ddraw::IDirectDrawSurface7::Lock(machine, this, rect, desc, flags, unused)
                .to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDrawSurface7::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_ReleaseDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _this = <u32>::from_stack(mem, esp + 4u32);
            let _hDC = <u32>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::ReleaseDC(machine, _this, _hDC).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Restore(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDrawSurface7::Restore(machine, _this).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_SetClipper(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let clipper = <u32>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::SetClipper(machine, this, clipper).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_SetPalette(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let palette = <u32>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::SetPalette(machine, this, palette).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Unlock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let rect = <Option<&mut RECT>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::Unlock(machine, this, rect).to_raw()
        }
        pub unsafe fn IDirectDrawSurface_GetAttachedSurface(
            machine: &mut Machine,
            esp: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSCaps = <Option<&DDSCAPS>>::from_stack(mem, esp + 8u32);
            let lpDirectDrawSurface = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDrawSurface::GetAttachedSurface(
                machine,
                this,
                lpDDSCaps,
                lpDirectDrawSurface,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface_GetCaps(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSCAPS = <Option<&mut DDSCAPS>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface::GetCaps(machine, this, lpDDSCAPS).to_raw()
        }
        pub unsafe fn IDirectDrawSurface_Lock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let rect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, esp + 12u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, esp + 16u32);
            let event = <u32>::from_stack(mem, esp + 20u32);
            winapi::ddraw::IDirectDrawSurface::Lock(machine, this, rect, desc, flags, event)
                .to_raw()
        }
        pub unsafe fn IDirectDrawSurface_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDrawSurface::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawSurface_Unlock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let ptr = <u32>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface::Unlock(machine, this, ptr).to_raw()
        }
        pub unsafe fn IDirectDraw_CreateSurface(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let desc = <Option<&DDSURFACEDESC>>::from_stack(mem, esp + 8u32);
            let lplpDDSurface = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 16u32);
            winapi::ddraw::IDirectDraw::CreateSurface(machine, this, desc, lplpDDSurface, pUnkOuter)
                .to_raw()
        }
        pub unsafe fn IDirectDraw_EnumDisplayModes(
            machine: &mut Machine,
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpSurfaceDesc = <Option<&DDSURFACEDESC>>::from_stack(mem, esp + 12u32);
            let lpContext = <u32>::from_stack(mem, esp + 16u32);
            let lpEnumCallback = <u32>::from_stack(mem, esp + 20u32);
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
        pub unsafe fn IDirectDraw_QueryInterface(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let riid = <Option<&GUID>>::from_stack(mem, esp + 8u32);
            let ppvObject = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDraw::QueryInterface(machine, this, riid, ppvObject).to_raw()
        }
        pub unsafe fn IDirectDraw_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDraw::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDraw_SetDisplayMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let width = <u32>::from_stack(mem, esp + 8u32);
            let height = <u32>::from_stack(mem, esp + 12u32);
            let bpp = <u32>::from_stack(mem, esp + 16u32);
            winapi::ddraw::IDirectDraw::SetDisplayMode(machine, this, width, height, bpp).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const DirectDrawCreate: Shim = Shim {
            name: "DirectDrawCreate",
            func: crate::shims::Handler::Sync(impls::DirectDrawCreate),
        };
        pub const DirectDrawCreateClipper: Shim = Shim {
            name: "DirectDrawCreateClipper",
            func: crate::shims::Handler::Sync(impls::DirectDrawCreateClipper),
        };
        pub const DirectDrawCreateEx: Shim = Shim {
            name: "DirectDrawCreateEx",
            func: crate::shims::Handler::Sync(impls::DirectDrawCreateEx),
        };
        pub const IDirectDraw2_CreateSurface: Shim = Shim {
            name: "IDirectDraw2::CreateSurface",
            func: crate::shims::Handler::Sync(impls::IDirectDraw2_CreateSurface),
        };
        pub const IDirectDraw2_EnumDisplayModes: Shim = Shim {
            name: "IDirectDraw2::EnumDisplayModes",
            func: crate::shims::Handler::Async(impls::IDirectDraw2_EnumDisplayModes),
        };
        pub const IDirectDraw2_GetDisplayMode: Shim = Shim {
            name: "IDirectDraw2::GetDisplayMode",
            func: crate::shims::Handler::Sync(impls::IDirectDraw2_GetDisplayMode),
        };
        pub const IDirectDraw2_QueryInterface: Shim = Shim {
            name: "IDirectDraw2::QueryInterface",
            func: crate::shims::Handler::Sync(impls::IDirectDraw2_QueryInterface),
        };
        pub const IDirectDraw2_Release: Shim = Shim {
            name: "IDirectDraw2::Release",
            func: crate::shims::Handler::Sync(impls::IDirectDraw2_Release),
        };
        pub const IDirectDraw2_SetDisplayMode: Shim = Shim {
            name: "IDirectDraw2::SetDisplayMode",
            func: crate::shims::Handler::Sync(impls::IDirectDraw2_SetDisplayMode),
        };
        pub const IDirectDraw7_CreatePalette: Shim = Shim {
            name: "IDirectDraw7::CreatePalette",
            func: crate::shims::Handler::Sync(impls::IDirectDraw7_CreatePalette),
        };
        pub const IDirectDraw7_CreateSurface: Shim = Shim {
            name: "IDirectDraw7::CreateSurface",
            func: crate::shims::Handler::Sync(impls::IDirectDraw7_CreateSurface),
        };
        pub const IDirectDraw7_EnumDisplayModes: Shim = Shim {
            name: "IDirectDraw7::EnumDisplayModes",
            func: crate::shims::Handler::Async(impls::IDirectDraw7_EnumDisplayModes),
        };
        pub const IDirectDraw7_GetDisplayMode: Shim = Shim {
            name: "IDirectDraw7::GetDisplayMode",
            func: crate::shims::Handler::Sync(impls::IDirectDraw7_GetDisplayMode),
        };
        pub const IDirectDraw7_Release: Shim = Shim {
            name: "IDirectDraw7::Release",
            func: crate::shims::Handler::Sync(impls::IDirectDraw7_Release),
        };
        pub const IDirectDraw7_RestoreDisplayMode: Shim = Shim {
            name: "IDirectDraw7::RestoreDisplayMode",
            func: crate::shims::Handler::Sync(impls::IDirectDraw7_RestoreDisplayMode),
        };
        pub const IDirectDraw7_SetCooperativeLevel: Shim = Shim {
            name: "IDirectDraw7::SetCooperativeLevel",
            func: crate::shims::Handler::Sync(impls::IDirectDraw7_SetCooperativeLevel),
        };
        pub const IDirectDraw7_SetDisplayMode: Shim = Shim {
            name: "IDirectDraw7::SetDisplayMode",
            func: crate::shims::Handler::Sync(impls::IDirectDraw7_SetDisplayMode),
        };
        pub const IDirectDraw7_WaitForVerticalBlank: Shim = Shim {
            name: "IDirectDraw7::WaitForVerticalBlank",
            func: crate::shims::Handler::Sync(impls::IDirectDraw7_WaitForVerticalBlank),
        };
        pub const IDirectDrawClipper_Release: Shim = Shim {
            name: "IDirectDrawClipper::Release",
            func: crate::shims::Handler::Sync(impls::IDirectDrawClipper_Release),
        };
        pub const IDirectDrawClipper_SetHWnd: Shim = Shim {
            name: "IDirectDrawClipper::SetHWnd",
            func: crate::shims::Handler::Sync(impls::IDirectDrawClipper_SetHWnd),
        };
        pub const IDirectDrawPalette_Release: Shim = Shim {
            name: "IDirectDrawPalette::Release",
            func: crate::shims::Handler::Sync(impls::IDirectDrawPalette_Release),
        };
        pub const IDirectDrawPalette_SetEntries: Shim = Shim {
            name: "IDirectDrawPalette::SetEntries",
            func: crate::shims::Handler::Sync(impls::IDirectDrawPalette_SetEntries),
        };
        pub const IDirectDrawSurface2_GetAttachedSurface: Shim = Shim {
            name: "IDirectDrawSurface2::GetAttachedSurface",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface2_GetAttachedSurface),
        };
        pub const IDirectDrawSurface2_GetCaps: Shim = Shim {
            name: "IDirectDrawSurface2::GetCaps",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface2_GetCaps),
        };
        pub const IDirectDrawSurface2_GetSurfaceDesc: Shim = Shim {
            name: "IDirectDrawSurface2::GetSurfaceDesc",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface2_GetSurfaceDesc),
        };
        pub const IDirectDrawSurface2_Lock: Shim = Shim {
            name: "IDirectDrawSurface2::Lock",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface2_Lock),
        };
        pub const IDirectDrawSurface2_Release: Shim = Shim {
            name: "IDirectDrawSurface2::Release",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface2_Release),
        };
        pub const IDirectDrawSurface2_Unlock: Shim = Shim {
            name: "IDirectDrawSurface2::Unlock",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface2_Unlock),
        };
        pub const IDirectDrawSurface7_Blt: Shim = Shim {
            name: "IDirectDrawSurface7::Blt",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_Blt),
        };
        pub const IDirectDrawSurface7_BltFast: Shim = Shim {
            name: "IDirectDrawSurface7::BltFast",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_BltFast),
        };
        pub const IDirectDrawSurface7_Flip: Shim = Shim {
            name: "IDirectDrawSurface7::Flip",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_Flip),
        };
        pub const IDirectDrawSurface7_GetAttachedSurface: Shim = Shim {
            name: "IDirectDrawSurface7::GetAttachedSurface",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_GetAttachedSurface),
        };
        pub const IDirectDrawSurface7_GetCaps: Shim = Shim {
            name: "IDirectDrawSurface7::GetCaps",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_GetCaps),
        };
        pub const IDirectDrawSurface7_GetDC: Shim = Shim {
            name: "IDirectDrawSurface7::GetDC",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_GetDC),
        };
        pub const IDirectDrawSurface7_GetPixelFormat: Shim = Shim {
            name: "IDirectDrawSurface7::GetPixelFormat",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_GetPixelFormat),
        };
        pub const IDirectDrawSurface7_GetSurfaceDesc: Shim = Shim {
            name: "IDirectDrawSurface7::GetSurfaceDesc",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_GetSurfaceDesc),
        };
        pub const IDirectDrawSurface7_Lock: Shim = Shim {
            name: "IDirectDrawSurface7::Lock",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_Lock),
        };
        pub const IDirectDrawSurface7_Release: Shim = Shim {
            name: "IDirectDrawSurface7::Release",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_Release),
        };
        pub const IDirectDrawSurface7_ReleaseDC: Shim = Shim {
            name: "IDirectDrawSurface7::ReleaseDC",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_ReleaseDC),
        };
        pub const IDirectDrawSurface7_Restore: Shim = Shim {
            name: "IDirectDrawSurface7::Restore",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_Restore),
        };
        pub const IDirectDrawSurface7_SetClipper: Shim = Shim {
            name: "IDirectDrawSurface7::SetClipper",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_SetClipper),
        };
        pub const IDirectDrawSurface7_SetPalette: Shim = Shim {
            name: "IDirectDrawSurface7::SetPalette",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_SetPalette),
        };
        pub const IDirectDrawSurface7_Unlock: Shim = Shim {
            name: "IDirectDrawSurface7::Unlock",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface7_Unlock),
        };
        pub const IDirectDrawSurface_GetAttachedSurface: Shim = Shim {
            name: "IDirectDrawSurface::GetAttachedSurface",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface_GetAttachedSurface),
        };
        pub const IDirectDrawSurface_GetCaps: Shim = Shim {
            name: "IDirectDrawSurface::GetCaps",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface_GetCaps),
        };
        pub const IDirectDrawSurface_Lock: Shim = Shim {
            name: "IDirectDrawSurface::Lock",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface_Lock),
        };
        pub const IDirectDrawSurface_Release: Shim = Shim {
            name: "IDirectDrawSurface::Release",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface_Release),
        };
        pub const IDirectDrawSurface_Unlock: Shim = Shim {
            name: "IDirectDrawSurface::Unlock",
            func: crate::shims::Handler::Sync(impls::IDirectDrawSurface_Unlock),
        };
        pub const IDirectDraw_CreateSurface: Shim = Shim {
            name: "IDirectDraw::CreateSurface",
            func: crate::shims::Handler::Sync(impls::IDirectDraw_CreateSurface),
        };
        pub const IDirectDraw_EnumDisplayModes: Shim = Shim {
            name: "IDirectDraw::EnumDisplayModes",
            func: crate::shims::Handler::Async(impls::IDirectDraw_EnumDisplayModes),
        };
        pub const IDirectDraw_QueryInterface: Shim = Shim {
            name: "IDirectDraw::QueryInterface",
            func: crate::shims::Handler::Sync(impls::IDirectDraw_QueryInterface),
        };
        pub const IDirectDraw_Release: Shim = Shim {
            name: "IDirectDraw::Release",
            func: crate::shims::Handler::Sync(impls::IDirectDraw_Release),
        };
        pub const IDirectDraw_SetDisplayMode: Shim = Shim {
            name: "IDirectDraw::SetDisplayMode",
            func: crate::shims::Handler::Sync(impls::IDirectDraw_SetDisplayMode),
        };
    }
    const SHIMS: [Shim; 53usize] = [
        shims::DirectDrawCreate,
        shims::DirectDrawCreateClipper,
        shims::DirectDrawCreateEx,
        shims::IDirectDraw2_CreateSurface,
        shims::IDirectDraw2_EnumDisplayModes,
        shims::IDirectDraw2_GetDisplayMode,
        shims::IDirectDraw2_QueryInterface,
        shims::IDirectDraw2_Release,
        shims::IDirectDraw2_SetDisplayMode,
        shims::IDirectDraw7_CreatePalette,
        shims::IDirectDraw7_CreateSurface,
        shims::IDirectDraw7_EnumDisplayModes,
        shims::IDirectDraw7_GetDisplayMode,
        shims::IDirectDraw7_Release,
        shims::IDirectDraw7_RestoreDisplayMode,
        shims::IDirectDraw7_SetCooperativeLevel,
        shims::IDirectDraw7_SetDisplayMode,
        shims::IDirectDraw7_WaitForVerticalBlank,
        shims::IDirectDrawClipper_Release,
        shims::IDirectDrawClipper_SetHWnd,
        shims::IDirectDrawPalette_Release,
        shims::IDirectDrawPalette_SetEntries,
        shims::IDirectDrawSurface2_GetAttachedSurface,
        shims::IDirectDrawSurface2_GetCaps,
        shims::IDirectDrawSurface2_GetSurfaceDesc,
        shims::IDirectDrawSurface2_Lock,
        shims::IDirectDrawSurface2_Release,
        shims::IDirectDrawSurface2_Unlock,
        shims::IDirectDrawSurface7_Blt,
        shims::IDirectDrawSurface7_BltFast,
        shims::IDirectDrawSurface7_Flip,
        shims::IDirectDrawSurface7_GetAttachedSurface,
        shims::IDirectDrawSurface7_GetCaps,
        shims::IDirectDrawSurface7_GetDC,
        shims::IDirectDrawSurface7_GetPixelFormat,
        shims::IDirectDrawSurface7_GetSurfaceDesc,
        shims::IDirectDrawSurface7_Lock,
        shims::IDirectDrawSurface7_Release,
        shims::IDirectDrawSurface7_ReleaseDC,
        shims::IDirectDrawSurface7_Restore,
        shims::IDirectDrawSurface7_SetClipper,
        shims::IDirectDrawSurface7_SetPalette,
        shims::IDirectDrawSurface7_Unlock,
        shims::IDirectDrawSurface_GetAttachedSurface,
        shims::IDirectDrawSurface_GetCaps,
        shims::IDirectDrawSurface_Lock,
        shims::IDirectDrawSurface_Release,
        shims::IDirectDrawSurface_Unlock,
        shims::IDirectDraw_CreateSurface,
        shims::IDirectDraw_EnumDisplayModes,
        shims::IDirectDraw_QueryInterface,
        shims::IDirectDraw_Release,
        shims::IDirectDraw_SetDisplayMode,
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
        pub unsafe fn DirectSoundCreate(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, esp + 4u32);
            let ppDS = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 12u32);
            winapi::dsound::DirectSoundCreate(machine, lpGuid, ppDS, pUnkOuter).to_raw()
        }
        pub unsafe fn DirectSoundEnumerateA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpDSEnumCallback = <u32>::from_stack(mem, esp + 4u32);
            let lpContext = <u32>::from_stack(mem, esp + 8u32);
            winapi::dsound::DirectSoundEnumerateA(machine, lpDSEnumCallback, lpContext).to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_GetCurrentPosition(
            machine: &mut Machine,
            esp: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpdwCurrentPlayCursor = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let lpdwCurrentWriteCursor = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            winapi::dsound::IDirectSoundBuffer::GetCurrentPosition(
                machine,
                this,
                lpdwCurrentPlayCursor,
                lpdwCurrentWriteCursor,
            )
            .to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_GetStatus(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpdwStatus = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::dsound::IDirectSoundBuffer::GetStatus(machine, this, lpdwStatus).to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_Lock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let dwWriteCursor = <u32>::from_stack(mem, esp + 8u32);
            let dwWriteBytes = <u32>::from_stack(mem, esp + 12u32);
            let lplpvAudioPtr1 = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpdwAudioBytes1 = <Option<&mut u32>>::from_stack(mem, esp + 20u32);
            let lplpvAudioPtr2 = <Option<&mut u32>>::from_stack(mem, esp + 24u32);
            let lpdwAudioBytes2 = <Option<&mut u32>>::from_stack(mem, esp + 28u32);
            let dwFlags = <Result<DSBLOCK, u32>>::from_stack(mem, esp + 32u32);
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
        pub unsafe fn IDirectSoundBuffer_Play(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let dwReserved1 = <u32>::from_stack(mem, esp + 8u32);
            let dwReserved2 = <u32>::from_stack(mem, esp + 12u32);
            let dwFlags = <u32>::from_stack(mem, esp + 16u32);
            winapi::dsound::IDirectSoundBuffer::Play(
                machine,
                this,
                dwReserved1,
                dwReserved2,
                dwFlags,
            )
            .to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::dsound::IDirectSoundBuffer::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_SetFormat(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpcfxFormat = <Option<&WAVEFORMATEX>>::from_stack(mem, esp + 8u32);
            winapi::dsound::IDirectSoundBuffer::SetFormat(machine, this, lpcfxFormat).to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_Unlock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpvAudioPtr1 = <u32>::from_stack(mem, esp + 8u32);
            let dwAudioBytes1 = <u32>::from_stack(mem, esp + 12u32);
            let lpvAudioPtr2 = <u32>::from_stack(mem, esp + 16u32);
            let dwAudioBytes2 = <u32>::from_stack(mem, esp + 20u32);
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
        pub unsafe fn IDirectSound_CreateSoundBuffer(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpcDSBufferDesc = <Option<&DSBUFFERDESC>>::from_stack(mem, esp + 8u32);
            let lplpDirectSoundBuffer = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 16u32);
            winapi::dsound::IDirectSound::CreateSoundBuffer(
                machine,
                this,
                lpcDSBufferDesc,
                lplpDirectSoundBuffer,
                pUnkOuter,
            )
            .to_raw()
        }
        pub unsafe fn IDirectSound_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::dsound::IDirectSound::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectSound_SetCooperativeLevel(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let hwnd = <u32>::from_stack(mem, esp + 8u32);
            let dwLevel = <u32>::from_stack(mem, esp + 12u32);
            winapi::dsound::IDirectSound::SetCooperativeLevel(machine, this, hwnd, dwLevel).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const DirectSoundCreate: Shim = Shim {
            name: "DirectSoundCreate",
            func: crate::shims::Handler::Sync(impls::DirectSoundCreate),
        };
        pub const DirectSoundEnumerateA: Shim = Shim {
            name: "DirectSoundEnumerateA",
            func: crate::shims::Handler::Sync(impls::DirectSoundEnumerateA),
        };
        pub const IDirectSoundBuffer_GetCurrentPosition: Shim = Shim {
            name: "IDirectSoundBuffer::GetCurrentPosition",
            func: crate::shims::Handler::Sync(impls::IDirectSoundBuffer_GetCurrentPosition),
        };
        pub const IDirectSoundBuffer_GetStatus: Shim = Shim {
            name: "IDirectSoundBuffer::GetStatus",
            func: crate::shims::Handler::Sync(impls::IDirectSoundBuffer_GetStatus),
        };
        pub const IDirectSoundBuffer_Lock: Shim = Shim {
            name: "IDirectSoundBuffer::Lock",
            func: crate::shims::Handler::Sync(impls::IDirectSoundBuffer_Lock),
        };
        pub const IDirectSoundBuffer_Play: Shim = Shim {
            name: "IDirectSoundBuffer::Play",
            func: crate::shims::Handler::Sync(impls::IDirectSoundBuffer_Play),
        };
        pub const IDirectSoundBuffer_Release: Shim = Shim {
            name: "IDirectSoundBuffer::Release",
            func: crate::shims::Handler::Sync(impls::IDirectSoundBuffer_Release),
        };
        pub const IDirectSoundBuffer_SetFormat: Shim = Shim {
            name: "IDirectSoundBuffer::SetFormat",
            func: crate::shims::Handler::Sync(impls::IDirectSoundBuffer_SetFormat),
        };
        pub const IDirectSoundBuffer_Unlock: Shim = Shim {
            name: "IDirectSoundBuffer::Unlock",
            func: crate::shims::Handler::Sync(impls::IDirectSoundBuffer_Unlock),
        };
        pub const IDirectSound_CreateSoundBuffer: Shim = Shim {
            name: "IDirectSound::CreateSoundBuffer",
            func: crate::shims::Handler::Sync(impls::IDirectSound_CreateSoundBuffer),
        };
        pub const IDirectSound_Release: Shim = Shim {
            name: "IDirectSound::Release",
            func: crate::shims::Handler::Sync(impls::IDirectSound_Release),
        };
        pub const IDirectSound_SetCooperativeLevel: Shim = Shim {
            name: "IDirectSound::SetCooperativeLevel",
            func: crate::shims::Handler::Sync(impls::IDirectSound_SetCooperativeLevel),
        };
    }
    const SHIMS: [Shim; 12usize] = [
        shims::DirectSoundCreate,
        shims::DirectSoundEnumerateA,
        shims::IDirectSoundBuffer_GetCurrentPosition,
        shims::IDirectSoundBuffer_GetStatus,
        shims::IDirectSoundBuffer_Lock,
        shims::IDirectSoundBuffer_Play,
        shims::IDirectSoundBuffer_Release,
        shims::IDirectSoundBuffer_SetFormat,
        shims::IDirectSoundBuffer_Unlock,
        shims::IDirectSound_CreateSoundBuffer,
        shims::IDirectSound_Release,
        shims::IDirectSound_SetCooperativeLevel,
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
        pub unsafe fn BitBlt(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <i32>::from_stack(mem, esp + 8u32);
            let y = <i32>::from_stack(mem, esp + 12u32);
            let cx = <u32>::from_stack(mem, esp + 16u32);
            let cy = <u32>::from_stack(mem, esp + 20u32);
            let hdcSrc = <HDC>::from_stack(mem, esp + 24u32);
            let x1 = <i32>::from_stack(mem, esp + 28u32);
            let y1 = <i32>::from_stack(mem, esp + 32u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, esp + 36u32);
            winapi::gdi32::BitBlt(machine, hdc, x, y, cx, cy, hdcSrc, x1, y1, rop).to_raw()
        }
        pub unsafe fn CreateBitmap(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nWidth = <u32>::from_stack(mem, esp + 4u32);
            let nHeight = <u32>::from_stack(mem, esp + 8u32);
            let nPlanes = <u32>::from_stack(mem, esp + 12u32);
            let nBitCount = <u32>::from_stack(mem, esp + 16u32);
            let lpBits = <u32>::from_stack(mem, esp + 20u32);
            winapi::gdi32::CreateBitmap(machine, nWidth, nHeight, nPlanes, nBitCount, lpBits)
                .to_raw()
        }
        pub unsafe fn CreateCompatibleBitmap(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let cx = <u32>::from_stack(mem, esp + 8u32);
            let cy = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::CreateCompatibleBitmap(machine, hdc, cx, cy).to_raw()
        }
        pub unsafe fn CreateCompatibleDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            winapi::gdi32::CreateCompatibleDC(machine, hdc).to_raw()
        }
        pub unsafe fn CreateDIBSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let pbmi = <Option<&BITMAPINFOHEADER>>::from_stack(mem, esp + 8u32);
            let usage = <u32>::from_stack(mem, esp + 12u32);
            let ppvBits = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let hSection = <u32>::from_stack(mem, esp + 20u32);
            let offset = <u32>::from_stack(mem, esp + 24u32);
            winapi::gdi32::CreateDIBSection(machine, hdc, pbmi, usage, ppvBits, hSection, offset)
                .to_raw()
        }
        pub unsafe fn CreateFontA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let cHeight = <i32>::from_stack(mem, esp + 4u32);
            let cWidth = <i32>::from_stack(mem, esp + 8u32);
            let cEscapement = <i32>::from_stack(mem, esp + 12u32);
            let cOrientation = <i32>::from_stack(mem, esp + 16u32);
            let cWeight = <u32>::from_stack(mem, esp + 20u32);
            let bItalic = <u32>::from_stack(mem, esp + 24u32);
            let bUnderline = <u32>::from_stack(mem, esp + 28u32);
            let bStrikeOut = <u32>::from_stack(mem, esp + 32u32);
            let iCharSet = <u32>::from_stack(mem, esp + 36u32);
            let iOutPrecision = <u32>::from_stack(mem, esp + 40u32);
            let iClipPrecision = <u32>::from_stack(mem, esp + 44u32);
            let iQuality = <u32>::from_stack(mem, esp + 48u32);
            let iPitchAndFamily = <u32>::from_stack(mem, esp + 52u32);
            let pszFaceName = <Option<&str>>::from_stack(mem, esp + 56u32);
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
        pub unsafe fn CreatePen(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let iStyle = <Result<PS, u32>>::from_stack(mem, esp + 4u32);
            let cWidth = <u32>::from_stack(mem, esp + 8u32);
            let color = <COLORREF>::from_stack(mem, esp + 12u32);
            winapi::gdi32::CreatePen(machine, iStyle, cWidth, color).to_raw()
        }
        pub unsafe fn CreateSolidBrush(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let color = <COLORREF>::from_stack(mem, esp + 4u32);
            winapi::gdi32::CreateSolidBrush(machine, color).to_raw()
        }
        pub unsafe fn DeleteDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <u32>::from_stack(mem, esp + 4u32);
            winapi::gdi32::DeleteDC(machine, hdc).to_raw()
        }
        pub unsafe fn DeleteObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let handle = <HGDIOBJ>::from_stack(mem, esp + 4u32);
            winapi::gdi32::DeleteObject(machine, handle).to_raw()
        }
        pub unsafe fn GetDCOrgEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lpPoint = <Option<&mut POINT>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::GetDCOrgEx(machine, hdc, lpPoint).to_raw()
        }
        pub unsafe fn GetDeviceCaps(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let index = <Result<GetDeviceCapsArg, u32>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::GetDeviceCaps(machine, hdc, index).to_raw()
        }
        pub unsafe fn GetLayout(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            winapi::gdi32::GetLayout(machine, hdc).to_raw()
        }
        pub unsafe fn GetObjectA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let handle = <HGDIOBJ>::from_stack(mem, esp + 4u32);
            let bytes = <u32>::from_stack(mem, esp + 8u32);
            let out = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::GetObjectA(machine, handle, bytes, out).to_raw()
        }
        pub unsafe fn GetPixel(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::GetPixel(machine, hdc, x, y).to_raw()
        }
        pub unsafe fn GetStockObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let i = <Result<GetStockObjectArg, u32>>::from_stack(mem, esp + 4u32);
            winapi::gdi32::GetStockObject(machine, i).to_raw()
        }
        pub unsafe fn GetTextExtentPoint32A(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, esp + 8u32);
            let c = <i32>::from_stack(mem, esp + 12u32);
            let psizl = <Option<&mut SIZE>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::GetTextExtentPoint32A(machine, hdc, lpString, c, psizl).to_raw()
        }
        pub unsafe fn GetTextExtentPoint32W(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, esp + 8u32);
            let c = <i32>::from_stack(mem, esp + 12u32);
            let psizl = <Option<&mut SIZE>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::GetTextExtentPoint32W(machine, hdc, lpString, c, psizl).to_raw()
        }
        pub unsafe fn GetTextMetricsA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lptm = <Option<&mut TEXTMETRICA>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::GetTextMetricsA(machine, hdc, lptm).to_raw()
        }
        pub unsafe fn GetTextMetricsW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lptm = <Option<&mut TEXTMETRICW>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::GetTextMetricsW(machine, hdc, lptm).to_raw()
        }
        pub unsafe fn LineDDA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let xStart = <i32>::from_stack(mem, esp + 4u32);
            let yStart = <i32>::from_stack(mem, esp + 8u32);
            let xEnd = <i32>::from_stack(mem, esp + 12u32);
            let yEnd = <i32>::from_stack(mem, esp + 16u32);
            let lpProc = <u32>::from_stack(mem, esp + 20u32);
            let data = <u32>::from_stack(mem, esp + 24u32);
            winapi::gdi32::LineDDA(machine, xStart, yStart, xEnd, yEnd, lpProc, data).to_raw()
        }
        pub unsafe fn LineTo(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::LineTo(machine, hdc, x, y).to_raw()
        }
        pub unsafe fn MoveToEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let lppt = <Option<&mut POINT>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::MoveToEx(machine, hdc, x, y, lppt).to_raw()
        }
        pub unsafe fn PatBlt(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <i32>::from_stack(mem, esp + 8u32);
            let y = <i32>::from_stack(mem, esp + 12u32);
            let w = <i32>::from_stack(mem, esp + 16u32);
            let h = <i32>::from_stack(mem, esp + 20u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, esp + 24u32);
            winapi::gdi32::PatBlt(machine, hdc, x, y, w, h, rop).to_raw()
        }
        pub unsafe fn PtVisible(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <i32>::from_stack(mem, esp + 8u32);
            let y = <i32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::PtVisible(machine, hdc, x, y).to_raw()
        }
        pub unsafe fn SelectObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let hGdiObj = <HGDIOBJ>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SelectObject(machine, hdc, hGdiObj).to_raw()
        }
        pub unsafe fn SetBkColor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let color = <COLORREF>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetBkColor(machine, hdc, color).to_raw()
        }
        pub unsafe fn SetBkMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let mode = <i32>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetBkMode(machine, hdc, mode).to_raw()
        }
        pub unsafe fn SetBrushOrgEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <i32>::from_stack(mem, esp + 8u32);
            let y = <i32>::from_stack(mem, esp + 12u32);
            let lppt = <Option<&mut POINT>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::SetBrushOrgEx(machine, hdc, x, y, lppt).to_raw()
        }
        pub unsafe fn SetDIBitsToDevice(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let xDest = <u32>::from_stack(mem, esp + 8u32);
            let yDest = <u32>::from_stack(mem, esp + 12u32);
            let w = <u32>::from_stack(mem, esp + 16u32);
            let h = <u32>::from_stack(mem, esp + 20u32);
            let xSrc = <u32>::from_stack(mem, esp + 24u32);
            let ySrc = <u32>::from_stack(mem, esp + 28u32);
            let StartScan = <u32>::from_stack(mem, esp + 32u32);
            let cLines = <u32>::from_stack(mem, esp + 36u32);
            let lpvBits = <u32>::from_stack(mem, esp + 40u32);
            let lpbmi = <u32>::from_stack(mem, esp + 44u32);
            let ColorUse = <u32>::from_stack(mem, esp + 48u32);
            winapi::gdi32::SetDIBitsToDevice(
                machine, hdc, xDest, yDest, w, h, xSrc, ySrc, StartScan, cLines, lpvBits, lpbmi,
                ColorUse,
            )
            .to_raw()
        }
        pub unsafe fn SetPixel(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let color = <COLORREF>::from_stack(mem, esp + 16u32);
            winapi::gdi32::SetPixel(machine, hdc, x, y, color).to_raw()
        }
        pub unsafe fn SetROP2(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let rop2 = <Result<R2, u32>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetROP2(machine, hdc, rop2).to_raw()
        }
        pub unsafe fn SetTextColor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let color = <COLORREF>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetTextColor(machine, hdc, color).to_raw()
        }
        pub unsafe fn StretchBlt(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdcDest = <HDC>::from_stack(mem, esp + 4u32);
            let xDest = <i32>::from_stack(mem, esp + 8u32);
            let yDest = <i32>::from_stack(mem, esp + 12u32);
            let wDest = <u32>::from_stack(mem, esp + 16u32);
            let hDest = <u32>::from_stack(mem, esp + 20u32);
            let hdcSrc = <HDC>::from_stack(mem, esp + 24u32);
            let xSrc = <i32>::from_stack(mem, esp + 28u32);
            let ySrc = <i32>::from_stack(mem, esp + 32u32);
            let wSrc = <u32>::from_stack(mem, esp + 36u32);
            let hSrc = <u32>::from_stack(mem, esp + 40u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, esp + 44u32);
            winapi::gdi32::StretchBlt(
                machine, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
            )
            .to_raw()
        }
        pub unsafe fn StretchDIBits(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let xDest = <u32>::from_stack(mem, esp + 8u32);
            let yDest = <u32>::from_stack(mem, esp + 12u32);
            let DestWidth = <u32>::from_stack(mem, esp + 16u32);
            let DestHeight = <u32>::from_stack(mem, esp + 20u32);
            let xSrc = <u32>::from_stack(mem, esp + 24u32);
            let ySrc = <u32>::from_stack(mem, esp + 28u32);
            let SrcWidth = <u32>::from_stack(mem, esp + 32u32);
            let SrcHeight = <u32>::from_stack(mem, esp + 36u32);
            let lpBits = <u32>::from_stack(mem, esp + 40u32);
            let lpbmi = <u32>::from_stack(mem, esp + 44u32);
            let iUsage = <u32>::from_stack(mem, esp + 48u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, esp + 52u32);
            winapi::gdi32::StretchDIBits(
                machine, hdc, xDest, yDest, DestWidth, DestHeight, xSrc, ySrc, SrcWidth, SrcHeight,
                lpBits, lpbmi, iUsage, rop,
            )
            .to_raw()
        }
        pub unsafe fn TextOutA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let lpString = <ArrayWithSize<u8>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::TextOutA(machine, hdc, x, y, lpString).to_raw()
        }
        pub unsafe fn TextOutW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let lpString = <ArrayWithSize<u16>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::TextOutW(machine, hdc, x, y, lpString).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const BitBlt: Shim = Shim {
            name: "BitBlt",
            func: crate::shims::Handler::Sync(impls::BitBlt),
        };
        pub const CreateBitmap: Shim = Shim {
            name: "CreateBitmap",
            func: crate::shims::Handler::Sync(impls::CreateBitmap),
        };
        pub const CreateCompatibleBitmap: Shim = Shim {
            name: "CreateCompatibleBitmap",
            func: crate::shims::Handler::Sync(impls::CreateCompatibleBitmap),
        };
        pub const CreateCompatibleDC: Shim = Shim {
            name: "CreateCompatibleDC",
            func: crate::shims::Handler::Sync(impls::CreateCompatibleDC),
        };
        pub const CreateDIBSection: Shim = Shim {
            name: "CreateDIBSection",
            func: crate::shims::Handler::Sync(impls::CreateDIBSection),
        };
        pub const CreateFontA: Shim = Shim {
            name: "CreateFontA",
            func: crate::shims::Handler::Sync(impls::CreateFontA),
        };
        pub const CreatePen: Shim = Shim {
            name: "CreatePen",
            func: crate::shims::Handler::Sync(impls::CreatePen),
        };
        pub const CreateSolidBrush: Shim = Shim {
            name: "CreateSolidBrush",
            func: crate::shims::Handler::Sync(impls::CreateSolidBrush),
        };
        pub const DeleteDC: Shim = Shim {
            name: "DeleteDC",
            func: crate::shims::Handler::Sync(impls::DeleteDC),
        };
        pub const DeleteObject: Shim = Shim {
            name: "DeleteObject",
            func: crate::shims::Handler::Sync(impls::DeleteObject),
        };
        pub const GetDCOrgEx: Shim = Shim {
            name: "GetDCOrgEx",
            func: crate::shims::Handler::Sync(impls::GetDCOrgEx),
        };
        pub const GetDeviceCaps: Shim = Shim {
            name: "GetDeviceCaps",
            func: crate::shims::Handler::Sync(impls::GetDeviceCaps),
        };
        pub const GetLayout: Shim = Shim {
            name: "GetLayout",
            func: crate::shims::Handler::Sync(impls::GetLayout),
        };
        pub const GetObjectA: Shim = Shim {
            name: "GetObjectA",
            func: crate::shims::Handler::Sync(impls::GetObjectA),
        };
        pub const GetPixel: Shim = Shim {
            name: "GetPixel",
            func: crate::shims::Handler::Sync(impls::GetPixel),
        };
        pub const GetStockObject: Shim = Shim {
            name: "GetStockObject",
            func: crate::shims::Handler::Sync(impls::GetStockObject),
        };
        pub const GetTextExtentPoint32A: Shim = Shim {
            name: "GetTextExtentPoint32A",
            func: crate::shims::Handler::Sync(impls::GetTextExtentPoint32A),
        };
        pub const GetTextExtentPoint32W: Shim = Shim {
            name: "GetTextExtentPoint32W",
            func: crate::shims::Handler::Sync(impls::GetTextExtentPoint32W),
        };
        pub const GetTextMetricsA: Shim = Shim {
            name: "GetTextMetricsA",
            func: crate::shims::Handler::Sync(impls::GetTextMetricsA),
        };
        pub const GetTextMetricsW: Shim = Shim {
            name: "GetTextMetricsW",
            func: crate::shims::Handler::Sync(impls::GetTextMetricsW),
        };
        pub const LineDDA: Shim = Shim {
            name: "LineDDA",
            func: crate::shims::Handler::Sync(impls::LineDDA),
        };
        pub const LineTo: Shim = Shim {
            name: "LineTo",
            func: crate::shims::Handler::Sync(impls::LineTo),
        };
        pub const MoveToEx: Shim = Shim {
            name: "MoveToEx",
            func: crate::shims::Handler::Sync(impls::MoveToEx),
        };
        pub const PatBlt: Shim = Shim {
            name: "PatBlt",
            func: crate::shims::Handler::Sync(impls::PatBlt),
        };
        pub const PtVisible: Shim = Shim {
            name: "PtVisible",
            func: crate::shims::Handler::Sync(impls::PtVisible),
        };
        pub const SelectObject: Shim = Shim {
            name: "SelectObject",
            func: crate::shims::Handler::Sync(impls::SelectObject),
        };
        pub const SetBkColor: Shim = Shim {
            name: "SetBkColor",
            func: crate::shims::Handler::Sync(impls::SetBkColor),
        };
        pub const SetBkMode: Shim = Shim {
            name: "SetBkMode",
            func: crate::shims::Handler::Sync(impls::SetBkMode),
        };
        pub const SetBrushOrgEx: Shim = Shim {
            name: "SetBrushOrgEx",
            func: crate::shims::Handler::Sync(impls::SetBrushOrgEx),
        };
        pub const SetDIBitsToDevice: Shim = Shim {
            name: "SetDIBitsToDevice",
            func: crate::shims::Handler::Sync(impls::SetDIBitsToDevice),
        };
        pub const SetPixel: Shim = Shim {
            name: "SetPixel",
            func: crate::shims::Handler::Sync(impls::SetPixel),
        };
        pub const SetROP2: Shim = Shim {
            name: "SetROP2",
            func: crate::shims::Handler::Sync(impls::SetROP2),
        };
        pub const SetTextColor: Shim = Shim {
            name: "SetTextColor",
            func: crate::shims::Handler::Sync(impls::SetTextColor),
        };
        pub const StretchBlt: Shim = Shim {
            name: "StretchBlt",
            func: crate::shims::Handler::Sync(impls::StretchBlt),
        };
        pub const StretchDIBits: Shim = Shim {
            name: "StretchDIBits",
            func: crate::shims::Handler::Sync(impls::StretchDIBits),
        };
        pub const TextOutA: Shim = Shim {
            name: "TextOutA",
            func: crate::shims::Handler::Sync(impls::TextOutA),
        };
        pub const TextOutW: Shim = Shim {
            name: "TextOutW",
            func: crate::shims::Handler::Sync(impls::TextOutW),
        };
    }
    const SHIMS: [Shim; 37usize] = [
        shims::BitBlt,
        shims::CreateBitmap,
        shims::CreateCompatibleBitmap,
        shims::CreateCompatibleDC,
        shims::CreateDIBSection,
        shims::CreateFontA,
        shims::CreatePen,
        shims::CreateSolidBrush,
        shims::DeleteDC,
        shims::DeleteObject,
        shims::GetDCOrgEx,
        shims::GetDeviceCaps,
        shims::GetLayout,
        shims::GetObjectA,
        shims::GetPixel,
        shims::GetStockObject,
        shims::GetTextExtentPoint32A,
        shims::GetTextExtentPoint32W,
        shims::GetTextMetricsA,
        shims::GetTextMetricsW,
        shims::LineDDA,
        shims::LineTo,
        shims::MoveToEx,
        shims::PatBlt,
        shims::PtVisible,
        shims::SelectObject,
        shims::SetBkColor,
        shims::SetBkMode,
        shims::SetBrushOrgEx,
        shims::SetDIBitsToDevice,
        shims::SetPixel,
        shims::SetROP2,
        shims::SetTextColor,
        shims::StretchBlt,
        shims::StretchDIBits,
        shims::TextOutA,
        shims::TextOutW,
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
        pub unsafe fn AcquireSRWLockExclusive(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::AcquireSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn AcquireSRWLockShared(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::AcquireSRWLockShared(machine, SRWLock).to_raw()
        }
        pub unsafe fn AddVectoredExceptionHandler(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let first = <u32>::from_stack(mem, esp + 4u32);
            let handler = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::AddVectoredExceptionHandler(machine, first, handler).to_raw()
        }
        pub unsafe fn CloseHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hObject = <HFILE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::CloseHandle(machine, hObject).to_raw()
        }
        pub unsafe fn CreateDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::CreateDirectoryA(machine, lpPathName, lpSecurityAttributes).to_raw()
        }
        pub unsafe fn CreateEventA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpEventAttributes = <u32>::from_stack(mem, esp + 4u32);
            let bManualReset = <bool>::from_stack(mem, esp + 8u32);
            let bInitialState = <bool>::from_stack(mem, esp + 12u32);
            let lpName = <Option<&str>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::CreateEventA(
                machine,
                lpEventAttributes,
                bManualReset,
                bInitialState,
                lpName,
            )
            .to_raw()
        }
        pub unsafe fn CreateFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let dwDesiredAccess = <u32>::from_stack(mem, esp + 8u32);
            let dwShareMode = <u32>::from_stack(mem, esp + 12u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 16u32);
            let dwCreationDisposition =
                <Result<CreationDisposition, u32>>::from_stack(mem, esp + 20u32);
            let dwFlagsAndAttributes = <Result<FileAttribute, u32>>::from_stack(mem, esp + 24u32);
            let hTemplateFile = <HFILE>::from_stack(mem, esp + 28u32);
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
        pub unsafe fn CreateFileW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let dwDesiredAccess = <u32>::from_stack(mem, esp + 8u32);
            let dwShareMode = <u32>::from_stack(mem, esp + 12u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 16u32);
            let dwCreationDisposition =
                <Result<CreationDisposition, u32>>::from_stack(mem, esp + 20u32);
            let dwFlagsAndAttributes = <Result<FileAttribute, u32>>::from_stack(mem, esp + 24u32);
            let hTemplateFile = <HFILE>::from_stack(mem, esp + 28u32);
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
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let lpThreadAttributes = <u32>::from_stack(mem, esp + 4u32);
            let dwStackSize = <u32>::from_stack(mem, esp + 8u32);
            let lpStartAddress = <u32>::from_stack(mem, esp + 12u32);
            let lpParameter = <u32>::from_stack(mem, esp + 16u32);
            let dwCreationFlags = <u32>::from_stack(mem, esp + 20u32);
            let lpThreadId = <u32>::from_stack(mem, esp + 24u32);
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
        pub unsafe fn DebugBreak(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::DebugBreak(machine).to_raw()
        }
        pub unsafe fn DeleteCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::DeleteCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn DeleteFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::DeleteFileA(machine, lpFileName).to_raw()
        }
        pub unsafe fn DisableThreadLibraryCalls(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hLibModule = <HMODULE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::DisableThreadLibraryCalls(machine, hLibModule).to_raw()
        }
        pub unsafe fn EnterCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::EnterCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn ExitProcess(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uExitCode = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::ExitProcess(machine, uExitCode).to_raw()
        }
        pub unsafe fn FileTimeToSystemTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileTime = <Option<&FILETIME>>::from_stack(mem, esp + 4u32);
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::FileTimeToSystemTime(machine, lpFileTime, lpSystemTime).to_raw()
        }
        pub unsafe fn FindClose(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFindFile = <HFIND>::from_stack(mem, esp + 4u32);
            winapi::kernel32::FindClose(machine, hFindFile).to_raw()
        }
        pub unsafe fn FindFirstFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpFindFileData = <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::FindFirstFileA(machine, lpFileName, lpFindFileData).to_raw()
        }
        pub unsafe fn FindNextFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFindFile = <HFIND>::from_stack(mem, esp + 4u32);
            let lpFindFileData = <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::FindNextFileA(machine, hFindFile, lpFindFileData).to_raw()
        }
        pub unsafe fn FindResourceA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let lpName = <ResourceKey<&str>>::from_stack(mem, esp + 8u32);
            let lpType = <ResourceKey<&str>>::from_stack(mem, esp + 12u32);
            winapi::kernel32::FindResourceA(machine, hModule, lpName, lpType).to_raw()
        }
        pub unsafe fn FindResourceW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let lpName = <ResourceKey<&Str16>>::from_stack(mem, esp + 8u32);
            let lpType = <ResourceKey<&Str16>>::from_stack(mem, esp + 12u32);
            winapi::kernel32::FindResourceW(machine, hModule, lpName, lpType).to_raw()
        }
        pub unsafe fn FlushFileBuffers(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::FlushFileBuffers(machine, hFile).to_raw()
        }
        pub unsafe fn FormatMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, esp + 4u32);
            let lpSource = <u32>::from_stack(mem, esp + 8u32);
            let dwMessageId = <u32>::from_stack(mem, esp + 12u32);
            let dwLanguageId = <u32>::from_stack(mem, esp + 16u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 20u32);
            let nSize = <u32>::from_stack(mem, esp + 24u32);
            let args = <u32>::from_stack(mem, esp + 28u32);
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
        pub unsafe fn FormatMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <Result<FormatMessageFlags, u32>>::from_stack(mem, esp + 4u32);
            let lpSource = <u32>::from_stack(mem, esp + 8u32);
            let dwMessageId = <u32>::from_stack(mem, esp + 12u32);
            let dwLanguageId = <u32>::from_stack(mem, esp + 16u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 20u32);
            let nSize = <u32>::from_stack(mem, esp + 24u32);
            let args = <u32>::from_stack(mem, esp + 28u32);
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
        pub unsafe fn FreeEnvironmentStringsA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _penv = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::FreeEnvironmentStringsA(machine, _penv).to_raw()
        }
        pub unsafe fn FreeEnvironmentStringsW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::FreeEnvironmentStringsW(machine).to_raw()
        }
        pub unsafe fn FreeLibrary(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hLibModule = <HMODULE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::FreeLibrary(machine, hLibModule).to_raw()
        }
        pub unsafe fn GetACP(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetACP(machine).to_raw()
        }
        pub unsafe fn GetCPInfo(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _CodePage = <u32>::from_stack(mem, esp + 4u32);
            let _lpCPInfo = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo).to_raw()
        }
        pub unsafe fn GetCommandLineA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCommandLineA(machine).to_raw()
        }
        pub unsafe fn GetCommandLineW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCommandLineW(machine).to_raw()
        }
        pub unsafe fn GetConsoleMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleHandle = <HFILE>::from_stack(mem, esp + 4u32);
            let lpMode = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetConsoleMode(machine, hConsoleHandle, lpMode).to_raw()
        }
        pub unsafe fn GetConsoleScreenBufferInfo(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _hConsoleOutput = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            let lpConsoleScreenBufferInfo =
                <Option<&mut CONSOLE_SCREEN_BUFFER_INFO>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetConsoleScreenBufferInfo(
                machine,
                _hConsoleOutput,
                lpConsoleScreenBufferInfo,
            )
            .to_raw()
        }
        pub unsafe fn GetCurrentDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nBufferLength = <u32>::from_stack(mem, esp + 4u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetCurrentDirectoryA(machine, nBufferLength, lpBuffer).to_raw()
        }
        pub unsafe fn GetCurrentProcess(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentProcess(machine).to_raw()
        }
        pub unsafe fn GetCurrentProcessId(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentProcessId(machine).to_raw()
        }
        pub unsafe fn GetCurrentThread(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentThread(machine).to_raw()
        }
        pub unsafe fn GetCurrentThreadId(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentThreadId(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentStrings(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetEnvironmentStrings(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentStringsW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetEnvironmentStringsW(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentVariableA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&str>>::from_stack(mem, esp + 4u32);
            let buf = <ArrayWithSize<u8>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetEnvironmentVariableA(machine, name, buf).to_raw()
        }
        pub unsafe fn GetEnvironmentVariableW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let buf = <ArrayWithSize<u16>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetEnvironmentVariableW(machine, name, buf).to_raw()
        }
        pub unsafe fn GetFileAttributesA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetFileAttributesA(machine, lpFileName).to_raw()
        }
        pub unsafe fn GetFileInformationByHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpFileInformation =
                <Option<&mut BY_HANDLE_FILE_INFORMATION>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetFileInformationByHandle(machine, hFile, lpFileInformation).to_raw()
        }
        pub unsafe fn GetFileSize(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpFileSizeHigh = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetFileSize(machine, hFile, lpFileSizeHigh).to_raw()
        }
        pub unsafe fn GetFileTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpCreationTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 8u32);
            let lpLastAccessTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 12u32);
            let lpLastWriteTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetFileTime(
                machine,
                hFile,
                lpCreationTime,
                lpLastAccessTime,
                lpLastWriteTime,
            )
            .to_raw()
        }
        pub unsafe fn GetFileType(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetFileType(machine, hFile).to_raw()
        }
        pub unsafe fn GetFullPathNameA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let nBufferLength = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let lpFilePart = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetFullPathNameA(
                machine,
                lpFileName,
                nBufferLength,
                lpBuffer,
                lpFilePart,
            )
            .to_raw()
        }
        pub unsafe fn GetFullPathNameW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let nBufferLength = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let lpFilePart = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetFullPathNameW(
                machine,
                lpFileName,
                nBufferLength,
                lpBuffer,
                lpFilePart,
            )
            .to_raw()
        }
        pub unsafe fn GetLastError(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetLastError(machine).to_raw()
        }
        pub unsafe fn GetLocalTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetLocalTime(machine, lpSystemTime).to_raw()
        }
        pub unsafe fn GetModuleFileNameA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let filename = <ArrayWithSizeMut<u8>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetModuleFileNameA(machine, hModule, filename).to_raw()
        }
        pub unsafe fn GetModuleFileNameW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let _lpFilename = <u32>::from_stack(mem, esp + 8u32);
            let _nSize = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize).to_raw()
        }
        pub unsafe fn GetModuleHandleA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpModuleName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetModuleHandleA(machine, lpModuleName).to_raw()
        }
        pub unsafe fn GetModuleHandleExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, esp + 4u32);
            let lpModuleName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let hModule = <Option<&mut HMODULE>>::from_stack(mem, esp + 12u32);
            winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule).to_raw()
        }
        pub unsafe fn GetModuleHandleW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpModuleName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetModuleHandleW(machine, lpModuleName).to_raw()
        }
        pub unsafe fn GetOEMCP(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetOEMCP(machine).to_raw()
        }
        pub unsafe fn GetPrivateProfileIntW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let nDefault = <u32>::from_stack(mem, esp + 12u32);
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetPrivateProfileIntW(
                machine, lpAppName, lpKeyName, nDefault, lpFileName,
            )
            .to_raw()
        }
        pub unsafe fn GetPrivateProfileStringW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpDefault = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let lpReturnedString = <ArrayWithSizeMut<u16>>::from_stack(mem, esp + 16u32);
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 24u32);
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
        pub unsafe fn GetProcAddress(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let lpProcName = <GetProcAddressArg>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetProcAddress(machine, hModule, lpProcName).to_raw()
        }
        pub unsafe fn GetProcessHeap(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetProcessHeap(machine).to_raw()
        }
        pub unsafe fn GetProfileIntW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let nDefault = <i32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::GetProfileIntW(machine, lpAppName, lpKeyName, nDefault).to_raw()
        }
        pub unsafe fn GetProfileStringW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpDefault = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let lpReturnedString = <ArrayWithSizeMut<u16>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetProfileStringW(
                machine,
                lpAppName,
                lpKeyName,
                lpDefault,
                lpReturnedString,
            )
            .to_raw()
        }
        pub unsafe fn GetStartupInfoA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo).to_raw()
        }
        pub unsafe fn GetStartupInfoW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo).to_raw()
        }
        pub unsafe fn GetStdHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nStdHandle = <Result<STD, u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetStdHandle(machine, nStdHandle).to_raw()
        }
        pub unsafe fn GetStringTypeA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let Locale = <LCID>::from_stack(mem, esp + 4u32);
            let dwInfoType = <u32>::from_stack(mem, esp + 8u32);
            let lpSrcStr = <u32>::from_stack(mem, esp + 12u32);
            let cchSrc = <i32>::from_stack(mem, esp + 16u32);
            let lpCharType = <Option<&mut u32>>::from_stack(mem, esp + 20u32);
            winapi::kernel32::GetStringTypeA(
                machine, Locale, dwInfoType, lpSrcStr, cchSrc, lpCharType,
            )
            .to_raw()
        }
        pub unsafe fn GetStringTypeW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwInfoType = <u32>::from_stack(mem, esp + 4u32);
            let lpSrcStr = <u32>::from_stack(mem, esp + 8u32);
            let cchSrc = <i32>::from_stack(mem, esp + 12u32);
            let lpCharType = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetStringTypeW(machine, dwInfoType, lpSrcStr, cchSrc, lpCharType)
                .to_raw()
        }
        pub unsafe fn GetSystemDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpBuffer = <u32>::from_stack(mem, esp + 4u32);
            let uSize = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetSystemDirectoryA(machine, lpBuffer, uSize).to_raw()
        }
        pub unsafe fn GetSystemTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetSystemTime(machine, lpSystemTime).to_raw()
        }
        pub unsafe fn GetSystemTimeAsFileTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTimeAsFileTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetSystemTimeAsFileTime(machine, lpSystemTimeAsFileTime).to_raw()
        }
        pub unsafe fn GetTickCount(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetTickCount(machine).to_raw()
        }
        pub unsafe fn GetTimeZoneInformation(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpTimeZoneInformation =
                <Option<&mut TIME_ZONE_INFORMATION>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetTimeZoneInformation(machine, lpTimeZoneInformation).to_raw()
        }
        pub unsafe fn GetVersion(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetVersion(machine).to_raw()
        }
        pub unsafe fn GetVersionExA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpVersionInformation = <Option<&mut OSVERSIONINFO>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetVersionExA(machine, lpVersionInformation).to_raw()
        }
        pub unsafe fn GetWindowsDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpBuffer = <u32>::from_stack(mem, esp + 4u32);
            let uSize = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetWindowsDirectoryA(machine, lpBuffer, uSize).to_raw()
        }
        pub unsafe fn GlobalAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uFlags = <GMEM>::from_stack(mem, esp + 4u32);
            let dwBytes = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GlobalAlloc(machine, uFlags, dwBytes).to_raw()
        }
        pub unsafe fn GlobalFlags(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GlobalFlags(machine, hMem).to_raw()
        }
        pub unsafe fn GlobalFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GlobalFree(machine, hMem).to_raw()
        }
        pub unsafe fn GlobalReAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, esp + 4u32);
            let dwBytes = <u32>::from_stack(mem, esp + 8u32);
            let uFlags = <GMEM>::from_stack(mem, esp + 12u32);
            winapi::kernel32::GlobalReAlloc(machine, hMem, dwBytes, uFlags).to_raw()
        }
        pub unsafe fn HeapAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <Result<HeapAllocFlags, u32>>::from_stack(mem, esp + 8u32);
            let dwBytes = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes).to_raw()
        }
        pub unsafe fn HeapCreate(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let flOptions = <Result<HeapCreateFlags, u32>>::from_stack(mem, esp + 4u32);
            let dwInitialSize = <u32>::from_stack(mem, esp + 8u32);
            let dwMaximumSize = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, dwMaximumSize).to_raw()
        }
        pub unsafe fn HeapDestroy(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::HeapDestroy(machine, hHeap).to_raw()
        }
        pub unsafe fn HeapFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMem = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem).to_raw()
        }
        pub unsafe fn HeapReAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMem = <u32>::from_stack(mem, esp + 12u32);
            let dwBytes = <u32>::from_stack(mem, esp + 16u32);
            winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes).to_raw()
        }
        pub unsafe fn HeapSetInformation(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let HeapHandle = <u32>::from_stack(mem, esp + 4u32);
            let HeapInformationClass = <u32>::from_stack(mem, esp + 8u32);
            let HeapInformation = <u32>::from_stack(mem, esp + 12u32);
            let HeapInformationLength = <u32>::from_stack(mem, esp + 16u32);
            winapi::kernel32::HeapSetInformation(
                machine,
                HeapHandle,
                HeapInformationClass,
                HeapInformation,
                HeapInformationLength,
            )
            .to_raw()
        }
        pub unsafe fn HeapSize(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMem = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem).to_raw()
        }
        pub unsafe fn HeapValidate(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMem = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapValidate(machine, hHeap, dwFlags, lpMem).to_raw()
        }
        pub unsafe fn InitOnceBeginInitialize(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let fPending = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            let lpContext = <u32>::from_stack(mem, esp + 16u32);
            winapi::kernel32::InitOnceBeginInitialize(
                machine, lpInitOnce, dwFlags, fPending, lpContext,
            )
            .to_raw()
        }
        pub unsafe fn InitOnceComplete(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpContext = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::InitOnceComplete(machine, lpInitOnce, dwFlags, lpContext).to_raw()
        }
        pub unsafe fn InitializeCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::InitializeCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn InitializeCriticalSectionAndSpinCount(
            machine: &mut Machine,
            esp: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            let dwSpinCount = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::InitializeCriticalSectionAndSpinCount(
                machine,
                lpCriticalSection,
                dwSpinCount,
            )
            .to_raw()
        }
        pub unsafe fn InitializeCriticalSectionEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            let dwSpinCount = <u32>::from_stack(mem, esp + 8u32);
            let flags = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::InitializeCriticalSectionEx(
                machine,
                lpCriticalSection,
                dwSpinCount,
                flags,
            )
            .to_raw()
        }
        pub unsafe fn InitializeSListHead(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let ListHead = <Option<&mut SLIST_HEADER>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::InitializeSListHead(machine, ListHead).to_raw()
        }
        pub unsafe fn InterlockedDecrement(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let addend = <Option<&mut u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::InterlockedDecrement(machine, addend).to_raw()
        }
        pub unsafe fn InterlockedIncrement(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let addend = <Option<&mut u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::InterlockedIncrement(machine, addend).to_raw()
        }
        pub unsafe fn IsBadCodePtr(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpfn = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::IsBadCodePtr(machine, lpfn).to_raw()
        }
        pub unsafe fn IsBadReadPtr(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lp = <u32>::from_stack(mem, esp + 4u32);
            let ucb = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::IsBadReadPtr(machine, lp, ucb).to_raw()
        }
        pub unsafe fn IsBadWritePtr(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lp = <u32>::from_stack(mem, esp + 4u32);
            let ucb = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::IsBadWritePtr(machine, lp, ucb).to_raw()
        }
        pub unsafe fn IsDBCSLeadByte(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _TestChar = <u8>::from_stack(mem, esp + 4u32);
            winapi::kernel32::IsDBCSLeadByte(machine, _TestChar).to_raw()
        }
        pub unsafe fn IsDBCSLeadByteEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _TestChar = <u8>::from_stack(mem, esp + 4u32);
            let _CodePage = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::IsDBCSLeadByteEx(machine, _TestChar, _CodePage).to_raw()
        }
        pub unsafe fn IsDebuggerPresent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::IsDebuggerPresent(machine).to_raw()
        }
        pub unsafe fn IsProcessorFeaturePresent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let feature = <Result<ProcessorFeature, u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::IsProcessorFeaturePresent(machine, feature).to_raw()
        }
        pub unsafe fn IsValidCodePage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::IsValidCodePage(machine, CodePage).to_raw()
        }
        pub unsafe fn LCMapStringA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let locale = <LCID>::from_stack(mem, esp + 4u32);
            let dwMapFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpSrcStr = <u32>::from_stack(mem, esp + 12u32);
            let cchSrc = <i32>::from_stack(mem, esp + 16u32);
            let lpDestStr = <ArrayWithSizeMut<u8>>::from_stack(mem, esp + 20u32);
            winapi::kernel32::LCMapStringA(machine, locale, dwMapFlags, lpSrcStr, cchSrc, lpDestStr)
                .to_raw()
        }
        pub unsafe fn LCMapStringW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let locale = <LCID>::from_stack(mem, esp + 4u32);
            let dwMapFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpSrcStr = <u32>::from_stack(mem, esp + 12u32);
            let cchSrc = <i32>::from_stack(mem, esp + 16u32);
            let lpDestStr = <ArrayWithSizeMut<u16>>::from_stack(mem, esp + 20u32);
            winapi::kernel32::LCMapStringW(machine, locale, dwMapFlags, lpSrcStr, cchSrc, lpDestStr)
                .to_raw()
        }
        pub unsafe fn LeaveCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LeaveCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn LoadLibraryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let filename = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LoadLibraryA(machine, filename).to_raw()
        }
        pub unsafe fn LoadLibraryExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpLibFileName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let hFile = <HFILE>::from_stack(mem, esp + 8u32);
            let dwFlags = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags).to_raw()
        }
        pub unsafe fn LoadResource(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let hResInfo = <HRSRC>::from_stack(mem, esp + 8u32);
            winapi::kernel32::LoadResource(machine, hModule, hResInfo).to_raw()
        }
        pub unsafe fn LocalAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uFlags = <GMEM>::from_stack(mem, esp + 4u32);
            let dwBytes = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::LocalAlloc(machine, uFlags, dwBytes).to_raw()
        }
        pub unsafe fn LocalFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LocalFree(machine, hMem).to_raw()
        }
        pub unsafe fn LockResource(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hResData = <HRSRC>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LockResource(machine, hResData).to_raw()
        }
        pub unsafe fn MulDiv(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nNumber = <i32>::from_stack(mem, esp + 4u32);
            let nNumerator = <i32>::from_stack(mem, esp + 8u32);
            let nDenominator = <i32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::MulDiv(machine, nNumber, nNumerator, nDenominator).to_raw()
        }
        pub unsafe fn MultiByteToWideChar(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <Result<CP, u32>>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMultiByteStr = <u32>::from_stack(mem, esp + 12u32);
            let cbMultiByte = <i32>::from_stack(mem, esp + 16u32);
            let lpWideCharStr = <ArrayWithSizeMut<u16>>::from_stack(mem, esp + 20u32);
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
        pub unsafe fn NtCurrentTeb(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::NtCurrentTeb(machine).to_raw()
        }
        pub unsafe fn OutputDebugStringA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let msg = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::OutputDebugStringA(machine, msg).to_raw()
        }
        pub unsafe fn QueryPerformanceCounter(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPerformanceCount = <Option<&mut LARGE_INTEGER>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::QueryPerformanceCounter(machine, lpPerformanceCount).to_raw()
        }
        pub unsafe fn QueryPerformanceFrequency(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFrequency = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::QueryPerformanceFrequency(machine, lpFrequency).to_raw()
        }
        pub unsafe fn RaiseException(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwExceptionCode = <u32>::from_stack(mem, esp + 4u32);
            let dwExceptionFlags = <u32>::from_stack(mem, esp + 8u32);
            let nNumberOfArguments = <u32>::from_stack(mem, esp + 12u32);
            let lpArguments = <u32>::from_stack(mem, esp + 16u32);
            winapi::kernel32::RaiseException(
                machine,
                dwExceptionCode,
                dwExceptionFlags,
                nNumberOfArguments,
                lpArguments,
            )
            .to_raw()
        }
        pub unsafe fn ReadFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSizeMut<u8>>::from_stack(mem, esp + 8u32);
            let lpNumberOfBytesRead = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpOverlapped = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::ReadFile(machine, hFile, lpBuffer, lpNumberOfBytesRead, lpOverlapped)
                .to_raw()
        }
        pub unsafe fn ReleaseSRWLockExclusive(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::ReleaseSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn ReleaseSRWLockShared(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::ReleaseSRWLockShared(machine, SRWLock).to_raw()
        }
        pub unsafe fn RemoveDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::RemoveDirectoryA(machine, lpPathName).to_raw()
        }
        pub unsafe fn ResumeThread(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, esp + 4u32);
            winapi::kernel32::ResumeThread(machine, hThread).to_raw()
        }
        pub unsafe fn RtlUnwind(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let TargetFrame = <u32>::from_stack(mem, esp + 4u32);
            let TargetIp = <u32>::from_stack(mem, esp + 8u32);
            let ExceptionRecord = <u32>::from_stack(mem, esp + 12u32);
            let ReturnValue = <u32>::from_stack(mem, esp + 16u32);
            winapi::kernel32::RtlUnwind(
                machine,
                TargetFrame,
                TargetIp,
                ExceptionRecord,
                ReturnValue,
            )
            .to_raw()
        }
        pub unsafe fn SetConsoleCtrlHandler(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _handlerRoutine = <DWORD>::from_stack(mem, esp + 4u32);
            let _add = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetConsoleCtrlHandler(machine, _handlerRoutine, _add).to_raw()
        }
        pub unsafe fn SetEndOfFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetEndOfFile(machine, hFile).to_raw()
        }
        pub unsafe fn SetEnvironmentVariableA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&str>>::from_stack(mem, esp + 4u32);
            let value = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetEnvironmentVariableA(machine, name, value).to_raw()
        }
        pub unsafe fn SetEvent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hEvent = <HEVENT>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetEvent(machine, hEvent).to_raw()
        }
        pub unsafe fn SetFileAttributesA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let dwFileAttributes = <Result<FileAttribute, u32>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetFileAttributesA(machine, lpFileName, dwFileAttributes).to_raw()
        }
        pub unsafe fn SetFilePointer(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lDistanceToMove = <i32>::from_stack(mem, esp + 8u32);
            let lpDistanceToMoveHigh = <Option<&mut i32>>::from_stack(mem, esp + 12u32);
            let dwMoveMethod = <Result<FILE, u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::SetFilePointer(
                machine,
                hFile,
                lDistanceToMove,
                lpDistanceToMoveHigh,
                dwMoveMethod,
            )
            .to_raw()
        }
        pub unsafe fn SetFileTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpCreationTime = <Option<&FILETIME>>::from_stack(mem, esp + 8u32);
            let lpLastAccessTime = <Option<&FILETIME>>::from_stack(mem, esp + 12u32);
            let lpLastWriteTime = <Option<&FILETIME>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::SetFileTime(
                machine,
                hFile,
                lpCreationTime,
                lpLastAccessTime,
                lpLastWriteTime,
            )
            .to_raw()
        }
        pub unsafe fn SetHandleCount(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uNumber = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetHandleCount(machine, uNumber).to_raw()
        }
        pub unsafe fn SetLastError(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwErrCode = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetLastError(machine, dwErrCode).to_raw()
        }
        pub unsafe fn SetPriorityClass(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hProcess = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            let dwPriorityClass = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetPriorityClass(machine, hProcess, dwPriorityClass).to_raw()
        }
        pub unsafe fn SetStdHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nStdHandle = <Result<STD, u32>>::from_stack(mem, esp + 4u32);
            let hHandle = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetStdHandle(machine, nStdHandle, hHandle).to_raw()
        }
        pub unsafe fn SetThreadDescription(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, esp + 4u32);
            let lpThreadDescription = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetThreadDescription(machine, hThread, lpThreadDescription).to_raw()
        }
        pub unsafe fn SetThreadPriority(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, esp + 4u32);
            let nPriority = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetThreadPriority(machine, hThread, nPriority).to_raw()
        }
        pub unsafe fn SetThreadStackGuarantee(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let StackSizeInBytes = <Option<&mut u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetThreadStackGuarantee(machine, StackSizeInBytes).to_raw()
        }
        pub unsafe fn SetUnhandledExceptionFilter(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _lpTopLevelExceptionFilter = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter)
                .to_raw()
        }
        pub unsafe fn SizeofResource(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let hResInfo = <HRSRC>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SizeofResource(machine, hModule, hResInfo).to_raw()
        }
        pub unsafe fn Sleep(
            machine: &mut Machine,
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let dwMilliseconds = <u32>::from_stack(mem, esp + 4u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::kernel32::Sleep(machine, dwMilliseconds)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn SystemTimeToFileTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&SYSTEMTIME>>::from_stack(mem, esp + 4u32);
            let lpFileTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SystemTimeToFileTime(machine, lpSystemTime, lpFileTime).to_raw()
        }
        pub unsafe fn TerminateProcess(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hProcess = <u32>::from_stack(mem, esp + 4u32);
            let uExitCode = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::TerminateProcess(machine, hProcess, uExitCode).to_raw()
        }
        pub unsafe fn TlsAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::TlsAlloc(machine).to_raw()
        }
        pub unsafe fn TlsFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::TlsFree(machine, dwTlsIndex).to_raw()
        }
        pub unsafe fn TlsGetValue(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::TlsGetValue(machine, dwTlsIndex).to_raw()
        }
        pub unsafe fn TlsSetValue(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, esp + 4u32);
            let lpTlsValue = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue).to_raw()
        }
        pub unsafe fn TryAcquireSRWLockExclusive(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::TryAcquireSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn UnhandledExceptionFilter(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _exceptionInfo = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo).to_raw()
        }
        pub unsafe fn VirtualAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let dwSize = <u32>::from_stack(mem, esp + 8u32);
            let flAllocationType = <Result<MEM, u32>>::from_stack(mem, esp + 12u32);
            let flProtec = <Result<PAGE, u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::VirtualAlloc(machine, lpAddress, dwSize, flAllocationType, flProtec)
                .to_raw()
        }
        pub unsafe fn VirtualFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let dwSize = <u32>::from_stack(mem, esp + 8u32);
            let dwFreeType = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType).to_raw()
        }
        pub unsafe fn VirtualProtect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let dwSize = <u32>::from_stack(mem, esp + 8u32);
            let flNewProtect = <u32>::from_stack(mem, esp + 12u32);
            let lpflOldProtect = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::VirtualProtect(
                machine,
                lpAddress,
                dwSize,
                flNewProtect,
                lpflOldProtect,
            )
            .to_raw()
        }
        pub unsafe fn VirtualQuery(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let lpBuffer = <Option<&mut MEMORY_BASIC_INFORMATION>>::from_stack(mem, esp + 8u32);
            let dwLength = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::VirtualQuery(machine, lpAddress, lpBuffer, dwLength).to_raw()
        }
        pub unsafe fn WaitForSingleObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHandle = <HEVENT>::from_stack(mem, esp + 4u32);
            let dwMilliseconds = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::WaitForSingleObject(machine, hHandle, dwMilliseconds).to_raw()
        }
        pub unsafe fn WideCharToMultiByte(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <Result<CP, u32>>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpWideCharStr = <u32>::from_stack(mem, esp + 12u32);
            let cchWideChar = <i32>::from_stack(mem, esp + 16u32);
            let lpMultiByteStr = <u32>::from_stack(mem, esp + 20u32);
            let cbMultiByte = <i32>::from_stack(mem, esp + 24u32);
            let lpUsedDefaultChar = <Option<&mut u32>>::from_stack(mem, esp + 28u32);
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
        pub unsafe fn WriteConsoleA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleOutput = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, esp + 8u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpReserved = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::WriteConsoleA(
                machine,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                lpReserved,
            )
            .to_raw()
        }
        pub unsafe fn WriteConsoleW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleOutput = <HFILE>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSize<u16>>::from_stack(mem, esp + 8u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let _lpReserved = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::WriteConsoleW(
                machine,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                _lpReserved,
            )
            .to_raw()
        }
        pub unsafe fn WriteFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, esp + 8u32);
            let lpNumberOfBytesWritten = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpOverlapped = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::WriteFile(
                machine,
                hFile,
                lpBuffer,
                lpNumberOfBytesWritten,
                lpOverlapped,
            )
            .to_raw()
        }
        pub unsafe fn lstrcmpiA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpString2 = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::lstrcmpiA(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrcpyA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <u32>::from_stack(mem, esp + 4u32);
            let lpString2 = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::lstrcpyA(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrcpyW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <u32>::from_stack(mem, esp + 4u32);
            let lpString2 = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::lstrcpyW(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrlenA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::lstrlenA(machine, lpString).to_raw()
        }
        pub unsafe fn lstrlenW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::lstrlenW(machine, lpString).to_raw()
        }
        pub unsafe fn retrowin32_main(
            machine: &mut Machine,
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let entry_point = <u32>::from_stack(mem, esp + 4u32);
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
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let entry_point = <u32>::from_stack(mem, esp + 4u32);
            let param = <u32>::from_stack(mem, esp + 8u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::kernel32::retrowin32_thread_main(machine, entry_point, param)
                    .await
                    .to_raw()
            })
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const AcquireSRWLockExclusive: Shim = Shim {
            name: "AcquireSRWLockExclusive",
            func: crate::shims::Handler::Sync(impls::AcquireSRWLockExclusive),
        };
        pub const AcquireSRWLockShared: Shim = Shim {
            name: "AcquireSRWLockShared",
            func: crate::shims::Handler::Sync(impls::AcquireSRWLockShared),
        };
        pub const AddVectoredExceptionHandler: Shim = Shim {
            name: "AddVectoredExceptionHandler",
            func: crate::shims::Handler::Sync(impls::AddVectoredExceptionHandler),
        };
        pub const CloseHandle: Shim = Shim {
            name: "CloseHandle",
            func: crate::shims::Handler::Sync(impls::CloseHandle),
        };
        pub const CreateDirectoryA: Shim = Shim {
            name: "CreateDirectoryA",
            func: crate::shims::Handler::Sync(impls::CreateDirectoryA),
        };
        pub const CreateEventA: Shim = Shim {
            name: "CreateEventA",
            func: crate::shims::Handler::Sync(impls::CreateEventA),
        };
        pub const CreateFileA: Shim = Shim {
            name: "CreateFileA",
            func: crate::shims::Handler::Sync(impls::CreateFileA),
        };
        pub const CreateFileW: Shim = Shim {
            name: "CreateFileW",
            func: crate::shims::Handler::Sync(impls::CreateFileW),
        };
        pub const CreateThread: Shim = Shim {
            name: "CreateThread",
            func: crate::shims::Handler::Async(impls::CreateThread),
        };
        pub const DebugBreak: Shim = Shim {
            name: "DebugBreak",
            func: crate::shims::Handler::Sync(impls::DebugBreak),
        };
        pub const DeleteCriticalSection: Shim = Shim {
            name: "DeleteCriticalSection",
            func: crate::shims::Handler::Sync(impls::DeleteCriticalSection),
        };
        pub const DeleteFileA: Shim = Shim {
            name: "DeleteFileA",
            func: crate::shims::Handler::Sync(impls::DeleteFileA),
        };
        pub const DisableThreadLibraryCalls: Shim = Shim {
            name: "DisableThreadLibraryCalls",
            func: crate::shims::Handler::Sync(impls::DisableThreadLibraryCalls),
        };
        pub const EnterCriticalSection: Shim = Shim {
            name: "EnterCriticalSection",
            func: crate::shims::Handler::Sync(impls::EnterCriticalSection),
        };
        pub const ExitProcess: Shim = Shim {
            name: "ExitProcess",
            func: crate::shims::Handler::Sync(impls::ExitProcess),
        };
        pub const FileTimeToSystemTime: Shim = Shim {
            name: "FileTimeToSystemTime",
            func: crate::shims::Handler::Sync(impls::FileTimeToSystemTime),
        };
        pub const FindClose: Shim = Shim {
            name: "FindClose",
            func: crate::shims::Handler::Sync(impls::FindClose),
        };
        pub const FindFirstFileA: Shim = Shim {
            name: "FindFirstFileA",
            func: crate::shims::Handler::Sync(impls::FindFirstFileA),
        };
        pub const FindNextFileA: Shim = Shim {
            name: "FindNextFileA",
            func: crate::shims::Handler::Sync(impls::FindNextFileA),
        };
        pub const FindResourceA: Shim = Shim {
            name: "FindResourceA",
            func: crate::shims::Handler::Sync(impls::FindResourceA),
        };
        pub const FindResourceW: Shim = Shim {
            name: "FindResourceW",
            func: crate::shims::Handler::Sync(impls::FindResourceW),
        };
        pub const FlushFileBuffers: Shim = Shim {
            name: "FlushFileBuffers",
            func: crate::shims::Handler::Sync(impls::FlushFileBuffers),
        };
        pub const FormatMessageA: Shim = Shim {
            name: "FormatMessageA",
            func: crate::shims::Handler::Sync(impls::FormatMessageA),
        };
        pub const FormatMessageW: Shim = Shim {
            name: "FormatMessageW",
            func: crate::shims::Handler::Sync(impls::FormatMessageW),
        };
        pub const FreeEnvironmentStringsA: Shim = Shim {
            name: "FreeEnvironmentStringsA",
            func: crate::shims::Handler::Sync(impls::FreeEnvironmentStringsA),
        };
        pub const FreeEnvironmentStringsW: Shim = Shim {
            name: "FreeEnvironmentStringsW",
            func: crate::shims::Handler::Sync(impls::FreeEnvironmentStringsW),
        };
        pub const FreeLibrary: Shim = Shim {
            name: "FreeLibrary",
            func: crate::shims::Handler::Sync(impls::FreeLibrary),
        };
        pub const GetACP: Shim = Shim {
            name: "GetACP",
            func: crate::shims::Handler::Sync(impls::GetACP),
        };
        pub const GetCPInfo: Shim = Shim {
            name: "GetCPInfo",
            func: crate::shims::Handler::Sync(impls::GetCPInfo),
        };
        pub const GetCommandLineA: Shim = Shim {
            name: "GetCommandLineA",
            func: crate::shims::Handler::Sync(impls::GetCommandLineA),
        };
        pub const GetCommandLineW: Shim = Shim {
            name: "GetCommandLineW",
            func: crate::shims::Handler::Sync(impls::GetCommandLineW),
        };
        pub const GetConsoleMode: Shim = Shim {
            name: "GetConsoleMode",
            func: crate::shims::Handler::Sync(impls::GetConsoleMode),
        };
        pub const GetConsoleScreenBufferInfo: Shim = Shim {
            name: "GetConsoleScreenBufferInfo",
            func: crate::shims::Handler::Sync(impls::GetConsoleScreenBufferInfo),
        };
        pub const GetCurrentDirectoryA: Shim = Shim {
            name: "GetCurrentDirectoryA",
            func: crate::shims::Handler::Sync(impls::GetCurrentDirectoryA),
        };
        pub const GetCurrentProcess: Shim = Shim {
            name: "GetCurrentProcess",
            func: crate::shims::Handler::Sync(impls::GetCurrentProcess),
        };
        pub const GetCurrentProcessId: Shim = Shim {
            name: "GetCurrentProcessId",
            func: crate::shims::Handler::Sync(impls::GetCurrentProcessId),
        };
        pub const GetCurrentThread: Shim = Shim {
            name: "GetCurrentThread",
            func: crate::shims::Handler::Sync(impls::GetCurrentThread),
        };
        pub const GetCurrentThreadId: Shim = Shim {
            name: "GetCurrentThreadId",
            func: crate::shims::Handler::Sync(impls::GetCurrentThreadId),
        };
        pub const GetEnvironmentStrings: Shim = Shim {
            name: "GetEnvironmentStrings",
            func: crate::shims::Handler::Sync(impls::GetEnvironmentStrings),
        };
        pub const GetEnvironmentStringsW: Shim = Shim {
            name: "GetEnvironmentStringsW",
            func: crate::shims::Handler::Sync(impls::GetEnvironmentStringsW),
        };
        pub const GetEnvironmentVariableA: Shim = Shim {
            name: "GetEnvironmentVariableA",
            func: crate::shims::Handler::Sync(impls::GetEnvironmentVariableA),
        };
        pub const GetEnvironmentVariableW: Shim = Shim {
            name: "GetEnvironmentVariableW",
            func: crate::shims::Handler::Sync(impls::GetEnvironmentVariableW),
        };
        pub const GetFileAttributesA: Shim = Shim {
            name: "GetFileAttributesA",
            func: crate::shims::Handler::Sync(impls::GetFileAttributesA),
        };
        pub const GetFileInformationByHandle: Shim = Shim {
            name: "GetFileInformationByHandle",
            func: crate::shims::Handler::Sync(impls::GetFileInformationByHandle),
        };
        pub const GetFileSize: Shim = Shim {
            name: "GetFileSize",
            func: crate::shims::Handler::Sync(impls::GetFileSize),
        };
        pub const GetFileTime: Shim = Shim {
            name: "GetFileTime",
            func: crate::shims::Handler::Sync(impls::GetFileTime),
        };
        pub const GetFileType: Shim = Shim {
            name: "GetFileType",
            func: crate::shims::Handler::Sync(impls::GetFileType),
        };
        pub const GetFullPathNameA: Shim = Shim {
            name: "GetFullPathNameA",
            func: crate::shims::Handler::Sync(impls::GetFullPathNameA),
        };
        pub const GetFullPathNameW: Shim = Shim {
            name: "GetFullPathNameW",
            func: crate::shims::Handler::Sync(impls::GetFullPathNameW),
        };
        pub const GetLastError: Shim = Shim {
            name: "GetLastError",
            func: crate::shims::Handler::Sync(impls::GetLastError),
        };
        pub const GetLocalTime: Shim = Shim {
            name: "GetLocalTime",
            func: crate::shims::Handler::Sync(impls::GetLocalTime),
        };
        pub const GetModuleFileNameA: Shim = Shim {
            name: "GetModuleFileNameA",
            func: crate::shims::Handler::Sync(impls::GetModuleFileNameA),
        };
        pub const GetModuleFileNameW: Shim = Shim {
            name: "GetModuleFileNameW",
            func: crate::shims::Handler::Sync(impls::GetModuleFileNameW),
        };
        pub const GetModuleHandleA: Shim = Shim {
            name: "GetModuleHandleA",
            func: crate::shims::Handler::Sync(impls::GetModuleHandleA),
        };
        pub const GetModuleHandleExW: Shim = Shim {
            name: "GetModuleHandleExW",
            func: crate::shims::Handler::Sync(impls::GetModuleHandleExW),
        };
        pub const GetModuleHandleW: Shim = Shim {
            name: "GetModuleHandleW",
            func: crate::shims::Handler::Sync(impls::GetModuleHandleW),
        };
        pub const GetOEMCP: Shim = Shim {
            name: "GetOEMCP",
            func: crate::shims::Handler::Sync(impls::GetOEMCP),
        };
        pub const GetPrivateProfileIntW: Shim = Shim {
            name: "GetPrivateProfileIntW",
            func: crate::shims::Handler::Sync(impls::GetPrivateProfileIntW),
        };
        pub const GetPrivateProfileStringW: Shim = Shim {
            name: "GetPrivateProfileStringW",
            func: crate::shims::Handler::Sync(impls::GetPrivateProfileStringW),
        };
        pub const GetProcAddress: Shim = Shim {
            name: "GetProcAddress",
            func: crate::shims::Handler::Sync(impls::GetProcAddress),
        };
        pub const GetProcessHeap: Shim = Shim {
            name: "GetProcessHeap",
            func: crate::shims::Handler::Sync(impls::GetProcessHeap),
        };
        pub const GetProfileIntW: Shim = Shim {
            name: "GetProfileIntW",
            func: crate::shims::Handler::Sync(impls::GetProfileIntW),
        };
        pub const GetProfileStringW: Shim = Shim {
            name: "GetProfileStringW",
            func: crate::shims::Handler::Sync(impls::GetProfileStringW),
        };
        pub const GetStartupInfoA: Shim = Shim {
            name: "GetStartupInfoA",
            func: crate::shims::Handler::Sync(impls::GetStartupInfoA),
        };
        pub const GetStartupInfoW: Shim = Shim {
            name: "GetStartupInfoW",
            func: crate::shims::Handler::Sync(impls::GetStartupInfoW),
        };
        pub const GetStdHandle: Shim = Shim {
            name: "GetStdHandle",
            func: crate::shims::Handler::Sync(impls::GetStdHandle),
        };
        pub const GetStringTypeA: Shim = Shim {
            name: "GetStringTypeA",
            func: crate::shims::Handler::Sync(impls::GetStringTypeA),
        };
        pub const GetStringTypeW: Shim = Shim {
            name: "GetStringTypeW",
            func: crate::shims::Handler::Sync(impls::GetStringTypeW),
        };
        pub const GetSystemDirectoryA: Shim = Shim {
            name: "GetSystemDirectoryA",
            func: crate::shims::Handler::Sync(impls::GetSystemDirectoryA),
        };
        pub const GetSystemTime: Shim = Shim {
            name: "GetSystemTime",
            func: crate::shims::Handler::Sync(impls::GetSystemTime),
        };
        pub const GetSystemTimeAsFileTime: Shim = Shim {
            name: "GetSystemTimeAsFileTime",
            func: crate::shims::Handler::Sync(impls::GetSystemTimeAsFileTime),
        };
        pub const GetTickCount: Shim = Shim {
            name: "GetTickCount",
            func: crate::shims::Handler::Sync(impls::GetTickCount),
        };
        pub const GetTimeZoneInformation: Shim = Shim {
            name: "GetTimeZoneInformation",
            func: crate::shims::Handler::Sync(impls::GetTimeZoneInformation),
        };
        pub const GetVersion: Shim = Shim {
            name: "GetVersion",
            func: crate::shims::Handler::Sync(impls::GetVersion),
        };
        pub const GetVersionExA: Shim = Shim {
            name: "GetVersionExA",
            func: crate::shims::Handler::Sync(impls::GetVersionExA),
        };
        pub const GetWindowsDirectoryA: Shim = Shim {
            name: "GetWindowsDirectoryA",
            func: crate::shims::Handler::Sync(impls::GetWindowsDirectoryA),
        };
        pub const GlobalAlloc: Shim = Shim {
            name: "GlobalAlloc",
            func: crate::shims::Handler::Sync(impls::GlobalAlloc),
        };
        pub const GlobalFlags: Shim = Shim {
            name: "GlobalFlags",
            func: crate::shims::Handler::Sync(impls::GlobalFlags),
        };
        pub const GlobalFree: Shim = Shim {
            name: "GlobalFree",
            func: crate::shims::Handler::Sync(impls::GlobalFree),
        };
        pub const GlobalReAlloc: Shim = Shim {
            name: "GlobalReAlloc",
            func: crate::shims::Handler::Sync(impls::GlobalReAlloc),
        };
        pub const HeapAlloc: Shim = Shim {
            name: "HeapAlloc",
            func: crate::shims::Handler::Sync(impls::HeapAlloc),
        };
        pub const HeapCreate: Shim = Shim {
            name: "HeapCreate",
            func: crate::shims::Handler::Sync(impls::HeapCreate),
        };
        pub const HeapDestroy: Shim = Shim {
            name: "HeapDestroy",
            func: crate::shims::Handler::Sync(impls::HeapDestroy),
        };
        pub const HeapFree: Shim = Shim {
            name: "HeapFree",
            func: crate::shims::Handler::Sync(impls::HeapFree),
        };
        pub const HeapReAlloc: Shim = Shim {
            name: "HeapReAlloc",
            func: crate::shims::Handler::Sync(impls::HeapReAlloc),
        };
        pub const HeapSetInformation: Shim = Shim {
            name: "HeapSetInformation",
            func: crate::shims::Handler::Sync(impls::HeapSetInformation),
        };
        pub const HeapSize: Shim = Shim {
            name: "HeapSize",
            func: crate::shims::Handler::Sync(impls::HeapSize),
        };
        pub const HeapValidate: Shim = Shim {
            name: "HeapValidate",
            func: crate::shims::Handler::Sync(impls::HeapValidate),
        };
        pub const InitOnceBeginInitialize: Shim = Shim {
            name: "InitOnceBeginInitialize",
            func: crate::shims::Handler::Sync(impls::InitOnceBeginInitialize),
        };
        pub const InitOnceComplete: Shim = Shim {
            name: "InitOnceComplete",
            func: crate::shims::Handler::Sync(impls::InitOnceComplete),
        };
        pub const InitializeCriticalSection: Shim = Shim {
            name: "InitializeCriticalSection",
            func: crate::shims::Handler::Sync(impls::InitializeCriticalSection),
        };
        pub const InitializeCriticalSectionAndSpinCount: Shim = Shim {
            name: "InitializeCriticalSectionAndSpinCount",
            func: crate::shims::Handler::Sync(impls::InitializeCriticalSectionAndSpinCount),
        };
        pub const InitializeCriticalSectionEx: Shim = Shim {
            name: "InitializeCriticalSectionEx",
            func: crate::shims::Handler::Sync(impls::InitializeCriticalSectionEx),
        };
        pub const InitializeSListHead: Shim = Shim {
            name: "InitializeSListHead",
            func: crate::shims::Handler::Sync(impls::InitializeSListHead),
        };
        pub const InterlockedDecrement: Shim = Shim {
            name: "InterlockedDecrement",
            func: crate::shims::Handler::Sync(impls::InterlockedDecrement),
        };
        pub const InterlockedIncrement: Shim = Shim {
            name: "InterlockedIncrement",
            func: crate::shims::Handler::Sync(impls::InterlockedIncrement),
        };
        pub const IsBadCodePtr: Shim = Shim {
            name: "IsBadCodePtr",
            func: crate::shims::Handler::Sync(impls::IsBadCodePtr),
        };
        pub const IsBadReadPtr: Shim = Shim {
            name: "IsBadReadPtr",
            func: crate::shims::Handler::Sync(impls::IsBadReadPtr),
        };
        pub const IsBadWritePtr: Shim = Shim {
            name: "IsBadWritePtr",
            func: crate::shims::Handler::Sync(impls::IsBadWritePtr),
        };
        pub const IsDBCSLeadByte: Shim = Shim {
            name: "IsDBCSLeadByte",
            func: crate::shims::Handler::Sync(impls::IsDBCSLeadByte),
        };
        pub const IsDBCSLeadByteEx: Shim = Shim {
            name: "IsDBCSLeadByteEx",
            func: crate::shims::Handler::Sync(impls::IsDBCSLeadByteEx),
        };
        pub const IsDebuggerPresent: Shim = Shim {
            name: "IsDebuggerPresent",
            func: crate::shims::Handler::Sync(impls::IsDebuggerPresent),
        };
        pub const IsProcessorFeaturePresent: Shim = Shim {
            name: "IsProcessorFeaturePresent",
            func: crate::shims::Handler::Sync(impls::IsProcessorFeaturePresent),
        };
        pub const IsValidCodePage: Shim = Shim {
            name: "IsValidCodePage",
            func: crate::shims::Handler::Sync(impls::IsValidCodePage),
        };
        pub const LCMapStringA: Shim = Shim {
            name: "LCMapStringA",
            func: crate::shims::Handler::Sync(impls::LCMapStringA),
        };
        pub const LCMapStringW: Shim = Shim {
            name: "LCMapStringW",
            func: crate::shims::Handler::Sync(impls::LCMapStringW),
        };
        pub const LeaveCriticalSection: Shim = Shim {
            name: "LeaveCriticalSection",
            func: crate::shims::Handler::Sync(impls::LeaveCriticalSection),
        };
        pub const LoadLibraryA: Shim = Shim {
            name: "LoadLibraryA",
            func: crate::shims::Handler::Sync(impls::LoadLibraryA),
        };
        pub const LoadLibraryExW: Shim = Shim {
            name: "LoadLibraryExW",
            func: crate::shims::Handler::Sync(impls::LoadLibraryExW),
        };
        pub const LoadResource: Shim = Shim {
            name: "LoadResource",
            func: crate::shims::Handler::Sync(impls::LoadResource),
        };
        pub const LocalAlloc: Shim = Shim {
            name: "LocalAlloc",
            func: crate::shims::Handler::Sync(impls::LocalAlloc),
        };
        pub const LocalFree: Shim = Shim {
            name: "LocalFree",
            func: crate::shims::Handler::Sync(impls::LocalFree),
        };
        pub const LockResource: Shim = Shim {
            name: "LockResource",
            func: crate::shims::Handler::Sync(impls::LockResource),
        };
        pub const MulDiv: Shim = Shim {
            name: "MulDiv",
            func: crate::shims::Handler::Sync(impls::MulDiv),
        };
        pub const MultiByteToWideChar: Shim = Shim {
            name: "MultiByteToWideChar",
            func: crate::shims::Handler::Sync(impls::MultiByteToWideChar),
        };
        pub const NtCurrentTeb: Shim = Shim {
            name: "NtCurrentTeb",
            func: crate::shims::Handler::Sync(impls::NtCurrentTeb),
        };
        pub const OutputDebugStringA: Shim = Shim {
            name: "OutputDebugStringA",
            func: crate::shims::Handler::Sync(impls::OutputDebugStringA),
        };
        pub const QueryPerformanceCounter: Shim = Shim {
            name: "QueryPerformanceCounter",
            func: crate::shims::Handler::Sync(impls::QueryPerformanceCounter),
        };
        pub const QueryPerformanceFrequency: Shim = Shim {
            name: "QueryPerformanceFrequency",
            func: crate::shims::Handler::Sync(impls::QueryPerformanceFrequency),
        };
        pub const RaiseException: Shim = Shim {
            name: "RaiseException",
            func: crate::shims::Handler::Sync(impls::RaiseException),
        };
        pub const ReadFile: Shim = Shim {
            name: "ReadFile",
            func: crate::shims::Handler::Sync(impls::ReadFile),
        };
        pub const ReleaseSRWLockExclusive: Shim = Shim {
            name: "ReleaseSRWLockExclusive",
            func: crate::shims::Handler::Sync(impls::ReleaseSRWLockExclusive),
        };
        pub const ReleaseSRWLockShared: Shim = Shim {
            name: "ReleaseSRWLockShared",
            func: crate::shims::Handler::Sync(impls::ReleaseSRWLockShared),
        };
        pub const RemoveDirectoryA: Shim = Shim {
            name: "RemoveDirectoryA",
            func: crate::shims::Handler::Sync(impls::RemoveDirectoryA),
        };
        pub const ResumeThread: Shim = Shim {
            name: "ResumeThread",
            func: crate::shims::Handler::Sync(impls::ResumeThread),
        };
        pub const RtlUnwind: Shim = Shim {
            name: "RtlUnwind",
            func: crate::shims::Handler::Sync(impls::RtlUnwind),
        };
        pub const SetConsoleCtrlHandler: Shim = Shim {
            name: "SetConsoleCtrlHandler",
            func: crate::shims::Handler::Sync(impls::SetConsoleCtrlHandler),
        };
        pub const SetEndOfFile: Shim = Shim {
            name: "SetEndOfFile",
            func: crate::shims::Handler::Sync(impls::SetEndOfFile),
        };
        pub const SetEnvironmentVariableA: Shim = Shim {
            name: "SetEnvironmentVariableA",
            func: crate::shims::Handler::Sync(impls::SetEnvironmentVariableA),
        };
        pub const SetEvent: Shim = Shim {
            name: "SetEvent",
            func: crate::shims::Handler::Sync(impls::SetEvent),
        };
        pub const SetFileAttributesA: Shim = Shim {
            name: "SetFileAttributesA",
            func: crate::shims::Handler::Sync(impls::SetFileAttributesA),
        };
        pub const SetFilePointer: Shim = Shim {
            name: "SetFilePointer",
            func: crate::shims::Handler::Sync(impls::SetFilePointer),
        };
        pub const SetFileTime: Shim = Shim {
            name: "SetFileTime",
            func: crate::shims::Handler::Sync(impls::SetFileTime),
        };
        pub const SetHandleCount: Shim = Shim {
            name: "SetHandleCount",
            func: crate::shims::Handler::Sync(impls::SetHandleCount),
        };
        pub const SetLastError: Shim = Shim {
            name: "SetLastError",
            func: crate::shims::Handler::Sync(impls::SetLastError),
        };
        pub const SetPriorityClass: Shim = Shim {
            name: "SetPriorityClass",
            func: crate::shims::Handler::Sync(impls::SetPriorityClass),
        };
        pub const SetStdHandle: Shim = Shim {
            name: "SetStdHandle",
            func: crate::shims::Handler::Sync(impls::SetStdHandle),
        };
        pub const SetThreadDescription: Shim = Shim {
            name: "SetThreadDescription",
            func: crate::shims::Handler::Sync(impls::SetThreadDescription),
        };
        pub const SetThreadPriority: Shim = Shim {
            name: "SetThreadPriority",
            func: crate::shims::Handler::Sync(impls::SetThreadPriority),
        };
        pub const SetThreadStackGuarantee: Shim = Shim {
            name: "SetThreadStackGuarantee",
            func: crate::shims::Handler::Sync(impls::SetThreadStackGuarantee),
        };
        pub const SetUnhandledExceptionFilter: Shim = Shim {
            name: "SetUnhandledExceptionFilter",
            func: crate::shims::Handler::Sync(impls::SetUnhandledExceptionFilter),
        };
        pub const SizeofResource: Shim = Shim {
            name: "SizeofResource",
            func: crate::shims::Handler::Sync(impls::SizeofResource),
        };
        pub const Sleep: Shim = Shim {
            name: "Sleep",
            func: crate::shims::Handler::Async(impls::Sleep),
        };
        pub const SystemTimeToFileTime: Shim = Shim {
            name: "SystemTimeToFileTime",
            func: crate::shims::Handler::Sync(impls::SystemTimeToFileTime),
        };
        pub const TerminateProcess: Shim = Shim {
            name: "TerminateProcess",
            func: crate::shims::Handler::Sync(impls::TerminateProcess),
        };
        pub const TlsAlloc: Shim = Shim {
            name: "TlsAlloc",
            func: crate::shims::Handler::Sync(impls::TlsAlloc),
        };
        pub const TlsFree: Shim = Shim {
            name: "TlsFree",
            func: crate::shims::Handler::Sync(impls::TlsFree),
        };
        pub const TlsGetValue: Shim = Shim {
            name: "TlsGetValue",
            func: crate::shims::Handler::Sync(impls::TlsGetValue),
        };
        pub const TlsSetValue: Shim = Shim {
            name: "TlsSetValue",
            func: crate::shims::Handler::Sync(impls::TlsSetValue),
        };
        pub const TryAcquireSRWLockExclusive: Shim = Shim {
            name: "TryAcquireSRWLockExclusive",
            func: crate::shims::Handler::Sync(impls::TryAcquireSRWLockExclusive),
        };
        pub const UnhandledExceptionFilter: Shim = Shim {
            name: "UnhandledExceptionFilter",
            func: crate::shims::Handler::Sync(impls::UnhandledExceptionFilter),
        };
        pub const VirtualAlloc: Shim = Shim {
            name: "VirtualAlloc",
            func: crate::shims::Handler::Sync(impls::VirtualAlloc),
        };
        pub const VirtualFree: Shim = Shim {
            name: "VirtualFree",
            func: crate::shims::Handler::Sync(impls::VirtualFree),
        };
        pub const VirtualProtect: Shim = Shim {
            name: "VirtualProtect",
            func: crate::shims::Handler::Sync(impls::VirtualProtect),
        };
        pub const VirtualQuery: Shim = Shim {
            name: "VirtualQuery",
            func: crate::shims::Handler::Sync(impls::VirtualQuery),
        };
        pub const WaitForSingleObject: Shim = Shim {
            name: "WaitForSingleObject",
            func: crate::shims::Handler::Sync(impls::WaitForSingleObject),
        };
        pub const WideCharToMultiByte: Shim = Shim {
            name: "WideCharToMultiByte",
            func: crate::shims::Handler::Sync(impls::WideCharToMultiByte),
        };
        pub const WriteConsoleA: Shim = Shim {
            name: "WriteConsoleA",
            func: crate::shims::Handler::Sync(impls::WriteConsoleA),
        };
        pub const WriteConsoleW: Shim = Shim {
            name: "WriteConsoleW",
            func: crate::shims::Handler::Sync(impls::WriteConsoleW),
        };
        pub const WriteFile: Shim = Shim {
            name: "WriteFile",
            func: crate::shims::Handler::Sync(impls::WriteFile),
        };
        pub const lstrcmpiA: Shim = Shim {
            name: "lstrcmpiA",
            func: crate::shims::Handler::Sync(impls::lstrcmpiA),
        };
        pub const lstrcpyA: Shim = Shim {
            name: "lstrcpyA",
            func: crate::shims::Handler::Sync(impls::lstrcpyA),
        };
        pub const lstrcpyW: Shim = Shim {
            name: "lstrcpyW",
            func: crate::shims::Handler::Sync(impls::lstrcpyW),
        };
        pub const lstrlenA: Shim = Shim {
            name: "lstrlenA",
            func: crate::shims::Handler::Sync(impls::lstrlenA),
        };
        pub const lstrlenW: Shim = Shim {
            name: "lstrlenW",
            func: crate::shims::Handler::Sync(impls::lstrlenW),
        };
        pub const retrowin32_main: Shim = Shim {
            name: "retrowin32_main",
            func: crate::shims::Handler::Async(impls::retrowin32_main),
        };
        pub const retrowin32_thread_main: Shim = Shim {
            name: "retrowin32_thread_main",
            func: crate::shims::Handler::Async(impls::retrowin32_thread_main),
        };
    }
    const SHIMS: [Shim; 167usize] = [
        shims::AcquireSRWLockExclusive,
        shims::AcquireSRWLockShared,
        shims::AddVectoredExceptionHandler,
        shims::CloseHandle,
        shims::CreateDirectoryA,
        shims::CreateEventA,
        shims::CreateFileA,
        shims::CreateFileW,
        shims::CreateThread,
        shims::DebugBreak,
        shims::DeleteCriticalSection,
        shims::DeleteFileA,
        shims::DisableThreadLibraryCalls,
        shims::EnterCriticalSection,
        shims::ExitProcess,
        shims::FileTimeToSystemTime,
        shims::FindClose,
        shims::FindFirstFileA,
        shims::FindNextFileA,
        shims::FindResourceA,
        shims::FindResourceW,
        shims::FlushFileBuffers,
        shims::FormatMessageA,
        shims::FormatMessageW,
        shims::FreeEnvironmentStringsA,
        shims::FreeEnvironmentStringsW,
        shims::FreeLibrary,
        shims::GetACP,
        shims::GetCPInfo,
        shims::GetCommandLineA,
        shims::GetCommandLineW,
        shims::GetConsoleMode,
        shims::GetConsoleScreenBufferInfo,
        shims::GetCurrentDirectoryA,
        shims::GetCurrentProcess,
        shims::GetCurrentProcessId,
        shims::GetCurrentThread,
        shims::GetCurrentThreadId,
        shims::GetEnvironmentStrings,
        shims::GetEnvironmentStringsW,
        shims::GetEnvironmentVariableA,
        shims::GetEnvironmentVariableW,
        shims::GetFileAttributesA,
        shims::GetFileInformationByHandle,
        shims::GetFileSize,
        shims::GetFileTime,
        shims::GetFileType,
        shims::GetFullPathNameA,
        shims::GetFullPathNameW,
        shims::GetLastError,
        shims::GetLocalTime,
        shims::GetModuleFileNameA,
        shims::GetModuleFileNameW,
        shims::GetModuleHandleA,
        shims::GetModuleHandleExW,
        shims::GetModuleHandleW,
        shims::GetOEMCP,
        shims::GetPrivateProfileIntW,
        shims::GetPrivateProfileStringW,
        shims::GetProcAddress,
        shims::GetProcessHeap,
        shims::GetProfileIntW,
        shims::GetProfileStringW,
        shims::GetStartupInfoA,
        shims::GetStartupInfoW,
        shims::GetStdHandle,
        shims::GetStringTypeA,
        shims::GetStringTypeW,
        shims::GetSystemDirectoryA,
        shims::GetSystemTime,
        shims::GetSystemTimeAsFileTime,
        shims::GetTickCount,
        shims::GetTimeZoneInformation,
        shims::GetVersion,
        shims::GetVersionExA,
        shims::GetWindowsDirectoryA,
        shims::GlobalAlloc,
        shims::GlobalFlags,
        shims::GlobalFree,
        shims::GlobalReAlloc,
        shims::HeapAlloc,
        shims::HeapCreate,
        shims::HeapDestroy,
        shims::HeapFree,
        shims::HeapReAlloc,
        shims::HeapSetInformation,
        shims::HeapSize,
        shims::HeapValidate,
        shims::InitOnceBeginInitialize,
        shims::InitOnceComplete,
        shims::InitializeCriticalSection,
        shims::InitializeCriticalSectionAndSpinCount,
        shims::InitializeCriticalSectionEx,
        shims::InitializeSListHead,
        shims::InterlockedDecrement,
        shims::InterlockedIncrement,
        shims::IsBadCodePtr,
        shims::IsBadReadPtr,
        shims::IsBadWritePtr,
        shims::IsDBCSLeadByte,
        shims::IsDBCSLeadByteEx,
        shims::IsDebuggerPresent,
        shims::IsProcessorFeaturePresent,
        shims::IsValidCodePage,
        shims::LCMapStringA,
        shims::LCMapStringW,
        shims::LeaveCriticalSection,
        shims::LoadLibraryA,
        shims::LoadLibraryExW,
        shims::LoadResource,
        shims::LocalAlloc,
        shims::LocalFree,
        shims::LockResource,
        shims::MulDiv,
        shims::MultiByteToWideChar,
        shims::NtCurrentTeb,
        shims::OutputDebugStringA,
        shims::QueryPerformanceCounter,
        shims::QueryPerformanceFrequency,
        shims::RaiseException,
        shims::ReadFile,
        shims::ReleaseSRWLockExclusive,
        shims::ReleaseSRWLockShared,
        shims::RemoveDirectoryA,
        shims::ResumeThread,
        shims::RtlUnwind,
        shims::SetConsoleCtrlHandler,
        shims::SetEndOfFile,
        shims::SetEnvironmentVariableA,
        shims::SetEvent,
        shims::SetFileAttributesA,
        shims::SetFilePointer,
        shims::SetFileTime,
        shims::SetHandleCount,
        shims::SetLastError,
        shims::SetPriorityClass,
        shims::SetStdHandle,
        shims::SetThreadDescription,
        shims::SetThreadPriority,
        shims::SetThreadStackGuarantee,
        shims::SetUnhandledExceptionFilter,
        shims::SizeofResource,
        shims::Sleep,
        shims::SystemTimeToFileTime,
        shims::TerminateProcess,
        shims::TlsAlloc,
        shims::TlsFree,
        shims::TlsGetValue,
        shims::TlsSetValue,
        shims::TryAcquireSRWLockExclusive,
        shims::UnhandledExceptionFilter,
        shims::VirtualAlloc,
        shims::VirtualFree,
        shims::VirtualProtect,
        shims::VirtualQuery,
        shims::WaitForSingleObject,
        shims::WideCharToMultiByte,
        shims::WriteConsoleA,
        shims::WriteConsoleW,
        shims::WriteFile,
        shims::lstrcmpiA,
        shims::lstrcpyA,
        shims::lstrcpyW,
        shims::lstrlenA,
        shims::lstrlenW,
        shims::retrowin32_main,
        shims::retrowin32_thread_main,
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
        pub unsafe fn NtReadFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let FileHandle = <HFILE>::from_stack(mem, esp + 4u32);
            let Event = <u32>::from_stack(mem, esp + 8u32);
            let ApcRoutine = <u32>::from_stack(mem, esp + 12u32);
            let ApcContext = <u32>::from_stack(mem, esp + 16u32);
            let IoStatusBlock = <Option<&mut IO_STATUS_BLOCK>>::from_stack(mem, esp + 20u32);
            let Buffer = <ArrayWithSizeMut<u8>>::from_stack(mem, esp + 24u32);
            let ByteOffset = <Option<&mut u64>>::from_stack(mem, esp + 32u32);
            let Key = <u32>::from_stack(mem, esp + 36u32);
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
        pub unsafe fn RtlExitUserProcess(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let exit_code = <u32>::from_stack(mem, esp + 4u32);
            winapi::ntdll::RtlExitUserProcess(machine, exit_code).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const NtReadFile: Shim = Shim {
            name: "NtReadFile",
            func: crate::shims::Handler::Sync(impls::NtReadFile),
        };
        pub const RtlExitUserProcess: Shim = Shim {
            name: "RtlExitUserProcess",
            func: crate::shims::Handler::Sync(impls::RtlExitUserProcess),
        };
    }
    const SHIMS: [Shim; 2usize] = [shims::NtReadFile, shims::RtlExitUserProcess];
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
        pub unsafe fn OleInitialize(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _pvReserved = <u32>::from_stack(mem, esp + 4u32);
            winapi::ole32::OleInitialize(machine, _pvReserved).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const OleInitialize: Shim = Shim {
            name: "OleInitialize",
            func: crate::shims::Handler::Sync(impls::OleInitialize),
        };
    }
    const SHIMS: [Shim; 1usize] = [shims::OleInitialize];
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
    mod shims {
        use super::impls;
        use crate::shims::Shim;
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
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let func = <u32>::from_stack(mem, esp + 4u32);
            let data = <u32>::from_stack(mem, esp + 8u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::retrowin32_test::retrowin32_test_callback1(machine, func, data)
                    .await
                    .to_raw()
            })
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const retrowin32_test_callback1: Shim = Shim {
            name: "retrowin32_test_callback1",
            func: crate::shims::Handler::Async(impls::retrowin32_test_callback1),
        };
    }
    const SHIMS: [Shim; 1usize] = [shims::retrowin32_test_callback1];
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
        pub unsafe fn __dllonexit(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let func = <u32>::from_stack(mem, esp + 4u32);
            let d = <u32>::from_stack(mem, esp + 8u32);
            let f = <u32>::from_stack(mem, esp + 12u32);
            winapi::ucrtbase::__dllonexit(machine, func, d, f).to_raw()
        }
        pub unsafe fn __getmainargs(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let argc = <Option<&mut u32>>::from_stack(mem, esp + 4u32);
            let argv = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let env = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            let doWildCard = <u32>::from_stack(mem, esp + 16u32);
            let startInfo = <u32>::from_stack(mem, esp + 20u32);
            winapi::ucrtbase::__getmainargs(machine, argc, argv, env, doWildCard, startInfo)
                .to_raw()
        }
        pub unsafe fn __p___argc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p___argc(machine).to_raw()
        }
        pub unsafe fn __p___argv(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p___argv(machine).to_raw()
        }
        pub unsafe fn __p__commode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p__commode(machine).to_raw()
        }
        pub unsafe fn __p__fmode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p__fmode(machine).to_raw()
        }
        pub unsafe fn __set_app_type(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _app_type = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::__set_app_type(machine, _app_type).to_raw()
        }
        pub unsafe fn _configthreadlocale(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let per_thread_locale_type = <i32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_configthreadlocale(machine, per_thread_locale_type).to_raw()
        }
        pub unsafe fn _configure_narrow_argv(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _mode = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_configure_narrow_argv(machine, _mode).to_raw()
        }
        pub unsafe fn _controlfp(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _new = <u32>::from_stack(mem, esp + 4u32);
            let _mask = <u32>::from_stack(mem, esp + 8u32);
            winapi::ucrtbase::_controlfp(machine, _new, _mask).to_raw()
        }
        pub unsafe fn _controlfp_s(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _currentControl = <u32>::from_stack(mem, esp + 4u32);
            let _newControl = <u32>::from_stack(mem, esp + 8u32);
            let _mask = <u32>::from_stack(mem, esp + 12u32);
            winapi::ucrtbase::_controlfp_s(machine, _currentControl, _newControl, _mask).to_raw()
        }
        pub unsafe fn _crt_atexit(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _function = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_crt_atexit(machine, _function).to_raw()
        }
        pub unsafe fn _get_initial_narrow_environment(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::_get_initial_narrow_environment(machine).to_raw()
        }
        pub unsafe fn _initialize_narrow_environment(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::_initialize_narrow_environment(machine).to_raw()
        }
        pub unsafe fn _initterm(
            machine: &mut Machine,
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let start = <u32>::from_stack(mem, esp + 4u32);
            let end = <u32>::from_stack(mem, esp + 8u32);
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
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let start = <u32>::from_stack(mem, esp + 4u32);
            let end = <u32>::from_stack(mem, esp + 8u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::ucrtbase::_initterm_e(machine, start, end)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn _lock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let locknum = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_lock(machine, locknum).to_raw()
        }
        pub unsafe fn _set_app_type(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _app_type = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_set_app_type(machine, _app_type).to_raw()
        }
        pub unsafe fn _set_fmode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _mode = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_set_fmode(machine, _mode).to_raw()
        }
        pub unsafe fn _set_new_mode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let newhandlermode = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_set_new_mode(machine, newhandlermode).to_raw()
        }
        pub unsafe fn _time64(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let destTime = <Option<&mut u64>>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_time64(machine, destTime).to_raw()
        }
        pub unsafe fn _unlock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let locknum = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_unlock(machine, locknum).to_raw()
        }
        pub unsafe fn exit(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let status = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::exit(machine, status).to_raw()
        }
        pub unsafe fn free(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let ptr = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::free(machine, ptr).to_raw()
        }
        pub unsafe fn malloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let size = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::malloc(machine, size).to_raw()
        }
        pub unsafe fn rand(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::rand(machine).to_raw()
        }
        pub unsafe fn srand(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let seed = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::srand(machine, seed).to_raw()
        }
        pub unsafe fn time(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let destTime = <Option<&mut u64>>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::time(machine, destTime).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const __dllonexit: Shim = Shim {
            name: "__dllonexit",
            func: crate::shims::Handler::Sync(impls::__dllonexit),
        };
        pub const __getmainargs: Shim = Shim {
            name: "__getmainargs",
            func: crate::shims::Handler::Sync(impls::__getmainargs),
        };
        pub const __p___argc: Shim = Shim {
            name: "__p___argc",
            func: crate::shims::Handler::Sync(impls::__p___argc),
        };
        pub const __p___argv: Shim = Shim {
            name: "__p___argv",
            func: crate::shims::Handler::Sync(impls::__p___argv),
        };
        pub const __p__commode: Shim = Shim {
            name: "__p__commode",
            func: crate::shims::Handler::Sync(impls::__p__commode),
        };
        pub const __p__fmode: Shim = Shim {
            name: "__p__fmode",
            func: crate::shims::Handler::Sync(impls::__p__fmode),
        };
        pub const __set_app_type: Shim = Shim {
            name: "__set_app_type",
            func: crate::shims::Handler::Sync(impls::__set_app_type),
        };
        pub const _configthreadlocale: Shim = Shim {
            name: "_configthreadlocale",
            func: crate::shims::Handler::Sync(impls::_configthreadlocale),
        };
        pub const _configure_narrow_argv: Shim = Shim {
            name: "_configure_narrow_argv",
            func: crate::shims::Handler::Sync(impls::_configure_narrow_argv),
        };
        pub const _controlfp: Shim = Shim {
            name: "_controlfp",
            func: crate::shims::Handler::Sync(impls::_controlfp),
        };
        pub const _controlfp_s: Shim = Shim {
            name: "_controlfp_s",
            func: crate::shims::Handler::Sync(impls::_controlfp_s),
        };
        pub const _crt_atexit: Shim = Shim {
            name: "_crt_atexit",
            func: crate::shims::Handler::Sync(impls::_crt_atexit),
        };
        pub const _get_initial_narrow_environment: Shim = Shim {
            name: "_get_initial_narrow_environment",
            func: crate::shims::Handler::Sync(impls::_get_initial_narrow_environment),
        };
        pub const _initialize_narrow_environment: Shim = Shim {
            name: "_initialize_narrow_environment",
            func: crate::shims::Handler::Sync(impls::_initialize_narrow_environment),
        };
        pub const _initterm: Shim = Shim {
            name: "_initterm",
            func: crate::shims::Handler::Async(impls::_initterm),
        };
        pub const _initterm_e: Shim = Shim {
            name: "_initterm_e",
            func: crate::shims::Handler::Async(impls::_initterm_e),
        };
        pub const _lock: Shim = Shim {
            name: "_lock",
            func: crate::shims::Handler::Sync(impls::_lock),
        };
        pub const _set_app_type: Shim = Shim {
            name: "_set_app_type",
            func: crate::shims::Handler::Sync(impls::_set_app_type),
        };
        pub const _set_fmode: Shim = Shim {
            name: "_set_fmode",
            func: crate::shims::Handler::Sync(impls::_set_fmode),
        };
        pub const _set_new_mode: Shim = Shim {
            name: "_set_new_mode",
            func: crate::shims::Handler::Sync(impls::_set_new_mode),
        };
        pub const _time64: Shim = Shim {
            name: "_time64",
            func: crate::shims::Handler::Sync(impls::_time64),
        };
        pub const _unlock: Shim = Shim {
            name: "_unlock",
            func: crate::shims::Handler::Sync(impls::_unlock),
        };
        pub const exit: Shim = Shim {
            name: "exit",
            func: crate::shims::Handler::Sync(impls::exit),
        };
        pub const free: Shim = Shim {
            name: "free",
            func: crate::shims::Handler::Sync(impls::free),
        };
        pub const malloc: Shim = Shim {
            name: "malloc",
            func: crate::shims::Handler::Sync(impls::malloc),
        };
        pub const rand: Shim = Shim {
            name: "rand",
            func: crate::shims::Handler::Sync(impls::rand),
        };
        pub const srand: Shim = Shim {
            name: "srand",
            func: crate::shims::Handler::Sync(impls::srand),
        };
        pub const time: Shim = Shim {
            name: "time",
            func: crate::shims::Handler::Sync(impls::time),
        };
    }
    const SHIMS: [Shim; 28usize] = [
        shims::__dllonexit,
        shims::__getmainargs,
        shims::__p___argc,
        shims::__p___argv,
        shims::__p__commode,
        shims::__p__fmode,
        shims::__set_app_type,
        shims::_configthreadlocale,
        shims::_configure_narrow_argv,
        shims::_controlfp,
        shims::_controlfp_s,
        shims::_crt_atexit,
        shims::_get_initial_narrow_environment,
        shims::_initialize_narrow_environment,
        shims::_initterm,
        shims::_initterm_e,
        shims::_lock,
        shims::_set_app_type,
        shims::_set_fmode,
        shims::_set_new_mode,
        shims::_time64,
        shims::_unlock,
        shims::exit,
        shims::free,
        shims::malloc,
        shims::rand,
        shims::srand,
        shims::time,
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
        pub unsafe fn _CxxThrowException(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let pExceptionObject = <u32>::from_stack(mem, esp + 4u32);
            let pThrowInfo = <u32>::from_stack(mem, esp + 8u32);
            winapi::vcruntime140::_CxxThrowException(machine, pExceptionObject, pThrowInfo).to_raw()
        }
        pub unsafe fn memcmp(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lhs = <u32>::from_stack(mem, esp + 4u32);
            let rhs = <u32>::from_stack(mem, esp + 8u32);
            let len = <u32>::from_stack(mem, esp + 12u32);
            winapi::vcruntime140::memcmp(machine, lhs, rhs, len).to_raw()
        }
        pub unsafe fn memcpy(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dst = <u32>::from_stack(mem, esp + 4u32);
            let src = <u32>::from_stack(mem, esp + 8u32);
            let len = <u32>::from_stack(mem, esp + 12u32);
            winapi::vcruntime140::memcpy(machine, dst, src, len).to_raw()
        }
        pub unsafe fn memset(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dst = <u32>::from_stack(mem, esp + 4u32);
            let val = <u32>::from_stack(mem, esp + 8u32);
            let len = <u32>::from_stack(mem, esp + 12u32);
            winapi::vcruntime140::memset(machine, dst, val, len).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const _CxxThrowException: Shim = Shim {
            name: "_CxxThrowException",
            func: crate::shims::Handler::Sync(impls::_CxxThrowException),
        };
        pub const memcmp: Shim = Shim {
            name: "memcmp",
            func: crate::shims::Handler::Sync(impls::memcmp),
        };
        pub const memcpy: Shim = Shim {
            name: "memcpy",
            func: crate::shims::Handler::Sync(impls::memcpy),
        };
        pub const memset: Shim = Shim {
            name: "memset",
            func: crate::shims::Handler::Sync(impls::memset),
        };
    }
    const SHIMS: [Shim; 4usize] = [
        shims::_CxxThrowException,
        shims::memcmp,
        shims::memcpy,
        shims::memset,
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
        pub unsafe fn GetFileVersionInfoSizeA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lptstrFilename = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpdwHandle = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::version::GetFileVersionInfoSizeA(machine, lptstrFilename, lpdwHandle).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const GetFileVersionInfoSizeA: Shim = Shim {
            name: "GetFileVersionInfoSizeA",
            func: crate::shims::Handler::Sync(impls::GetFileVersionInfoSizeA),
        };
    }
    const SHIMS: [Shim; 1usize] = [shims::GetFileVersionInfoSizeA];
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
        pub unsafe fn AdjustWindowRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpRect = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 8u32);
            let bMenu = <bool>::from_stack(mem, esp + 12u32);
            winapi::user32::AdjustWindowRect(machine, lpRect, dwStyle, bMenu).to_raw()
        }
        pub unsafe fn AdjustWindowRectEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpRect = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 8u32);
            let bMenu = <bool>::from_stack(mem, esp + 12u32);
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, esp + 16u32);
            winapi::user32::AdjustWindowRectEx(machine, lpRect, dwStyle, bMenu, dwExStyle).to_raw()
        }
        pub unsafe fn AppendMenuA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, esp + 4u32);
            let uFlags = <u32>::from_stack(mem, esp + 8u32);
            let uIDNewItem = <u32>::from_stack(mem, esp + 12u32);
            let lpNewItem = <Option<&str>>::from_stack(mem, esp + 16u32);
            winapi::user32::AppendMenuA(machine, hMenu, uFlags, uIDNewItem, lpNewItem).to_raw()
        }
        pub unsafe fn BeginPaint(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpPaint = <Option<&mut PAINTSTRUCT>>::from_stack(mem, esp + 8u32);
            winapi::user32::BeginPaint(machine, hWnd, lpPaint).to_raw()
        }
        pub unsafe fn CheckMenuItem(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, esp + 4u32);
            let uIDCheckItem = <u32>::from_stack(mem, esp + 8u32);
            let uCheck = <u32>::from_stack(mem, esp + 12u32);
            winapi::user32::CheckMenuItem(machine, hMenu, uIDCheckItem, uCheck).to_raw()
        }
        pub unsafe fn ClientToScreen(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpPoint = <Option<&mut POINT>>::from_stack(mem, esp + 8u32);
            winapi::user32::ClientToScreen(machine, hWnd, lpPoint).to_raw()
        }
        pub unsafe fn CreateCursor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInst = <u32>::from_stack(mem, esp + 4u32);
            let xHotSpot = <u32>::from_stack(mem, esp + 8u32);
            let yHotSpot = <u32>::from_stack(mem, esp + 12u32);
            let nWidth = <u32>::from_stack(mem, esp + 16u32);
            let nHeight = <u32>::from_stack(mem, esp + 20u32);
            let pvANDPlane = <u32>::from_stack(mem, esp + 24u32);
            let pvXORPlane = <u32>::from_stack(mem, esp + 28u32);
            winapi::user32::CreateCursor(
                machine, hInst, xHotSpot, yHotSpot, nWidth, nHeight, pvANDPlane, pvXORPlane,
            )
            .to_raw()
        }
        pub unsafe fn CreatePopupMenu(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::CreatePopupMenu(machine).to_raw()
        }
        pub unsafe fn CreateWindowExA(
            machine: &mut Machine,
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, esp + 4u32);
            let lpClassName = <CreateWindowClassName<'_, str>>::from_stack(mem, esp + 8u32);
            let lpWindowName = <Option<&str>>::from_stack(mem, esp + 12u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 16u32);
            let X = <u32>::from_stack(mem, esp + 20u32);
            let Y = <u32>::from_stack(mem, esp + 24u32);
            let nWidth = <u32>::from_stack(mem, esp + 28u32);
            let nHeight = <u32>::from_stack(mem, esp + 32u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 36u32);
            let hMenu = <u32>::from_stack(mem, esp + 40u32);
            let hInstance = <u32>::from_stack(mem, esp + 44u32);
            let lpParam = <u32>::from_stack(mem, esp + 48u32);
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
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, esp + 4u32);
            let lpClassName = <CreateWindowClassName<'_, Str16>>::from_stack(mem, esp + 8u32);
            let lpWindowName = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 16u32);
            let X = <u32>::from_stack(mem, esp + 20u32);
            let Y = <u32>::from_stack(mem, esp + 24u32);
            let nWidth = <u32>::from_stack(mem, esp + 28u32);
            let nHeight = <u32>::from_stack(mem, esp + 32u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 36u32);
            let hMenu = <u32>::from_stack(mem, esp + 40u32);
            let hInstance = <u32>::from_stack(mem, esp + 44u32);
            let lpParam = <u32>::from_stack(mem, esp + 48u32);
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
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let msg = <Result<WM, u32>>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
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
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let msg = <Result<WM, u32>>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::DefWindowProcW(machine, hWnd, msg, wParam, lParam)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn DestroyWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::DestroyWindow(machine, hWnd).to_raw()
        }
        pub unsafe fn DialogBoxIndirectParamA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let hDialogTemplate = <u32>::from_stack(mem, esp + 8u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 12u32);
            let lpDialogFunc = <u32>::from_stack(mem, esp + 16u32);
            let dwInitParam = <u32>::from_stack(mem, esp + 20u32);
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
        pub unsafe fn DialogBoxParamA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpTemplateName = <u32>::from_stack(mem, esp + 8u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 12u32);
            let lpDialogFunc = <u32>::from_stack(mem, esp + 16u32);
            let dwInitParam = <u32>::from_stack(mem, esp + 20u32);
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
        pub unsafe fn DispatchMessageA(
            machine: &mut Machine,
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 4u32);
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
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 4u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::DispatchMessageW(machine, lpMsg)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn DrawTextW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, esp + 4u32);
            let lpString = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let nCount = <i32>::from_stack(mem, esp + 12u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, esp + 16u32);
            let uFormat = <u32>::from_stack(mem, esp + 20u32);
            winapi::user32::DrawTextW(machine, hDC, lpString, nCount, lpRect, uFormat).to_raw()
        }
        pub unsafe fn EndDialog(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, esp + 4u32);
            let nResult = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::user32::EndDialog(machine, hDlg, nResult).to_raw()
        }
        pub unsafe fn EndPaint(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpPaint = <Option<&PAINTSTRUCT>>::from_stack(mem, esp + 8u32);
            winapi::user32::EndPaint(machine, hWnd, lpPaint).to_raw()
        }
        pub unsafe fn FillRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, esp + 4u32);
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let hbr = <BrushOrColor>::from_stack(mem, esp + 12u32);
            winapi::user32::FillRect(machine, hDC, lprc, hbr).to_raw()
        }
        pub unsafe fn FindWindowA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpClassName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpWindowName = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::user32::FindWindowA(machine, lpClassName, lpWindowName).to_raw()
        }
        pub unsafe fn FrameRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, esp + 4u32);
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let hbr = <HBRUSH>::from_stack(mem, esp + 12u32);
            winapi::user32::FrameRect(machine, hDC, lprc, hbr).to_raw()
        }
        pub unsafe fn GetActiveWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetActiveWindow(machine).to_raw()
        }
        pub unsafe fn GetClientRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpRect = <Option<&mut RECT>>::from_stack(mem, esp + 8u32);
            winapi::user32::GetClientRect(machine, hWnd, lpRect).to_raw()
        }
        pub unsafe fn GetDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::GetDC(machine, hWnd).to_raw()
        }
        pub unsafe fn GetDesktopWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetDesktopWindow(machine).to_raw()
        }
        pub unsafe fn GetFocus(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetFocus(machine).to_raw()
        }
        pub unsafe fn GetForegroundWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetForegroundWindow(machine).to_raw()
        }
        pub unsafe fn GetKeyState(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nVirtKey = <u32>::from_stack(mem, esp + 4u32);
            winapi::user32::GetKeyState(machine, nVirtKey).to_raw()
        }
        pub unsafe fn GetLastActivePopup(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetLastActivePopup(machine).to_raw()
        }
        pub unsafe fn GetMessageA(
            machine: &mut Machine,
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
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
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::GetMessageW(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn GetSubMenu(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, esp + 4u32);
            let nPos = <i32>::from_stack(mem, esp + 8u32);
            winapi::user32::GetSubMenu(machine, hMenu, nPos).to_raw()
        }
        pub unsafe fn GetSystemMenu(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let bRevert = <bool>::from_stack(mem, esp + 8u32);
            winapi::user32::GetSystemMenu(machine, hWnd, bRevert).to_raw()
        }
        pub unsafe fn GetSystemMetrics(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nIndex = <Result<SystemMetric, u32>>::from_stack(mem, esp + 4u32);
            winapi::user32::GetSystemMetrics(machine, nIndex).to_raw()
        }
        pub unsafe fn GetWindowDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::GetWindowDC(machine, hWnd).to_raw()
        }
        pub unsafe fn GetWindowLongA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let nIndex = <i32>::from_stack(mem, esp + 8u32);
            winapi::user32::GetWindowLongA(machine, hWnd, nIndex).to_raw()
        }
        pub unsafe fn GetWindowPlacement(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpwndpl = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::user32::GetWindowPlacement(machine, hWnd, lpwndpl).to_raw()
        }
        pub unsafe fn GetWindowRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpRect = <Option<&mut RECT>>::from_stack(mem, esp + 8u32);
            winapi::user32::GetWindowRect(machine, hWnd, lpRect).to_raw()
        }
        pub unsafe fn IntersectRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprcDst = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let lprcSrc1 = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let lprcSrc2 = <Option<&RECT>>::from_stack(mem, esp + 12u32);
            winapi::user32::IntersectRect(machine, lprcDst, lprcSrc1, lprcSrc2).to_raw()
        }
        pub unsafe fn InvalidateRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let bErase = <bool>::from_stack(mem, esp + 12u32);
            winapi::user32::InvalidateRect(machine, hWnd, lpRect, bErase).to_raw()
        }
        pub unsafe fn InvalidateRgn(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hRgn = <HRGN>::from_stack(mem, esp + 8u32);
            let bErase = <bool>::from_stack(mem, esp + 12u32);
            winapi::user32::InvalidateRgn(machine, hWnd, hRgn, bErase).to_raw()
        }
        pub unsafe fn IsIconic(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::IsIconic(machine, hwnd).to_raw()
        }
        pub unsafe fn IsRectEmpty(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 4u32);
            winapi::user32::IsRectEmpty(machine, lprc).to_raw()
        }
        pub unsafe fn KillTimer(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let uIDEvent = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::KillTimer(machine, hWnd, uIDEvent).to_raw()
        }
        pub unsafe fn LoadAcceleratorsW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpTableName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadAcceleratorsW(machine, hInstance, lpTableName).to_raw()
        }
        pub unsafe fn LoadBitmapA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <HINSTANCE>::from_stack(mem, esp + 4u32);
            let lpBitmapName = <ResourceKey<&str>>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadBitmapA(machine, hInstance, lpBitmapName).to_raw()
        }
        pub unsafe fn LoadCursorA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpCursorName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadCursorA(machine, hInstance, lpCursorName).to_raw()
        }
        pub unsafe fn LoadCursorW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpCursorName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadCursorW(machine, hInstance, lpCursorName).to_raw()
        }
        pub unsafe fn LoadIconA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpIconName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadIconA(machine, hInstance, lpIconName).to_raw()
        }
        pub unsafe fn LoadIconW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpIconName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadIconW(machine, hInstance, lpIconName).to_raw()
        }
        pub unsafe fn LoadImageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let name = <ResourceKey<&str>>::from_stack(mem, esp + 8u32);
            let typ = <u32>::from_stack(mem, esp + 12u32);
            let cx = <u32>::from_stack(mem, esp + 16u32);
            let cy = <u32>::from_stack(mem, esp + 20u32);
            let fuLoad = <u32>::from_stack(mem, esp + 24u32);
            winapi::user32::LoadImageA(machine, hInstance, name, typ, cx, cy, fuLoad).to_raw()
        }
        pub unsafe fn LoadImageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let name = <ResourceKey<&Str16>>::from_stack(mem, esp + 8u32);
            let typ = <u32>::from_stack(mem, esp + 12u32);
            let cx = <u32>::from_stack(mem, esp + 16u32);
            let cy = <u32>::from_stack(mem, esp + 20u32);
            let fuLoad = <u32>::from_stack(mem, esp + 24u32);
            winapi::user32::LoadImageW(machine, hInstance, name, typ, cx, cy, fuLoad).to_raw()
        }
        pub unsafe fn LoadMenuA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpMenuName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadMenuA(machine, hInstance, lpMenuName).to_raw()
        }
        pub unsafe fn LoadMenuW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpMenuName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadMenuW(machine, hInstance, lpMenuName).to_raw()
        }
        pub unsafe fn LoadStringA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let uID = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let cchBufferMax = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::LoadStringA(machine, hInstance, uID, lpBuffer, cchBufferMax).to_raw()
        }
        pub unsafe fn LoadStringW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let uID = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let cchBufferMax = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::LoadStringW(machine, hInstance, uID, lpBuffer, cchBufferMax).to_raw()
        }
        pub unsafe fn MapWindowPoints(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWndFrom = <HWND>::from_stack(mem, esp + 4u32);
            let hWndTo = <HWND>::from_stack(mem, esp + 8u32);
            let lpPoints = <ArrayWithSize<POINT>>::from_stack(mem, esp + 12u32);
            winapi::user32::MapWindowPoints(machine, hWndFrom, hWndTo, lpPoints).to_raw()
        }
        pub unsafe fn MessageBoxA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpText = <Option<&str>>::from_stack(mem, esp + 8u32);
            let lpCaption = <Option<&str>>::from_stack(mem, esp + 12u32);
            let uType = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::MessageBoxA(machine, hWnd, lpText, lpCaption, uType).to_raw()
        }
        pub unsafe fn MessageBoxW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpText = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpCaption = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let uType = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::MessageBoxW(machine, hWnd, lpText, lpCaption, uType).to_raw()
        }
        pub unsafe fn MoveWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let X = <u32>::from_stack(mem, esp + 8u32);
            let Y = <u32>::from_stack(mem, esp + 12u32);
            let nWidth = <u32>::from_stack(mem, esp + 16u32);
            let nHeight = <u32>::from_stack(mem, esp + 20u32);
            let bRepaint = <bool>::from_stack(mem, esp + 24u32);
            winapi::user32::MoveWindow(machine, hWnd, X, Y, nWidth, nHeight, bRepaint).to_raw()
        }
        pub unsafe fn MsgWaitForMultipleObjects(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nCount = <u32>::from_stack(mem, esp + 4u32);
            let pHandles = <u32>::from_stack(mem, esp + 8u32);
            let fWaitAll = <bool>::from_stack(mem, esp + 12u32);
            let dwMilliseconds = <u32>::from_stack(mem, esp + 16u32);
            let dwWakeMask = <u32>::from_stack(mem, esp + 20u32);
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
        pub unsafe fn PeekMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, esp + 20u32);
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
        pub unsafe fn PeekMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, esp + 20u32);
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
        pub unsafe fn PostMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let Msg = <u32>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::PostMessageW(machine, hWnd, Msg, wParam, lParam).to_raw()
        }
        pub unsafe fn PostQuitMessage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nExitCode = <i32>::from_stack(mem, esp + 4u32);
            winapi::user32::PostQuitMessage(machine, nExitCode).to_raw()
        }
        pub unsafe fn PtInRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 4u32);
            let pt = <POINT>::from_stack(mem, esp + 8u32);
            winapi::user32::PtInRect(machine, lprc, pt).to_raw()
        }
        pub unsafe fn RegisterClassA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassA(machine, lpWndClass).to_raw()
        }
        pub unsafe fn RegisterClassExA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClassEx = <Option<&WNDCLASSEXA>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassExA(machine, lpWndClassEx).to_raw()
        }
        pub unsafe fn RegisterClassExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClassEx = <Option<&WNDCLASSEXW>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassExW(machine, lpWndClassEx).to_raw()
        }
        pub unsafe fn RegisterClassW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassW(machine, lpWndClass).to_raw()
        }
        pub unsafe fn RegisterWindowMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterWindowMessageW(machine, lpString).to_raw()
        }
        pub unsafe fn ReleaseCapture(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::ReleaseCapture(machine).to_raw()
        }
        pub unsafe fn ReleaseDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, esp + 4u32);
            let hdc = <HDC>::from_stack(mem, esp + 8u32);
            winapi::user32::ReleaseDC(machine, hwnd, hdc).to_raw()
        }
        pub unsafe fn SendMessageA(
            machine: &mut Machine,
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let Msg = <Result<WM, u32>>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::SendMessageA(machine, hWnd, Msg, wParam, lParam)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn SetCapture(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::SetCapture(machine, hwnd).to_raw()
        }
        pub unsafe fn SetCursor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hCursor = <u32>::from_stack(mem, esp + 4u32);
            winapi::user32::SetCursor(machine, hCursor).to_raw()
        }
        pub unsafe fn SetDlgItemTextA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDlg = <HWND>::from_stack(mem, esp + 4u32);
            let nIDDlgItem = <i32>::from_stack(mem, esp + 8u32);
            let lpString = <Option<&str>>::from_stack(mem, esp + 12u32);
            winapi::user32::SetDlgItemTextA(machine, hDlg, nIDDlgItem, lpString).to_raw()
        }
        pub unsafe fn SetFocus(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::SetFocus(machine, hWnd).to_raw()
        }
        pub unsafe fn SetForegroundWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::SetForegroundWindow(machine, hWnd).to_raw()
        }
        pub unsafe fn SetMenu(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hMenu = <HMENU>::from_stack(mem, esp + 8u32);
            winapi::user32::SetMenu(machine, hWnd, hMenu).to_raw()
        }
        pub unsafe fn SetMenuItemInfoA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, esp + 4u32);
            let item = <u32>::from_stack(mem, esp + 8u32);
            let fByPosition = <bool>::from_stack(mem, esp + 12u32);
            let lpmii = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::SetMenuItemInfoA(machine, hMenu, item, fByPosition, lpmii).to_raw()
        }
        pub unsafe fn SetRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let xLeft = <i32>::from_stack(mem, esp + 8u32);
            let yTop = <i32>::from_stack(mem, esp + 12u32);
            let xRight = <i32>::from_stack(mem, esp + 16u32);
            let yBottom = <i32>::from_stack(mem, esp + 20u32);
            winapi::user32::SetRect(machine, lprc, xLeft, yTop, xRight, yBottom).to_raw()
        }
        pub unsafe fn SetRectEmpty(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            winapi::user32::SetRectEmpty(machine, lprc).to_raw()
        }
        pub unsafe fn SetTimer(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let nIDEvent = <u32>::from_stack(mem, esp + 8u32);
            let uElapse = <u32>::from_stack(mem, esp + 12u32);
            let lpTimerFunc = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::SetTimer(machine, hWnd, nIDEvent, uElapse, lpTimerFunc).to_raw()
        }
        pub unsafe fn SetWindowPos(
            machine: &mut Machine,
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hWndInsertAfter = <HWND>::from_stack(mem, esp + 8u32);
            let X = <i32>::from_stack(mem, esp + 12u32);
            let Y = <i32>::from_stack(mem, esp + 16u32);
            let cx = <i32>::from_stack(mem, esp + 20u32);
            let cy = <i32>::from_stack(mem, esp + 24u32);
            let uFlags = <Result<SWP, u32>>::from_stack(mem, esp + 28u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::SetWindowPos(machine, hWnd, hWndInsertAfter, X, Y, cx, cy, uFlags)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn SetWindowTextA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::user32::SetWindowTextA(machine, hWnd, lpString).to_raw()
        }
        pub unsafe fn ShowCursor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let bShow = <bool>::from_stack(mem, esp + 4u32);
            winapi::user32::ShowCursor(machine, bShow).to_raw()
        }
        pub unsafe fn ShowWindow(
            machine: &mut Machine,
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let nCmdShow = <Result<SW, u32>>::from_stack(mem, esp + 8u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::ShowWindow(machine, hWnd, nCmdShow)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn TranslateAcceleratorW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hAccTable = <u32>::from_stack(mem, esp + 8u32);
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 12u32);
            winapi::user32::TranslateAcceleratorW(machine, hWnd, hAccTable, lpMsg).to_raw()
        }
        pub unsafe fn TranslateMessage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 4u32);
            winapi::user32::TranslateMessage(machine, lpMsg).to_raw()
        }
        pub unsafe fn UpdateWindow(
            machine: &mut Machine,
            esp: u32,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::UpdateWindow(machine, hWnd).await.to_raw()
            })
        }
        pub unsafe fn ValidateRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            winapi::user32::ValidateRect(machine, hWnd, lpRect).to_raw()
        }
        pub unsafe fn WaitMessage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::WaitMessage(machine).to_raw()
        }
        pub unsafe fn wsprintfA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let buf = <u32>::from_stack(mem, esp + 4u32);
            let fmt = <Option<&str>>::from_stack(mem, esp + 8u32);
            let args = <VarArgs>::from_stack(mem, esp + 12u32);
            winapi::user32::wsprintfA(machine, buf, fmt, args).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const AdjustWindowRect: Shim = Shim {
            name: "AdjustWindowRect",
            func: crate::shims::Handler::Sync(impls::AdjustWindowRect),
        };
        pub const AdjustWindowRectEx: Shim = Shim {
            name: "AdjustWindowRectEx",
            func: crate::shims::Handler::Sync(impls::AdjustWindowRectEx),
        };
        pub const AppendMenuA: Shim = Shim {
            name: "AppendMenuA",
            func: crate::shims::Handler::Sync(impls::AppendMenuA),
        };
        pub const BeginPaint: Shim = Shim {
            name: "BeginPaint",
            func: crate::shims::Handler::Sync(impls::BeginPaint),
        };
        pub const CheckMenuItem: Shim = Shim {
            name: "CheckMenuItem",
            func: crate::shims::Handler::Sync(impls::CheckMenuItem),
        };
        pub const ClientToScreen: Shim = Shim {
            name: "ClientToScreen",
            func: crate::shims::Handler::Sync(impls::ClientToScreen),
        };
        pub const CreateCursor: Shim = Shim {
            name: "CreateCursor",
            func: crate::shims::Handler::Sync(impls::CreateCursor),
        };
        pub const CreatePopupMenu: Shim = Shim {
            name: "CreatePopupMenu",
            func: crate::shims::Handler::Sync(impls::CreatePopupMenu),
        };
        pub const CreateWindowExA: Shim = Shim {
            name: "CreateWindowExA",
            func: crate::shims::Handler::Async(impls::CreateWindowExA),
        };
        pub const CreateWindowExW: Shim = Shim {
            name: "CreateWindowExW",
            func: crate::shims::Handler::Async(impls::CreateWindowExW),
        };
        pub const DefWindowProcA: Shim = Shim {
            name: "DefWindowProcA",
            func: crate::shims::Handler::Async(impls::DefWindowProcA),
        };
        pub const DefWindowProcW: Shim = Shim {
            name: "DefWindowProcW",
            func: crate::shims::Handler::Async(impls::DefWindowProcW),
        };
        pub const DestroyWindow: Shim = Shim {
            name: "DestroyWindow",
            func: crate::shims::Handler::Sync(impls::DestroyWindow),
        };
        pub const DialogBoxIndirectParamA: Shim = Shim {
            name: "DialogBoxIndirectParamA",
            func: crate::shims::Handler::Sync(impls::DialogBoxIndirectParamA),
        };
        pub const DialogBoxParamA: Shim = Shim {
            name: "DialogBoxParamA",
            func: crate::shims::Handler::Sync(impls::DialogBoxParamA),
        };
        pub const DispatchMessageA: Shim = Shim {
            name: "DispatchMessageA",
            func: crate::shims::Handler::Async(impls::DispatchMessageA),
        };
        pub const DispatchMessageW: Shim = Shim {
            name: "DispatchMessageW",
            func: crate::shims::Handler::Async(impls::DispatchMessageW),
        };
        pub const DrawTextW: Shim = Shim {
            name: "DrawTextW",
            func: crate::shims::Handler::Sync(impls::DrawTextW),
        };
        pub const EndDialog: Shim = Shim {
            name: "EndDialog",
            func: crate::shims::Handler::Sync(impls::EndDialog),
        };
        pub const EndPaint: Shim = Shim {
            name: "EndPaint",
            func: crate::shims::Handler::Sync(impls::EndPaint),
        };
        pub const FillRect: Shim = Shim {
            name: "FillRect",
            func: crate::shims::Handler::Sync(impls::FillRect),
        };
        pub const FindWindowA: Shim = Shim {
            name: "FindWindowA",
            func: crate::shims::Handler::Sync(impls::FindWindowA),
        };
        pub const FrameRect: Shim = Shim {
            name: "FrameRect",
            func: crate::shims::Handler::Sync(impls::FrameRect),
        };
        pub const GetActiveWindow: Shim = Shim {
            name: "GetActiveWindow",
            func: crate::shims::Handler::Sync(impls::GetActiveWindow),
        };
        pub const GetClientRect: Shim = Shim {
            name: "GetClientRect",
            func: crate::shims::Handler::Sync(impls::GetClientRect),
        };
        pub const GetDC: Shim = Shim {
            name: "GetDC",
            func: crate::shims::Handler::Sync(impls::GetDC),
        };
        pub const GetDesktopWindow: Shim = Shim {
            name: "GetDesktopWindow",
            func: crate::shims::Handler::Sync(impls::GetDesktopWindow),
        };
        pub const GetFocus: Shim = Shim {
            name: "GetFocus",
            func: crate::shims::Handler::Sync(impls::GetFocus),
        };
        pub const GetForegroundWindow: Shim = Shim {
            name: "GetForegroundWindow",
            func: crate::shims::Handler::Sync(impls::GetForegroundWindow),
        };
        pub const GetKeyState: Shim = Shim {
            name: "GetKeyState",
            func: crate::shims::Handler::Sync(impls::GetKeyState),
        };
        pub const GetLastActivePopup: Shim = Shim {
            name: "GetLastActivePopup",
            func: crate::shims::Handler::Sync(impls::GetLastActivePopup),
        };
        pub const GetMessageA: Shim = Shim {
            name: "GetMessageA",
            func: crate::shims::Handler::Async(impls::GetMessageA),
        };
        pub const GetMessageW: Shim = Shim {
            name: "GetMessageW",
            func: crate::shims::Handler::Async(impls::GetMessageW),
        };
        pub const GetSubMenu: Shim = Shim {
            name: "GetSubMenu",
            func: crate::shims::Handler::Sync(impls::GetSubMenu),
        };
        pub const GetSystemMenu: Shim = Shim {
            name: "GetSystemMenu",
            func: crate::shims::Handler::Sync(impls::GetSystemMenu),
        };
        pub const GetSystemMetrics: Shim = Shim {
            name: "GetSystemMetrics",
            func: crate::shims::Handler::Sync(impls::GetSystemMetrics),
        };
        pub const GetWindowDC: Shim = Shim {
            name: "GetWindowDC",
            func: crate::shims::Handler::Sync(impls::GetWindowDC),
        };
        pub const GetWindowLongA: Shim = Shim {
            name: "GetWindowLongA",
            func: crate::shims::Handler::Sync(impls::GetWindowLongA),
        };
        pub const GetWindowPlacement: Shim = Shim {
            name: "GetWindowPlacement",
            func: crate::shims::Handler::Sync(impls::GetWindowPlacement),
        };
        pub const GetWindowRect: Shim = Shim {
            name: "GetWindowRect",
            func: crate::shims::Handler::Sync(impls::GetWindowRect),
        };
        pub const IntersectRect: Shim = Shim {
            name: "IntersectRect",
            func: crate::shims::Handler::Sync(impls::IntersectRect),
        };
        pub const InvalidateRect: Shim = Shim {
            name: "InvalidateRect",
            func: crate::shims::Handler::Sync(impls::InvalidateRect),
        };
        pub const InvalidateRgn: Shim = Shim {
            name: "InvalidateRgn",
            func: crate::shims::Handler::Sync(impls::InvalidateRgn),
        };
        pub const IsIconic: Shim = Shim {
            name: "IsIconic",
            func: crate::shims::Handler::Sync(impls::IsIconic),
        };
        pub const IsRectEmpty: Shim = Shim {
            name: "IsRectEmpty",
            func: crate::shims::Handler::Sync(impls::IsRectEmpty),
        };
        pub const KillTimer: Shim = Shim {
            name: "KillTimer",
            func: crate::shims::Handler::Sync(impls::KillTimer),
        };
        pub const LoadAcceleratorsW: Shim = Shim {
            name: "LoadAcceleratorsW",
            func: crate::shims::Handler::Sync(impls::LoadAcceleratorsW),
        };
        pub const LoadBitmapA: Shim = Shim {
            name: "LoadBitmapA",
            func: crate::shims::Handler::Sync(impls::LoadBitmapA),
        };
        pub const LoadCursorA: Shim = Shim {
            name: "LoadCursorA",
            func: crate::shims::Handler::Sync(impls::LoadCursorA),
        };
        pub const LoadCursorW: Shim = Shim {
            name: "LoadCursorW",
            func: crate::shims::Handler::Sync(impls::LoadCursorW),
        };
        pub const LoadIconA: Shim = Shim {
            name: "LoadIconA",
            func: crate::shims::Handler::Sync(impls::LoadIconA),
        };
        pub const LoadIconW: Shim = Shim {
            name: "LoadIconW",
            func: crate::shims::Handler::Sync(impls::LoadIconW),
        };
        pub const LoadImageA: Shim = Shim {
            name: "LoadImageA",
            func: crate::shims::Handler::Sync(impls::LoadImageA),
        };
        pub const LoadImageW: Shim = Shim {
            name: "LoadImageW",
            func: crate::shims::Handler::Sync(impls::LoadImageW),
        };
        pub const LoadMenuA: Shim = Shim {
            name: "LoadMenuA",
            func: crate::shims::Handler::Sync(impls::LoadMenuA),
        };
        pub const LoadMenuW: Shim = Shim {
            name: "LoadMenuW",
            func: crate::shims::Handler::Sync(impls::LoadMenuW),
        };
        pub const LoadStringA: Shim = Shim {
            name: "LoadStringA",
            func: crate::shims::Handler::Sync(impls::LoadStringA),
        };
        pub const LoadStringW: Shim = Shim {
            name: "LoadStringW",
            func: crate::shims::Handler::Sync(impls::LoadStringW),
        };
        pub const MapWindowPoints: Shim = Shim {
            name: "MapWindowPoints",
            func: crate::shims::Handler::Sync(impls::MapWindowPoints),
        };
        pub const MessageBoxA: Shim = Shim {
            name: "MessageBoxA",
            func: crate::shims::Handler::Sync(impls::MessageBoxA),
        };
        pub const MessageBoxW: Shim = Shim {
            name: "MessageBoxW",
            func: crate::shims::Handler::Sync(impls::MessageBoxW),
        };
        pub const MoveWindow: Shim = Shim {
            name: "MoveWindow",
            func: crate::shims::Handler::Sync(impls::MoveWindow),
        };
        pub const MsgWaitForMultipleObjects: Shim = Shim {
            name: "MsgWaitForMultipleObjects",
            func: crate::shims::Handler::Sync(impls::MsgWaitForMultipleObjects),
        };
        pub const PeekMessageA: Shim = Shim {
            name: "PeekMessageA",
            func: crate::shims::Handler::Sync(impls::PeekMessageA),
        };
        pub const PeekMessageW: Shim = Shim {
            name: "PeekMessageW",
            func: crate::shims::Handler::Sync(impls::PeekMessageW),
        };
        pub const PostMessageW: Shim = Shim {
            name: "PostMessageW",
            func: crate::shims::Handler::Sync(impls::PostMessageW),
        };
        pub const PostQuitMessage: Shim = Shim {
            name: "PostQuitMessage",
            func: crate::shims::Handler::Sync(impls::PostQuitMessage),
        };
        pub const PtInRect: Shim = Shim {
            name: "PtInRect",
            func: crate::shims::Handler::Sync(impls::PtInRect),
        };
        pub const RegisterClassA: Shim = Shim {
            name: "RegisterClassA",
            func: crate::shims::Handler::Sync(impls::RegisterClassA),
        };
        pub const RegisterClassExA: Shim = Shim {
            name: "RegisterClassExA",
            func: crate::shims::Handler::Sync(impls::RegisterClassExA),
        };
        pub const RegisterClassExW: Shim = Shim {
            name: "RegisterClassExW",
            func: crate::shims::Handler::Sync(impls::RegisterClassExW),
        };
        pub const RegisterClassW: Shim = Shim {
            name: "RegisterClassW",
            func: crate::shims::Handler::Sync(impls::RegisterClassW),
        };
        pub const RegisterWindowMessageW: Shim = Shim {
            name: "RegisterWindowMessageW",
            func: crate::shims::Handler::Sync(impls::RegisterWindowMessageW),
        };
        pub const ReleaseCapture: Shim = Shim {
            name: "ReleaseCapture",
            func: crate::shims::Handler::Sync(impls::ReleaseCapture),
        };
        pub const ReleaseDC: Shim = Shim {
            name: "ReleaseDC",
            func: crate::shims::Handler::Sync(impls::ReleaseDC),
        };
        pub const SendMessageA: Shim = Shim {
            name: "SendMessageA",
            func: crate::shims::Handler::Async(impls::SendMessageA),
        };
        pub const SetCapture: Shim = Shim {
            name: "SetCapture",
            func: crate::shims::Handler::Sync(impls::SetCapture),
        };
        pub const SetCursor: Shim = Shim {
            name: "SetCursor",
            func: crate::shims::Handler::Sync(impls::SetCursor),
        };
        pub const SetDlgItemTextA: Shim = Shim {
            name: "SetDlgItemTextA",
            func: crate::shims::Handler::Sync(impls::SetDlgItemTextA),
        };
        pub const SetFocus: Shim = Shim {
            name: "SetFocus",
            func: crate::shims::Handler::Sync(impls::SetFocus),
        };
        pub const SetForegroundWindow: Shim = Shim {
            name: "SetForegroundWindow",
            func: crate::shims::Handler::Sync(impls::SetForegroundWindow),
        };
        pub const SetMenu: Shim = Shim {
            name: "SetMenu",
            func: crate::shims::Handler::Sync(impls::SetMenu),
        };
        pub const SetMenuItemInfoA: Shim = Shim {
            name: "SetMenuItemInfoA",
            func: crate::shims::Handler::Sync(impls::SetMenuItemInfoA),
        };
        pub const SetRect: Shim = Shim {
            name: "SetRect",
            func: crate::shims::Handler::Sync(impls::SetRect),
        };
        pub const SetRectEmpty: Shim = Shim {
            name: "SetRectEmpty",
            func: crate::shims::Handler::Sync(impls::SetRectEmpty),
        };
        pub const SetTimer: Shim = Shim {
            name: "SetTimer",
            func: crate::shims::Handler::Sync(impls::SetTimer),
        };
        pub const SetWindowPos: Shim = Shim {
            name: "SetWindowPos",
            func: crate::shims::Handler::Async(impls::SetWindowPos),
        };
        pub const SetWindowTextA: Shim = Shim {
            name: "SetWindowTextA",
            func: crate::shims::Handler::Sync(impls::SetWindowTextA),
        };
        pub const ShowCursor: Shim = Shim {
            name: "ShowCursor",
            func: crate::shims::Handler::Sync(impls::ShowCursor),
        };
        pub const ShowWindow: Shim = Shim {
            name: "ShowWindow",
            func: crate::shims::Handler::Async(impls::ShowWindow),
        };
        pub const TranslateAcceleratorW: Shim = Shim {
            name: "TranslateAcceleratorW",
            func: crate::shims::Handler::Sync(impls::TranslateAcceleratorW),
        };
        pub const TranslateMessage: Shim = Shim {
            name: "TranslateMessage",
            func: crate::shims::Handler::Sync(impls::TranslateMessage),
        };
        pub const UpdateWindow: Shim = Shim {
            name: "UpdateWindow",
            func: crate::shims::Handler::Async(impls::UpdateWindow),
        };
        pub const ValidateRect: Shim = Shim {
            name: "ValidateRect",
            func: crate::shims::Handler::Sync(impls::ValidateRect),
        };
        pub const WaitMessage: Shim = Shim {
            name: "WaitMessage",
            func: crate::shims::Handler::Sync(impls::WaitMessage),
        };
        pub const wsprintfA: Shim = Shim {
            name: "wsprintfA",
            func: crate::shims::Handler::Sync(impls::wsprintfA),
        };
    }
    const SHIMS: [Shim; 96usize] = [
        shims::AdjustWindowRect,
        shims::AdjustWindowRectEx,
        shims::AppendMenuA,
        shims::BeginPaint,
        shims::CheckMenuItem,
        shims::ClientToScreen,
        shims::CreateCursor,
        shims::CreatePopupMenu,
        shims::CreateWindowExA,
        shims::CreateWindowExW,
        shims::DefWindowProcA,
        shims::DefWindowProcW,
        shims::DestroyWindow,
        shims::DialogBoxIndirectParamA,
        shims::DialogBoxParamA,
        shims::DispatchMessageA,
        shims::DispatchMessageW,
        shims::DrawTextW,
        shims::EndDialog,
        shims::EndPaint,
        shims::FillRect,
        shims::FindWindowA,
        shims::FrameRect,
        shims::GetActiveWindow,
        shims::GetClientRect,
        shims::GetDC,
        shims::GetDesktopWindow,
        shims::GetFocus,
        shims::GetForegroundWindow,
        shims::GetKeyState,
        shims::GetLastActivePopup,
        shims::GetMessageA,
        shims::GetMessageW,
        shims::GetSubMenu,
        shims::GetSystemMenu,
        shims::GetSystemMetrics,
        shims::GetWindowDC,
        shims::GetWindowLongA,
        shims::GetWindowPlacement,
        shims::GetWindowRect,
        shims::IntersectRect,
        shims::InvalidateRect,
        shims::InvalidateRgn,
        shims::IsIconic,
        shims::IsRectEmpty,
        shims::KillTimer,
        shims::LoadAcceleratorsW,
        shims::LoadBitmapA,
        shims::LoadCursorA,
        shims::LoadCursorW,
        shims::LoadIconA,
        shims::LoadIconW,
        shims::LoadImageA,
        shims::LoadImageW,
        shims::LoadMenuA,
        shims::LoadMenuW,
        shims::LoadStringA,
        shims::LoadStringW,
        shims::MapWindowPoints,
        shims::MessageBoxA,
        shims::MessageBoxW,
        shims::MoveWindow,
        shims::MsgWaitForMultipleObjects,
        shims::PeekMessageA,
        shims::PeekMessageW,
        shims::PostMessageW,
        shims::PostQuitMessage,
        shims::PtInRect,
        shims::RegisterClassA,
        shims::RegisterClassExA,
        shims::RegisterClassExW,
        shims::RegisterClassW,
        shims::RegisterWindowMessageW,
        shims::ReleaseCapture,
        shims::ReleaseDC,
        shims::SendMessageA,
        shims::SetCapture,
        shims::SetCursor,
        shims::SetDlgItemTextA,
        shims::SetFocus,
        shims::SetForegroundWindow,
        shims::SetMenu,
        shims::SetMenuItemInfoA,
        shims::SetRect,
        shims::SetRectEmpty,
        shims::SetTimer,
        shims::SetWindowPos,
        shims::SetWindowTextA,
        shims::ShowCursor,
        shims::ShowWindow,
        shims::TranslateAcceleratorW,
        shims::TranslateMessage,
        shims::UpdateWindow,
        shims::ValidateRect,
        shims::WaitMessage,
        shims::wsprintfA,
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
        pub unsafe fn InternetOpenA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpszAgent = <Option<&str>>::from_stack(mem, esp + 4u32);
            let dwAccessType = <u32>::from_stack(mem, esp + 8u32);
            let lpszProxy = <Option<&str>>::from_stack(mem, esp + 12u32);
            let lpszProxyBypass = <Option<&str>>::from_stack(mem, esp + 16u32);
            let dwFlags = <u32>::from_stack(mem, esp + 20u32);
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
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const InternetOpenA: Shim = Shim {
            name: "InternetOpenA",
            func: crate::shims::Handler::Sync(impls::InternetOpenA),
        };
    }
    const SHIMS: [Shim; 1usize] = [shims::InternetOpenA];
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
        pub unsafe fn timeBeginPeriod(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uPeriod = <u32>::from_stack(mem, esp + 4u32);
            winapi::winmm::timeBeginPeriod(machine, uPeriod).to_raw()
        }
        pub unsafe fn timeGetTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::winmm::timeGetTime(machine).to_raw()
        }
        pub unsafe fn timeSetEvent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uDelay = <u32>::from_stack(mem, esp + 4u32);
            let uResolution = <u32>::from_stack(mem, esp + 8u32);
            let lpTimeProc = <u32>::from_stack(mem, esp + 12u32);
            let dwUser = <u32>::from_stack(mem, esp + 16u32);
            let fuEvent = <u32>::from_stack(mem, esp + 20u32);
            winapi::winmm::timeSetEvent(machine, uDelay, uResolution, lpTimeProc, dwUser, fuEvent)
                .to_raw()
        }
        pub unsafe fn waveOutClose(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            winapi::winmm::waveOutClose(machine, hwo).to_raw()
        }
        pub unsafe fn waveOutGetDevCapsA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uDeviceID = <u32>::from_stack(mem, esp + 4u32);
            let pwoc = <Option<&mut WAVEOUTCAPS>>::from_stack(mem, esp + 8u32);
            let cbwoc = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutGetDevCapsA(machine, uDeviceID, pwoc, cbwoc).to_raw()
        }
        pub unsafe fn waveOutGetNumDevs(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::winmm::waveOutGetNumDevs(machine).to_raw()
        }
        pub unsafe fn waveOutGetPosition(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            let pmmt = <Option<&mut MMTIME>>::from_stack(mem, esp + 8u32);
            let cbmmt = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutGetPosition(machine, hwo, pmmt, cbmmt).to_raw()
        }
        pub unsafe fn waveOutOpen(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let phwo = <Option<&mut HWAVEOUT>>::from_stack(mem, esp + 4u32);
            let uDeviceID = <u32>::from_stack(mem, esp + 8u32);
            let pwfx = <Option<&WAVEFORMATEX>>::from_stack(mem, esp + 12u32);
            let dwCallback = <u32>::from_stack(mem, esp + 16u32);
            let dwInstance = <u32>::from_stack(mem, esp + 20u32);
            let fdwOpen = <Result<WaveOutOpenFlags, u32>>::from_stack(mem, esp + 24u32);
            winapi::winmm::waveOutOpen(
                machine, phwo, uDeviceID, pwfx, dwCallback, dwInstance, fdwOpen,
            )
            .to_raw()
        }
        pub unsafe fn waveOutPrepareHeader(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            let pwh = <Option<&WAVEHDR>>::from_stack(mem, esp + 8u32);
            let cbwh = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutPrepareHeader(machine, hwo, pwh, cbwh).to_raw()
        }
        pub unsafe fn waveOutReset(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            winapi::winmm::waveOutReset(machine, hwo).to_raw()
        }
        pub unsafe fn waveOutUnprepareHeader(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            let pwh = <Option<&mut WAVEHDR>>::from_stack(mem, esp + 8u32);
            let cbwh = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutUnprepareHeader(machine, hwo, pwh, cbwh).to_raw()
        }
        pub unsafe fn waveOutWrite(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            let pwh = <Option<&WAVEHDR>>::from_stack(mem, esp + 8u32);
            let cbwh = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutWrite(machine, hwo, pwh, cbwh).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const timeBeginPeriod: Shim = Shim {
            name: "timeBeginPeriod",
            func: crate::shims::Handler::Sync(impls::timeBeginPeriod),
        };
        pub const timeGetTime: Shim = Shim {
            name: "timeGetTime",
            func: crate::shims::Handler::Sync(impls::timeGetTime),
        };
        pub const timeSetEvent: Shim = Shim {
            name: "timeSetEvent",
            func: crate::shims::Handler::Sync(impls::timeSetEvent),
        };
        pub const waveOutClose: Shim = Shim {
            name: "waveOutClose",
            func: crate::shims::Handler::Sync(impls::waveOutClose),
        };
        pub const waveOutGetDevCapsA: Shim = Shim {
            name: "waveOutGetDevCapsA",
            func: crate::shims::Handler::Sync(impls::waveOutGetDevCapsA),
        };
        pub const waveOutGetNumDevs: Shim = Shim {
            name: "waveOutGetNumDevs",
            func: crate::shims::Handler::Sync(impls::waveOutGetNumDevs),
        };
        pub const waveOutGetPosition: Shim = Shim {
            name: "waveOutGetPosition",
            func: crate::shims::Handler::Sync(impls::waveOutGetPosition),
        };
        pub const waveOutOpen: Shim = Shim {
            name: "waveOutOpen",
            func: crate::shims::Handler::Sync(impls::waveOutOpen),
        };
        pub const waveOutPrepareHeader: Shim = Shim {
            name: "waveOutPrepareHeader",
            func: crate::shims::Handler::Sync(impls::waveOutPrepareHeader),
        };
        pub const waveOutReset: Shim = Shim {
            name: "waveOutReset",
            func: crate::shims::Handler::Sync(impls::waveOutReset),
        };
        pub const waveOutUnprepareHeader: Shim = Shim {
            name: "waveOutUnprepareHeader",
            func: crate::shims::Handler::Sync(impls::waveOutUnprepareHeader),
        };
        pub const waveOutWrite: Shim = Shim {
            name: "waveOutWrite",
            func: crate::shims::Handler::Sync(impls::waveOutWrite),
        };
    }
    const SHIMS: [Shim; 12usize] = [
        shims::timeBeginPeriod,
        shims::timeGetTime,
        shims::timeSetEvent,
        shims::waveOutClose,
        shims::waveOutGetDevCapsA,
        shims::waveOutGetNumDevs,
        shims::waveOutGetPosition,
        shims::waveOutOpen,
        shims::waveOutPrepareHeader,
        shims::waveOutReset,
        shims::waveOutUnprepareHeader,
        shims::waveOutWrite,
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "winmm.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/winmm.dll"),
    };
}
