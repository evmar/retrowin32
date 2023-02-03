#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::collections::HashMap;

use crate::{
    host,
    machine::Machine,
    memory::{Memory, Pod},
    winapi::vtable,
    winapi::winapi_shims,
};

use super::{alloc::Alloc, types::DWORD};
use bitflags::bitflags;

#[repr(C)]
#[derive(Debug)]
struct RECT {
    left: DWORD,
    top: DWORD,
    right: DWORD,
    bottom: DWORD,
}
unsafe impl Pod for RECT {}

pub struct Surface {
    pub host: Box<dyn host::Surface>,
    pub width: u32,
    pub height: u32,
}

pub struct State {
    hheap: u32,
    vtable_IDirectDraw: u32,
    vtable_IDirectDrawSurface: u32,
    vtable_IDirectDraw7: u32,
    vtable_IDirectDrawSurface7: u32,

    // TODO: this is per-IDirectDraw state.
    hwnd: u32,
    width: u32,
    height: u32,
    pub surfaces: HashMap<u32, Surface>,
}
impl State {
    pub fn new_empty() -> Self {
        State {
            hheap: 0,
            vtable_IDirectDraw: 0,
            vtable_IDirectDrawSurface: 0,
            vtable_IDirectDraw7: 0,
            vtable_IDirectDrawSurface7: 0,
            hwnd: 0,
            width: 0,
            height: 0,
            surfaces: HashMap::new(),
        }
    }

    pub fn new_init(machine: &mut Machine) -> Self {
        let mut ddraw = State::new_empty();
        ddraw.hheap =
            machine
                .state
                .kernel32
                .new_heap(&mut machine.x86.mem, 0x1000, "ddraw.dll heap".into());

        ddraw.vtable_IDirectDraw = IDirectDraw::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDrawSurface = IDirectDrawSurface::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDraw7 = IDirectDraw7::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDrawSurface7 = IDirectDrawSurface7::vtable(&mut ddraw, machine);
        ddraw
    }
}

const DD_OK: u32 = 0;
// DD error codes are generated with this MAKE_HRESULT macro, maybe it doesn't matter too much.
const DDERR_GENERIC: u32 = 0x80004005;

#[repr(C)]
#[derive(Debug)]
struct DDSCAPS2 {
    dwCaps: DWORD,
    dwCaps2: DWORD,
    dwCaps3: DWORD,
    dwCaps4: DWORD,
}
unsafe impl Pod for DDSCAPS2 {}
impl DDSCAPS2 {
    fn caps1(&self) -> DDSCAPS {
        unsafe { DDSCAPS::from_bits_unchecked(self.dwCaps) }
    }
}

bitflags! {
    pub struct DDSCAPS: u32 {
        const ALPHA = 0x00000002;
        const BACKBUFFER = 0x00000004;
        const COMPLEX = 0x00000008;
        const FLIP = 0x00000010;
        const FRONTBUFFER = 0x00000020;
        const OFFSCREENPLAIN = 0x00000040;
        const OVERLAY = 0x00000080;
        const PALETTE = 0x00000100;
        const PRIMARYSURFACE = 0x00000200;
        const PRIMARYSURFACELEFT = 0x00000400;
        const SYSTEMMEMORY = 0x00000800;
        const TEXTURE = 0x00001000;
        const _3DDEVICE = 0x00002000;
        const VIDEOMEMORY = 0x00004000;
        const VISIBLE = 0x00008000;
        const WRITEONLY = 0x00010000;
        const ZBUFFER = 0x00020000;
        const OWNDC = 0x00040000;
        const LIVEVIDEO = 0x00080000;
        const HWCODEC = 0x00100000;
        const MODEX = 0x00200000;
        const MIPMAP = 0x00400000;
        const ALLOCONLOAD = 0x04000000;
        const VIDEOPORT = 0x08000000;
        const LOCALVIDMEM = 0x10000000;
        const NONLOCALVIDMEM = 0x20000000;
        const STANDARDVGAMODE = 0x40000000;
    }
}

bitflags! {
    pub struct DDSD: u32 {
        const CAPS = 0x00000001;
        const HEIGHT = 0x00000002;
        const WIDTH = 0x00000004;
        const PITCH = 0x00000008;
        const BACKBUFFERCOUNT = 0x00000020;
        const ZBUFFERBITDEPTH = 0x00000040;
        const ALPHABITDEPTH = 0x00000080;
        const LPSURFACE = 0x00000800;
        const PIXELFORMAT = 0x00001000;
        const CKDESTOVERLAY = 0x00002000;
        const CKDESTBLT = 0x00004000;
        const CKSRCOVERLAY= 0x00008000;
        const CKSRCBLT = 0x00010000;
        const MIPMAPCOUNT = 0x00020000;
        const REFRESHRATE = 0x00040000;
        const LINEARSIZE = 0x00080000;
        const TEXTURESTAGE = 0x00100000;
        const FVF = 0x00200000;
        const SRCVBHANDLE = 0x00400000;
        const DEPTH = 0x00800000;
    }
}

