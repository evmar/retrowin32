#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::{heap::Heap, types::*};
use crate::{host, machine::Machine, winapi::vtable};
use bitflags::bitflags;
use memory::Pod;
use std::collections::HashMap;

const TRACE_CONTEXT: &'static str = "ddraw";

#[repr(C)]
#[derive(Debug)]
struct RECT {
    left: DWORD,
    top: DWORD,
    right: DWORD,
    bottom: DWORD,
}
unsafe impl memory::Pod for RECT {}

pub struct Surface {
    pub host: Box<dyn host::Surface>,
    pub width: u32,
    pub height: u32,
    pub palette: u32, // same as key in palettes
    /// x86 address to pixel buffer, or 0 if unused.
    pixels: u32,
}

pub struct State {
    heap: Heap,
    vtable_IDirectDraw: u32,
    vtable_IDirectDrawSurface: u32,
    vtable_IDirectDraw7: u32,
    vtable_IDirectDrawSurface7: u32,
    vtable_IDirectDrawPalette: u32,

    // TODO: this is per-IDirectDraw state.
    hwnd: HWND,
    /// Display width, after SetDisplayMode.
    width: u32,
    /// Display height, after SetDisplayMode.
    height: u32,
    pub surfaces: HashMap<u32, Surface>,
    palettes: HashMap<u32, Box<[PALETTEENTRY]>>,
    /// XXX monolife attaches palette only to back surface, then flips; we need to rearrange
    /// how surface flipping works for the palettes to work out, so this is hacked for now.
    palette_hack: u32,
}

impl State {
    pub fn new_init(machine: &mut Machine) -> Self {
        let mut ddraw = State::default();
        ddraw.heap = machine.state.kernel32.new_private_heap(
            &mut machine.memory,
            1 << 20,
            "ddraw.dll heap".into(),
        );

        ddraw.vtable_IDirectDraw = IDirectDraw::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDrawSurface = IDirectDrawSurface::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDraw7 = IDirectDraw7::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDrawSurface7 = IDirectDrawSurface7::vtable(&mut ddraw, machine);
        ddraw.vtable_IDirectDrawPalette = IDirectDrawPalette::vtable(&mut ddraw, machine);

        ddraw
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            heap: Heap::default(),
            vtable_IDirectDraw: 0,
            vtable_IDirectDrawSurface: 0,
            vtable_IDirectDraw7: 0,
            vtable_IDirectDrawSurface7: 0,
            vtable_IDirectDrawPalette: 0,
            hwnd: HWND::null(),
            width: 0,
            height: 0,
            surfaces: HashMap::new(),
            palettes: HashMap::new(),
            palette_hack: 0,
        }
    }
}

const DD_OK: u32 = 0;
// DD error codes are generated with this MAKE_HRESULT macro, maybe it doesn't matter too much.
const DDERR_GENERIC: u32 = 0x80004005;

#[repr(C)]
#[derive(Debug, Default)]
struct DDSCAPS2 {
    dwCaps: DWORD,
    dwCaps2: DWORD,
    dwCaps3: DWORD,
    dwCaps4: DWORD,
}
unsafe impl memory::Pod for DDSCAPS2 {}
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
    #[derive(Default)]
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

bitflags! {
    pub struct DDPCAPS: u32 {
        const _4BIT = 0x00000001;
        const _8BITENTRIES = 0x00000002;
        const _8BIT = 0x00000004;
        const INITIALIZE = 0x00000008;
        const PRIMARYSURFACE = 0x00000010;
        const PRIMARYSURFACELEFT = 0x00000020;
        const ALLOW256 = 0x00000040;
        const VSYNC = 0x00000080;
        const _1BIT = 0x00000100;
        const _2BIT = 0x00000200;
        const ALPHA =  0x00000400;
    }
}
impl TryFrom<u32> for DDPCAPS {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        DDPCAPS::from_bits(value).ok_or(value)
    }
}

