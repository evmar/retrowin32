#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::collections::HashMap;

use crate::{winapi, X86};

use super::{kernel32, DWORD};

pub struct State {
    hheap: u32,
    vtable_IDirectDraw7: u32,
}
impl State {
    pub fn new() -> Self {
        State {
            hheap: 0,
            vtable_IDirectDraw7: 0,
        }
    }

    fn init(
        &mut self,
        imports: &mut HashMap<u32, Option<fn(&mut X86)>>,
        kernel32: &mut kernel32::State,
        mem: &mut Vec<u8>,
    ) {
        if self.hheap != 0 {
            return;
        }
        self.hheap = kernel32.new_heap(mem, 0x1000, "ddraw.dll heap".into());

        self.vtable_IDirectDraw7 = self
            .heap(kernel32)
            .alloc(mem, std::mem::size_of::<IDirectDraw7_Vtable>() as u32);
        let buf = &mut mem[self.vtable_IDirectDraw7 as usize
            ..self.vtable_IDirectDraw7 as usize + std::mem::size_of::<IDirectDraw7_Vtable>()];
        let vtable: &mut IDirectDraw7_Vtable = unsafe {
            (buf.as_mut_ptr() as *mut IDirectDraw7_Vtable)
                .as_mut()
                .unwrap()
        };

        let id = 0xf1a7_dd00u32;
        vtable.SetCooperativeLevel.set(id);
        imports.insert(id, Some(IDirectDraw7::shims::SetCooperativeLevel));
    }

    fn heap<'a>(&mut self, kernel32: &'a mut kernel32::State) -> &'a mut kernel32::Heap {
        kernel32.heaps.get_mut(&self.hheap).unwrap()
    }
}

const DD_OK: u32 = 0;

const IID_IDirectDraw7: [u8; 16] = [
    0xc0, 0x5e, 0xe6, 0x15, 0x9c, 0x3b, 0xd2, 0x11, 0xb9, 0x2f, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b,
];

#[repr(C)]
struct IDirectDraw7_Vtable {
    QueryInterface: DWORD,
    AddRef: DWORD,
    Release: DWORD,
    Compact: DWORD,
    CreateClipper: DWORD,
    CreatePalette: DWORD,
    CreateSurface: DWORD,
    DuplicateSurface: DWORD,
    EnumDisplayModes: DWORD,
    EnumSurfaces: DWORD,
    FlipToGDISurface: DWORD,
    GetCaps: DWORD,
    GetDisplayMode: DWORD,
    GetFourCCCodes: DWORD,
    GetGDISurface: DWORD,
    GetMonitorFrequency: DWORD,
    GetScanLine: DWORD,
    GetVerticalBlankStatus: DWORD,
    Initialize: DWORD,
    RestoreDisplayMode: DWORD,
    SetCooperativeLevel: DWORD,
    SetDisplayMode: DWORD,
    WaitForVerticalBlank: DWORD,
    GetAvailableVidMem: DWORD,
    GetSurfaceFromDC: DWORD,
    RestoreAllSurfaces: DWORD,
    TestCooperativeLevel: DWORD,
    GetDeviceIdentifier: DWORD,
    StartModeTest: DWORD,
    EvaluateMode: DWORD,
}

mod IDirectDraw7 {
    use super::*;

    fn SetCooperativeLevel(_x86: &mut X86, hwnd: u32, flags: u32) -> u32 {
        log::warn!("SetCooperativeLevel({hwnd:x}, {flags:x})");
        DD_OK
    }

    pub mod shims {
        use super::X86;
        use crate::winapi_shim;

        winapi_shim!(fn SetCooperativeLevel(hwnd: u32, flags: u32));
    }
}

fn DirectDrawCreateEx(x86: &mut X86, lpGuid: u32, lplpDD: u32, iid: u32, pUnkOuter: u32) -> u32 {
    assert!(lpGuid == 0);
    assert!(pUnkOuter == 0);

    let ddraw = &mut x86.state.ddraw;
    ddraw.init(&mut x86.imports, &mut x86.state.kernel32, &mut x86.mem);

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
        1 // fail
    }
}

winapi!(
    fn DirectDrawCreateEx(lpGuid: u32, lplpDD: u32, iid: u32, pUnkOuter: u32);
);
