#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::collections::HashMap;

use crate::{
    memory::{Memory, Pod, DWORD},
    winapi, X86,
};

use super::kernel32;
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

struct Surface {
    width: u32,
    height: u32,
}

pub struct State {
    hheap: u32,
    vtable_IDirectDraw7: u32,
    vtable_IDirectDrawSurface7: u32,

    width: u32,
    height: u32,
    surfaces: HashMap<u32, Surface>,
}
impl State {
    pub fn new(x86: &mut X86) -> Self {
        let mut ddraw = State {
            hheap: 0,
            vtable_IDirectDraw7: 0,
            vtable_IDirectDrawSurface7: 0,
            width: 0,
            height: 0,
            surfaces: HashMap::new(),
        };
        ddraw.hheap = x86
            .state
            .kernel32
            .new_heap(&mut x86.mem, 0x1000, "ddraw.dll heap".into());

        ddraw.vtable_IDirectDraw7 = IDirectDraw7::vtable(&mut ddraw, x86);
        ddraw.vtable_IDirectDrawSurface7 = IDirectDrawSurface7::vtable(&mut ddraw, x86);
        ddraw
    }

    fn heap<'a>(&mut self, kernel32: &'a mut kernel32::State) -> &'a mut kernel32::Heap {
        kernel32.heaps.get_mut(&self.hheap).unwrap()
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
        unsafe { DDSCAPS::from_bits_unchecked(self.dwCaps.get()) }
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
        unsafe { DDSD::from_bits_unchecked(self.dwFlags.get()) }
    }
    fn back_buffer_count(&self) -> Option<u32> {
        if !self.flags().contains(DDSD::BACKBUFFERCOUNT) {
            return None;
        }
        Some(self.dwBackBufferCount_dwDepth.get())
    }
    fn caps(&self) -> Option<&DDSCAPS2> {
        if !self.flags().contains(DDSD::CAPS) {
            return None;
        }
        Some(&self.ddsCaps)
    }
}

const IID_IDirectDraw7: [u8; 16] = [
    0xc0, 0x5e, 0xe6, 0x15, 0x9c, 0x3b, 0xd2, 0x11, 0xb9, 0x2f, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b,
];

mod IDirectDraw7 {
    use crate::memory::DWORD;

    use super::*;

    #[repr(C)]
    pub(super) struct Vtable {
        pub QueryInterface: DWORD,
        pub AddRef: DWORD,
        pub Release: DWORD,
        pub Compact: DWORD,
        pub CreateClipper: DWORD,
        pub CreatePalette: DWORD,
        pub CreateSurface: DWORD,
        pub DuplicateSurface: DWORD,
        pub EnumDisplayModes: DWORD,
        pub EnumSurfaces: DWORD,
        pub FlipToGDISurface: DWORD,
        pub GetCaps: DWORD,
        pub GetDisplayMode: DWORD,
        pub GetFourCCCodes: DWORD,
        pub GetGDISurface: DWORD,
        pub GetMonitorFrequency: DWORD,
        pub GetScanLine: DWORD,
        pub GetVerticalBlankStatus: DWORD,
        pub Initialize: DWORD,
        pub RestoreDisplayMode: DWORD,
        pub SetCooperativeLevel: DWORD,
        pub SetDisplayMode: DWORD,
        pub WaitForVerticalBlank: DWORD,
        pub GetAvailableVidMem: DWORD,
        pub GetSurfaceFromDC: DWORD,
        pub RestoreAllSurfaces: DWORD,
        pub TestCooperativeLevel: DWORD,
        pub GetDeviceIdentifier: DWORD,
        pub StartModeTest: DWORD,
        pub EvaluateMode: DWORD,
    }
    unsafe impl crate::memory::Pod for Vtable {}

