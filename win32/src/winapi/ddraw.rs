#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::{memory::Memory, winapi, x86::write_u32, X86};

use super::kernel32;

pub struct State {
    hheap: u32,
    vtable_IDirectDraw7: u32,
    vtable_IDirectDrawSurface7: u32,
}
impl State {
    pub fn new(x86: &mut X86) -> Self {
        let mut ddraw = State {
            hheap: 0,
            vtable_IDirectDraw7: 0,
            vtable_IDirectDrawSurface7: 0,
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
        log::warn!("{this:x}->CreateSurface({lpSurfaceDesc:x}, {lpDirectDrawSurface7:x})");
        let surface = IDirectDrawSurface7::new(x86);
        write_u32(&mut x86.mem, lpDirectDrawSurface7, surface);
        DD_OK
    }

    fn SetCooperativeLevel(_x86: &mut X86, this: u32, hwnd: u32, flags: u32) -> u32 {
        log::warn!("{this:x}->SetCooperativeLevel({hwnd:x}, {flags:x})");
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
        log::warn!(
            "{this:x}->SetDisplayMode({width:x}, {height:x}, {bpp:x}, {refresh:x}, {flags:x})"
        );
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
            BltFast: x86
                .shims
                .add(Err("IDirectDrawSurface::BltFast unimplemented".into()))
                .into(),
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
            Flip: x86
                .shims
                .add(Err("IDirectDrawSurface::Flip unimplemented".into()))
                .into(),
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
            GetDC: x86
                .shims
                .add(Err("IDirectDrawSurface::GetDC unimplemented".into()))
                .into(),
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
            GetSurfaceDesc: x86
                .shims
                .add(Err(
                    "IDirectDrawSurface::GetSurfaceDesc unimplemented".into()
                ))
                .into(),
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
        x86.write_u32(lpDirectDrawSurface7, vtable);
        lpDirectDrawSurface7
    }

    fn Release(_x86: &mut X86, this: u32) -> u32 {
        log::warn!("{this:x}->Release()");
        0 // TODO: return refcount?
    }

    fn GetAttachedSurface(
        x86: &mut X86,
        this: u32,
        lpDDSCaps2: u32,
        lpDirectDrawSurface7: u32,
    ) -> u32 {
        log::warn!("{this:x}->GetAttachedSurface({lpDDSCaps2:x}, {lpDirectDrawSurface7:x})");
        let surf = new(x86);
        write_u32(&mut x86.mem, lpDirectDrawSurface7, surf);
        DD_OK
    }

    fn Restore(_x86: &mut X86, _this: u32) -> u32 {
        DD_OK
    }

    winapi_shims!(
        fn Release(this: u32);
        fn GetAttachedSurface(this: u32, lpDDSCaps2: u32, lpDirectDrawSurface7: u32);
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
