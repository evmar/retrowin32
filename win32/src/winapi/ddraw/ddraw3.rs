use crate::{Machine, System};
use memory::ExtensionsMut;
use win32_winapi::{com::GUID, vtable};

pub const IID_IDirectDraw3: GUID = GUID((
    0xda044e00,
    0x69b2,
    0x11d0,
    [0xa1, 0xd5, 0x00, 0xaa, 0x00, 0xb8, 0xdf, 0xbb],
));

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
        let lpDirectDrawSurface = machine.memory.process_heap.alloc(machine.memory.mem(), 4);
        let vtable = crate::loader::get_symbol(machine, "ddraw.dll", "IDirectDrawSurface3");
        machine.mem().put_pod::<u32>(lpDirectDrawSurface, vtable);
        lpDirectDrawSurface
    }

    #[win32_derive::dllexport]
    pub fn Release(sys: &dyn System, this: u32) -> u32 {
        0 // TODO: return refcount?
    }
}