    pub fn vtable(ddraw: &mut State, x86: &mut X86) -> u32 {
        let addr = ddraw.heap(&mut x86.state.kernel32).alloc(
            &mut x86.mem,
            std::mem::size_of::<IDirectDraw7::Vtable>() as u32,
        );
        let vtable = x86.mem.view_mut::<Vtable>(addr);
        *vtable = Vtable {
            QueryInterface: x86
                .shims
                .add(Err("IDirectDraw7::QueryInterface unimplemented".into()))
                .into(),
            AddRef: x86
                .shims
                .add(Err("IDirectDraw7::AddRef unimplemented".into()))
                .into(),
            Release: x86.shims.add(Ok(shims::Release)).into(),
            Compact: x86
                .shims
                .add(Err("IDirectDraw7::Compact unimplemented".into()))
                .into(),
            CreateClipper: x86
                .shims
                .add(Err("IDirectDraw7::CreateClipper unimplemented".into()))
                .into(),
            CreatePalette: x86
                .shims
                .add(Err("IDirectDraw7::CreatePalette unimplemented".into()))
                .into(),
            CreateSurface: x86.shims.add(Ok(shims::CreateSurface)).into(),
            DuplicateSurface: x86
                .shims
                .add(Err("IDirectDraw7::DuplicateSurface unimplemented".into()))
                .into(),
            EnumDisplayModes: x86
                .shims
                .add(Err("IDirectDraw7::EnumDisplayModes unimplemented".into()))
                .into(),
            EnumSurfaces: x86
                .shims
                .add(Err("IDirectDraw7::EnumSurfaces unimplemented".into()))
                .into(),
            FlipToGDISurface: x86
                .shims
                .add(Err("IDirectDraw7::FlipToGDISurface unimplemented".into()))
                .into(),
            GetCaps: x86
                .shims
                .add(Err("IDirectDraw7::GetCaps unimplemented".into()))
                .into(),
            GetDisplayMode: x86
                .shims
                .add(Err("IDirectDraw7::GetDisplayMode unimplemented".into()))
                .into(),
            GetFourCCCodes: x86
                .shims
                .add(Err("IDirectDraw7::GetFourCCCodes unimplemented".into()))
                .into(),
            GetGDISurface: x86
                .shims
                .add(Err("IDirectDraw7::GetGDISurface unimplemented".into()))
                .into(),
            GetMonitorFrequency: x86
                .shims
                .add(Err("IDirectDraw7::GetMonitorFrequency unimplemented".into()))
                .into(),
            GetScanLine: x86
                .shims
                .add(Err("IDirectDraw7::GetScanLine unimplemented".into()))
                .into(),
            GetVerticalBlankStatus: x86
                .shims
                .add(Err(
                    "IDirectDraw7::GetVerticalBlankStatus unimplemented".into()
                ))
                .into(),
            Initialize: x86
                .shims
                .add(Err("IDirectDraw7::Initialize unimplemented".into()))
                .into(),
            RestoreDisplayMode: x86
                .shims
                .add(Err("IDirectDraw7::RestoreDisplayMode unimplemented".into()))
                .into(),
            SetCooperativeLevel: x86.shims.add(Ok(shims::SetCooperativeLevel)).into(),
            SetDisplayMode: x86.shims.add(Ok(shims::SetDisplayMode)).into(),
            WaitForVerticalBlank: x86
                .shims
                .add(Err(
                    "IDirectDraw7::WaitForVerticalBlank unimplemented".into()
                ))
                .into(),
            GetAvailableVidMem: x86
                .shims
                .add(Err("IDirectDraw7::GetAvailableVidMem unimplemented".into()))
                .into(),
            GetSurfaceFromDC: x86
                .shims
                .add(Err("IDirectDraw7::GetSurfaceFromDC unimplemented".into()))
                .into(),
            RestoreAllSurfaces: x86
                .shims
                .add(Err("IDirectDraw7::RestoreAllSurfaces unimplemented".into()))
                .into(),
            TestCooperativeLevel: x86
                .shims
                .add(Err(
                    "IDirectDraw7::TestCooperativeLevel unimplemented".into()
                ))
                .into(),
            GetDeviceIdentifier: x86
                .shims
                .add(Err("IDirectDraw7::GetDeviceIdentifier unimplemented".into()))
                .into(),
            StartModeTest: x86
                .shims
                .add(Err("IDirectDraw7::StartModeTest unimplemented".into()))
                .into(),
            EvaluateMode: x86
                .shims
                .add(Err("IDirectDraw7::EvaluateMode unimplemented".into()))
                .into(),
        };

        addr
    }

