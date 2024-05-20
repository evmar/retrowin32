//! Implementation of DirectDraw1 interfaces, which typically don't have
//! a "1" suffix but contrast with intefaces with names like IDirectDraw7.

use super::{
    ddraw7::{IDirectDraw7, IDirectDrawSurface7},
    types::*,
    State, DD_OK,
};
use crate::{
    machine::Emulator,
    winapi::{ddraw, types::*, vtable},
    Machine,
};
use memory::Pod;

const TRACE_CONTEXT: &'static str = "ddraw/1";

#[win32_derive::shims_from_x86]
pub(super) mod IDirectDraw {
    use super::*;

    vtable![IDirectDraw shims
        QueryInterface todo,
        AddRef todo,
        Release ok,
        Compact todo,
        CreateClipper todo,
        CreatePalette (IDirectDraw7::shims::CreatePalette),
        CreateSurface ok,
        DuplicateSurface todo,
        EnumDisplayModes ok,
        EnumSurfaces todo,
        FlipToGDISurface todo,
        GetCaps todo,
        GetDisplayMode todo,
        GetFourCCCodes todo,
        GetGDISurface todo,
        GetMonitorFrequency todo,
        GetScanLine todo,
        GetVerticalBlankStatus todo,
        Initialize todo,
        RestoreDisplayMode (IDirectDraw7::shims::RestoreDisplayMode),
        SetCooperativeLevel (IDirectDraw7::shims::SetCooperativeLevel),
        SetDisplayMode ok,
        WaitForVerticalBlank (IDirectDraw7::shims::WaitForVerticalBlank),
    ];

    #[win32_derive::dllexport]
    fn CreateSurface(
        machine: &mut Machine,
        this: u32,
        desc: Option<&DDSURFACEDESC>,
        lplpDDSurface: Option<&mut u32>,
        pUnkOuter: u32,
    ) -> u32 {
        let surfaces = ddraw::Surface::create(
            machine,
            machine.state.ddraw.hwnd,
            &DDSURFACEDESC2::from_desc(desc.unwrap()),
        );
        if surfaces.len() > 2 {
            todo!()
        }

        let mut prev = 0;
        for mut surface in surfaces.into_iter().rev() {
            let ptr = IDirectDrawSurface::new(machine);
            surface.attached = prev;
            machine.state.ddraw.surfaces.insert(ptr, surface);
            prev = ptr;
        }

        *lplpDDSurface.unwrap() = prev;

        DD_OK
    }

    #[win32_derive::dllexport]
    async fn EnumDisplayModes(
        machine: &mut Machine,
        this: u32,
        dwFlags: u32,
        lpSurfaceDesc: Option<&DDSURFACEDESC>,
        lpContext: u32,
        lpEnumCallback: u32,
    ) -> u32 {
        if lpSurfaceDesc.is_some() {
            todo!()
        }
        let mem = machine.emu.memory.mem();
        let desc_addr = machine
            .state
            .ddraw
            .heap
            .alloc(mem, std::mem::size_of::<DDSURFACEDESC>() as u32);
        let desc = mem.view_mut::<DDSURFACEDESC>(desc_addr);
        desc.clear_struct();
        // TODO: offer multiple display modes rather than hardcoding this one.
        desc.dwSize = std::mem::size_of::<DDSURFACEDESC>() as u32;
        desc.dwWidth = 320;
        desc.dwHeight = 200;
        desc.ddpfPixelFormat = DDPIXELFORMAT {
            dwSize: std::mem::size_of::<DDPIXELFORMAT>() as u32,
            dwFlags: 0,
            dwFourCC: 0,
            dwRGBBitCount: 8,
            dwRBitMask: 0xFF000000,
            dwGBitMask: 0x00FF0000,
            dwBBitMask: 0x0000FF00,
            dwRGBAlphaBitMask: 0x000000FF,
        };

        machine
            .call_x86(lpEnumCallback, vec![desc_addr, lpContext])
            .await;

        machine
            .state
            .ddraw
            .heap
            .free(machine.emu.memory.mem(), desc_addr);

        DD_OK
    }

    #[win32_derive::dllexport]
    fn Release(_machine: &mut Machine, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    fn SetDisplayMode(machine: &mut Machine, this: u32, width: u32, height: u32, bpp: u32) -> u32 {
        IDirectDraw7::SetDisplayMode(machine, 0, width, height, bpp, 0, 0)
    }
}

#[win32_derive::shims_from_x86]
pub(super) mod IDirectDrawSurface {
    use super::*;

