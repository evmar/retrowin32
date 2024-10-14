use crate::{
    winapi::{
        com::{vtable, GUID},
        kernel32::get_symbol,
    },
    Machine,
};
use memory::ExtensionsMut;

pub const IID_IDirectDraw3: GUID = GUID {
    Data1: 0xda044e00,
    Data2: 0x69b2,
    Data3: 0x11d0,
    Data4: [0xa1, 0xd5, 0x00, 0xaa, 0x00, 0xb8, 0xdf, 0xbb],
};

#[win32_derive::dllexport]
pub mod IDirectDrawSurface3 {
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
        GetAttachedSurface: todo,
        GetBltStatus: todo,
        GetCaps: todo,
        GetClipper: todo,
        GetColorKey: todo,
        GetDC: (IDirectDrawSurface7::GetDC),
        GetFlipStatus: todo,
        GetOverlayPosition: todo,
        GetPalette: todo,
        GetPixelFormat: (IDirectDrawSurface7::GetPixelFormat),
        GetSurfaceDesc: todo,
        Initialize: todo,
        IsLost: todo,
        Lock: todo,
        ReleaseDC: (IDirectDrawSurface7::ReleaseDC),
        Restore: todo,
        SetClipper: (IDirectDrawSurface7::SetClipper),
        SetColorKey: todo,
        SetOverlayPosition: todo,
        SetPalette: (IDirectDrawSurface7::SetPalette),
        Unlock: todo,
        UpdateOverlay: todo,
        UpdateOverlayDisplay: todo,
        UpdateOverlayZOrder: todo,

        GetDDInterface: todo,
        PageLock: todo,
        PageUnlock: todo,

        SetSurfaceDesc: todo,
    ];

    pub fn new(machine: &mut Machine) -> u32 {
        let ddraw = &mut machine.state.ddraw;
        let lpDirectDrawSurface = ddraw.heap.alloc(machine.emu.memory.mem(), 4);
        let vtable = get_symbol(machine, "ddraw.dll", "IDirectDrawSurface3");
        machine.mem().put_pod::<u32>(lpDirectDrawSurface, vtable);
        lpDirectDrawSurface
    }

    #[win32_derive::dllexport]
    pub fn Release(_machine: &mut Machine, this: u32) -> u32 {
        0 // TODO: return refcount?
    }
}
