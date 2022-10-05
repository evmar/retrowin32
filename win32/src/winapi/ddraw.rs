#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::{
    winapi,
    x86::{write_u32, Shims},
    X86,
};

use super::{kernel32, DWORD};

pub struct State {
    hheap: u32,
    vtable_IDirectDraw7: u32,
    vtable_IDirectDrawSurface7: u32,
}
impl State {
    pub fn new() -> Self {
        State {
            hheap: 0,
            vtable_IDirectDraw7: 0,
            vtable_IDirectDrawSurface7: 0,
        }
    }

    fn heap<'a>(&mut self, kernel32: &'a mut kernel32::State) -> &'a mut kernel32::Heap {
        kernel32.heaps.get_mut(&self.hheap).unwrap()
    }

    fn init(&mut self, shims: &mut Shims, kernel32: &mut kernel32::State, mem: &mut Vec<u8>) {
        if self.hheap != 0 {
            return;
        }
        self.hheap = kernel32.new_heap(mem, 0x1000, "ddraw.dll heap".into());

        self.init_IDirectDraw7(shims, kernel32, mem);
        self.init_IDirectDrawSurface7(shims, kernel32, mem);
    }

    fn init_IDirectDraw7(
        &mut self,
        shims: &mut Shims,
        kernel32: &mut kernel32::State,
        mem: &mut Vec<u8>,
    ) {
        self.vtable_IDirectDraw7 = self
            .heap(kernel32)
            .alloc(mem, std::mem::size_of::<IDirectDraw7::Vtable>() as u32);
        let buf = &mut mem[self.vtable_IDirectDraw7 as usize
            ..self.vtable_IDirectDraw7 as usize + std::mem::size_of::<IDirectDraw7::Vtable>()];

        // Fill vtable with "unimplemented" callback.
        for i in 0..(buf.len() / 4) {
            let id = shims.add(Err(format!("IDirectDraw method {:x} unimplemented", i)));
            write_u32(buf, (i * 4) as u32, id);
        }

        let vtable: &mut IDirectDraw7::Vtable = unsafe {
            (buf.as_mut_ptr() as *mut IDirectDraw7::Vtable)
                .as_mut()
                .unwrap()
        };

        vtable
            .Release
            .set(shims.add(Ok(IDirectDraw7::shims::Release)));
        vtable
            .CreateSurface
            .set(shims.add(Ok(IDirectDraw7::shims::CreateSurface)));
        vtable
            .SetCooperativeLevel
            .set(shims.add(Ok(IDirectDraw7::shims::SetCooperativeLevel)));
        vtable
            .SetDisplayMode
            .set(shims.add(Ok(IDirectDraw7::shims::SetDisplayMode)));
    }

    fn init_IDirectDrawSurface7(
        &mut self,
        shims: &mut Shims,
        kernel32: &mut kernel32::State,
        mem: &mut Vec<u8>,
    ) {
        self.vtable_IDirectDrawSurface7 = self.heap(kernel32).alloc(
            mem,
            std::mem::size_of::<IDirectDrawSurface7::Vtable>() as u32,
        );
        let buf = &mut mem[self.vtable_IDirectDrawSurface7 as usize
            ..self.vtable_IDirectDrawSurface7 as usize
                + std::mem::size_of::<IDirectDrawSurface7::Vtable>()];

        // Fill vtable with "unimplemented" callback.
        for i in 0..(buf.len() / 4) {
            let id = shims.add(Err(format!(
                "IDirectDrawSurface method {:x} unimplemented",
                i
            )));
            write_u32(buf, (i * 4) as u32, id);
        }

        let vtable: &mut IDirectDrawSurface7::Vtable = unsafe {
            (buf.as_mut_ptr() as *mut IDirectDrawSurface7::Vtable)
                .as_mut()
                .unwrap()
        };

        vtable
            .Release
            .set(shims.add(Ok(IDirectDrawSurface7::shims::Release)));
        vtable
            .GetAttachedSurface
            .set(shims.add(Ok(IDirectDrawSurface7::shims::GetAttachedSurface)));
        vtable
            .Restore
            .set(shims.add(Ok(IDirectDrawSurface7::shims::Restore)));
    }
}

const DD_OK: u32 = 0;
// DD error codes are generated with this MAKE_HRESULT macro, maybe it doesn't matter too much.
const DDERR_GENERIC: u32 = 0x80004005;

const IID_IDirectDraw7: [u8; 16] = [
    0xc0, 0x5e, 0xe6, 0x15, 0x9c, 0x3b, 0xd2, 0x11, 0xb9, 0x2f, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b,
];

mod IDirectDraw7 {
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

    pub fn new(x86: &mut X86) -> u32 {
        let ddraw = &mut x86.state.ddraw;
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

    let ddraw = &mut x86.state.ddraw;
    ddraw.init(&mut x86.shims, &mut x86.state.kernel32, &mut x86.mem);

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
