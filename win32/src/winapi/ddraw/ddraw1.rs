//! Implementation of DirectDraw1 interfaces, which typically don't have
//! a "1" suffix but contrast with intefaces with names like IDirectDraw7.

use super::{
    ddraw7::{IDirectDraw7, IDirectDrawSurface7},
    types::*,
    State, DD_OK,
};
use crate::{
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
        RestoreDisplayMode todo,
        SetCooperativeLevel (IDirectDraw7::shims::SetCooperativeLevel),
        SetDisplayMode ok,
        WaitForVerticalBlank todo,
    ];

    #[win32_derive::dllexport]
    fn CreateSurface(
        machine: &mut Machine,
        _this: u32,
        desc: Option<&DDSURFACEDESC>,
        lplpDDSurface: u32,
        _pUnkOuter: u32,
    ) -> u32 {
        let desc = desc.unwrap();
        assert!(std::mem::size_of::<DDSURFACEDESC>() == desc.dwSize as usize);

        let mut opts = crate::host::SurfaceOptions::default();
        let mut flags = desc.dwFlags;
        if let Some(caps) = desc.caps() {
            flags.remove(DDSD::CAPS);
            if caps.contains(DDSCAPS::PRIMARYSURFACE) {
                opts.width = machine.state.ddraw.width;
                opts.height = machine.state.ddraw.height;
                opts.primary = true;
            }
        }
        if let Some(count) = desc.back_buffer_count() {
            flags.remove(DDSD::BACKBUFFERCOUNT);
            assert!(count == 1);
        }
        assert!(flags.is_empty());

        let surface = machine.host.create_surface(&opts);

        let x86_surface = IDirectDrawSurface::new(machine);
        machine.mem().put::<u32>(lplpDDSurface, x86_surface);
        machine.state.ddraw.surfaces.insert(
            x86_surface,
            ddraw::Surface {
                host: surface,
                width: opts.width,
                height: opts.height,
                palette: 0,
                pixels: 0,
            },
        );

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
        let mem = machine.memory.mem();
        let desc_addr = machine
            .state
            .ddraw
            .heap
            .alloc(mem, std::mem::size_of::<DDSURFACEDESC>() as u32);
        let desc = mem.view_mut::<DDSURFACEDESC>(desc_addr);
        unsafe { desc.clear_struct() };
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

        crate::shims::call_x86(machine, lpEnumCallback, vec![desc_addr, lpContext]).await;

        machine
            .state
            .ddraw
            .heap
            .free(machine.memory.mem(), desc_addr);

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
        Release todo,
        AddAttachedSurface todo,
        AddOverlayDirtyRect todo,
        Blt todo,
        BltBatch todo,
        BltFast todo,
        DeleteAttachedSurface todo,
        EnumAttachedSurfaces todo,
        EnumOverlayZOrders todo,
        Flip todo,
        GetAttachedSurface ok,
        GetBltStatus todo,
        GetCaps todo,
        GetClipper todo,
        GetColorKey todo,
        GetDC todo,
        GetFlipStatus todo,
        GetOverlayPosition todo,
        GetPalette todo,
        GetPixelFormat ok,
        GetSurfaceDesc todo,
        Initialize todo,
        IsLost todo,
        Lock ok,
        ReleaseDC todo,
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
        let lpDirectDrawSurface = ddraw.heap.alloc(machine.memory.mem(), 4);
        let vtable = ddraw.vtable_IDirectDrawSurface;
        machine.mem().put::<u32>(lpDirectDrawSurface, vtable);
        lpDirectDrawSurface
    }

    #[win32_derive::dllexport]
    fn GetAttachedSurface(
        machine: &mut Machine,
        this: u32,
        _lpDDSCaps: u32,
        lpDirectDrawSurface: u32,
    ) -> u32 {
        // TODO: consider caps.
        // log::warn!("{this:x}->GetAttachedSurface({lpDDSCaps2:x}, {lpDirectDrawSurface7:x})");
        let this_surface = machine.state.ddraw.surfaces.get(&this).unwrap();
        let host = this_surface.host.get_attached();

        let surface = ddraw::Surface {
            host,
            width: this_surface.width,
            height: this_surface.height,
            palette: this_surface.palette,
            pixels: this_surface.pixels,
        };
        let x86_surface = new(machine);

        machine.mem().put::<u32>(lpDirectDrawSurface, x86_surface);
        machine.state.ddraw.surfaces.insert(x86_surface, surface);
        DD_OK
    }

    #[win32_derive::dllexport]
    fn GetPixelFormat(_machine: &mut Machine, fmt: Option<&mut DDPIXELFORMAT>) -> u32 {
        let fmt = fmt.unwrap();
        *fmt = unsafe { std::mem::zeroed() };
        fmt.dwSize = std::mem::size_of::<DDPIXELFORMAT>() as u32;
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
        let mut desc2 = DDSURFACEDESC2::default();
        desc2.dwSize = std::mem::size_of::<DDSURFACEDESC2>() as u32;
        let ret = IDirectDrawSurface7::Lock(machine, this, rect, Some(&mut desc2), flags, 0);
        if ret != DD_OK {
            return ret;
        }

        let desc = desc.unwrap();
        desc.from_desc2(&desc2);

        ret
    }

    #[win32_derive::dllexport]
    fn Unlock(machine: &mut Machine, this: u32, rect: Option<&mut RECT>) -> u32 {
        IDirectDrawSurface7::Unlock(machine, this, rect)
    }
}