#[repr(C)]
#[derive(Debug)]
struct DDCOLORKEY {
    dwColorSpaceLowValue: DWORD,
    dwColorSpaceHighValue: DWORD,
}
unsafe impl Pod for DDCOLORKEY {}

#[repr(C)]
#[derive(Debug)]
struct DDSURFACEDESC {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwHeight: DWORD,
    dwWidth: DWORD,
    lPitch_dwLinearSize: DWORD,
    dwBackBufferCount: DWORD,
    dwMipMapCount_dwZBufferBitDepth_dwRefreshRate: DWORD,
    dwAlphaBitDepth: DWORD,
    dwReserved: DWORD,
    lpSurface: DWORD,
    ddckCKDestOverlay: DDCOLORKEY,
    ddckCKDestBlt: DDCOLORKEY,
    ddckCKSrcOverlay: DDCOLORKEY,
    ddckCKSrcBlt: DDCOLORKEY,
    ddpfPixelFormat: [DWORD; 8],
    ddsCaps: DDSCAPS,
}
unsafe impl Pod for DDSURFACEDESC {}
impl DDSURFACEDESC {
    fn flags(&self) -> DDSD {
        unsafe { DDSD::from_bits_unchecked(self.dwFlags) }
    }
    fn caps(&self) -> Option<&DDSCAPS> {
        if !self.flags().contains(DDSD::CAPS) {
            return None;
        }
        Some(&self.ddsCaps)
    }
    fn back_buffer_count(&self) -> Option<u32> {
        if !self.flags().contains(DDSD::BACKBUFFERCOUNT) {
            return None;
        }
        Some(self.dwBackBufferCount)
    }
}

#[repr(C)]
#[derive(Debug)]
struct DDSURFACEDESC2 {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwHeight: DWORD,
    dwWidth: DWORD,

    lPitch_dwLinearSize: DWORD,
    dwBackBufferCount_dwDepth: DWORD,
    dwMipMapCount_dwRefreshRate_dwSrcVBHandle: DWORD,

    dwAlphaBitDepth: DWORD,
    dwReserved: DWORD,
    lpSurface: DWORD,
    ddckCKDestOverlay_dwEmptyFaceColor: DDCOLORKEY,
    ddckCKDestBlt: DDCOLORKEY,
    ddckCKSrcOverlay: DDCOLORKEY,
    ddckCKSrcBlt: DDCOLORKEY,

    ddpfPixelFormat: [DWORD; 8],
    // TODO: dwFVF
    ddsCaps: DDSCAPS2,
    dwTextureStage: DWORD,
}
unsafe impl Pod for DDSURFACEDESC2 {}
impl DDSURFACEDESC2 {
    fn flags(&self) -> DDSD {
        unsafe { DDSD::from_bits_unchecked(self.dwFlags) }
    }
    fn back_buffer_count(&self) -> Option<u32> {
        if !self.flags().contains(DDSD::BACKBUFFERCOUNT) {
            return None;
        }
        Some(self.dwBackBufferCount_dwDepth)
    }
    fn caps(&self) -> Option<&DDSCAPS2> {
        if !self.flags().contains(DDSD::CAPS) {
            return None;
        }
        Some(&self.ddsCaps)
    }
}

#[repr(C)]
#[derive(Debug)]
struct DDPIXELFORMAT {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwFourCC: DWORD,
    u1: DWORD,
    u2: DWORD,
    u3: DWORD,
    u4: DWORD,
    u5: DWORD,
}
unsafe impl Pod for DDPIXELFORMAT {}

mod IDirectDraw {
    use super::*;

