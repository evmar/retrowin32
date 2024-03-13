//! Implementation of DirectDraw7 interfaces.

use super::{types::*, IDirectDrawPalette, State, DDERR_GENERIC, DD_OK};
use crate::{
    machine::Emulator,
    winapi::{ddraw, types::*, vtable},
    Machine,
};
use bitflags::bitflags;
use memory::{Extensions, Pod};

const TRACE_CONTEXT: &'static str = "ddraw/7";

pub const IID_IDirectDraw7: [u8; 16] = [
    0xc0, 0x5e, 0xe6, 0x15, 0x9c, 0x3b, 0xd2, 0x11, 0xb9, 0x2f, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b,
];

#[win32_derive::shims_from_x86]
pub(super) mod IDirectDraw7 {
    use super::*;

    vtable![IDirectDraw7 shims
        QueryInterface todo,
        AddRef todo,
        Release ok,
        Compact todo,
        CreateClipper todo,
        CreatePalette ok,
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
        SetCooperativeLevel ok,
        SetDisplayMode ok,
        WaitForVerticalBlank ok,
        GetAvailableVidMem todo,
        GetSurfaceFromDC todo,
        RestoreAllSurfaces todo,
        TestCooperativeLevel todo,
        GetDeviceIdentifier todo,
        StartModeTest todo,
        EvaluateMode todo,
    ];