    vtable![IDirectDrawSurface shims
        QueryInterface todo,
        AddRef todo,
        Release ok,
        AddAttachedSurface todo,
        AddOverlayDirtyRect todo,
        Blt (IDirectDrawSurface7::shims::Blt),
        BltBatch todo,
        BltFast todo,
        DeleteAttachedSurface todo,
        EnumAttachedSurfaces todo,
        EnumOverlayZOrders todo,
        Flip (IDirectDrawSurface7::shims::Flip),
        GetAttachedSurface ok,
        GetBltStatus todo,
        GetCaps ok,
        GetClipper todo,
        GetColorKey todo,
        GetDC (IDirectDrawSurface7::shims::GetDC),
        GetFlipStatus todo,
        GetOverlayPosition todo,
        GetPalette todo,
        GetPixelFormat ok,
        GetSurfaceDesc ok,
        Initialize todo,
        IsLost todo,
        Lock ok,
        ReleaseDC (IDirectDrawSurface7::shims::ReleaseDC),
        Restore todo,
        SetClipper todo,
        SetColorKey todo,
        SetOverlayPosition todo,
        SetPalette (IDirectDrawSurface7::shims::SetPalette),
        Unlock ok,
        UpdateOverlay todo,
        UpdateOverlayDisplay todo,
        UpdateOverlayZOrder todo,
    ];

    pub fn new(machine: &mut Machine) -> u32 {
        let ddraw = &mut machine.state.ddraw;
        let lpDirectDrawSurface = ddraw.heap.alloc(machine.emu.memory.mem(), 4);
        let vtable = ddraw.vtable_IDirectDrawSurface;
        machine.mem().put::<u32>(lpDirectDrawSurface, vtable);
        lpDirectDrawSurface
    }

    #[win32_derive::dllexport]
    pub fn Release(_machine: &mut Machine, this: u32) -> u32 {
        0
    }

    #[win32_derive::dllexport]
    fn GetAttachedSurface(
        machine: &mut Machine,
        this: u32,
        lpDDSCaps: Option<&DDSCAPS>,
        lpDirectDrawSurface: Option<&mut u32>,
    ) -> u32 {
        // TODO: consider caps.
        let surface = machine.state.ddraw.surfaces.get(&this).unwrap();
        *lpDirectDrawSurface.unwrap() = surface.attached;
        DD_OK
    }

    #[win32_derive::dllexport]
    fn GetCaps(_machine: &mut Machine, this: u32, lpDDSCAPS: Option<&mut DDSCAPS>) -> u32 {
        DD_OK
    }

    #[win32_derive::dllexport]
    fn GetPixelFormat(_machine: &mut Machine, this: u32, fmt: Option<&mut DDPIXELFORMAT>) -> u32 {
        let fmt = fmt.unwrap();
        *fmt = unsafe { std::mem::zeroed() };
        fmt.dwSize = std::mem::size_of::<DDPIXELFORMAT>() as u32;
        DD_OK
    }

    #[win32_derive::dllexport]
    fn GetSurfaceDesc(machine: &mut Machine, this: u32, desc: Option<&mut DDSURFACEDESC>) -> u32 {
        let surface = machine.state.ddraw.surfaces.get(&this).unwrap();
        let desc = desc.unwrap();
        desc.dwWidth = surface.width;
        desc.dwHeight = surface.height;
        desc.dwFlags = DDSD::WIDTH | DDSD::HEIGHT;
        // TODO: fill out more of desc?
        DD_OK
    }

    #[win32_derive::dllexport]
    fn Lock(
        machine: &mut Machine,
        this: u32,
        rect: Option<&RECT>,
        desc: Option<&mut DDSURFACEDESC>,
        flags: Result<DDLOCK, u32>,
        event: u32,
    ) -> u32 {
        if event != 0 {
            todo!()
        }
        let desc = desc.unwrap();
        let mut desc2 = DDSURFACEDESC2::from_desc(desc);
        desc2.dwSize = std::mem::size_of::<DDSURFACEDESC2>() as u32;
        let ret = IDirectDrawSurface7::Lock(machine, this, rect, Some(&mut desc2), flags, 0);
        if ret != DD_OK {
            return ret;
        }

        *desc = DDSURFACEDESC::from_desc2(&desc2);

        ret
    }

    #[win32_derive::dllexport]
    fn Unlock(machine: &mut Machine, this: u32, ptr: u32) -> u32 {
        IDirectDrawSurface7::Unlock(machine, this, None)
    }
}
