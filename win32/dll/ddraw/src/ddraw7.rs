//! Implementation of DirectDraw7 interfaces.

use super::types::*;
use super::{IDirectDrawClipper, IDirectDrawPalette, ddraw, get_state, palette::Palette};
use bitflags::bitflags;
use builtin_gdi32 as gdi32;
use builtin_user32 as user32;
use gdi32::{PALETTEENTRY, bitmap::Bitmap};
use memory::{Extensions, ExtensionsMut, Pod};
use std::{cell::RefCell, rc::Rc};
use win32_system::System;
use win32_winapi::{HWND, RECT, com::GUID, vtable};

pub const IID_IDirectDraw7: GUID = GUID((
    0x15e65ec0,
    0x3b9c,
    0x11d2,
    [0xb9, 0x2f, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b],
));

bitflags! {
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct DDSCL: u32 {
        const FULLSCREEN = 0x0001;
        const ALLOWREBOOT = 0x0002;
        const NOWINDOWCHANGES = 0x0004;
        const NORMAL = 0x0008;
        const EXCLUSIVE = 0x0010;
        const ALLOWMODEX = 0x0040;
        const SETFOCUSWINDOW = 0x0080;
        const SETDEVICEWINDOW = 0x0100;
        const CREATEDEVICEWINDOW = 0x0200;
        const MULTITHREADED = 0x0400;
        const FPUSETUP = 0x0800;
        const FPUPRESERVE =  0x1000;
    }
}

bitflags! {
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct DDFLIP: u32 {
        const DDFLIP_WAIT = 0x00000001;
        const DDFLIP_EVEN = 0x00000002;
        const DDFLIP_ODD = 0x00000004;
        const DDFLIP_NOVSYNC = 0x00000008;
        const DDFLIP_STEREO = 0x00000010;
        const DDFLIP_DONOTWAIT= 0x00000020;
        const DDFLIP_INTERVAL2= 0x02000000;
        const DDFLIP_INTERVAL3= 0x03000000;
        const DDFLIP_INTERVAL4= 0x04000000;
    }
}

bitflags! {
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct DDCKEY: u32 {
        const COLORSPACE  = 0x00000001;
        const DESTBLT     = 0x00000002;
        const DESTOVERLAY = 0x00000004;
        const SRCBLT      = 0x00000008;
        const SRCOVERLAY  = 0x00000010;
    }
}

#[win32_derive::dllexport]
pub mod IDirectDraw7 {

    use super::*;

    vtable![
        QueryInterface: todo,
        AddRef: todo,
        Release: ok,
        Compact: todo,
        CreateClipper: ok,
        CreatePalette: ok,
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
        RestoreDisplayMode: ok,
        SetCooperativeLevel: ok,
        SetDisplayMode: ok,
        WaitForVerticalBlank: ok,
        GetAvailableVidMem: todo,
        GetSurfaceFromDC: todo,
        RestoreAllSurfaces: todo,
        TestCooperativeLevel: todo,
        GetDeviceIdentifier: todo,
        StartModeTest: todo,
        EvaluateMode: todo,
    ];

    pub fn new(sys: &mut dyn System) -> u32 {
        let vtable = sys.get_symbol("ddraw.dll", "IDirectDraw7");
        sys.memory().store(vtable)
    }

