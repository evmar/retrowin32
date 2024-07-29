use super::{HINSTANCE, HMENU};
use crate::{
    pe,
    winapi::{
        bitmap::BitmapRGBA32,
        gdi32::{self, HGDIOBJ},
        kernel32::ResourceKey,
        types::*,
    },
    Machine,
};
use memory::{Extensions, Mem};

const TRACE_CONTEXT: &'static str = "user32/resource";

// TODO: switch to the HANDLE<T> type?
pub type HCURSOR = u32;
pub type HICON = u32;
pub type HBRUSH = HGDIOBJ;

#[win32_derive::dllexport]
pub fn LoadIconA(_machine: &mut Machine, hInstance: u32, lpIconName: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn LoadIconW(_machine: &mut Machine, hInstance: u32, lpIconName: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn LoadCursorA(_machine: &mut Machine, hInstance: u32, lpCursorName: u32) -> HCURSOR {
    0
}

#[win32_derive::dllexport]
pub fn LoadCursorW(_machine: &mut Machine, hInstance: u32, lpCursorName: u32) -> HCURSOR {
    0
}

#[win32_derive::dllexport]
pub fn CreateCursor(
    _machine: &mut Machine,
    hInst: u32,
    xHotSpot: u32,
    yHotSpot: u32,
    nWidth: u32,
    nHeight: u32,
    pvANDPlane: u32,
    pvXORPlane: u32,
) -> HCURSOR {
    0
}

#[win32_derive::dllexport]
pub fn ShowCursor(_machine: &mut Machine, bShow: bool) -> u32 {
    // TODO: increment/decrement refcount
    1 // ref=1
}

#[win32_derive::dllexport]
pub fn SetCursor(_machine: &mut Machine, hCursor: u32) -> u32 {
    0 // previous: null
}

fn load_bitmap(
    machine: &mut Machine,
    hInstance: HINSTANCE,
    name: ResourceKey<&Str16>,
) -> Option<HGDIOBJ> {
    let buf = crate::winapi::kernel32::find_resource(
        &machine.state.kernel32,
        machine.mem(),
        hInstance,
        ResourceKey::Id(pe::RT::BITMAP as u32),
        name,
    )?;
    let bmp = BitmapRGBA32::parse(Mem::from_slice(buf));
    Some(
        machine
            .state
            .gdi32
            .objects
            .add(gdi32::Object::Bitmap(gdi32::BitmapType::RGBA32(bmp))),
    )
}

#[win32_derive::dllexport]
pub fn LoadImageA(
    machine: &mut Machine,
    hInstance: u32,
    name: ResourceKey<&str>,
    typ: u32,
    cx: u32,
    cy: u32,
    fuLoad: u32,
) -> HGDIOBJ {
    if fuLoad != 0 {
        log::error!("unimplemented fuLoad {:x}", fuLoad);
        return HGDIOBJ::null();
    }

    let name = name.to_string16();

    // TODO: it's unclear whether the width/height is obeyed when loading an image.

    const IMAGE_BITMAP: u32 = 0;
    match typ {
        IMAGE_BITMAP => load_bitmap(machine, hInstance, name.as_ref()).unwrap(),
        _ => {
            log::error!("unimplemented image type {:x}", typ);
            return HGDIOBJ::null();
        }
    }
}

#[win32_derive::dllexport]
pub fn LoadImageW(
    machine: &mut Machine,
    hInstance: u32,
    name: ResourceKey<&Str16>,
    typ: u32,
    cx: u32,
    cy: u32,
    fuLoad: u32,
) -> HGDIOBJ {
    if fuLoad != 0 {
        log::error!("unimplemented fuLoad {:x}", fuLoad);
        return HGDIOBJ::null();
    }

    // TODO: it's unclear whether the width/height is obeyed when loading an image.

    const IMAGE_BITMAP: u32 = 0;
    const IMAGE_ICON: u32 = 1;
    match typ {
        IMAGE_BITMAP => load_bitmap(machine, hInstance, name).unwrap(),
        IMAGE_ICON => {
            return HGDIOBJ::null();
        }
        _ => {
            log::error!("unimplemented image type {:x}", typ);
            return HGDIOBJ::null();
        }
    }
}

#[win32_derive::dllexport]
pub fn LoadBitmapA(
    machine: &mut Machine,
    hInstance: HINSTANCE,
    lpBitmapName: ResourceKey<&str>,
) -> HGDIOBJ {
    let name = lpBitmapName.to_string16();
    load_bitmap(machine, hInstance, name.as_ref()).unwrap()
}

fn find_string(machine: &Machine, hInstance: HINSTANCE, uID: u32) -> Option<&[u8]> {
    // Strings are stored as blocks of 16 consecutive strings.
    let (resource_id, index) = ((uID >> 4) + 1, uID & 0xF);

    let block = crate::winapi::kernel32::find_resource(
        &machine.state.kernel32,
        machine.mem(),
        hInstance,
        ResourceKey::Id(pe::RT::STRING as u32),
        ResourceKey::Id(resource_id),
    )?;

    // Each block is a sequence of two byte length-prefixed strings.
    // Iterate through them to find the requested index.
    let mut ofs = 0;
    for _ in 0..index {
        let len = block.get_pod::<u16>(ofs) as u32;
        ofs += (1 + len) * 2;
    }
    let len = block.get_pod::<u16>(ofs) as u32;
    let str = block.sub32(ofs + 2, len * 2);
    Some(str)
}

#[win32_derive::dllexport]
pub fn LoadStringA(
    machine: &mut Machine,
    hInstance: u32,
    uID: u32,
    lpBuffer: u32,
    cchBufferMax: u32,
) -> u32 {
    let str = match find_string(machine, hInstance, uID) {
        Some(str) => Str16::from_bytes(str),
        None => return 0,
    };
    assert!(cchBufferMax != 0); // MSDN claims this is invalid

    let dst = machine
        .mem()
        .sub(lpBuffer, cchBufferMax)
        .as_mut_slice_todo();
    let copy_len = std::cmp::min(dst.len() as usize - 1, str.len());
    for i in 0..copy_len {
        dst[i] = str.buf()[i] as u8;
    }
    dst[copy_len] = 0;
    copy_len as u32
}

#[win32_derive::dllexport]
pub fn LoadStringW(
    machine: &mut Machine,
    hInstance: u32,
    uID: u32,
    lpBuffer: u32,
    cchBufferMax: u32,
) -> u32 {
    let str = match find_string(machine, hInstance, uID) {
        Some(str) => str,
        None => return 0,
    };
    if cchBufferMax == 0 {
        machine
            .mem()
            .put::<u32>(lpBuffer, machine.mem().offset_of(str.as_ptr()));
        str.len() as u32
    } else {
        let dst = machine.mem().sub(lpBuffer, cchBufferMax * 2);
        let copy_len = std::cmp::min(dst.len() as usize - 2, str.len());
        dst.as_mut_slice_todo()[..copy_len].copy_from_slice(&str[..copy_len]);
        dst.put::<u16>(copy_len as u32, 0);
        copy_len as u32
    }
}

#[win32_derive::dllexport]
pub fn LoadMenuW(_machine: &mut Machine, hInstance: u32, lpMenuName: u32) -> HMENU {
    0
}

#[win32_derive::dllexport]
pub fn LoadAcceleratorsW(_machine: &mut Machine, hInstance: u32, lpTableName: u32) -> HMENU {
    0
}