bitflags! {
    pub struct DDLOCK: u32 {
        const SURFACEMEMORYPTR= 0x00000000;
        const WAIT = 0x00000001;
        const EVENT = 0x00000002;
        const READONLY = 0x00000010;
        const WRITEONLY = 0x00000020;
        const NOSYSLOCK = 0x00000800;
        const NOOVERWRITE = 0x00001000;
        const DISCARDCONTENTS = 0x00002000;
        const OKTOSWAP = 0x00002000;
        const DONOTWAIT = 0x00004000;
        const HASVOLUMETEXTUREBOXRECT = 0x00008000;
        const NODIRTYUPDATE = 0x00010000;
    }
}
impl TryFrom<u32> for DDLOCK {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        DDLOCK::from_bits(value).ok_or(value)
    }
}

#[repr(C)]
#[derive(Debug, Default)]
struct DDCOLORKEY {
    dwColorSpaceLowValue: DWORD,
    dwColorSpaceHighValue: DWORD,
}
unsafe impl memory::Pod for DDCOLORKEY {}

#[repr(C)]
#[derive(Debug)]
struct DDSURFACEDESC {
    dwSize: DWORD,
    dwFlags: DDSD,
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
    ddpfPixelFormat: DDPIXELFORMAT,
    ddsCaps: DDSCAPS,
}
unsafe impl memory::Pod for DDSURFACEDESC {}
impl DDSURFACEDESC {
    fn caps(&self) -> Option<&DDSCAPS> {
        if !self.dwFlags.contains(DDSD::CAPS) {
            return None;
        }
        Some(&self.ddsCaps)
    }
    fn back_buffer_count(&self) -> Option<u32> {
        if !self.dwFlags.contains(DDSD::BACKBUFFERCOUNT) {
            return None;
        }
        Some(self.dwBackBufferCount)
    }

    fn from_desc2(&mut self, desc2: &DDSURFACEDESC2) {
        self.dwFlags = desc2.dwFlags;
        self.dwHeight = desc2.dwHeight;
        self.dwWidth = desc2.dwWidth;

        self.lpSurface = desc2.lpSurface;
        self.lPitch_dwLinearSize = desc2.lPitch_dwLinearSize;
    }
}

#[repr(C)]
#[derive(Debug, Default)]
struct DDSURFACEDESC2 {
    dwSize: DWORD,
    dwFlags: DDSD,
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

    ddpfPixelFormat: DDPIXELFORMAT,
    // TODO: dwFVF
    ddsCaps: DDSCAPS2,
    dwTextureStage: DWORD,
}
unsafe impl memory::Pod for DDSURFACEDESC2 {}
impl DDSURFACEDESC2 {
    fn back_buffer_count(&self) -> Option<u32> {
        if !self.dwFlags.contains(DDSD::BACKBUFFERCOUNT) {
            return None;
        }
        Some(self.dwBackBufferCount_dwDepth)
    }
    fn caps(&self) -> Option<&DDSCAPS2> {
        if !self.dwFlags.contains(DDSD::CAPS) {
            return None;
        }
        Some(&self.ddsCaps)
    }
}

#[repr(C)]
#[derive(Debug, Default)]
struct DDPIXELFORMAT {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwFourCC: DWORD,
    dwRGBBitCount: DWORD,
    dwRBitMask: DWORD,
    dwGBitMask: DWORD,
    dwBBitMask: DWORD,
    dwRGBAlphaBitMask: DWORD,
}
unsafe impl memory::Pod for DDPIXELFORMAT {}

#[win32_derive::shims_from_x86]
mod IDirectDraw {
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

