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
    use winapi::ddraw::*;
    pub unsafe fn DirectDrawCreate(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpGuid = <Option<&GUID>>::from_stack(mem, stack_args + 0u32);
        let lplpDD = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let pUnkOuter = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("ddraw") {
            crate::trace::Record::new(
                winapi::ddraw::DirectDrawCreate_pos,
                "ddraw",
                "DirectDrawCreate",
                &[
                    ("lpGuid", &lpGuid),
                    ("lplpDD", &lplpDD),
                    ("pUnkOuter", &pUnkOuter),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn DirectDrawCreateClipper(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let dwFlags = <u32>::from_stack(mem, stack_args + 0u32);
        let lplpDDClipper = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let pUnkOuter = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("ddraw/clipper") {
            crate::trace::Record::new(
                winapi::ddraw::DirectDrawCreateClipper_pos,
                "ddraw/clipper",
                "DirectDrawCreateClipper",
                &[
                    ("dwFlags", &dwFlags),
                    ("lplpDDClipper", &lplpDDClipper),
                    ("pUnkOuter", &pUnkOuter),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::ddraw::DirectDrawCreateClipper(machine, dwFlags, lplpDDClipper, pUnkOuter);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn DirectDrawCreateEx(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpGuid = <Option<&GUID>>::from_stack(mem, stack_args + 0u32);
        let lplpDD = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let iid = <Option<&GUID>>::from_stack(mem, stack_args + 8u32);
        let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("ddraw") {
            crate::trace::Record::new(
                winapi::ddraw::DirectDrawCreateEx_pos,
                "ddraw",
                "DirectDrawCreateEx",
                &[
                    ("lpGuid", &lpGuid),
                    ("lplpDD", &lplpDD),
                    ("iid", &iid),
                    ("pUnkOuter", &pUnkOuter),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDraw2_CreateSurface(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let desc = <Option<&DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
        let lplpDDSurface = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
        let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw2") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw2::CreateSurface_pos,
                "ddraw/ddraw2",
                "IDirectDraw2::CreateSurface",
                &[
                    ("this", &this),
                    ("desc", &desc),
                    ("lplpDDSurface", &lplpDDSurface),
                    ("pUnkOuter", &pUnkOuter),
                ],
            )
            .enter()
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
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw2") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw2::EnumDisplayModes_pos,
                "ddraw/ddraw2",
                "IDirectDraw2::EnumDisplayModes",
                &[
                    ("this", &this),
                    ("dwFlags", &dwFlags),
                    ("lpSurfaceDesc", &lpSurfaceDesc),
                    ("lpContext", &lpContext),
                    ("lpEnumCallback", &lpEnumCallback),
                ],
            )
            .enter()
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
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn IDirectDraw2_GetDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpDDSurfaceDesc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw2") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw2::GetDisplayMode_pos,
                "ddraw/ddraw2",
                "IDirectDraw2::GetDisplayMode",
                &[("this", &this), ("lpDDSurfaceDesc", &lpDDSurfaceDesc)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDraw2::GetDisplayMode(machine, this, lpDDSurfaceDesc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDraw2_QueryInterface(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let riid = <Option<&GUID>>::from_stack(mem, stack_args + 4u32);
        let ppvObject = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw2") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw2::QueryInterface_pos,
                "ddraw/ddraw2",
                "IDirectDraw2::QueryInterface",
                &[("this", &this), ("riid", &riid), ("ppvObject", &ppvObject)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDraw2::QueryInterface(machine, this, riid, ppvObject);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDraw2_Release(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw2") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw2::Release_pos,
                "ddraw/ddraw2",
                "IDirectDraw2::Release",
                &[("this", &this)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDraw2::Release(machine, this);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDraw2_SetDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let width = <u32>::from_stack(mem, stack_args + 4u32);
        let height = <u32>::from_stack(mem, stack_args + 8u32);
        let bpp = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw2") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw2::SetDisplayMode_pos,
                "ddraw/ddraw2",
                "IDirectDraw2::SetDisplayMode",
                &[
                    ("this", &this),
                    ("width", &width),
                    ("height", &height),
                    ("bpp", &bpp),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDraw2::SetDisplayMode(machine, this, width, height, bpp);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw7::CreatePalette_pos,
                "ddraw/ddraw7",
                "IDirectDraw7::CreatePalette",
                &[
                    ("this", &this),
                    ("flags", &flags),
                    ("entries", &entries),
                    ("lplpPalette", &lplpPalette),
                    ("unused", &unused),
                ],
            )
            .enter()
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
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDraw7_CreateSurface(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let desc = <Option<&DDSURFACEDESC2>>::from_stack(mem, stack_args + 4u32);
        let lpDirectDrawSurface7 = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
        let unused = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw7::CreateSurface_pos,
                "ddraw/ddraw7",
                "IDirectDraw7::CreateSurface",
                &[
                    ("this", &this),
                    ("desc", &desc),
                    ("lpDirectDrawSurface7", &lpDirectDrawSurface7),
                    ("unused", &unused),
                ],
            )
            .enter()
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
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw7::EnumDisplayModes_pos,
                "ddraw/ddraw7",
                "IDirectDraw7::EnumDisplayModes",
                &[
                    ("this", &this),
                    ("dwFlags", &dwFlags),
                    ("lpSurfaceDesc", &lpSurfaceDesc),
                    ("lpContext", &lpContext),
                    ("lpEnumCallback", &lpEnumCallback),
                ],
            )
            .enter()
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
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn IDirectDraw7_GetDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpDDSurfaceDesc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw7::GetDisplayMode_pos,
                "ddraw/ddraw7",
                "IDirectDraw7::GetDisplayMode",
                &[("this", &this), ("lpDDSurfaceDesc", &lpDDSurfaceDesc)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDraw7::GetDisplayMode(machine, this, lpDDSurfaceDesc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDraw7_Release(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw7::Release_pos,
                "ddraw/ddraw7",
                "IDirectDraw7::Release",
                &[("this", &this)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDraw7::Release(machine, this);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDraw7_RestoreDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw7::RestoreDisplayMode_pos,
                "ddraw/ddraw7",
                "IDirectDraw7::RestoreDisplayMode",
                &[("this", &this)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDraw7::RestoreDisplayMode(machine, this);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDraw7_SetCooperativeLevel(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let hwnd = <HWND>::from_stack(mem, stack_args + 4u32);
        let flags = <Result<DDSCL, u32>>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw7::SetCooperativeLevel_pos,
                "ddraw/ddraw7",
                "IDirectDraw7::SetCooperativeLevel",
                &[("this", &this), ("hwnd", &hwnd), ("flags", &flags)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDraw7::SetCooperativeLevel(machine, this, hwnd, flags);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw7::SetDisplayMode_pos,
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
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDraw7::SetDisplayMode(
            machine, this, width, height, bpp, refresh, flags,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDraw7_WaitForVerticalBlank(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let flags = <u32>::from_stack(mem, stack_args + 4u32);
        let _unused = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw7::WaitForVerticalBlank_pos,
                "ddraw/ddraw7",
                "IDirectDraw7::WaitForVerticalBlank",
                &[("this", &this), ("flags", &flags), ("unused", &_unused)],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::ddraw::IDirectDraw7::WaitForVerticalBlank(machine, this, flags, _unused);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawClipper_Release(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("ddraw/clipper") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawClipper::Release_pos,
                "ddraw/clipper",
                "IDirectDrawClipper::Release",
                &[("this", &this)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawClipper::Release(machine, this);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawClipper_SetHWnd(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let unused = <u32>::from_stack(mem, stack_args + 4u32);
        let hwnd = <HWND>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("ddraw/clipper") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawClipper::SetHWnd_pos,
                "ddraw/clipper",
                "IDirectDrawClipper::SetHWnd",
                &[("this", &this), ("unused", &unused), ("hwnd", &hwnd)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawClipper::SetHWnd(machine, this, unused, hwnd);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawPalette_Release(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("ddraw/palette") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawPalette::Release_pos,
                "ddraw/palette",
                "IDirectDrawPalette::Release",
                &[("this", &this)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawPalette::Release(machine, this);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/palette") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawPalette::SetEntries_pos,
                "ddraw/palette",
                "IDirectDrawPalette::SetEntries",
                &[
                    ("this", &this),
                    ("unused", &unused),
                    ("start", &start),
                    ("count", &count),
                    ("entries", &entries),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawPalette::SetEntries(
            machine, this, unused, start, count, entries,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw2") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface2::GetAttachedSurface_pos,
                "ddraw/ddraw2",
                "IDirectDrawSurface2::GetAttachedSurface",
                &[
                    ("this", &this),
                    ("lpDDSCaps", &lpDDSCaps),
                    ("lpDirectDrawSurface", &lpDirectDrawSurface),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface2::GetAttachedSurface(
            machine,
            this,
            lpDDSCaps,
            lpDirectDrawSurface,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface2_GetCaps(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpDDSCAPS = <Option<&mut DDSCAPS>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw2") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface2::GetCaps_pos,
                "ddraw/ddraw2",
                "IDirectDrawSurface2::GetCaps",
                &[("this", &this), ("lpDDSCAPS", &lpDDSCAPS)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface2::GetCaps(machine, this, lpDDSCAPS);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw2") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface2::GetSurfaceDesc_pos,
                "ddraw/ddraw2",
                "IDirectDrawSurface2::GetSurfaceDesc",
                &[("this", &this), ("desc", &desc)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface2::GetSurfaceDesc(machine, this, desc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw2") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface2::Lock_pos,
                "ddraw/ddraw2",
                "IDirectDrawSurface2::Lock",
                &[
                    ("this", &this),
                    ("rect", &rect),
                    ("desc", &desc),
                    ("flags", &flags),
                    ("event", &event),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::ddraw::IDirectDrawSurface2::Lock(machine, this, rect, desc, flags, event);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface2_Release(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw2") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface2::Release_pos,
                "ddraw/ddraw2",
                "IDirectDrawSurface2::Release",
                &[("this", &this)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface2::Release(machine, this);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface2_Unlock(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let ptr = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw2") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface2::Unlock_pos,
                "ddraw/ddraw2",
                "IDirectDrawSurface2::Unlock",
                &[("this", &this), ("ptr", &ptr)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface2::Unlock(machine, this, ptr);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface3_Release(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw3") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface3::Release_pos,
                "ddraw/ddraw3",
                "IDirectDrawSurface3::Release",
                &[("this", &this)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface3::Release(machine, this);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::Blt_pos,
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
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface7::Blt(
            machine, this, lpDstRect, lpSrc, lpSrcRect, flags, lpDDBLTFX,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::BltFast_pos,
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
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::ddraw::IDirectDrawSurface7::BltFast(machine, this, x, y, lpSrc, lpRect, flags);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface7_Flip(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpSurf = <u32>::from_stack(mem, stack_args + 4u32);
        let flags = <Result<DDFLIP, u32>>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::Flip_pos,
                "ddraw/ddraw7",
                "IDirectDrawSurface7::Flip",
                &[("this", &this), ("lpSurf", &lpSurf), ("flags", &flags)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface7::Flip(machine, this, lpSurf, flags);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::GetAttachedSurface_pos,
                "ddraw/ddraw7",
                "IDirectDrawSurface7::GetAttachedSurface",
                &[
                    ("this", &this),
                    ("lpDDSCaps2", &lpDDSCaps2),
                    ("lpDirectDrawSurface7", &lpDirectDrawSurface7),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface7::GetAttachedSurface(
            machine,
            this,
            lpDDSCaps2,
            lpDirectDrawSurface7,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface7_GetCaps(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpDDSCAPS2 = <Option<&mut DDSCAPS2>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::GetCaps_pos,
                "ddraw/ddraw7",
                "IDirectDrawSurface7::GetCaps",
                &[("this", &this), ("lpDDSCAPS2", &lpDDSCAPS2)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface7::GetCaps(machine, this, lpDDSCAPS2);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface7_GetDC(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpHDC = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::GetDC_pos,
                "ddraw/ddraw7",
                "IDirectDrawSurface7::GetDC",
                &[("this", &this), ("lpHDC", &lpHDC)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface7::GetDC(machine, this, lpHDC);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::GetPixelFormat_pos,
                "ddraw/ddraw7",
                "IDirectDrawSurface7::GetPixelFormat",
                &[("this", &this), ("fmt", &fmt)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface7::GetPixelFormat(machine, this, fmt);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::GetSurfaceDesc_pos,
                "ddraw/ddraw7",
                "IDirectDrawSurface7::GetSurfaceDesc",
                &[("this", &this), ("lpDesc", &lpDesc)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface7::GetSurfaceDesc(machine, this, lpDesc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::Lock_pos,
                "ddraw/ddraw7",
                "IDirectDrawSurface7::Lock",
                &[
                    ("this", &this),
                    ("rect", &rect),
                    ("desc", &desc),
                    ("flags", &flags),
                    ("unused", &unused),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::ddraw::IDirectDrawSurface7::Lock(machine, this, rect, desc, flags, unused);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface7_Release(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::Release_pos,
                "ddraw/ddraw7",
                "IDirectDrawSurface7::Release",
                &[("this", &this)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface7::Release(machine, this);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface7_ReleaseDC(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _this = <u32>::from_stack(mem, stack_args + 0u32);
        let _hDC = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::ReleaseDC_pos,
                "ddraw/ddraw7",
                "IDirectDrawSurface7::ReleaseDC",
                &[("this", &_this), ("hDC", &_hDC)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface7::ReleaseDC(machine, _this, _hDC);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface7_Restore(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::Restore_pos,
                "ddraw/ddraw7",
                "IDirectDrawSurface7::Restore",
                &[("this", &_this)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface7::Restore(machine, _this);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface7_SetClipper(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let clipper = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::SetClipper_pos,
                "ddraw/ddraw7",
                "IDirectDrawSurface7::SetClipper",
                &[("this", &this), ("clipper", &clipper)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface7::SetClipper(machine, this, clipper);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface7_SetPalette(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let palette = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::SetPalette_pos,
                "ddraw/ddraw7",
                "IDirectDrawSurface7::SetPalette",
                &[("this", &this), ("palette", &palette)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface7::SetPalette(machine, this, palette);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface7_Unlock(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let rect = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw7") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface7::Unlock_pos,
                "ddraw/ddraw7",
                "IDirectDrawSurface7::Unlock",
                &[("this", &this), ("rect", &rect)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface7::Unlock(machine, this, rect);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw1") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface::GetAttachedSurface_pos,
                "ddraw/ddraw1",
                "IDirectDrawSurface::GetAttachedSurface",
                &[
                    ("this", &this),
                    ("lpDDSCaps", &lpDDSCaps),
                    ("lpDirectDrawSurface", &lpDirectDrawSurface),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface::GetAttachedSurface(
            machine,
            this,
            lpDDSCaps,
            lpDirectDrawSurface,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface_GetCaps(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpDDSCAPS = <Option<&mut DDSCAPS>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw1") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface::GetCaps_pos,
                "ddraw/ddraw1",
                "IDirectDrawSurface::GetCaps",
                &[("this", &this), ("lpDDSCAPS", &lpDDSCAPS)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface::GetCaps(machine, this, lpDDSCAPS);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
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
        let __trace_record = if crate::trace::enabled("ddraw/ddraw1") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface::Lock_pos,
                "ddraw/ddraw1",
                "IDirectDrawSurface::Lock",
                &[
                    ("this", &this),
                    ("rect", &rect),
                    ("desc", &desc),
                    ("flags", &flags),
                    ("event", &event),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::ddraw::IDirectDrawSurface::Lock(machine, this, rect, desc, flags, event);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface_QueryInterface(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let riid = <Option<&GUID>>::from_stack(mem, stack_args + 4u32);
        let ppvObject = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw1") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface::QueryInterface_pos,
                "ddraw/ddraw1",
                "IDirectDrawSurface::QueryInterface",
                &[("this", &this), ("riid", &riid), ("ppvObject", &ppvObject)],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::ddraw::IDirectDrawSurface::QueryInterface(machine, this, riid, ppvObject);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface_Release(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw1") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface::Release_pos,
                "ddraw/ddraw1",
                "IDirectDrawSurface::Release",
                &[("this", &this)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface::Release(machine, this);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDrawSurface_Unlock(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let ptr = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw1") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDrawSurface::Unlock_pos,
                "ddraw/ddraw1",
                "IDirectDrawSurface::Unlock",
                &[("this", &this), ("ptr", &ptr)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDrawSurface::Unlock(machine, this, ptr);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDraw_CreateSurface(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let desc = <Option<&DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
        let lplpDDSurface = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
        let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw1") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw::CreateSurface_pos,
                "ddraw/ddraw1",
                "IDirectDraw::CreateSurface",
                &[
                    ("this", &this),
                    ("desc", &desc),
                    ("lplpDDSurface", &lplpDDSurface),
                    ("pUnkOuter", &pUnkOuter),
                ],
            )
            .enter()
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
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDraw_QueryInterface(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let riid = <Option<&GUID>>::from_stack(mem, stack_args + 4u32);
        let ppvObject = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw1") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw::QueryInterface_pos,
                "ddraw/ddraw1",
                "IDirectDraw::QueryInterface",
                &[("this", &this), ("riid", &riid), ("ppvObject", &ppvObject)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDraw::QueryInterface(machine, this, riid, ppvObject);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDraw_Release(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw1") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw::Release_pos,
                "ddraw/ddraw1",
                "IDirectDraw::Release",
                &[("this", &this)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDraw::Release(machine, this);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectDraw_SetDisplayMode(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let width = <u32>::from_stack(mem, stack_args + 4u32);
        let height = <u32>::from_stack(mem, stack_args + 8u32);
        let bpp = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("ddraw/ddraw1") {
            crate::trace::Record::new(
                winapi::ddraw::IDirectDraw::SetDisplayMode_pos,
                "ddraw/ddraw1",
                "IDirectDraw::SetDisplayMode",
                &[
                    ("this", &this),
                    ("width", &width),
                    ("height", &height),
                    ("bpp", &bpp),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ddraw::IDirectDraw::SetDisplayMode(machine, this, width, height, bpp);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
}
const SHIMS: [Shim; 54usize] = [
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
        name: "IDirectDrawSurface3::Release",
        func: Handler::Sync(wrappers::IDirectDrawSurface3_Release),
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
        name: "IDirectDrawSurface::QueryInterface",
        func: Handler::Sync(wrappers::IDirectDrawSurface_QueryInterface),
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
    raw: std::include_bytes!("../../../dll/ddraw.dll"),
};
