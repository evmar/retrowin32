//! Implementation of DirectDraw1 interfaces, which typically don't have
//! a "1" suffix but contrast with intefaces with names like IDirectDraw7.

use super::{
    ddraw2, ddraw3,
    ddraw7::{IDirectDraw7, IDirectDrawSurface7},
    types::*,
};
use crate::{
    winapi::{
        com::{vtable, GUID},
        ddraw,
        kernel32::get_symbol,
        *,
    },
    Machine,
};
use memory::ExtensionsMut;

#[win32_derive::dllexport]
pub mod IDirectDraw {
    use super::*;

    vtable![
        QueryInterface: ok,
        AddRef: todo,
        Release: ok,
        Compact: todo,
        CreateClipper: (IDirectDraw7::CreateClipper),
        CreatePalette: (IDirectDraw7::CreatePalette),
        CreateSurface: ok,
        DuplicateSurface: todo,
        EnumDisplayModes: (IDirectDraw2::EnumDisplayModes),
        EnumSurfaces: todo,
        FlipToGDISurface: todo,
        GetCaps: todo,
        GetDisplayMode: todo,
        GetFourCCCodes: todo,
        GetGDISurface: todo,
        GetMonitorFrequency: todo,
        GetScanLine: todo,
        GetVerticalBlankStatus: todo,
        Initialize: todo,
        RestoreDisplayMode: (IDirectDraw7::RestoreDisplayMode),
        SetCooperativeLevel: (IDirectDraw7::SetCooperativeLevel),
        SetDisplayMode: ok,
        WaitForVerticalBlank: (IDirectDraw7::WaitForVerticalBlank),
    ];

    pub fn new(machine: &mut Machine) -> u32 {
        let lpDirectDraw = machine
            .state
            .kernel32
            .process_heap
            .alloc(machine.emu.memory.mem(), 4);
        let vtable = get_symbol(machine, "ddraw.dll", "IDirectDraw");
        machine.mem().put_pod::<u32>(lpDirectDraw, vtable);
        lpDirectDraw
    }

    #[win32_derive::dllexport]
    pub fn QueryInterface(
        machine: &mut Machine,
        this: u32,
        riid: Option<&GUID>,
        ppvObject: Option<&mut u32>,
    ) -> DD {
        match riid.unwrap() {
            &ddraw2::IID_IDirectDraw2 => {
                *ppvObject.unwrap() = ddraw2::IDirectDraw2::new(machine);
                DD::OK
            }
            _ => DD::E_NOINTERFACE,
        }
    }

