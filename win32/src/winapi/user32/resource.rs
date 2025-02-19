use super::{HINSTANCE, HMENU};
use crate::{
    winapi::{
        bitmap::{Bitmap, BITMAPFILEHEADER},
        gdi32::HGDIOBJ,
        kernel32::ResourceKey,
        types::*,
    },
    FileOptions, Machine,
};
use bitflags::bitflags;
use memory::{Extensions, ExtensionsMut};
use std::{borrow::Cow, ops::Range};
use typed_path::WindowsPath;

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

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum IMAGE {
    BITMAP = 0,
    ICON = 1,
    CURSOR = 2,
}

bitflags! {
    #[derive(win32_derive::TryFromBitflags)]
    pub struct LR: u32 {
        // const DEFAULTCOLOR = 0x00000000;
        const MONOCHROME       = 0x00000001;
        const LOADFROMFILE     = 0x00000010;
        const LOADTRANSPARENT  = 0x00000020;
        const VGACOLOR         = 0x00000080;
        const DEFAULTSIZE      = 0x00000040;
        const LOADMAP3DCOLORS  = 0x00001000;
        const CREATEDIBSECTION = 0x00002000;
        const SHARED = 0x00008000;
    }
}

fn load_image(
    machine: &mut Machine,
    hInstance: u32,
    name: ResourceKey<&Str16>,
    typ: IMAGE,
    cx: u32,
    cy: u32,
    fuLoad: LR,
) -> HGDIOBJ {
    if cx != 0 || cy != 0 {
        // TODO: it's unclear whether the width/height is obeyed when loading an image.
        log::warn!("LoadImage: ignoring cx/cy");
    }

    let mut flags = fuLoad;
    flags.remove(LR::CREATEDIBSECTION); // TODO: we always assume DIBs
    let load_from_file = flags.contains(LR::LOADFROMFILE);
    flags.remove(LR::LOADFROMFILE);
    if !flags.is_empty() {
        log::error!("LoadImage: unimplemented fuLoad {:?}", fuLoad);
        return HGDIOBJ::null();
    }

    let buf = if load_from_file {
        let ResourceKey::Name(name) = name else {
            panic!();
        };
        let path = name.to_string();
        let mut file = machine
            .host
            .open(WindowsPath::new(&path), FileOptions::read())
            .unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        Cow::Owned(buf)
    } else {
        let typ = ResourceKey::Id(match typ {
            IMAGE::CURSOR => pe::RT::CURSOR,
            IMAGE::BITMAP => pe::RT::BITMAP,
            IMAGE::ICON => pe::RT::ICON,
        } as u32);
        let Some(slice) = crate::winapi::kernel32::find_resource(
            &machine.state.kernel32,
            machine.mem(),
            hInstance,
            typ,
            &name,
        ) else {
            return HGDIOBJ::null();
        };
        Cow::Borrowed(machine.mem().slice(slice))
    };
    let buf = &*buf;

    match typ {
        IMAGE::BITMAP => {
            let buf = if load_from_file {
                let file_header = buf.get_pod::<BITMAPFILEHEADER>(0);
                assert!(file_header.bfType == 0x4D42); // "BM"

                // Rest of the header doesn't seem useful (?).
                &buf[14..]
            } else {
                buf
            };
            let bmp = Bitmap::parse(buf, None);
            machine.state.gdi32.objects.add_bitmap(bmp)
        }
        typ => {
            log::error!("LoadImage: unimplemented image type {:?}", typ);
            return HGDIOBJ::null();
        }
    }
}

#[win32_derive::dllexport]
pub fn LoadImageA(
    machine: &mut Machine,
    hInstance: u32,
    name: ResourceKey<&str>,
    typ: Result<IMAGE, u32>,
    cx: u32,
    cy: u32,
    fuLoad: Result<LR, u32>,
) -> HGDIOBJ {
    load_image(
        machine,
        hInstance,
        name.to_string16().as_ref(),
        typ.unwrap(),
        cx,
        cy,
        fuLoad.unwrap(),
    )
}

#[win32_derive::dllexport]
pub fn LoadImageW(
    machine: &mut Machine,
    hInstance: u32,
    name: ResourceKey<&Str16>,
    typ: Result<IMAGE, u32>,
    cx: u32,
    cy: u32,
    fuLoad: Result<LR, u32>,
) -> HGDIOBJ {
    load_image(
        machine,
        hInstance,
        name,
        typ.unwrap(),
        cx,
        cy,
        fuLoad.unwrap(),
    )
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
        &name,
    )?;
    let buf = machine.mem().slice(buf);
    let bmp = Bitmap::parse(buf, None);
    Some(machine.state.gdi32.objects.add_bitmap(bmp))
}

#[win32_derive::dllexport]
pub fn LoadBitmapA(
    machine: &mut Machine,
    hInstance: HINSTANCE,
    lpBitmapName: ResourceKey<&str>,
) -> HGDIOBJ {
    load_bitmap(machine, hInstance, lpBitmapName.to_string16().as_ref()).unwrap()
}

fn find_string(machine: &Machine, hInstance: HINSTANCE, uID: u32) -> Option<Range<u32>> {
    // Strings are stored as blocks of 16 consecutive strings.
    let (resource_id, index) = ((uID >> 4) + 1, uID & 0xF);

    let block = crate::winapi::kernel32::find_resource(
        &machine.state.kernel32,
        machine.mem(),
        hInstance,
        ResourceKey::Id(pe::RT::STRING as u32),
        &ResourceKey::Id(resource_id),
    )?;
    let block_ofs = block.start;
    let block = machine.mem().slice(block);

    // Each block is a sequence of two byte length-prefixed strings.
    // Iterate through them to find the requested index.
    let mut ofs = 0;
    for _ in 0..index {
        let len = block.get_pod::<u16>(ofs) as u32;
        ofs += (1 + len) * 2;
    }
    let len = block.get_pod::<u16>(ofs) as u32;
    let start = block_ofs + ofs + 2;
    Some(Range {
        start,
        end: start + len * 2,
    })
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
        Some(str) => Str16::from_bytes(machine.mem().slice(str)),
        None => return 0,
    };
    assert!(cchBufferMax != 0); // MSDN claims this is invalid

    let dst = machine.mem().sub32_mut(lpBuffer, cchBufferMax);
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
    let Some(str) = find_string(machine, hInstance, uID) else {
        return 0;
    };
    let mem = machine.mem();
    if cchBufferMax == 0 {
        mem.put_pod::<u32>(lpBuffer, str.start);
        str.len() as u32
    } else {
        let dst = machine.mem().sub32_mut(lpBuffer, cchBufferMax * 2);
        let copy_len = std::cmp::min(dst.len() - 2, str.len()) as u32;
        dst[..copy_len as usize].copy_from_slice(mem.sub32(str.start, copy_len));
        dst.put_pod::<u16>(copy_len as u32, 0);
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
