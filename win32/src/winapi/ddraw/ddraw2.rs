//! Implementation of DirectDraw2 interfaces.

use super::types::*;
use super::{
    ddraw::{self, *},
    ddraw7::{IDirectDraw7, IDirectDrawSurface7},
};
use memory::Pod;
use win32_system::System;
use win32_winapi::{RECT, vtable};

pub const IID_IDirectDraw2: GUID = GUID((
    0xb3a6f3e0,
    0x2b43,
    0x11cf,
    [0xa2, 0xde, 0x00, 0xaa, 0x00, 0xb9, 0x33, 0x56],
));

#[win32_derive::dllexport]
pub mod IDirectDraw2 {
    use super::*;

    vtable![
        QueryInterface: ok,
        AddRef: todo,
        Release: ok,
        Compact: todo,
        CreateClipper: todo,
        CreatePalette: (IDirectDraw7::CreatePalette),
        CreateSurface: ok,
        DuplicateSurface: todo,
        EnumDisplayModes: ok,
        EnumSurfaces: todo,
        FlipToGDISurface: todo,
        GetCaps: todo,
        GetDisplayMode: ok,
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

        GetAvailableVidMem: todo,
    ];

    pub fn new(sys: &mut dyn System) -> u32 {
        let vtable = sys.get_symbol("ddraw.dll", "IDirectDraw2");
        sys.memory().store(vtable)
    }

    #[win32_derive::dllexport]
    pub fn QueryInterface(sys: &dyn System, this: u32, riid: Option<&GUID>, ppvObject: u32) -> DD {
        DD::E_NOINTERFACE
    }

    #[win32_derive::dllexport]
    pub fn CreateSurface(
        sys: &mut dyn System,
        this: u32,
        desc: Option<&DDSURFACEDESC>,
        lplpDDSurface: Option<&mut u32>,
        pUnkOuter: u32,
    ) -> DD {
        let hwnd = get_state(sys).hwnd;
        let surfaces = ddraw::Surface::create(sys, hwnd, &DDSURFACEDESC2::from_desc(desc.unwrap()));
        if surfaces.len() > 2 {
            todo!()
        }

        let mut state = get_state(sys);
        let mut prev = 0;
        for mut surface in surfaces.into_iter().rev() {
            let ptr = IDirectDrawSurface2::new(sys);
            surface.attached = prev;
            state.surfaces.insert(ptr, surface);
            prev = ptr;
        }

        *lplpDDSurface.unwrap() = prev;

        DD::OK
    }

    #[win32_derive::dllexport]
    pub async fn EnumDisplayModes(
        sys: &mut dyn System,
        this: u32,
        dwFlags: u32,
        lpSurfaceDesc: Option<&DDSURFACEDESC>,
        lpContext: u32,
        lpEnumCallback: u32,
    ) -> DD {
        if lpSurfaceDesc.is_some() {
            todo!()
        }

        let mut desc = DDSURFACEDESC::default();
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

        let memory = sys.memory();
        let desc_addr = memory.store(desc);

        sys.call_x86(lpEnumCallback, vec![desc_addr, lpContext])
            .await;

        let memory = sys.memory();
        memory.process_heap.free(memory.mem(), desc_addr);

        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetDisplayMode(
        sys: &dyn System,
        this: u32,
        lpDDSurfaceDesc: Option<&mut DDSURFACEDESC>,
    ) -> DD {
        let desc = lpDDSurfaceDesc.unwrap();
        *desc = DDSURFACEDESC::zeroed();
        desc.dwSize = std::mem::size_of::<DDSURFACEDESC>() as u32;

        desc.dwFlags.insert(DDSD::WIDTH | DDSD::HEIGHT);
        desc.dwWidth = 1024;
        desc.dwHeight = 768;

        desc.dwFlags.insert(DDSD::PIXELFORMAT);
        // desc.ddpfPixelFormat.dwFlags = ...;
        desc.ddpfPixelFormat.dwRGBBitCount = 32;

        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Release(sys: &dyn System, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    pub fn SetDisplayMode(
        sys: &mut dyn System,
        this: u32,
        width: u32,
        height: u32,
        bpp: u32,
    ) -> DD {
        IDirectDraw7::SetDisplayMode(sys, 0, width, height, bpp, 0, 0)
    }
}

#[win32_derive::dllexport]
pub mod IDirectDrawSurface2 {
    use super::*;

    vtable![
        QueryInterface: todo,
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
        GetSurfaceDesc: ok,
        Initialize: todo,
        IsLost: todo,
        Lock: ok,
        ReleaseDC: (IDirectDrawSurface7::ReleaseDC),
        Restore: (IDirectDrawSurface7::Restore),
        SetClipper: (IDirectDrawSurface7::SetClipper),
        SetColorKey: todo,
        SetOverlayPosition: todo,
        SetPalette: (IDirectDrawSurface7::SetPalette),
        Unlock: ok,
        UpdateOverlay: todo,
        UpdateOverlayDisplay: todo,
        UpdateOverlayZOrder: todo,

        GetDDInterface: todo,
        PageLock: todo,
        PageUnlock: todo,
    ];

    pub fn new(sys: &dyn System) -> u32 {
        let vtable = sys.get_symbol("ddraw.dll", "IDirectDrawSurface2");
        sys.memory().store(vtable)
    }

    #[win32_derive::dllexport]
    pub fn Release(sys: &dyn System, this: u32) -> u32 {
        0
    }

    #[win32_derive::dllexport]
    pub fn GetAttachedSurface(
        sys: &mut dyn System,
        this: u32,
        lpDDSCaps: Option<&DDSCAPS>,
        lpDirectDrawSurface: Option<&mut u32>,
    ) -> DD {
        // TODO: consider caps.
        let state = get_state(sys);
        let surface = state.surfaces.get(&this).unwrap();
        *lpDirectDrawSurface.unwrap() = surface.attached;
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(sys: &dyn System, this: u32, lpDDSCAPS: Option<&mut DDSCAPS>) -> DD {
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetSurfaceDesc(sys: &mut dyn System, this: u32, desc: Option<&mut DDSURFACEDESC>) -> DD {
        let mut desc2 = DDSURFACEDESC2::default();
        let ret = IDirectDrawSurface7::GetSurfaceDesc(sys, this, Some(&mut desc2));
        if ret == DD::OK {
            *desc.unwrap() = DDSURFACEDESC::from_desc2(&desc2);
        }
        ret
    }

    #[win32_derive::dllexport]
    pub fn Lock(
        sys: &mut dyn System,
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
        let ret = IDirectDrawSurface7::Lock(sys, this, rect, Some(&mut desc2), flags, 0);
        if ret != DD::OK {
            return ret;
        }

        *desc = DDSURFACEDESC::from_desc2(&desc2);

        ret
    }

    #[win32_derive::dllexport]
    pub fn Unlock(sys: &mut dyn System, this: u32, ptr: u32) -> DD {
        IDirectDrawSurface7::Unlock(sys, this, None)
    }
}