    #[win32_derive::dllexport]
    pub fn CreateSurface(
        machine: &mut Machine,
        this: u32,
        desc: Option<&DDSURFACEDESC>,
        lplpDDSurface: Option<&mut u32>,
        pUnkOuter: u32,
    ) -> DD {
        if machine.state.ddraw.hwnd.is_null() {
            // This can happen if the app never called SetCooperativeLevel, but it
            // can also happen if it called SetCooperativeLevel with DDSCL_NORMAL and
            // passed a null hwnd.  In that case it appears at least in 5days it is
            // just probing for DirectDraw capability and doesn't even have a window(?).
            *lplpDDSurface.unwrap() = IDirectDrawSurface::new(machine);
            // TODO: register surface in surfaces list?
            return DD::OK;
        }

        let surfaces = ddraw::Surface::create(
            machine,
            machine.state.ddraw.hwnd,
            &DDSURFACEDESC2::from_desc(desc.unwrap()),
        );

        let mut prev = 0;
        for mut surface in surfaces.into_iter().rev() {
            let ptr = IDirectDrawSurface::new(machine);
            surface.attached = prev;
            machine.state.ddraw.surfaces.insert(ptr, surface);
            prev = ptr;
        }

        *lplpDDSurface.unwrap() = prev;

        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Release(_machine: &mut Machine, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    pub fn SetDisplayMode(
        machine: &mut Machine,
        this: u32,
        width: u32,
        height: u32,
        bpp: u32,
    ) -> DD {
        IDirectDraw7::SetDisplayMode(machine, 0, width, height, bpp, 0, 0)
    }
}

#[win32_derive::dllexport]
pub mod IDirectDrawSurface {
    use super::*;

    vtable![
        QueryInterface: ok,
        AddRef: todo,
        Release: ok,
        AddAttachedSurface: todo,
        AddOverlayDirtyRect: todo,
        Blt: (IDirectDrawSurface7::Blt),
        BltBatch: todo,
        BltFast: todo,
        DeleteAttachedSurface: todo,
        EnumAttachedSurfaces: todo,
        EnumOverlayZOrders: todo,
        Flip: (IDirectDrawSurface7::Flip),
        GetAttachedSurface: ok,
        GetBltStatus: todo,
        GetCaps: ok,
        GetClipper: todo,
        GetColorKey: todo,
        GetDC: (IDirectDrawSurface7::GetDC),
        GetFlipStatus: todo,
        GetOverlayPosition: todo,
        GetPalette: todo,
        GetPixelFormat: (IDirectDrawSurface7::GetPixelFormat),
        GetSurfaceDesc: (IDirectDrawSurface2::GetSurfaceDesc),
        Initialize: todo,
        IsLost: (IDirectDrawSurface7::IsLost),
        Lock: ok,
        ReleaseDC: (IDirectDrawSurface7::ReleaseDC),
        Restore: todo,
        SetClipper: (IDirectDrawSurface7::SetClipper),
        SetColorKey: (IDirectDrawSurface7::SetColorKey),
        SetOverlayPosition: todo,
        SetPalette: (IDirectDrawSurface7::SetPalette),
        Unlock: ok,
        UpdateOverlay: todo,
        UpdateOverlayDisplay: todo,
        UpdateOverlayZOrder: todo,
    ];

    pub fn new(machine: &mut Machine) -> u32 {
        let lpDirectDrawSurface = machine
            .state
            .kernel32
            .process_heap
            .alloc(machine.emu.memory.mem(), 4);
        let vtable = get_symbol(machine, "ddraw.dll", "IDirectDrawSurface");
        machine.mem().put_pod::<u32>(lpDirectDrawSurface, vtable);
        lpDirectDrawSurface
    }

    #[win32_derive::dllexport]
    pub fn QueryInterface(
        machine: &mut Machine,
        this: u32,
        riid: Option<&GUID>,
        ppvObject: Option<&mut u32>,
    ) -> DD {
        match riid.unwrap() {
            &ddraw3::IID_IDirectDraw3 => {
                let ptr = ddraw3::IDirectDrawSurface3::new(machine);
                // TODO: register ptr as a surface, but we need to share the same surface
                // between multiple ptrs then?
                *ppvObject.unwrap() = ptr;
                DD::OK
            }
            _ => DD::E_NOINTERFACE,
        }
    }

    #[win32_derive::dllexport]
    pub fn Release(_machine: &mut Machine, this: u32) -> u32 {
        0
    }

    #[win32_derive::dllexport]
    pub fn GetAttachedSurface(
        machine: &mut Machine,
        this: u32,
        lpDDSCaps: Option<&DDSCAPS>,
        lpDirectDrawSurface: Option<&mut u32>,
    ) -> DD {
        // TODO: consider caps.
        let surface = machine.state.ddraw.surfaces.get(&this).unwrap();
        *lpDirectDrawSurface.unwrap() = surface.attached;
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(_machine: &mut Machine, this: u32, lpDDSCAPS: Option<&mut DDSCAPS>) -> DD {
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Lock(
        machine: &mut Machine,
        this: u32,
        rect: Option<&RECT>,
        desc: Option<&mut DDSURFACEDESC>,
        flags: Result<DDLOCK, u32>,
        event: u32,
    ) -> DD {
        if event != 0 {
            todo!()
        }
        let desc = desc.unwrap();
        let mut desc2 = DDSURFACEDESC2::from_desc(desc);
        desc2.dwSize = std::mem::size_of::<DDSURFACEDESC2>() as u32;
        let ret = IDirectDrawSurface7::Lock(machine, this, rect, Some(&mut desc2), flags, 0);
        if ret != DD::OK {
            return ret;
        }

        *desc = DDSURFACEDESC::from_desc2(&desc2);

        ret
    }

    #[win32_derive::dllexport]
    pub fn Unlock(machine: &mut Machine, this: u32, ptr: u32) -> DD {
        IDirectDrawSurface7::Unlock(machine, this, None)
    }
}