    #[win32_derive::dllexport]
    fn Release(_machine: &mut Machine, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    fn CreatePalette(
        machine: &mut Machine,
        this: u32,
        flags: Result<DDPCAPS, u32>,
        entries: u32,
        lplpPalette: u32,
        unused: u32,
    ) -> u32 {
        let flags = flags.unwrap();
        if !flags.contains(DDPCAPS::_8BIT) {
            todo!();
        }
        // TODO: if palette is DDPCAPS_8BITENTRIES then SetEntries needs change too.

        let palette = IDirectDrawPalette::new(machine);
        let entries = machine
            .mem()
            .view_n::<PALETTEENTRY>(entries, 256)
            .to_vec()
            .into_boxed_slice();
        machine.state.ddraw.palettes.insert(palette, entries);
        machine.mem().put::<u32>(lplpPalette, palette);
        DD_OK
    }

    #[win32_derive::dllexport]
    fn CreateSurface(
        machine: &mut Machine,
        this: u32,
        desc: Option<&DDSURFACEDESC2>,
        lpDirectDrawSurface7: Option<&mut u32>,
        unused: u32,
    ) -> u32 {
        let surfaces = ddraw::Surface::create(machine, desc.unwrap());
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

        DD_OK
    }

    #[win32_derive::dllexport]
    async fn EnumDisplayModes(
        machine: &mut Machine,
        this: u32,
        dwFlags: u32,
        lpSurfaceDesc: Option<&DDSURFACEDESC2>,
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
            .alloc(mem, std::mem::size_of::<DDSURFACEDESC2>() as u32);
        let desc = mem.view_mut::<DDSURFACEDESC2>(desc_addr);
        desc.clear_struct();
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

    bitflags! {
        pub struct DDSCL: u32 {
            const DDSCL_FULLSCREEN = 0x0001;
            const DDSCL_ALLOWREBOOT = 0x0002;
            const DDSCL_NOWINDOWCHANGES = 0x0004;
            const DDSCL_NORMAL = 0x0008;
            const DDSCL_EXCLUSIVE = 0x0010;
            const DDSCL_ALLOWMODEX = 0x0040;
            const DDSCL_SETFOCUSWINDOW = 0x0080;
            const DDSCL_SETDEVICEWINDOW = 0x0100;
            const DDSCL_CREATEDEVICEWINDOW = 0x0200;
            const DDSCL_MULTITHREADED = 0x0400;
            const DDSCL_FPUSETUP = 0x0800;
            const DDSCL_FPUPRESERVE =  0x1000;
        }
    }
    impl TryFrom<u32> for DDSCL {
        type Error = u32;

        fn try_from(value: u32) -> Result<Self, Self::Error> {
            DDSCL::from_bits(value).ok_or(value)
        }
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel(
        machine: &mut Machine,
        this: u32,
        hwnd: HWND,
        flags: Result<DDSCL, u32>,
    ) -> u32 {
        // TODO: this triggers behaviors like fullscreen.
        machine.state.ddraw.hwnd = hwnd;
        DD_OK
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
    ) -> u32 {
        if let Some(wnd) = machine
            .state
            .user32
            .windows
            .get_mut(machine.state.ddraw.hwnd)
        {
            wnd.set_client_size(width, height);
        }
        machine.state.ddraw.bytes_per_pixel = bpp / 8;
        DD_OK
    }

    #[win32_derive::dllexport]
    pub fn WaitForVerticalBlank(
        _machine: &mut Machine,
        this: u32,
        flags: u32,
        _unused: u32,
    ) -> u32 {
        // TODO: effect.exe uses this to pace itself; actually sync to a clock here?
        #[cfg(not(target_arch = "wasm32"))]
        std::thread::sleep(std::time::Duration::from_millis(10));
        DD_OK
    }
}

#[win32_derive::shims_from_x86]
pub(super) mod IDirectDrawSurface7 {
    use super::*;

    vtable![IDirectDrawSurface7 shims
        QueryInterface todo,
        AddRef todo,
        Release ok,
        AddAttachedSurface todo,
        AddOverlayDirtyRect todo,
        Blt ok,
        BltBatch todo,
        BltFast ok,
        DeleteAttachedSurface todo,
        EnumAttachedSurfaces todo,
        EnumOverlayZOrders todo,
        Flip ok,
        GetAttachedSurface ok,
        GetBltStatus todo,
        GetCaps ok,
        GetClipper todo,
        GetColorKey todo,
        GetDC ok,
        GetFlipStatus todo,
        GetOverlayPosition todo,
        GetPalette todo,
        GetPixelFormat todo,
        GetSurfaceDesc ok,
        Initialize todo,
        IsLost todo,
        Lock ok,
        ReleaseDC ok,
        Restore ok,
        SetClipper todo,
        SetColorKey todo,
        SetOverlayPosition todo,
        SetPalette ok,
        Unlock ok,
        UpdateOverlay todo,
        UpdateOverlayDisplay todo,
        UpdateOverlayZOrder todo,
        GetDDInterface todo,
        PageLock todo,
        PageUnlock todo,
        SetSurfaceDesc todo,
        SetPrivateData todo,
        GetPrivateData todo,
        FreePrivateData todo,
        GetUniquenessValue todo,
        ChangeUniquenessValue todo,
        SetPriority todo,
        GetPriority todo,
        SetLOD todo,
        GetLOD todo,
    ];

    pub fn new(machine: &mut Machine) -> u32 {
        let ddraw = &mut machine.state.ddraw;
        let lpDirectDrawSurface7 = ddraw.heap.alloc(machine.emu.memory.mem(), 4);
        let vtable = ddraw.vtable_IDirectDrawSurface7;
        machine.mem().put::<u32>(lpDirectDrawSurface7, vtable);
        lpDirectDrawSurface7
    }

    #[win32_derive::dllexport]
    fn Release(_machine: &mut Machine, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    fn Blt(
        machine: &mut Machine,
        this: u32,
        lpDstRect: Option<&RECT>,
        lpSurf: u32,
        lpSrcRect: Option<&RECT>,
        flags: Result<DDBLT, u32>,
        lpDDBLTFX: Option<&DDBLTFX>,
    ) -> u32 {
        if lpDstRect.is_some() || lpSrcRect.is_some() {
            todo!()
        }
        log::warn!("Blt: ignoring behavioral flags");
        BltFast(machine, this, 0, 0, lpSurf, None, 0);
        DD_OK
    }

    #[win32_derive::dllexport]
    fn BltFast(
        machine: &mut Machine,
        this: u32,
        x: u32,
        y: u32,
        lpSurf: u32,
        lpRect: Option<&RECT>,
        flags: u32,
    ) -> u32 {
        if flags != 0 {
            log::warn!("BltFast flags: {:x}", flags);
        }
        let (dst, src) = unsafe {
            let dst = machine.state.ddraw.surfaces.get_mut(&this).unwrap() as *mut ddraw::Surface;
            let src = machine.state.ddraw.surfaces.get(&lpSurf).unwrap() as *const ddraw::Surface;
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
        DD_OK
    }

    bitflags! {
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
    impl TryFrom<u32> for DDFLIP {
        type Error = u32;

        fn try_from(value: u32) -> Result<Self, Self::Error> {
            DDFLIP::from_bits(value).ok_or(value)
        }
    }

    #[win32_derive::dllexport]
    fn Flip(machine: &mut Machine, this: u32, lpSurf: u32, flags: Result<DDFLIP, u32>) -> u32 {
        let surface = machine.state.ddraw.surfaces.get(&this).unwrap();
        let attached = surface.attached;
        let back = machine.state.ddraw.surfaces.get_mut(&attached).unwrap();
        back.host.show();
        DD_OK
    }

    #[win32_derive::dllexport]
    fn GetAttachedSurface(
        machine: &mut Machine,
        this: u32,
        lpDDSCaps2: Option<&DDSCAPS2>,
        lpDirectDrawSurface7: Option<&mut u32>,
    ) -> u32 {
        // TODO: consider caps.
        let surface = machine.state.ddraw.surfaces.get(&this).unwrap();
        *lpDirectDrawSurface7.unwrap() = surface.attached;
        DD_OK
    }

    #[win32_derive::dllexport]
    fn GetCaps(_machine: &mut Machine, this: u32, lpDDSCAPS2: Option<&mut DDSCAPS2>) -> u32 {
        DD_OK
    }

    #[win32_derive::dllexport]
    fn GetDC(machine: &mut Machine, this: u32, lpHDC: u32) -> u32 {
        let dc =
            crate::winapi::gdi32::DC::new(crate::winapi::gdi32::DCTarget::DirectDrawSurface(this));
        let handle = machine.state.gdi32.dcs.add(dc);
        machine.mem().put::<u32>(lpHDC, handle.to_raw());
        DD_OK
    }

    #[win32_derive::dllexport]
    fn GetSurfaceDesc(
        machine: &mut Machine,
        this: u32,
        lpDesc: Option<&mut DDSURFACEDESC2>,
    ) -> u32 {
        let surf = machine.state.ddraw.surfaces.get(&this).unwrap();
        let desc = lpDesc.unwrap();
        assert!(desc.dwSize as usize == std::mem::size_of::<DDSURFACEDESC2>());
        let mut flags = desc.dwFlags;
        if flags.contains(DDSD::WIDTH) {
            desc.dwWidth = surf.width;
            flags.remove(DDSD::WIDTH);
        }
        if flags.contains(DDSD::HEIGHT) {
            desc.dwHeight = surf.height;
            flags.remove(DDSD::HEIGHT);
        }
        if !flags.is_empty() {
            log::warn!(
                "unimp: {:?} for {this:x}->GetSurfaceDesc({desc:?})",
                desc.dwFlags
            );
        }
        DDERR_GENERIC
    }

    #[win32_derive::dllexport]
    pub fn Lock(
        machine: &mut Machine,
        this: u32,
        rect: Option<&RECT>,
        desc: Option<&mut DDSURFACEDESC2>,
        flags: Result<DDLOCK, u32>,
        unused: u32,
    ) -> u32 {
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
                surf.width * surf.height * machine.state.ddraw.bytes_per_pixel,
            );
        }
        // It seems callers (effect, monolife) don't provide flags for what they want,
        // and instead expect all fields to be included.
        desc.lpSurface = surf.pixels;
        desc.lPitch_dwLinearSize = surf.width * machine.state.ddraw.bytes_per_pixel;
        DD_OK
    }

    #[win32_derive::dllexport]
    fn ReleaseDC(_machine: &mut Machine, _this: u32, _hDC: u32) -> u32 {
        // leak
        DD_OK
    }

    #[win32_derive::dllexport]
    fn Restore(_machine: &mut Machine, _this: u32) -> u32 {
        DD_OK
    }

    #[win32_derive::dllexport]
    fn SetPalette(machine: &mut Machine, this: u32, palette: u32) -> u32 {
        machine.state.ddraw.surfaces.get_mut(&this).unwrap().palette = palette;
        machine.state.ddraw.palette_hack = palette;
        DD_OK
    }

    #[win32_derive::dllexport]
    pub fn Unlock(machine: &mut Machine, this: u32, rect: Option<&mut RECT>) -> u32 {
        let surf = machine.state.ddraw.surfaces.get_mut(&this).unwrap();
        if let Some(rect) = rect {
            // TODO: needs to match the rect passed in Lock.
            rect.left = 0;
            rect.top = 0;
            rect.right = surf.width as i32;
            rect.bottom = surf.height as i32;
        }
        assert!(surf.pixels != 0);
        match machine.state.ddraw.bytes_per_pixel {
            1 => {
                let pixels = machine
                    .emu
                    .memory
                    .mem()
                    .view_n::<u8>(surf.pixels, surf.width * surf.height);
                if let Some(palette) = machine
                    .state
                    .ddraw
                    .palettes
                    .get(&machine.state.ddraw.palette_hack)
                {
                    // XXX very inefficient
                    let pixels32: Vec<_> = pixels
                        .iter()
                        .map(|&i| {
                            let p = &palette[i as usize];
                            [p.peRed, p.peGreen, p.peBlue, 255]
                        })
                        .collect();
                    surf.host.write_pixels(&pixels32);
                }
            }
            4 => {
                let pixels = machine
                    .emu
                    .memory
                    .mem()
                    .view_n::<[u8; 4]>(surf.pixels, surf.width * surf.height);
                // XXX setting alpha channel manually, very inefficient :(
                let pixels32: Vec<_> = pixels.iter().map(|&[r, g, b, _a]| [r, g, b, 255]).collect();
                surf.host.write_pixels(&pixels32);
            }
            bpp => todo!("Unlock for {bpp}bpp"),
        }

        // If surface is primary then updates should show immediately.
        // XXX probably need something other than attached here
        if surf.attached == 0 {
            surf.host.show();
        }

        DD_OK
    }
}