    vtable![shims
        QueryInterface todo,
        AddRef todo,
        Release todo,
        Compact todo,
        CreateClipper todo,
        CreatePalette todo,
        CreateSurface ok,
        DuplicateSurface todo,
        EnumDisplayModes todo,
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

    fn CreateSurface(
        machine: &mut Machine,
        _this: u32,
        desc: Option<&DDSURFACEDESC>,
        lplpDDSurface: u32,
        _pUnkOuter: u32,
    ) -> u32 {
        let desc = desc.unwrap();
        assert!(std::mem::size_of::<DDSURFACEDESC>() == desc.dwSize as usize);

        let mut opts = host::SurfaceOptions::default();
        let mut flags = desc.flags();
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
        machine.x86.mem.write_u32(lplpDDSurface, x86_surface);
        machine.state.ddraw.surfaces.insert(
            x86_surface,
            Surface {
                host: surface,
                width: opts.width,
                height: opts.height,
            },
        );

        DD_OK
    }

    fn SetDisplayMode(machine: &mut Machine, this: u32, width: u32, height: u32, bpp: u32) -> u32 {
        log::warn!("{this:x}->SetDisplayMode({width}x{height}x{bpp})");
        machine.state.ddraw.width = width;
        machine.state.ddraw.height = height;
        DD_OK
    }

    winapi_shims!(
        fn CreateSurface(
            this: u32,
            desc: Option<&DDSURFACEDESC>,
            lplpDDSurface: u32,
            pUnkOuter: u32,
        );
        fn SetDisplayMode(this: u32, width: u32, height: u32, bpp: u32);
    );
}

mod IDirectDrawSurface {
    use super::*;

    vtable![shims
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
        Lock todo,
        ReleaseDC todo,
        Restore todo,
        SetClipper todo,
        SetColorKey todo,
        SetOverlayPosition todo,
        SetPalette todo,
        Unlock todo,
        UpdateOverlay todo,
        UpdateOverlayDisplay todo,
        UpdateOverlayZOrder todo,
    ];

    pub fn new(machine: &mut Machine) -> u32 {
        let ddraw = &machine.state.ddraw;
        let lpDirectDrawSurface = machine
            .state
            .kernel32
            .get_heap(&mut machine.x86.mem, ddraw.hheap)
            .unwrap()
            .alloc(4);
        let vtable = ddraw.vtable_IDirectDrawSurface;
        machine.x86.mem.write_u32(lpDirectDrawSurface, vtable);
        lpDirectDrawSurface
    }

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

        let surface = Surface {
            host,
            width: this_surface.width,
            height: this_surface.height,
        };
        let x86_surface = new(machine);

        machine.x86.mem.write_u32(lpDirectDrawSurface, x86_surface);
        machine.state.ddraw.surfaces.insert(x86_surface, surface);
        DD_OK
    }

    fn GetPixelFormat(_machine: &mut Machine, fmt: Option<&mut DDPIXELFORMAT>) -> u32 {
        let fmt = fmt.unwrap();
        *fmt = unsafe { std::mem::zeroed() };
        fmt.dwSize = std::mem::size_of::<DDPIXELFORMAT>() as u32;
        DD_OK
    }

    winapi_shims!(
        fn GetAttachedSurface(this: u32, lpDDSCaps: u32, lpDirectDrawSurface: u32);
        fn GetPixelFormat(fmt: Option<&mut DDPIXELFORMAT>);
    );
}

const IID_IDirectDraw7: [u8; 16] = [
    0xc0, 0x5e, 0xe6, 0x15, 0x9c, 0x3b, 0xd2, 0x11, 0xb9, 0x2f, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b,
];

mod IDirectDraw7 {
    use super::*;

    vtable![shims
        QueryInterface todo,
        AddRef todo,
        Release ok,
        Compact todo,
        CreateClipper todo,
        CreatePalette todo,
        CreateSurface ok,
        DuplicateSurface todo,
        EnumDisplayModes todo,
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
        WaitForVerticalBlank todo,
        GetAvailableVidMem todo,
        GetSurfaceFromDC todo,
        RestoreAllSurfaces todo,
        TestCooperativeLevel todo,
        GetDeviceIdentifier todo,
        StartModeTest todo,
        EvaluateMode todo,
    ];

