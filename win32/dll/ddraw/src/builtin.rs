#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate as ddraw;
    use crate::*;
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
    pub unsafe fn DirectDrawCreate(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, stack_args + 0u32);
            let lplpDD = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ddraw/ddraw") {
                trace::Record::new(
                    ddraw::ddraw::DirectDrawCreate_pos,
                    "ddraw/ddraw",
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
            let result = ddraw::ddraw::DirectDrawCreate(sys, lpGuid, lplpDD, pUnkOuter);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DirectDrawCreateClipper(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::clipper::*;
        unsafe {
            let mem = sys.mem().detach();
            let dwFlags = <u32>::from_stack(mem, stack_args + 0u32);
            let lplpDDClipper = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ddraw/clipper") {
                trace::Record::new(
                    ddraw::clipper::DirectDrawCreateClipper_pos,
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
                ddraw::clipper::DirectDrawCreateClipper(sys, dwFlags, lplpDDClipper, pUnkOuter);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DirectDrawCreateEx(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, stack_args + 0u32);
            let lplpDD = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let iid = <Option<&GUID>>::from_stack(mem, stack_args + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("ddraw/ddraw") {
                trace::Record::new(
                    ddraw::ddraw::DirectDrawCreateEx_pos,
                    "ddraw/ddraw",
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
            let result = ddraw::ddraw::DirectDrawCreateEx(sys, lpGuid, lplpDD, iid, pUnkOuter);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw2_CreateSurface(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw2::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let desc = <Option<&DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
            let lplpDDSurface = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("ddraw/ddraw2") {
                trace::Record::new(
                    ddraw::ddraw2::IDirectDraw2::CreateSurface_pos,
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
            let result = ddraw::ddraw2::IDirectDraw2::CreateSurface(
                sys,
                this,
                desc,
                lplpDDSurface,
                pUnkOuter,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw2_EnumDisplayModes(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use ddraw::ddraw2::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpSurfaceDesc = <Option<&DDSURFACEDESC>>::from_stack(mem, stack_args + 8u32);
            let lpContext = <u32>::from_stack(mem, stack_args + 12u32);
            let lpEnumCallback = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("ddraw/ddraw2") {
                trace::Record::new(
                    ddraw::ddraw2::IDirectDraw2::EnumDisplayModes_pos,
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
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = ddraw::ddraw2::IDirectDraw2::EnumDisplayModes(
                    sys,
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
                result.into()
            })
        }
    }
    pub unsafe fn IDirectDraw2_GetDisplayMode(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw2::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSurfaceDesc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw2") {
                trace::Record::new(
                    ddraw::ddraw2::IDirectDraw2::GetDisplayMode_pos,
                    "ddraw/ddraw2",
                    "IDirectDraw2::GetDisplayMode",
                    &[("this", &this), ("lpDDSurfaceDesc", &lpDDSurfaceDesc)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw2::IDirectDraw2::GetDisplayMode(sys, this, lpDDSurfaceDesc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw2_QueryInterface(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw2::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let riid = <Option<&GUID>>::from_stack(mem, stack_args + 4u32);
            let ppvObject = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ddraw/ddraw2") {
                trace::Record::new(
                    ddraw::ddraw2::IDirectDraw2::QueryInterface_pos,
                    "ddraw/ddraw2",
                    "IDirectDraw2::QueryInterface",
                    &[("this", &this), ("riid", &riid), ("ppvObject", &ppvObject)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw2::IDirectDraw2::QueryInterface(sys, this, riid, ppvObject);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw2_Release(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw2::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ddraw/ddraw2") {
                trace::Record::new(
                    ddraw::ddraw2::IDirectDraw2::Release_pos,
                    "ddraw/ddraw2",
                    "IDirectDraw2::Release",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw2::IDirectDraw2::Release(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw2_SetDisplayMode(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw2::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let width = <u32>::from_stack(mem, stack_args + 4u32);
            let height = <u32>::from_stack(mem, stack_args + 8u32);
            let bpp = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("ddraw/ddraw2") {
                trace::Record::new(
                    ddraw::ddraw2::IDirectDraw2::SetDisplayMode_pos,
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
            let result = ddraw::ddraw2::IDirectDraw2::SetDisplayMode(sys, this, width, height, bpp);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw7_CreateClipper(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let unused = <u32>::from_stack(mem, stack_args + 4u32);
            let lplpClipper = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let reserved = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDraw7::CreateClipper_pos,
                    "ddraw/ddraw7",
                    "IDirectDraw7::CreateClipper",
                    &[
                        ("this", &this),
                        ("unused", &unused),
                        ("lplpClipper", &lplpClipper),
                        ("reserved", &reserved),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDraw7::CreateClipper(
                sys,
                this,
                unused,
                lplpClipper,
                reserved,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw7_CreatePalette(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let flags = <Result<DDPCAPS, u32>>::from_stack(mem, stack_args + 4u32);
            let entries = <u32>::from_stack(mem, stack_args + 8u32);
            let lplpPalette = <u32>::from_stack(mem, stack_args + 12u32);
            let unused = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDraw7::CreatePalette_pos,
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
            let result = ddraw::ddraw7::IDirectDraw7::CreatePalette(
                sys,
                this,
                flags,
                entries,
                lplpPalette,
                unused,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw7_CreateSurface(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let desc = <Option<&DDSURFACEDESC2>>::from_stack(mem, stack_args + 4u32);
            let lpDirectDrawSurface7 = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let unused = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDraw7::CreateSurface_pos,
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
            let result = ddraw::ddraw7::IDirectDraw7::CreateSurface(
                sys,
                this,
                desc,
                lpDirectDrawSurface7,
                unused,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw7_EnumDisplayModes(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpSurfaceDesc = <Option<&DDSURFACEDESC2>>::from_stack(mem, stack_args + 8u32);
            let lpContext = <u32>::from_stack(mem, stack_args + 12u32);
            let lpEnumCallback = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDraw7::EnumDisplayModes_pos,
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
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = ddraw::ddraw7::IDirectDraw7::EnumDisplayModes(
                    sys,
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
                result.into()
            })
        }
    }
    pub unsafe fn IDirectDraw7_GetDisplayMode(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSurfaceDesc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDraw7::GetDisplayMode_pos,
                    "ddraw/ddraw7",
                    "IDirectDraw7::GetDisplayMode",
                    &[("this", &this), ("lpDDSurfaceDesc", &lpDDSurfaceDesc)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDraw7::GetDisplayMode(sys, this, lpDDSurfaceDesc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw7_Release(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDraw7::Release_pos,
                    "ddraw/ddraw7",
                    "IDirectDraw7::Release",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDraw7::Release(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw7_RestoreDisplayMode(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDraw7::RestoreDisplayMode_pos,
                    "ddraw/ddraw7",
                    "IDirectDraw7::RestoreDisplayMode",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDraw7::RestoreDisplayMode(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw7_SetCooperativeLevel(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let hwnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let flags = <Result<DDSCL, u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDraw7::SetCooperativeLevel_pos,
                    "ddraw/ddraw7",
                    "IDirectDraw7::SetCooperativeLevel",
                    &[("this", &this), ("hwnd", &hwnd), ("flags", &flags)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result =
                    ddraw::ddraw7::IDirectDraw7::SetCooperativeLevel(sys, this, hwnd, flags).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn IDirectDraw7_SetDisplayMode(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let width = <u32>::from_stack(mem, stack_args + 4u32);
            let height = <u32>::from_stack(mem, stack_args + 8u32);
            let bpp = <u32>::from_stack(mem, stack_args + 12u32);
            let refresh = <u32>::from_stack(mem, stack_args + 16u32);
            let flags = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDraw7::SetDisplayMode_pos,
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
            let result = ddraw::ddraw7::IDirectDraw7::SetDisplayMode(
                sys, this, width, height, bpp, refresh, flags,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw7_WaitForVerticalBlank(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let flags = <u32>::from_stack(mem, stack_args + 4u32);
            let _unused = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDraw7::WaitForVerticalBlank_pos,
                    "ddraw/ddraw7",
                    "IDirectDraw7::WaitForVerticalBlank",
                    &[("this", &this), ("flags", &flags), ("unused", &_unused)],
                )
                .enter()
            } else {
                None
            };
            let result =
                ddraw::ddraw7::IDirectDraw7::WaitForVerticalBlank(sys, this, flags, _unused);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawClipper_Release(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::clipper::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ddraw/clipper") {
                trace::Record::new(
                    ddraw::clipper::IDirectDrawClipper::Release_pos,
                    "ddraw/clipper",
                    "IDirectDrawClipper::Release",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::clipper::IDirectDrawClipper::Release(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawClipper_SetHWnd(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::clipper::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let unused = <u32>::from_stack(mem, stack_args + 4u32);
            let hwnd = <HWND>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ddraw/clipper") {
                trace::Record::new(
                    ddraw::clipper::IDirectDrawClipper::SetHWnd_pos,
                    "ddraw/clipper",
                    "IDirectDrawClipper::SetHWnd",
                    &[("this", &this), ("unused", &unused), ("hwnd", &hwnd)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::clipper::IDirectDrawClipper::SetHWnd(sys, this, unused, hwnd);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawPalette_Release(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::palette::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ddraw/palette") {
                trace::Record::new(
                    ddraw::palette::IDirectDrawPalette::Release_pos,
                    "ddraw/palette",
                    "IDirectDrawPalette::Release",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::palette::IDirectDrawPalette::Release(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawPalette_SetEntries(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ddraw::palette::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let unused = <u32>::from_stack(mem, stack_args + 4u32);
            let start = <u32>::from_stack(mem, stack_args + 8u32);
            let count = <u32>::from_stack(mem, stack_args + 12u32);
            let entries = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("ddraw/palette") {
                trace::Record::new(
                    ddraw::palette::IDirectDrawPalette::SetEntries_pos,
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
            let result = ddraw::palette::IDirectDrawPalette::SetEntries(
                sys, this, unused, start, count, entries,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface2_GetAttachedSurface(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ddraw::ddraw2::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCaps = <Option<&DDSCAPS>>::from_stack(mem, stack_args + 4u32);
            let lpDirectDrawSurface = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ddraw/ddraw2") {
                trace::Record::new(
                    ddraw::ddraw2::IDirectDrawSurface2::GetAttachedSurface_pos,
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
            let result = ddraw::ddraw2::IDirectDrawSurface2::GetAttachedSurface(
                sys,
                this,
                lpDDSCaps,
                lpDirectDrawSurface,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface2_GetCaps(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw2::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCAPS = <Option<&mut DDSCAPS>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw2") {
                trace::Record::new(
                    ddraw::ddraw2::IDirectDrawSurface2::GetCaps_pos,
                    "ddraw/ddraw2",
                    "IDirectDrawSurface2::GetCaps",
                    &[("this", &this), ("lpDDSCAPS", &lpDDSCAPS)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw2::IDirectDrawSurface2::GetCaps(sys, this, lpDDSCAPS);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface2_GetSurfaceDesc(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ddraw::ddraw2::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw2") {
                trace::Record::new(
                    ddraw::ddraw2::IDirectDrawSurface2::GetSurfaceDesc_pos,
                    "ddraw/ddraw2",
                    "IDirectDrawSurface2::GetSurfaceDesc",
                    &[("this", &this), ("desc", &desc)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw2::IDirectDrawSurface2::GetSurfaceDesc(sys, this, desc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface2_Lock(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw2::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let rect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, stack_args + 8u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, stack_args + 12u32);
            let event = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("ddraw/ddraw2") {
                trace::Record::new(
                    ddraw::ddraw2::IDirectDrawSurface2::Lock_pos,
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
                ddraw::ddraw2::IDirectDrawSurface2::Lock(sys, this, rect, desc, flags, event);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface2_Release(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw2::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ddraw/ddraw2") {
                trace::Record::new(
                    ddraw::ddraw2::IDirectDrawSurface2::Release_pos,
                    "ddraw/ddraw2",
                    "IDirectDrawSurface2::Release",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw2::IDirectDrawSurface2::Release(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface2_Unlock(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw2::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let ptr = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw2") {
                trace::Record::new(
                    ddraw::ddraw2::IDirectDrawSurface2::Unlock_pos,
                    "ddraw/ddraw2",
                    "IDirectDrawSurface2::Unlock",
                    &[("this", &this), ("ptr", &ptr)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw2::IDirectDrawSurface2::Unlock(sys, this, ptr);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface3_Release(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw3::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ddraw/ddraw3") {
                trace::Record::new(
                    ddraw::ddraw3::IDirectDrawSurface3::Release_pos,
                    "ddraw/ddraw3",
                    "IDirectDrawSurface3::Release",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw3::IDirectDrawSurface3::Release(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_Blt(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDstRect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let lpSrc = <u32>::from_stack(mem, stack_args + 8u32);
            let lpSrcRect = <Option<&RECT>>::from_stack(mem, stack_args + 12u32);
            let flags = <Result<DDBLT, u32>>::from_stack(mem, stack_args + 16u32);
            let lpDDBLTFX = <Option<&DDBLTFX>>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::Blt_pos,
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
            let result = ddraw::ddraw7::IDirectDrawSurface7::Blt(
                sys, this, lpDstRect, lpSrc, lpSrcRect, flags, lpDDBLTFX,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_BltFast(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let lpSrc = <u32>::from_stack(mem, stack_args + 12u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 16u32);
            let flags = <Result<DDBLTFAST, u32>>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::BltFast_pos,
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
                ddraw::ddraw7::IDirectDrawSurface7::BltFast(sys, this, x, y, lpSrc, lpRect, flags);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_Flip(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpSurf = <u32>::from_stack(mem, stack_args + 4u32);
            let flags = <Result<DDFLIP, u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::Flip_pos,
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::Flip",
                    &[("this", &this), ("lpSurf", &lpSurf), ("flags", &flags)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDrawSurface7::Flip(sys, this, lpSurf, flags);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_GetAttachedSurface(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCaps2 = <Option<&DDSCAPS2>>::from_stack(mem, stack_args + 4u32);
            let lpDirectDrawSurface7 = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::GetAttachedSurface_pos,
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
            let result = ddraw::ddraw7::IDirectDrawSurface7::GetAttachedSurface(
                sys,
                this,
                lpDDSCaps2,
                lpDirectDrawSurface7,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_GetCaps(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCAPS2 = <Option<&mut DDSCAPS2>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::GetCaps_pos,
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::GetCaps",
                    &[("this", &this), ("lpDDSCAPS2", &lpDDSCAPS2)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDrawSurface7::GetCaps(sys, this, lpDDSCAPS2);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_GetDC(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpHDC = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::GetDC_pos,
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::GetDC",
                    &[("this", &this), ("lpHDC", &lpHDC)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDrawSurface7::GetDC(sys, this, lpHDC);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_GetPixelFormat(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let fmt = <Option<&mut DDPIXELFORMAT>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::GetPixelFormat_pos,
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::GetPixelFormat",
                    &[("this", &this), ("fmt", &fmt)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDrawSurface7::GetPixelFormat(sys, this, fmt);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_GetSurfaceDesc(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDesc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::GetSurfaceDesc_pos,
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::GetSurfaceDesc",
                    &[("this", &this), ("lpDesc", &lpDesc)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDrawSurface7::GetSurfaceDesc(sys, this, lpDesc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_IsLost(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::IsLost_pos,
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::IsLost",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDrawSurface7::IsLost(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_Lock(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let rect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let desc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, stack_args + 8u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, stack_args + 12u32);
            let unused = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::Lock_pos,
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
                ddraw::ddraw7::IDirectDrawSurface7::Lock(sys, this, rect, desc, flags, unused);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_Release(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::Release_pos,
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::Release",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDrawSurface7::Release(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_ReleaseDC(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let _this = <u32>::from_stack(mem, stack_args + 0u32);
            let _hDC = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::ReleaseDC_pos,
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::ReleaseDC",
                    &[("this", &_this), ("hDC", &_hDC)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDrawSurface7::ReleaseDC(sys, _this, _hDC);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_Restore(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let _this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::Restore_pos,
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::Restore",
                    &[("this", &_this)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDrawSurface7::Restore(sys, _this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_SetClipper(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let clipper = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::SetClipper_pos,
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::SetClipper",
                    &[("this", &this), ("clipper", &clipper)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDrawSurface7::SetClipper(sys, this, clipper);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_SetColorKey(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let flags = <Result<DDCKEY, u32>>::from_stack(mem, stack_args + 4u32);
            let key = <Option<&DDCOLORKEY>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::SetColorKey_pos,
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::SetColorKey",
                    &[("this", &this), ("flags", &flags), ("key", &key)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDrawSurface7::SetColorKey(sys, this, flags, key);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_SetPalette(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let palette = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::SetPalette_pos,
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::SetPalette",
                    &[("this", &this), ("palette", &palette)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDrawSurface7::SetPalette(sys, this, palette);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface7_Unlock(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw7::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let rect = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw7") {
                trace::Record::new(
                    ddraw::ddraw7::IDirectDrawSurface7::Unlock_pos,
                    "ddraw/ddraw7",
                    "IDirectDrawSurface7::Unlock",
                    &[("this", &this), ("rect", &rect)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw7::IDirectDrawSurface7::Unlock(sys, this, rect);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface_GetAttachedSurface(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ddraw::ddraw1::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCaps = <Option<&DDSCAPS>>::from_stack(mem, stack_args + 4u32);
            let lpDirectDrawSurface = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ddraw/ddraw1") {
                trace::Record::new(
                    ddraw::ddraw1::IDirectDrawSurface::GetAttachedSurface_pos,
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
            let result = ddraw::ddraw1::IDirectDrawSurface::GetAttachedSurface(
                sys,
                this,
                lpDDSCaps,
                lpDirectDrawSurface,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface_GetCaps(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw1::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpDDSCAPS = <Option<&mut DDSCAPS>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw1") {
                trace::Record::new(
                    ddraw::ddraw1::IDirectDrawSurface::GetCaps_pos,
                    "ddraw/ddraw1",
                    "IDirectDrawSurface::GetCaps",
                    &[("this", &this), ("lpDDSCAPS", &lpDDSCAPS)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw1::IDirectDrawSurface::GetCaps(sys, this, lpDDSCAPS);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface_Lock(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw1::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let rect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, stack_args + 8u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, stack_args + 12u32);
            let event = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("ddraw/ddraw1") {
                trace::Record::new(
                    ddraw::ddraw1::IDirectDrawSurface::Lock_pos,
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
                ddraw::ddraw1::IDirectDrawSurface::Lock(sys, this, rect, desc, flags, event);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface_QueryInterface(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        use ddraw::ddraw1::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let riid = <Option<&GUID>>::from_stack(mem, stack_args + 4u32);
            let ppvObject = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ddraw/ddraw1") {
                trace::Record::new(
                    ddraw::ddraw1::IDirectDrawSurface::QueryInterface_pos,
                    "ddraw/ddraw1",
                    "IDirectDrawSurface::QueryInterface",
                    &[("this", &this), ("riid", &riid), ("ppvObject", &ppvObject)],
                )
                .enter()
            } else {
                None
            };
            let result =
                ddraw::ddraw1::IDirectDrawSurface::QueryInterface(sys, this, riid, ppvObject);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface_Release(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw1::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ddraw/ddraw1") {
                trace::Record::new(
                    ddraw::ddraw1::IDirectDrawSurface::Release_pos,
                    "ddraw/ddraw1",
                    "IDirectDrawSurface::Release",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw1::IDirectDrawSurface::Release(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDrawSurface_Unlock(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw1::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let ptr = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("ddraw/ddraw1") {
                trace::Record::new(
                    ddraw::ddraw1::IDirectDrawSurface::Unlock_pos,
                    "ddraw/ddraw1",
                    "IDirectDrawSurface::Unlock",
                    &[("this", &this), ("ptr", &ptr)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw1::IDirectDrawSurface::Unlock(sys, this, ptr);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw_CreateSurface(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw1::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let desc = <Option<&DDSURFACEDESC>>::from_stack(mem, stack_args + 4u32);
            let lplpDDSurface = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("ddraw/ddraw1") {
                trace::Record::new(
                    ddraw::ddraw1::IDirectDraw::CreateSurface_pos,
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
            let result = ddraw::ddraw1::IDirectDraw::CreateSurface(
                sys,
                this,
                desc,
                lplpDDSurface,
                pUnkOuter,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw_QueryInterface(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw1::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let riid = <Option<&GUID>>::from_stack(mem, stack_args + 4u32);
            let ppvObject = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("ddraw/ddraw1") {
                trace::Record::new(
                    ddraw::ddraw1::IDirectDraw::QueryInterface_pos,
                    "ddraw/ddraw1",
                    "IDirectDraw::QueryInterface",
                    &[("this", &this), ("riid", &riid), ("ppvObject", &ppvObject)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw1::IDirectDraw::QueryInterface(sys, this, riid, ppvObject);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw_Release(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw1::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("ddraw/ddraw1") {
                trace::Record::new(
                    ddraw::ddraw1::IDirectDraw::Release_pos,
                    "ddraw/ddraw1",
                    "IDirectDraw::Release",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = ddraw::ddraw1::IDirectDraw::Release(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectDraw_SetDisplayMode(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use ddraw::ddraw1::*;
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let width = <u32>::from_stack(mem, stack_args + 4u32);
            let height = <u32>::from_stack(mem, stack_args + 8u32);
            let bpp = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("ddraw/ddraw1") {
                trace::Record::new(
                    ddraw::ddraw1::IDirectDraw::SetDisplayMode_pos,
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
            let result = ddraw::ddraw1::IDirectDraw::SetDisplayMode(sys, this, width, height, bpp);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
}
const SHIMS: [Shim; 57usize] = [
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
        name: "IDirectDraw7::CreateClipper",
        func: Handler::Sync(wrappers::IDirectDraw7_CreateClipper),
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
        func: Handler::Async(wrappers::IDirectDraw7_SetCooperativeLevel),
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
        name: "IDirectDrawSurface7::IsLost",
        func: Handler::Sync(wrappers::IDirectDrawSurface7_IsLost),
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
        name: "IDirectDrawSurface7::SetColorKey",
        func: Handler::Sync(wrappers::IDirectDrawSurface7_SetColorKey),
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
    raw: std::include_bytes!("../ddraw.dll"),
};