    fn Release(_x86: &mut X86, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    fn CreateSurface(
        x86: &mut X86,
        this: u32,
        lpSurfaceDesc: u32,
        lpDirectDrawSurface7: u32,
        _unused: u32,
    ) -> u32 {
        let desc = x86.mem.view::<DDSURFACEDESC2>(lpSurfaceDesc);
        assert!(std::mem::size_of::<DDSURFACEDESC2>() == desc.dwSize.get() as usize);

        log::warn!(
            "{this:x}->CreateSurface({:?}, {desc:?}, {lpDirectDrawSurface7:x})",
            desc.flags()
        );

        let mut width = desc.dwWidth.get();
        let mut height = desc.dwHeight.get();
        if let Some(caps) = desc.caps() {
            if caps.caps1().contains(DDSCAPS::PRIMARYSURFACE) {
                width = x86.state.ddraw.as_ref().unwrap().width;
                height = x86.state.ddraw.as_ref().unwrap().height;
            }
        }

        if let Some(count) = desc.back_buffer_count() {
            log::warn!("  back_buffer: {count:x}");
        }
        if let Some(caps) = desc.caps() {
            log::warn!("  caps: {:?}", caps.caps1());
        }
        let x86_surface = IDirectDrawSurface7::new(x86);
        x86.mem.write_u32(lpDirectDrawSurface7, x86_surface);
        x86.state
            .ddraw
            .as_mut()
            .unwrap()
            .surfaces
            .insert(x86_surface, Surface { width, height });
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

    fn SetCooperativeLevel(_x86: &mut X86, _this: u32, _hwnd: u32, _flags: u32) -> u32 {
        // TODO: this triggers behaviors like fullscreen.
        // let flags = DDSCL::from_bits(flags).unwrap();
        // log::warn!("{this:x}->SetCooperativeLevel({hwnd:x}, {flags:?})");
        DD_OK
    }

    fn SetDisplayMode(
        _x86: &mut X86,
        this: u32,
        width: u32,
        height: u32,
        bpp: u32,
        refresh: u32,
        flags: u32,
    ) -> u32 {
        log::warn!("{this:x}->SetDisplayMode({width}x{height}x{bpp}@{refresh}hz, {flags:x})");
        DD_OK
    }

    winapi_shims!(
        fn Release(this: u32);
        fn CreateSurface(this: u32, lpSurfaceDesc: u32, lpDirectDrawSurface7: u32, unused: u32);
        fn SetCooperativeLevel(this: u32, hwnd: u32, flags: u32);
        fn SetDisplayMode(this: u32, width: u32, height: u32, bpp: u32, refresh: u32, flags: u32);
    );
}

mod IDirectDrawSurface7 {
    use crate::memory::DWORD;

    use super::*;

    #[repr(C)]
    pub(super) struct Vtable {
        pub QueryInterface: DWORD,
        pub AddRef: DWORD,
        pub Release: DWORD,
        pub AddAttachedSurface: DWORD,
        pub AddOverlayDirtyRect: DWORD,
        pub Blt: DWORD,
        pub BltBatch: DWORD,
        pub BltFast: DWORD,
        pub DeleteAttachedSurface: DWORD,
        pub EnumAttachedSurfaces: DWORD,
        pub EnumOverlayZOrders: DWORD,
        pub Flip: DWORD,
        pub GetAttachedSurface: DWORD,
        pub GetBltStatus: DWORD,
        pub GetCaps: DWORD,
        pub GetClipper: DWORD,
        pub GetColorKey: DWORD,
        pub GetDC: DWORD,
        pub GetFlipStatus: DWORD,
        pub GetOverlayPosition: DWORD,
        pub GetPalette: DWORD,
        pub GetPixelFormat: DWORD,
        pub GetSurfaceDesc: DWORD,
        pub Initialize: DWORD,
        pub IsLost: DWORD,
        pub Lock: DWORD,
        pub ReleaseDC: DWORD,
        pub Restore: DWORD,
        pub SetClipper: DWORD,
        pub SetColorKey: DWORD,
        pub SetOverlayPosition: DWORD,
        pub SetPalette: DWORD,
        pub Unlock: DWORD,
        pub UpdateOverlay: DWORD,
        pub UpdateOverlayDisplay: DWORD,
        pub UpdateOverlayZOrder: DWORD,
        pub GetDDInterface: DWORD,
        pub PageLock: DWORD,
        pub PageUnlock: DWORD,
        pub SetSurfaceDesc: DWORD,
        pub SetPrivateData: DWORD,
        pub GetPrivateData: DWORD,
        pub FreePrivateData: DWORD,
        pub GetUniquenessValue: DWORD,
        pub ChangeUniquenessValue: DWORD,
        pub SetPriority: DWORD,
        pub GetPriority: DWORD,
        pub SetLOD: DWORD,
        pub GetLOD: DWORD,
    }
    unsafe impl crate::memory::Pod for Vtable {}

    pub fn vtable(ddraw: &mut State, x86: &mut X86) -> u32 {
        let addr = ddraw
            .heap(&mut x86.state.kernel32)
            .alloc(&mut x86.mem, std::mem::size_of::<Vtable>() as u32);
        let vtable = x86.mem.view_mut::<Vtable>(addr);
        *vtable = Vtable {
            QueryInterface: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::QueryInterface unimplemented".into()
                ))
                .into(),
            AddRef: x86
                .shims
                .add(Err("IDirectDrawSurface::AddRef unimplemented".into()))
                .into(),
            Release: x86.shims.add(Ok(shims::Release)).into(),
            AddAttachedSurface: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::AddAttachedSurface unimplemented".into()
                ))
                .into(),
            AddOverlayDirtyRect: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::AddOverlayDirtyRect unimplemented".into()
                ))
                .into(),
            Blt: x86
                .shims
                .add(Err("IDirectDrawSurface::Blt unimplemented".into()))
                .into(),
            BltBatch: x86
                .shims
                .add(Err("IDirectDrawSurface::BltBatch unimplemented".into()))
                .into(),
            BltFast: x86.shims.add(Ok(shims::BltFast)).into(),
            DeleteAttachedSurface: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::DeleteAttachedSurface unimplemented".into(),
                ))
                .into(),
            EnumAttachedSurfaces: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::EnumAttachedSurfaces unimplemented".into(),
                ))
                .into(),
            EnumOverlayZOrders: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::EnumOverlayZOrders unimplemented".into()
                ))
                .into(),
            Flip: x86.shims.add(Ok(shims::Flip)).into(),
            GetAttachedSurface: x86.shims.add(Ok(shims::GetAttachedSurface)).into(),
            GetBltStatus: x86
                .shims
                .add(Err("IDirectDrawSurface::GetBltStatus unimplemented".into()))
                .into(),
            GetCaps: x86
                .shims
                .add(Err("IDirectDrawSurface::GetCaps unimplemented".into()))
                .into(),
            GetClipper: x86
                .shims
                .add(Err("IDirectDrawSurface::GetClipper unimplemented".into()))
                .into(),
            GetColorKey: x86
                .shims
                .add(Err("IDirectDrawSurface::GetColorKey unimplemented".into()))
                .into(),
            GetDC: x86.shims.add(Ok(shims::GetDC)).into(),
            GetFlipStatus: x86
                .shims
                .add(Err("IDirectDrawSurface::GetFlipStatus unimplemented".into()))
                .into(),
            GetOverlayPosition: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::GetOverlayPosition unimplemented".into()
                ))
                .into(),
            GetPalette: x86
                .shims
                .add(Err("IDirectDrawSurface::GetPalette unimplemented".into()))
                .into(),
            GetPixelFormat: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::GetPixelFormat unimplemented".into()
                ))
                .into(),
            GetSurfaceDesc: x86.shims.add(Ok(shims::GetSurfaceDesc)).into(),
            Initialize: x86
                .shims
                .add(Err("IDirectDrawSurface::Initialize unimplemented".into()))
                .into(),
            IsLost: x86
                .shims
                .add(Err("IDirectDrawSurface::IsLost unimplemented".into()))
                .into(),
            Lock: x86
                .shims
                .add(Err("IDirectDrawSurface::Lock unimplemented".into()))
                .into(),
            ReleaseDC: x86
                .shims
                .add(Err("IDirectDrawSurface::ReleaseDC unimplemented".into()))
                .into(),
            Restore: x86.shims.add(Ok(shims::Restore)).into(),
            SetClipper: x86
                .shims
                .add(Err("IDirectDrawSurface::SetClipper unimplemented".into()))
                .into(),
            SetColorKey: x86
                .shims
                .add(Err("IDirectDrawSurface::SetColorKey unimplemented".into()))
                .into(),
            SetOverlayPosition: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::SetOverlayPosition unimplemented".into()
                ))
                .into(),
            SetPalette: x86
                .shims
                .add(Err("IDirectDrawSurface::SetPalette unimplemented".into()))
                .into(),
            Unlock: x86
                .shims
                .add(Err("IDirectDrawSurface::Unlock unimplemented".into()))
                .into(),
            UpdateOverlay: x86
                .shims
                .add(Err("IDirectDrawSurface::UpdateOverlay unimplemented".into()))
                .into(),
            UpdateOverlayDisplay: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::UpdateOverlayDisplay unimplemented".into(),
                ))
                .into(),
            UpdateOverlayZOrder: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::UpdateOverlayZOrder unimplemented".into()
                ))
                .into(),
            GetDDInterface: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::GetDDInterface unimplemented".into()
                ))
                .into(),
            PageLock: x86
                .shims
                .add(Err("IDirectDrawSurface::PageLock unimplemented".into()))
                .into(),
            PageUnlock: x86
                .shims
                .add(Err("IDirectDrawSurface::PageUnlock unimplemented".into()))
                .into(),
            SetSurfaceDesc: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::SetSurfaceDesc unimplemented".into()
                ))
                .into(),
            SetPrivateData: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::SetPrivateData unimplemented".into()
                ))
                .into(),
            GetPrivateData: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::GetPrivateData unimplemented".into()
                ))
                .into(),
            FreePrivateData: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::FreePrivateData unimplemented".into()
                ))
                .into(),
            GetUniquenessValue: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::GetUniquenessValue unimplemented".into()
                ))
                .into(),
            ChangeUniquenessValue: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::ChangeUniquenessValue unimplemented".into(),
                ))
                .into(),
            SetPriority: x86
                .shims
                .add(Err("IDirectDrawSurface::SetPriority unimplemented".into()))
                .into(),
            GetPriority: x86
                .shims
                .add(Err("IDirectDrawSurface::GetPriority unimplemented".into()))
                .into(),
            SetLOD: x86
                .shims
                .add(Err("IDirectDrawSurface::SetLOD unimplemented".into()))
                .into(),
            GetLOD: x86
                .shims
                .add(Err("IDirectDrawSurface::GetLOD unimplemented".into()))
                .into(),
        };
        addr
    }

    pub fn new(x86: &mut X86) -> u32 {
        let ddraw = x86.state.ddraw.as_mut().unwrap();
        let lpDirectDrawSurface7 = ddraw.heap(&mut x86.state.kernel32).alloc(&mut x86.mem, 4);
        let vtable = ddraw.vtable_IDirectDrawSurface7;
        x86.mem.write_u32(lpDirectDrawSurface7, vtable);
        lpDirectDrawSurface7
    }

    fn Release(_x86: &mut X86, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    fn BltFast(
        x86: &mut X86,
        this: u32,
        x: u32,
        y: u32,
        lpSurf: u32,
        lpRect: u32,
        flags: u32,
    ) -> u32 {
        let rect = x86.mem.view::<RECT>(lpRect);
        log::warn!("{this:x}->BltFast({x:x}, {y:x}, {lpSurf:x}, {rect:?}, {flags:x})");
        DDERR_GENERIC
    }

    fn Flip(_x86: &mut X86, this: u32, lpSurf: u32, flags: u32) -> u32 {
        log::warn!("{this:x}->Flip({lpSurf:x}, {flags:x})");
        DDERR_GENERIC
    }

    fn GetAttachedSurface(
        x86: &mut X86,
        this: u32,
        lpDDSCaps2: u32,
        lpDirectDrawSurface7: u32,
    ) -> u32 {
        log::warn!("{this:x}->GetAttachedSurface({lpDDSCaps2:x}, {lpDirectDrawSurface7:x})");
        let surf = new(x86);
        x86.mem.write_u32(lpDirectDrawSurface7, surf);
        DD_OK
    }

    fn GetDC(_x86: &mut X86, this: u32, lpHDC: u32) -> u32 {
        log::warn!("unimp: {this:x}->GetDC({lpHDC:x})");
        DDERR_GENERIC
    }

    fn GetSurfaceDesc(x86: &mut X86, this: u32, lpDesc: u32) -> u32 {
        let surf = x86
            .state
            .ddraw
            .as_ref()
            .unwrap()
            .surfaces
            .get(&this)
            .unwrap();
        let desc = x86.mem.view_mut::<DDSURFACEDESC2>(lpDesc);
        assert!(desc.dwSize.get() as usize == std::mem::size_of::<DDSURFACEDESC2>());
        let mut flags = desc.flags();
        if flags.contains(DDSD::WIDTH) {
            desc.dwWidth.set(surf.width);
            flags.remove(DDSD::WIDTH);
        }
        if flags.contains(DDSD::HEIGHT) {
            desc.dwHeight.set(surf.height);
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

    fn Restore(_x86: &mut X86, _this: u32) -> u32 {
        DD_OK
    }

    winapi_shims!(
        fn Release(this: u32);
        fn BltFast(this: u32, x: u32, y: u32, lpSurf: u32, lpRect: u32, flags: u32);
        fn Flip(this: u32, lpSurf: u32, flags: u32);
        fn GetAttachedSurface(this: u32, lpDDSCaps2: u32, lpDirectDrawSurface7: u32);
        fn GetDC(this: u32, lpHDC: u32);
        fn GetSurfaceDesc(this: u32, lpDesc: u32);
        fn Restore(this: u32);
    );
}

fn DirectDrawCreateEx(x86: &mut X86, lpGuid: u32, lplpDD: u32, iid: u32, pUnkOuter: u32) -> u32 {
    assert!(lpGuid == 0);
    assert!(pUnkOuter == 0);

    let ddraw = match &mut x86.state.ddraw {
        Some(ddraw) => ddraw,
        None => {
            let ddraw = State::new(x86);
            x86.state.ddraw = Some(ddraw);
            x86.state.ddraw.as_mut().unwrap()
        }
    };

    let iid_slice = &x86.mem[iid as usize..(iid + 16) as usize];
    if iid_slice == IID_IDirectDraw7 {
        // Caller gives us:
        //   pointer (lplpDD) that they want us to fill in to point to ->
        //   [vtable, ...] (lpDirectDraw7), where vtable is pointer to ->
        //   [fn1, fn2, ...] (vtable_IDirectDraw7)
        let lpDirectDraw7 = ddraw.heap(&mut x86.state.kernel32).alloc(&mut x86.mem, 8);
        let vtable = ddraw.vtable_IDirectDraw7;
        x86.write_u32(lpDirectDraw7, vtable);
        x86.write_u32(lplpDD, lpDirectDraw7);
        DD_OK
    } else {
        log::error!("DirectDrawCreateEx: unknown IID {iid_slice:x?}");
        DDERR_GENERIC
    }
}

winapi!(
    fn DirectDrawCreateEx(lpGuid: u32, lplpDD: u32, iid: u32, pUnkOuter: u32);
);