    #[win32_derive::dllexport]
    pub fn Release(sys: &dyn System, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    pub fn CreateClipper(
        sys: &mut dyn System,
        this: u32,
        unused: u32,
        lplpClipper: Option<&mut u32>,
        reserved: u32,
    ) -> DD {
        *lplpClipper.unwrap() = IDirectDrawClipper::new(sys);
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn CreatePalette(
        sys: &mut dyn System,
        this: u32,
        flags: Result<DDPCAPS, u32>,
        entries: u32,
        lplpPalette: u32,
        unused: u32,
    ) -> DD {
        let flags = flags.unwrap();
        if !flags.contains(DDPCAPS::_8BIT) {
            todo!();
        }
        // TODO: if palette is DDPCAPS_8BITENTRIES then SetEntries needs change too.

        let ptr = IDirectDrawPalette::new(sys);
        let entries = sys
            .mem()
            .iter_pod::<PALETTEENTRY>(entries, 256)
            .collect::<Box<_>>();
        get_state(sys).palettes.insert(
            ptr,
            Rc::new(Palette {
                ptr,
                entries: RefCell::new(entries),
            }),
        );
        sys.mem().put_pod::<u32>(lplpPalette, ptr);
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn CreateSurface(
        sys: &mut dyn System,
        this: u32,
        desc: Option<&DDSURFACEDESC2>,
        lpDirectDrawSurface7: Option<&mut u32>,
        unused: u32,
    ) -> DD {
        let hwnd = get_state(sys).hwnd;
        let surfaces = ddraw::Surface::create(sys, hwnd, desc.unwrap());
        if surfaces.len() > 2 {
            todo!()
        }

        let mut state = get_state(sys);
        let mut prev = 0;
        for mut surface in surfaces.into_iter().rev() {
            let ptr = IDirectDrawSurface7::new(sys);
            surface.attached = prev;
            state.surfaces.insert(ptr, surface);
            prev = ptr;
        }

        *lpDirectDrawSurface7.unwrap() = prev;

        DD::OK
    }

    #[win32_derive::dllexport]
    pub async fn EnumDisplayModes(
        sys: &mut dyn System,
        this: u32,
        dwFlags: u32,
        lpSurfaceDesc: Option<&DDSURFACEDESC2>,
        lpContext: u32,
        lpEnumCallback: u32,
    ) -> DD {
        if lpSurfaceDesc.is_some() {
            todo!()
        }

        let mut desc = DDSURFACEDESC2::default();
        // TODO: offer multiple display modes rather than hardcoding this one.
        desc.dwSize = std::mem::size_of::<DDSURFACEDESC2>() as u32;
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
        lpDDSurfaceDesc: Option<&mut DDSURFACEDESC2>,
    ) -> DD {
        let desc = lpDDSurfaceDesc.unwrap();
        *desc = DDSURFACEDESC2::zeroed();
        desc.dwSize = std::mem::size_of::<DDSURFACEDESC2>() as u32;
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn RestoreDisplayMode(sys: &dyn System, this: u32) -> DD {
        DD::OK
    }

    #[win32_derive::dllexport]
    pub async fn SetCooperativeLevel(
        sys: &mut dyn System,
        this: u32,
        hwnd: HWND,
        flags: Result<DDSCL, u32>,
    ) -> DD {
        // TODO: this triggers behaviors like fullscreen.
        get_state(sys).hwnd = hwnd;
        let flags = flags.unwrap();
        if flags.contains(DDSCL::FULLSCREEN) {
            user32::activate_window(sys, hwnd).await; // dispatches WM_ACTIVATE{,APP} if needed
            let user32_state = user32::get_state(sys);
            let mut window = user32_state.windows.get(hwnd).unwrap().borrow_mut();
            window.expect_toplevel_mut().host.fullscreen();
        }
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn SetDisplayMode(
        sys: &mut dyn System,
        this: u32,
        width: u32,
        height: u32,
        bpp: u32,
        refresh: u32,
        flags: u32,
    ) -> DD {
        let mut state = get_state(sys);
        if let Some(wnd) = user32::get_state(sys).windows.get(state.hwnd) {
            wnd.borrow_mut().set_client_size(sys.host(), width, height);
        }
        state.screen_bytes_per_pixel = bpp / 8;
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn WaitForVerticalBlank(sys: &dyn System, this: u32, flags: u32, _unused: u32) -> DD {
        // TODO: effect.exe uses this to pace itself; actually sync to a clock here?
        #[cfg(not(target_family = "wasm"))]
        std::thread::sleep(std::time::Duration::from_millis(10));
        DD::OK
    }
}

#[win32_derive::dllexport]
pub mod IDirectDrawSurface7 {
    use super::*;

    vtable![
        QueryInterface: todo,
        AddRef: todo,
        Release: ok,
        AddAttachedSurface: todo,
        AddOverlayDirtyRect: todo,
        Blt: ok,
        BltBatch: todo,
        BltFast: ok,
        DeleteAttachedSurface: todo,
        EnumAttachedSurfaces: todo,
        EnumOverlayZOrders: todo,
        Flip: ok,
        GetAttachedSurface: ok,
        GetBltStatus: todo,
        GetCaps: ok,
        GetClipper: todo,
        GetColorKey: todo,
        GetDC: ok,
        GetFlipStatus: todo,
        GetOverlayPosition: todo,
        GetPalette: todo,
        GetPixelFormat: ok,
        GetSurfaceDesc: ok,
        Initialize: todo,
        IsLost: ok,
        Lock: ok,
        ReleaseDC: ok,
        Restore: ok,
        SetClipper: ok,
        SetColorKey: ok,
        SetOverlayPosition: todo,
        SetPalette: ok,
        Unlock: ok,
        UpdateOverlay: todo,
        UpdateOverlayDisplay: todo,
        UpdateOverlayZOrder: todo,
        GetDDInterface: todo,
        PageLock: todo,
        PageUnlock: todo,
        SetSurfaceDesc: todo,
        SetPrivateData: todo,
        GetPrivateData: todo,
        FreePrivateData: todo,
        GetUniquenessValue: todo,
        ChangeUniquenessValue: todo,
        SetPriority: todo,
        GetPriority: todo,
        SetLOD: todo,
        GetLOD: todo,
    ];

    pub fn new(sys: &dyn System) -> u32 {
        let vtable = sys.get_symbol("ddraw.dll", "IDirectDrawSurface7");
        sys.memory().store(vtable)
    }

    #[win32_derive::dllexport]
    pub fn Release(sys: &dyn System, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    pub fn Blt(
        sys: &mut dyn System,
        this: u32,
        lpDstRect: Option<&RECT>,
        lpSrc: u32,
        lpSrcRect: Option<&RECT>,
        flags: Result<DDBLT, u32>,
        lpDDBLTFX: Option<&DDBLTFX>,
    ) -> DD {
        let mut state = get_state(sys);
        let dst = state.surfaces.get_mut(&this).unwrap();

        let mut flags = flags.unwrap();
        flags.remove(DDBLT::WAIT); // ignored
        if flags.contains(DDBLT::COLORFILL) {
            let fx = lpDDBLTFX.unwrap();
            // TODO: obey dst rect
            dst.fill(sys.mem(), &sys.memory().process_heap, fx.dwFillColor);
            return DD::OK;
        }
        if !flags.is_empty() {
            log::warn!("Blt: ignoring flags: {flags:?}");
        }

        let dst = state.surfaces.get(&this).unwrap();
        let src = state.surfaces.get(&lpSrc).unwrap();

        let src_rect = lpSrcRect.copied().unwrap_or(RECT {
            left: 0,
            top: 0,
            right: src.width as i32,
            bottom: src.height as i32,
        });
        let dst_rect = lpDstRect.copied().unwrap_or(RECT {
            left: 0,
            top: 0,
            right: dst.width as i32,
            bottom: dst.height as i32,
        });

        dst.host.bit_blt(&dst_rect, src.host.as_ref(), &src_rect);
        if dst.primary {
            dst.host.show();
        }
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn BltFast(
        sys: &mut dyn System,
        this: u32,
        x: u32,
        y: u32,
        lpSrc: u32,
        lpRect: Option<&RECT>,
        flags: Result<DDBLTFAST, u32>,
    ) -> DD {
        let flags = flags.unwrap();
        if !flags.is_empty() {
            log::warn!("BltFast: ignoring flags: {:?}", flags);
        }

        let state = get_state(sys);
        let dst = state.surfaces.get(&this).unwrap();
        let src = state.surfaces.get(&lpSrc).unwrap();

        let src_rect = lpRect.copied().unwrap_or(RECT {
            left: 0,
            top: 0,
            right: src.width as i32,
            bottom: src.height as i32,
        });

        let dst_rect = RECT {
            left: 0,
            top: 0,
            right: dst.width as i32,
            bottom: dst.height as i32,
        };

        dst.host.bit_blt(&dst_rect, src.host.as_ref(), &src_rect);
        if dst.primary {
            dst.host.show();
        }
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Flip(sys: &mut dyn System, this: u32, lpSurf: u32, flags: Result<DDFLIP, u32>) -> DD {
        // Swap the back buffer's pixels with the front buffer.
        if lpSurf != 0 {
            todo!();
        }

        let mut state = get_state(sys);
        let surface = state.surfaces.get(&this).unwrap();
        if !surface.primary {
            // TODO: surface can be an arbitrary surface in the flip chain, not just the back buffer.
            todo!();
        }
        let old_pixels = surface.pixels;
        let attached = surface.attached;
        let back = state.surfaces.get_mut(&attached).unwrap();
        let new_pixels = back.pixels;
        back.pixels = old_pixels;

        let surface = state.surfaces.get_mut(&this).unwrap();
        assert!(new_pixels != 0);
        surface.pixels = new_pixels;
        surface.flush(sys.mem());

        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetAttachedSurface(
        sys: &mut dyn System,
        this: u32,
        lpDDSCaps2: Option<&DDSCAPS2>,
        lpDirectDrawSurface7: Option<&mut u32>,
    ) -> DD {
        // TODO: consider caps.
        let state = get_state(sys);
        let surface = state.surfaces.get(&this).unwrap();
        *lpDirectDrawSurface7.unwrap() = surface.attached;
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(sys: &dyn System, this: u32, lpDDSCAPS2: Option<&mut DDSCAPS2>) -> DD {
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetDC(sys: &mut dyn System, this: u32, lpHDC: u32) -> DD {
        let mut state = get_state(sys);
        let surf = state.surfaces.get_mut(&this).unwrap();
        // Ensure surface has backing store, since DC is for drawing on it.
        surf.lock(sys.mem(), &sys.memory().process_heap);

        struct SurfaceAsGDI(u32);
        impl gdi32::DCTarget for SurfaceAsGDI {
            fn get_bitmap(&self, sys: &dyn System) -> Rc<RefCell<Bitmap>> {
                let state = get_state(sys);
                let surface = state.surfaces.get(&self.0).unwrap();
                Rc::new(RefCell::new(surface.to_bitmap()))
            }

            fn flush(&self, sys: &dyn System) {
                let state = get_state(sys);
                let surface = state.surfaces.get(&self.0).unwrap();
                surface.flush(sys.mem());
            }
        }

        let handle = gdi32::get_state(sys).new_dc(Box::new(SurfaceAsGDI(this)));
        sys.mem().put_pod::<u32>(lpHDC, handle.to_raw());
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetPixelFormat(sys: &dyn System, this: u32, fmt: Option<&mut DDPIXELFORMAT>) -> DD {
        let fmt = fmt.unwrap();
        assert!(fmt.dwSize == std::mem::size_of::<DDPIXELFORMAT>() as u32);
        // The "RGBA32" we use has the R as the first byte, so due to little
        // endian that means the *low* end of the bit mask here.
        *fmt = DDPIXELFORMAT {
            dwSize: std::mem::size_of::<DDPIXELFORMAT>() as u32,
            dwFlags: 0x00000040,
            dwFourCC: 0,
            dwRGBBitCount: 32,
            dwRBitMask: 0x0000_00FF,
            dwGBitMask: 0x0000_FF00,
            dwBBitMask: 0x00FF_0000,
            dwRGBAlphaBitMask: 0xFF00_0000,
        };
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetSurfaceDesc(
        sys: &mut dyn System,
        this: u32,
        lpDesc: Option<&mut DDSURFACEDESC2>,
    ) -> DD {
        let state = get_state(sys);
        let surf = state.surfaces.get(&this).unwrap();
        let desc = lpDesc.unwrap();
        assert!(desc.dwSize as usize == std::mem::size_of::<DDSURFACEDESC2>());
        // TODO: a trace of a ddraw2 program had the result contain
        // CAPS, HEIGHT, PITCH, PIXELFORMAT, WIDTH
        desc.dwWidth = surf.width;
        desc.dwFlags.insert(DDSD::WIDTH);
        desc.dwHeight = surf.height;
        desc.dwFlags.insert(DDSD::HEIGHT);
        desc.lPitch_dwLinearSize = surf.width * 4;
        desc.dwFlags.insert(DDSD::PITCH);

        // TODO: a trace of a ddraw2 program had the result contain
        // DDPF_RGB, r/g/b bitmasks
        desc.ddpfPixelFormat.dwRGBBitCount = 32;
        desc.dwFlags.insert(DDSD::PIXELFORMAT);
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn IsLost(sys: &mut dyn System, this: u32) -> DD {
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Lock(
        sys: &mut dyn System,
        this: u32,
        rect: Option<&RECT>,
        desc: Option<&mut DDSURFACEDESC2>,
        flags: Result<DDLOCK, u32>,
        unused: u32,
    ) -> DD {
        if rect.is_some() {
            // TODO: once we implement this, we need corresponding logic in Unlock.
            // Note also ddraw1's Unlock has a different type than 7.
            todo!();
        }
        let desc = desc.unwrap();
        let mut state = get_state(sys);
        let surf = state.surfaces.get_mut(&this).unwrap();
        let pixels = surf.lock(sys.mem(), &sys.memory().process_heap);
        // It seems callers (effect, monolife) don't provide flags for what they want,
        // and instead expect all fields to be included.
        desc.lpSurface = pixels;
        desc.lPitch_dwLinearSize = surf.width * surf.bytes_per_pixel;
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn ReleaseDC(sys: &dyn System, _this: u32, _hDC: u32) -> DD {
        // leak
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Restore(sys: &dyn System, _this: u32) -> DD {
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn SetClipper(sys: &dyn System, this: u32, clipper: u32) -> DD {
        // e.g. machine.state.ddraw.surfaces.get_mut(&this).unwrap().palette = palette;
        log::warn!("TODO: SetClipper: stub");
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn SetColorKey(
        sys: &dyn System,
        this: u32,
        flags: Result<DDCKEY, u32>,
        key: Option<&DDCOLORKEY>,
    ) -> DD {
        log::warn!("TODO: SetColorKey: stub");
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn SetPalette(sys: &mut dyn System, this: u32, palette: u32) -> DD {
        let mut state = get_state(sys);
        let palette = state.palettes.get(&palette).unwrap();
        state.surfaces.get_mut(&this).unwrap().palette = Some(palette.clone());
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Unlock(sys: &mut dyn System, this: u32, rect: Option<&mut RECT>) -> DD {
        let mut state = get_state(sys);
        let surf = state.surfaces.get_mut(&this).unwrap();
        if let Some(rect) = rect {
            // TODO: needs to match the rect passed in Lock.
            rect.left = 0;
            rect.top = 0;
            rect.right = surf.width as i32;
            rect.bottom = surf.height as i32;
        }
        surf.unlock(sys.mem());

        DD::OK
    }
}
