//! Implementation of DirectDraw7 interfaces.

use super::{palette::IDirectDrawPalette, types::*};
use crate::{
    winapi::{
        bitmap::transmute_pixels_mut,
        com::{vtable, GUID},
        ddraw,
        kernel32::get_symbol,
        types::*,
    },
    Machine,
};
use bitflags::bitflags;
use memory::Pod;
use memory::{Extensions, ExtensionsMut};

pub const IID_IDirectDraw7: GUID = GUID((
    0x15e65ec0,
    0x3b9c,
    0x11d2,
    [0xb9, 0x2f, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b],
));

bitflags! {
    #[derive(win32_derive::TryFromBitflags)]
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
    #[derive(win32_derive::TryFromBitflags)]
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

#[win32_derive::dllexport]
pub mod IDirectDraw7 {
    use ddraw::IDirectDrawClipper;

    use crate::winapi::gdi32::PALETTEENTRY;

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

    pub fn new(machine: &mut Machine) -> u32 {
        let ddraw = &mut machine.state.ddraw;
        let lpDirectDraw = ddraw.heap.alloc(machine.emu.memory.mem(), 4);
        let vtable = get_symbol(machine, "ddraw.dll", "IDirectDraw7");
        machine.mem().put_pod::<u32>(lpDirectDraw, vtable);
        lpDirectDraw
    }

    #[win32_derive::dllexport]
    pub fn Release(_machine: &mut Machine, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    pub fn CreateClipper(
        machine: &mut Machine,
        this: u32,
        unused: u32,
        lplpClipper: Option<&mut u32>,
        reserved: u32,
    ) -> DD {
        *lplpClipper.unwrap() = IDirectDrawClipper::new(machine);
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn CreatePalette(
        machine: &mut Machine,
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

        let palette = IDirectDrawPalette::new(machine);
        let entries = machine
            .mem()
            .iter_pod::<PALETTEENTRY>(entries, 256)
            .collect();
        machine.state.ddraw.palettes.insert(palette, entries);
        machine.mem().put_pod::<u32>(lplpPalette, palette);
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn CreateSurface(
        machine: &mut Machine,
        this: u32,
        desc: Option<&DDSURFACEDESC2>,
        lpDirectDrawSurface7: Option<&mut u32>,
        unused: u32,
    ) -> DD {
        let surfaces = ddraw::Surface::create(machine, machine.state.ddraw.hwnd, desc.unwrap());
        if surfaces.len() > 2 {
            todo!()
        }

        let mut prev = 0;
        for mut surface in surfaces.into_iter().rev() {
            let ptr = IDirectDrawSurface7::new(machine);
            surface.attached = prev;
            machine.state.ddraw.surfaces.insert(ptr, surface);
            prev = ptr;
        }

        *lpDirectDrawSurface7.unwrap() = prev;

        DD::OK
    }

    #[win32_derive::dllexport]
    pub async fn EnumDisplayModes(
        machine: &mut Machine,
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

        let mem = machine.emu.memory.mem();
        let desc_addr = machine
            .state
            .ddraw
            .heap
            .alloc(mem, std::mem::size_of::<DDSURFACEDESC2>() as u32);
        mem.put_pod::<DDSURFACEDESC2>(desc_addr, desc);

        machine
            .call_x86(lpEnumCallback, vec![desc_addr, lpContext])
            .await;

        machine
            .state
            .ddraw
            .heap
            .free(machine.emu.memory.mem(), desc_addr);

        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetDisplayMode(
        _machine: &mut Machine,
        this: u32,
        lpDDSurfaceDesc: Option<&mut DDSURFACEDESC2>,
    ) -> DD {
        let desc = lpDDSurfaceDesc.unwrap();
        *desc = DDSURFACEDESC2::zeroed();
        desc.dwSize = std::mem::size_of::<DDSURFACEDESC2>() as u32;
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn RestoreDisplayMode(_machine: &mut Machine, this: u32) -> DD {
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel(
        machine: &mut Machine,
        this: u32,
        hwnd: HWND,
        flags: Result<DDSCL, u32>,
    ) -> DD {
        // TODO: this triggers behaviors like fullscreen.
        machine.state.ddraw.hwnd = hwnd;
        let flags = flags.unwrap();
        if flags.contains(DDSCL::FULLSCREEN) {
            let window = machine.state.user32.windows.get_mut(hwnd).unwrap();
            window.expect_toplevel_mut().host.fullscreen();
        }
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn SetDisplayMode(
        machine: &mut Machine,
        this: u32,
        width: u32,
        height: u32,
        bpp: u32,
        refresh: u32,
        flags: u32,
    ) -> DD {
        if let Some(wnd) = machine
            .state
            .user32
            .windows
            .get_mut(machine.state.ddraw.hwnd)
        {
            wnd.set_client_size(&mut *machine.host, width, height);
        }
        machine.state.ddraw.bytes_per_pixel = bpp / 8;
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn WaitForVerticalBlank(_machine: &mut Machine, this: u32, flags: u32, _unused: u32) -> DD {
        // TODO: effect.exe uses this to pace itself; actually sync to a clock here?
        #[cfg(not(feature = "wasm"))]
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

    pub fn new(machine: &mut Machine) -> u32 {
        let ddraw = &mut machine.state.ddraw;
        let lpDirectDrawSurface7 = ddraw.heap.alloc(machine.emu.memory.mem(), 4);
        let vtable = get_symbol(machine, "ddraw.dll", "IDirectDrawSurface7");
        machine.mem().put_pod::<u32>(lpDirectDrawSurface7, vtable);
        lpDirectDrawSurface7
    }

    #[win32_derive::dllexport]
    pub fn Release(_machine: &mut Machine, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    pub fn Blt(
        machine: &mut Machine,
        this: u32,
        lpDstRect: Option<&RECT>,
        lpSrc: u32,
        lpSrcRect: Option<&RECT>,
        flags: Result<DDBLT, u32>,
        lpDDBLTFX: Option<&DDBLTFX>,
    ) -> DD {
        if lpDstRect.is_some() || lpSrcRect.is_some() {
            log::warn!("todo: Blt with rects");
        }
        let flags = flags.unwrap();
        if flags.contains(DDBLT::COLORFILL) {
            log::warn!("todo: DDBLT::COLORFILL");
            return DD::OK;
        }
        if lpSrc == 0 {
            log::error!("Blt from null surface");
            return DD::OK;
        }
        log::warn!("Blt: ignoring behavioral flags");
        BltFast(machine, this, 0, 0, lpSrc, None, 0)
    }

    #[win32_derive::dllexport]
    pub fn BltFast(
        machine: &mut Machine,
        this: u32,
        x: u32,
        y: u32,
        lpSrc: u32,
        lpRect: Option<&RECT>,
        flags: u32,
    ) -> DD {
        if flags != 0 {
            log::warn!("BltFast flags: {:x}", flags);
        }
        let (dst, src) = unsafe {
            let dst = machine.state.ddraw.surfaces.get_mut(&this).unwrap() as *mut ddraw::Surface;
            let src = machine.state.ddraw.surfaces.get(&lpSrc).unwrap() as *const ddraw::Surface;
            assert_ne!(dst as *const ddraw::Surface, src);
            (&mut *dst, &*src)
        };
        if let Some(rect) = lpRect {
            let sx = rect.left as u32;
            let w = (rect.right - rect.left) as u32;
            let sy = rect.top as u32;
            let h = (rect.bottom - rect.top) as u32;
            dst.host.bit_blt(x, y, src.host.as_ref(), sx, sy, w, h);
        } else {
            dst.host
                .bit_blt(x, y, src.host.as_ref(), 0, 0, src.width, src.height);
        }
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Flip(machine: &mut Machine, this: u32, lpSurf: u32, flags: Result<DDFLIP, u32>) -> DD {
        let surface = machine.state.ddraw.surfaces.get(&this).unwrap();
        let attached = surface.attached;
        let back = machine.state.ddraw.surfaces.get_mut(&attached).unwrap();
        back.host.show();
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetAttachedSurface(
        machine: &mut Machine,
        this: u32,
        lpDDSCaps2: Option<&DDSCAPS2>,
        lpDirectDrawSurface7: Option<&mut u32>,
    ) -> DD {
        // TODO: consider caps.
        let surface = machine.state.ddraw.surfaces.get(&this).unwrap();
        *lpDirectDrawSurface7.unwrap() = surface.attached;
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetCaps(_machine: &mut Machine, this: u32, lpDDSCAPS2: Option<&mut DDSCAPS2>) -> DD {
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetDC(machine: &mut Machine, this: u32, lpHDC: u32) -> DD {
        let dc =
            crate::winapi::gdi32::DC::new(crate::winapi::gdi32::DCTarget::DirectDrawSurface(this));
        let handle = machine.state.gdi32.dcs.add(dc);
        machine.mem().put_pod::<u32>(lpHDC, handle.to_raw());
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetPixelFormat(
        _machine: &mut Machine,
        this: u32,
        fmt: Option<&mut DDPIXELFORMAT>,
    ) -> DD {
        let fmt = fmt.unwrap();
        assert!(fmt.dwSize == std::mem::size_of::<DDPIXELFORMAT>() as u32);
        *fmt = DDPIXELFORMAT {
            dwSize: std::mem::size_of::<DDPIXELFORMAT>() as u32,
            dwFlags: 0x00000040,
            dwFourCC: 0,
            dwRGBBitCount: 32,
            dwRBitMask: 0xFF00_0000,
            dwGBitMask: 0x00FF_0000,
            dwBBitMask: 0x0000_FF00,
            dwRGBAlphaBitMask: 0x0000_00FF,
        };
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn GetSurfaceDesc(
        machine: &mut Machine,
        this: u32,
        lpDesc: Option<&mut DDSURFACEDESC2>,
    ) -> DD {
        let surf = machine.state.ddraw.surfaces.get(&this).unwrap();
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
    pub fn IsLost(machine: &mut Machine, this: u32) -> DD {
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Lock(
        machine: &mut Machine,
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
        let surf = machine.state.ddraw.surfaces.get_mut(&this).unwrap();
        if surf.pixels == 0 {
            surf.pixels = machine.state.ddraw.heap.alloc(
                machine.emu.memory.mem(),
                surf.width * surf.height * surf.bytes_per_pixel,
            );
        }
        // It seems callers (effect, monolife) don't provide flags for what they want,
        // and instead expect all fields to be included.
        desc.lpSurface = surf.pixels;
        desc.lPitch_dwLinearSize = surf.width * surf.bytes_per_pixel;
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn ReleaseDC(_machine: &mut Machine, _this: u32, _hDC: u32) -> DD {
        // leak
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Restore(_machine: &mut Machine, _this: u32) -> DD {
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn SetClipper(_machine: &mut Machine, this: u32, clipper: u32) -> DD {
        // e.g. machine.state.ddraw.surfaces.get_mut(&this).unwrap().palette = palette;
        log::warn!("TODO: SetClipper: stub");
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn SetColorKey(_machine: &mut Machine, this: u32, flags: u32, key: u32) -> DD {
        log::warn!("TODO: SetColorKey: stub");
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn SetPalette(machine: &mut Machine, this: u32, palette: u32) -> DD {
        machine.state.ddraw.surfaces.get_mut(&this).unwrap().palette = palette;
        machine.state.ddraw.palette_hack = palette;
        DD::OK
    }

    #[win32_derive::dllexport]
    pub fn Unlock(machine: &mut Machine, this: u32, rect: Option<&mut RECT>) -> DD {
        let surf = machine.state.ddraw.surfaces.get_mut(&this).unwrap();
        if let Some(rect) = rect {
            // TODO: needs to match the rect passed in Lock.
            rect.left = 0;
            rect.top = 0;
            rect.right = surf.width as i32;
            rect.bottom = surf.height as i32;
        }
        assert!(surf.pixels != 0);

        // We need to copy surf.pixels to convert its format to the RGBA expected by the write_pixels API.
        let mut pixels_bytes = Vec::with_capacity((surf.width * surf.height * 4) as usize);
        unsafe { pixels_bytes.set_len(pixels_bytes.capacity()) };
        let pixels_quads: &mut [[u8; 4]] = transmute_pixels_mut(&mut pixels_bytes);
        match surf.bytes_per_pixel {
            1 => {
                let pixels = machine
                    .emu
                    .memory
                    .mem()
                    .iter_pod::<u8>(surf.pixels, surf.width * surf.height);
                if let Some(palette) = machine
                    .state
                    .ddraw
                    .palettes
                    .get(&machine.state.ddraw.palette_hack)
                {
                    for (pSrc, pDst) in pixels.zip(pixels_quads) {
                        let p = &palette[pSrc as usize];
                        *pDst = [p.peRed, p.peGreen, p.peBlue, 0xFF];
                    }
                }
            }
            2 => {
                let pixels = machine
                    .emu
                    .memory
                    .mem()
                    .iter_pod::<u16>(surf.pixels, surf.width * surf.height);

                for (pSrc, pDst) in pixels.zip(pixels_quads) {
                    // TODO: this depends on whether 16bpp is 555 or 565 etc.
                    let r = ((pSrc & 0b0111_1100_0000_0000) >> 7) as u8;
                    let g = ((pSrc & 0b0000_0011_1110_0000) >> 2) as u8;
                    let b = ((pSrc & 0b0000_0000_0001_1111) << 3) as u8;
                    *pDst = [r, g, b, 0xFF];
                }
            }
            4 => {
                let pixels = machine
                    .emu
                    .memory
                    .mem()
                    .iter_pod::<[u8; 4]>(surf.pixels, surf.width * surf.height);
                // Ignore alpha channel in input; output is always opaque.
                for (pSrc, pDst) in pixels.zip(pixels_quads) {
                    *pDst = [pSrc[0], pSrc[1], pSrc[2], 0xFF];
                }
            }
            bpp => todo!("Unlock for {bpp}bpp"),
        }
        surf.host.write_pixels(&pixels_bytes);

        // If surface is primary then updates should show immediately.
        // XXX probably need something other than attached here
        if surf.attached == 0 {
            surf.host.show();
        }

        DD::OK
    }
}