    fn Release(_machine: &mut Machine, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    fn CreateSurface(
        machine: &mut Machine,
        this: u32,
        desc: Option<&DDSURFACEDESC2>,
        lpDirectDrawSurface7: u32,
        _unused: u32,
    ) -> u32 {
        let desc = desc.unwrap();
        assert!(std::mem::size_of::<DDSURFACEDESC2>() == desc.dwSize as usize);

        log::warn!(
            "{this:x}->CreateSurface({:?}, {desc:?}, {lpDirectDrawSurface7:x})",
            desc.flags()
        );

        let mut opts = host::SurfaceOptions::default();
        if desc.flags().contains(DDSD::WIDTH) {
            opts.width = desc.dwWidth;
        }
        if desc.flags().contains(DDSD::HEIGHT) {
            opts.height = desc.dwHeight;
        }
        if let Some(caps) = desc.caps() {
            log::warn!("  caps: {:?}", caps.caps1());
            if caps.caps1().contains(DDSCAPS::PRIMARYSURFACE) {
                opts.width = machine.state.ddraw.width;
                opts.height = machine.state.ddraw.height;
                opts.primary = true;
            }
        }

        if let Some(count) = desc.back_buffer_count() {
            log::warn!("  back_buffer: {count:x}");
        }

        //let window = machine.state.user32.get_window(machine.state.ddraw.hwnd);
        let surface = machine.host.create_surface(&opts);

        let x86_surface = IDirectDrawSurface7::new(machine);
        machine.x86.mem.write_u32(lpDirectDrawSurface7, x86_surface);
        machine.state.ddraw.surfaces.insert(
            x86_surface,
            Surface {
                host: surface,
                width: opts.width,
                height: opts.height,
            },
        );

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

    pub fn SetCooperativeLevel(machine: &mut Machine, _this: u32, hwnd: u32, _flags: u32) -> u32 {
        // TODO: this triggers behaviors like fullscreen.
        // let flags = DDSCL::from_bits(flags).unwrap();
        // log::warn!("{this:x}->SetCooperativeLevel({hwnd:x}, {flags:?})");
        machine.state.ddraw.hwnd = hwnd;
        DD_OK
    }

    fn SetDisplayMode(
        machine: &mut Machine,
        this: u32,
        width: u32,
        height: u32,
        bpp: u32,
        refresh: u32,
        flags: u32,
    ) -> u32 {
        log::warn!("{this:x}->SetDisplayMode({width}x{height}x{bpp}@{refresh}hz, {flags:x})");
        machine.state.ddraw.width = width;
        machine.state.ddraw.height = height;
        DD_OK
    }

    winapi_shims!(
        fn Release(this: u32);
        fn CreateSurface(
            this: u32,
            desc: Option<&DDSURFACEDESC2>,
            lpDirectDrawSurface7: u32,
            unused: u32,
        );
        fn SetCooperativeLevel(this: u32, hwnd: u32, flags: u32);
        fn SetDisplayMode(this: u32, width: u32, height: u32, bpp: u32, refresh: u32, flags: u32);
    );
}

mod IDirectDrawSurface7 {
    use super::*;

    vtable![shims
        QueryInterface todo,
        AddRef todo,
        Release ok,
        AddAttachedSurface todo,
        AddOverlayDirtyRect todo,
        Blt todo,
        BltBatch todo,
        BltFast ok,
        DeleteAttachedSurface todo,
        EnumAttachedSurfaces todo,
        EnumOverlayZOrders todo,
        Flip ok,
        GetAttachedSurface ok,
        GetBltStatus todo,
        GetCaps todo,
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
        Lock todo,
        ReleaseDC ok,
        Restore ok,
        SetClipper todo,
        SetColorKey todo,
        SetOverlayPosition todo,
        SetPalette todo,
        Unlock todo,
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
        let lpDirectDrawSurface7 = machine
            .state
            .kernel32
            .get_heap(&mut machine.x86.mem, ddraw.hheap)
            .unwrap()
            .alloc(4);
        let vtable = ddraw.vtable_IDirectDrawSurface7;
        machine.x86.mem.write_u32(lpDirectDrawSurface7, vtable);
        lpDirectDrawSurface7
    }

    fn Release(_machine: &mut Machine, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    fn BltFast(
        machine: &mut Machine,
        this: u32,
        x: u32,
        y: u32,
        lpSurf: u32,
        lpRect: u32,
        flags: u32,
    ) -> u32 {
        if flags != 0 {
            log::warn!("BltFlat flags: {:x}", flags);
        }
        let (dst, src) = unsafe {
            let dst = machine.state.ddraw.surfaces.get_mut(&this).unwrap() as *mut Surface;
            let src = machine.state.ddraw.surfaces.get(&lpSurf).unwrap() as *const Surface;
            assert_ne!(dst as *const Surface, src);
            (&mut *dst, &*src)
        };
        let rect = machine.x86.mem.view::<RECT>(lpRect);
        let sx = rect.left;
        let w = rect.right - sx;
        let sy = rect.top;
        let h = rect.bottom - sy;
        dst.host.bit_blt(x, y, src.host.as_ref(), sx, sy, w, h);
        DD_OK
    }

    fn Flip(machine: &mut Machine, this: u32, lpSurf: u32, flags: u32) -> u32 {
        if lpSurf != 0 || flags != 0 {
            log::warn!("{this:x}->Flip({lpSurf:x}, {flags:x})");
        }
        let surface = machine.state.ddraw.surfaces.get_mut(&this).unwrap();
        surface.host.flip();
        DD_OK
    }

    fn GetAttachedSurface(
        machine: &mut Machine,
        this: u32,
        _lpDDSCaps2: u32,
        lpDirectDrawSurface7: u32,
    ) -> u32 {
        // TODO: consider caps.
        // log::warn!("{this:x}->GetAttachedSurface({lpDDSCaps2:x}, {lpDirectDrawSurface7:x})");
        let this_surface = machine.state.ddraw.surfaces.get(&this).unwrap();
        let host = this_surface.host.get_attached();

        let surface = Surface {
            host,
            width: this_surface.width,
            height: this_surface.height,
        };
        let x86_surface = new(machine);

        machine.x86.mem.write_u32(lpDirectDrawSurface7, x86_surface);
        machine.state.ddraw.surfaces.insert(x86_surface, surface);
        DD_OK
    }

    fn GetDC(machine: &mut Machine, this: u32, lpHDC: u32) -> u32 {
        let (handle, dc) = machine.state.gdi32.new_dc();
        dc.ddraw_surface = this;
        machine.x86.mem.write_u32(lpHDC, handle);
        DD_OK
    }

    fn GetSurfaceDesc(machine: &mut Machine, this: u32, lpDesc: u32) -> u32 {
        let surf = machine.state.ddraw.surfaces.get(&this).unwrap();
        let desc = machine.x86.mem.view_mut::<DDSURFACEDESC2>(lpDesc);
        assert!(desc.dwSize as usize == std::mem::size_of::<DDSURFACEDESC2>());
        let mut flags = desc.flags();
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
                "unimp: {:?} for {this:x}->GetSurfaceDesc({lpDesc:x})",
                desc.flags()
            );
        }
        DDERR_GENERIC
    }

    fn ReleaseDC(_machine: &mut Machine, _this: u32, _hDC: u32) -> u32 {
        // leak
        DD_OK
    }

    fn Restore(_machine: &mut Machine, _this: u32) -> u32 {
        DD_OK
    }

    winapi_shims!(
        fn Release(this: u32);
        fn BltFast(this: u32, x: u32, y: u32, lpSurf: u32, lpRect: u32, flags: u32);
        fn Flip(this: u32, lpSurf: u32, flags: u32);
        fn GetAttachedSurface(this: u32, lpDDSCaps2: u32, lpDirectDrawSurface7: u32);
        fn GetDC(this: u32, lpHDC: u32);
        fn GetSurfaceDesc(this: u32, lpDesc: u32);
        fn ReleaseDC(this: u32, hDC: u32);
        fn Restore(this: u32);
    );
}

pub fn DirectDrawCreate(machine: &mut Machine, lpGuid: u32, lplpDD: u32, pUnkOuter: u32) -> u32 {
    DirectDrawCreateEx(machine, lpGuid, lplpDD, 0, pUnkOuter)
}

pub fn DirectDrawCreateEx(
    machine: &mut Machine,
    lpGuid: u32,
    lplpDD: u32,
    iid: u32,
    pUnkOuter: u32,
) -> u32 {
    assert!(lpGuid == 0);
    assert!(pUnkOuter == 0);

    if machine.state.ddraw.hheap == 0 {
        machine.state.ddraw = State::new_init(machine);
    }
    let ddraw = &mut machine.state.ddraw;

    if iid == 0 {
        // DirectDrawCreate
        let lpDirectDraw = machine
            .state
            .kernel32
            .get_heap(&mut machine.x86.mem, ddraw.hheap)
            .unwrap()
            .alloc(4);
        let vtable = ddraw.vtable_IDirectDraw;
        machine.x86.write_u32(lpDirectDraw, vtable);
        machine.x86.write_u32(lplpDD, lpDirectDraw);
        return DD_OK;
    }

    let iid_slice = &machine.x86.mem[iid as usize..(iid + 16) as usize];
    if iid_slice == IID_IDirectDraw7 {
        // Caller gives us:
        //   pointer (lplpDD) that they want us to fill in to point to ->
        //   [vtable, ...] (lpDirectDraw7), where vtable is pointer to ->
        //   [fn1, fn2, ...] (vtable_IDirectDraw7)
        let lpDirectDraw7 = machine
            .state
            .kernel32
            .get_heap(&mut machine.x86.mem, ddraw.hheap)
            .unwrap()
            .alloc(4);
        let vtable = ddraw.vtable_IDirectDraw7;
        machine.x86.write_u32(lpDirectDraw7, vtable);
        machine.x86.write_u32(lplpDD, lpDirectDraw7);
        DD_OK
    } else {
        log::error!("DirectDrawCreateEx: unknown IID {iid_slice:x?}");
        DDERR_GENERIC
    }
}