        let mut opts = host::SurfaceOptions::default();
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
            Surface {
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
mod IDirectDrawSurface {
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

        let surface = Surface {
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

const IID_IDirectDraw7: [u8; 16] = [
    0xc0, 0x5e, 0xe6, 0x15, 0x9c, 0x3b, 0xd2, 0x11, 0xb9, 0x2f, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b,
];

#[win32_derive::shims_from_x86]
mod IDirectDraw7 {
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
        WaitForVerticalBlank todo,
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
        _unused: u32,
    ) -> u32 {
        let desc = desc.unwrap();
        assert!(std::mem::size_of::<DDSURFACEDESC2>() == desc.dwSize as usize);
        let lpDirectDrawSurface7 = lpDirectDrawSurface7.unwrap();

        let mut opts = host::SurfaceOptions::default();
        if desc.dwFlags.contains(DDSD::WIDTH) {
            opts.width = desc.dwWidth;
        }
        if desc.dwFlags.contains(DDSD::HEIGHT) {
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
        *lpDirectDrawSurface7 = x86_surface;
        machine.state.ddraw.surfaces.insert(
            x86_surface,
            Surface {
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
        lpSurfaceDesc: Option<&DDSURFACEDESC2>,
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
            .alloc(mem, std::mem::size_of::<DDSURFACEDESC2>() as u32);
        let desc = mem.view_mut::<DDSURFACEDESC2>(desc_addr);
        unsafe { desc.clear_struct() };
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

        crate::shims::call_x86(machine, lpEnumCallback, vec![desc_addr, lpContext]).await;

        machine
            .state
            .ddraw
            .heap
            .free(machine.memory.mem(), desc_addr);

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
        machine.state.ddraw.width = width;
        machine.state.ddraw.height = height;
        if !machine.state.ddraw.hwnd.is_null() {
            machine
                .state
                .user32
                .get_window(machine.state.ddraw.hwnd)
                .host
                .set_size(width, height);
        }
        DD_OK
    }
}

#[win32_derive::shims_from_x86]
mod IDirectDrawSurface7 {
    use super::*;

    vtable![IDirectDrawSurface7 shims
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
        let lpDirectDrawSurface7 = ddraw.heap.alloc(machine.memory.mem(), 4);
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
            log::warn!("BltFlat flags: {:x}", flags);
        }
        let (dst, src) = unsafe {
            let dst = machine.state.ddraw.surfaces.get_mut(&this).unwrap() as *mut Surface;
            let src = machine.state.ddraw.surfaces.get(&lpSurf).unwrap() as *const Surface;
            assert_ne!(dst as *const Surface, src);
            (&mut *dst, &*src)
        };
        let rect = lpRect.unwrap();
        let sx = rect.left;
        let w = rect.right - sx;
        let sy = rect.top;
        let h = rect.bottom - sy;
        dst.host.bit_blt(x, y, src.host.as_ref(), sx, sy, w, h);
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
        let surface = machine.state.ddraw.surfaces.get_mut(&this).unwrap();
        surface.host.flip();
        DD_OK
    }

    #[win32_derive::dllexport]
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
            palette: this_surface.palette,
            pixels: this_surface.pixels,
        };
        let x86_surface = new(machine);

        machine.mem().put::<u32>(lpDirectDrawSurface7, x86_surface);
        machine.state.ddraw.surfaces.insert(x86_surface, surface);
        DD_OK
    }

    #[win32_derive::dllexport]
    fn GetDC(machine: &mut Machine, this: u32, lpHDC: u32) -> u32 {
        let (handle, dc) = machine.state.gdi32.new_dc();
        dc.ddraw_surface = this;
        machine.mem().put::<u32>(lpHDC, handle);
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
    pub(super) fn Lock(
        machine: &mut Machine,
        this: u32,
        rect: Option<&RECT>,
        desc: Option<&mut DDSURFACEDESC2>,
        flags: Result<DDLOCK, u32>,
        unused: u32,
    ) -> u32 {
        if rect.is_some() {
            // TODO: once we implement this, we need corresponding logic in Unlock.
            todo!();
        }
        let desc = desc.unwrap();
        let surf = machine.state.ddraw.surfaces.get_mut(&this).unwrap();
        let bytes_per_pixel = 1; // TODO: where does this come from?
        if surf.pixels == 0 {
            surf.pixels = machine.state.ddraw.heap.alloc(
                machine.memory.mem(),
                surf.width * surf.height * bytes_per_pixel,
            );
        }
        desc.dwFlags = DDSD::LPSURFACE;
        desc.lpSurface = surf.pixels;
        desc.lPitch_dwLinearSize = surf.width * bytes_per_pixel;
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
    pub(super) fn Unlock(machine: &mut Machine, this: u32, rect: Option<&mut RECT>) -> u32 {
        let surf = machine.state.ddraw.surfaces.get_mut(&this).unwrap();
        if let Some(rect) = rect {
            // TODO: needs to match the rect passed in Lock.
            rect.left = 0;
            rect.top = 0;
            rect.right = surf.width;
            rect.bottom = surf.height;
        }
        let phack = machine.state.ddraw.palette_hack;
        if surf.pixels != 0 && phack != 0 {
            let bytes_per_pixel = 1; // TODO: where does this come from?
            let pixels = machine
                .memory
                .mem()
                .view_n::<u8>(surf.pixels, surf.width * surf.height * bytes_per_pixel);
            let palette = machine.state.ddraw.palettes.get(&phack).unwrap();
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
        DD_OK
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
struct PALETTEENTRY {
    peRed: u8,
    peGreen: u8,
    peBlue: u8,
    peFlags: u8,
}
unsafe impl memory::Pod for PALETTEENTRY {}

#[win32_derive::shims_from_x86]
mod IDirectDrawPalette {
    use super::*;

    vtable![IDirectDrawPalette shims
        QueryInterface todo,
        AddRef todo,
        Release ok,
        GetCaps todo,
        GetEntries todo,
        Initialize todo,
        SetEntries ok,
    ];

    pub fn new(machine: &mut Machine) -> u32 {
        let ddraw = &mut machine.state.ddraw;
        let lpDirectDrawPalette = ddraw.heap.alloc(machine.memory.mem(), 4);
        let vtable = ddraw.vtable_IDirectDrawPalette;
        machine.mem().put::<u32>(lpDirectDrawPalette, vtable);
        lpDirectDrawPalette
    }

    #[win32_derive::dllexport]
    fn Release(_machine: &mut Machine, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    #[win32_derive::dllexport]
    fn SetEntries(
        machine: &mut Machine,
        this: u32,
        unused: u32,
        start: u32,
        count: u32,
        entries: u32,
    ) -> u32 {
        let palette = machine.state.ddraw.palettes.get_mut(&this).unwrap();
        // TODO: if palette is DDPCAPS_8BITENTRIES then entries are one byte, not 4.
        let entries = machine.memory.mem().view_n::<PALETTEENTRY>(entries, count);
        palette[start as usize..][..count as usize].clone_from_slice(entries);
        DD_OK
    }
}

#[win32_derive::dllexport]
pub fn DirectDrawCreate(machine: &mut Machine, lpGuid: u32, lplpDD: u32, pUnkOuter: u32) -> u32 {
    DirectDrawCreateEx(machine, lpGuid, lplpDD, 0, pUnkOuter)
}

#[win32_derive::dllexport]
pub fn DirectDrawCreateEx(
    machine: &mut Machine,
    lpGuid: u32,
    lplpDD: u32,
    iid: u32,
    pUnkOuter: u32,
) -> u32 {
    assert!(lpGuid == 0);
    assert!(pUnkOuter == 0);

    if machine.state.ddraw.heap.addr == 0 {
        machine.state.ddraw = State::new_init(machine);
    }
    let ddraw = &mut machine.state.ddraw;

    if iid == 0 {
        // DirectDrawCreate
        let lpDirectDraw = ddraw.heap.alloc(machine.memory.mem(), 4);
        let vtable = ddraw.vtable_IDirectDraw;
        machine.mem().put::<u32>(lpDirectDraw, vtable);
        machine.mem().put::<u32>(lplpDD, lpDirectDraw);
        return DD_OK;
    }

    let iid_slice = machine.memory.mem().sub(iid, 16).as_slice_todo();
    if iid_slice == IID_IDirectDraw7 {
        // Caller gives us:
        //   pointer (lplpDD) that they want us to fill in to point to ->
        //   [vtable, ...] (lpDirectDraw7), where vtable is pointer to ->
        //   [fn1, fn2, ...] (vtable_IDirectDraw7)
        let lpDirectDraw7 = ddraw.heap.alloc(machine.memory.mem(), 4);
        let vtable = ddraw.vtable_IDirectDraw7;
        machine.mem().put::<u32>(lpDirectDraw7, vtable);
        machine.mem().put::<u32>(lplpDD, lpDirectDraw7);
        DD_OK
    } else {
        log::error!("DirectDrawCreateEx: unknown IID {iid_slice:x?}");
        DDERR_GENERIC
    }
}
