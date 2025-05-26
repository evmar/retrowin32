use super::{HINSTANCE, HMENU};
use bitflags::bitflags;
use builtin_gdi32 as gdi32;
use gdi32::{
    bitmap::{BITMAPFILEHEADER, Bitmap},
    {GDIHandles, HGDIOBJ},
};
use memory::{Extensions, ExtensionsMut};
use std::{borrow::Cow, ops::Range};
use win32_system::{
    System, host,
    resource::{ResourceKey, find_resource},
};
use win32_winapi::{
    Str16, WindowsPath,
    encoding::{Encoder, EncoderAnsi},
};

// TODO: switch to the HANDLE<T> type?
pub type HCURSOR = u32;
pub type HICON = u32;
pub type HBRUSH = HGDIOBJ;

#[win32_derive::dllexport]
pub fn LoadIconA(sys: &dyn System, hInstance: u32, lpIconName: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn LoadIconW(sys: &dyn System, hInstance: u32, lpIconName: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn LoadCursorA(sys: &dyn System, hInstance: u32, lpCursorName: u32) -> HCURSOR {
    0
}

#[win32_derive::dllexport]
pub fn LoadCursorW(sys: &dyn System, hInstance: u32, lpCursorName: u32) -> HCURSOR {
    0
}

#[win32_derive::dllexport]
pub fn CreateCursor(
    sys: &dyn System,
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
pub fn ShowCursor(sys: &dyn System, bShow: bool) -> u32 {
    // TODO: increment/decrement refcount
    1 // ref=1
}

#[win32_derive::dllexport]
pub fn SetCursor(sys: &dyn System, hCursor: u32) -> u32 {
    0 // previous: null
}

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum IMAGE {
    BITMAP = 0,
    ICON = 1,
    CURSOR = 2,
}

bitflags! {
    #[derive(Copy, Clone, Debug, win32_derive::TryFromBitflags)]
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
    sys: &mut dyn System,
    hInstance: HINSTANCE,
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
        let mut file = sys
            .host()
            .open(WindowsPath::new(&path), host::FileOptions::read())
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
        let Some(slice) = find_resource(sys, hInstance, typ, &name) else {
            return HGDIOBJ::null();
        };
        Cow::Borrowed(sys.mem().slice(slice))
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
            gdi32::get_state(sys).objects.add_bitmap(bmp)
        }
        typ => {
            log::error!("LoadImage: unimplemented image type {:?}", typ);
            return HGDIOBJ::null();
        }
    }
}

#[win32_derive::dllexport]
pub fn LoadImageA(
    sys: &mut dyn System,
    hInstance: HINSTANCE,
    name: ResourceKey<&str>,
    typ: Result<IMAGE, u32>,
    cx: u32,
    cy: u32,
    fuLoad: Result<LR, u32>,
) -> HGDIOBJ {
    load_image(
        sys,
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
    sys: &mut dyn System,
    hInstance: HINSTANCE,
    name: ResourceKey<&Str16>,
    typ: Result<IMAGE, u32>,
    cx: u32,
    cy: u32,
    fuLoad: Result<LR, u32>,
) -> HGDIOBJ {
    load_image(sys, hInstance, name, typ.unwrap(), cx, cy, fuLoad.unwrap())
}

fn load_bitmap(
    sys: &mut dyn System,
    hInstance: HINSTANCE,
    name: ResourceKey<&Str16>,
) -> Option<HGDIOBJ> {
    let buf = find_resource(
        sys,
        hInstance,
        ResourceKey::Id(pe::RT::BITMAP as u32),
        &name,
    )?;
    let buf = sys.mem().slice(buf);
    let bmp = Bitmap::parse(buf, None);
    Some(gdi32::get_state(sys).objects.add_bitmap(bmp))
}

#[win32_derive::dllexport]
pub fn LoadBitmapA(
    sys: &mut dyn System,
    hInstance: HINSTANCE,
    lpBitmapName: ResourceKey<&str>,
) -> HGDIOBJ {
    load_bitmap(sys, hInstance, lpBitmapName.to_string16().as_ref()).unwrap()
}

fn find_string(sys: &dyn System, hInstance: HINSTANCE, uID: u32) -> Option<Range<u32>> {
    // Strings are stored as blocks of 16 consecutive strings.
    let (resource_id, index) = ((uID >> 4) + 1, uID & 0xF);

    let block = find_resource(
        sys,
        hInstance,
        ResourceKey::Id(pe::RT::STRING as u32),
        &ResourceKey::Id(resource_id),
    )?;
    let block_ofs = block.start;
    let block = sys.mem().slice(block);

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
    sys: &mut dyn System,
    hInstance: HINSTANCE,
    uID: u32,
    lpBuffer: u32,
    cchBufferMax: u32,
) -> u32 {
    // TODO: merge with LoadStringW
    let str = match find_string(sys, hInstance, uID) {
        Some(str) => Str16::from_bytes(sys.mem().slice(str)).to_string(),
        None => return 0,
    }
    .to_string();
    assert!(cchBufferMax != 0); // MSDN claims this is invalid

    let mut enc = EncoderAnsi::from_mem(sys.mem(), lpBuffer, cchBufferMax);
    enc.write_nul(str.as_str());
    let len = enc.status().unwrap();
    len as u32 - 1
}

#[win32_derive::dllexport]
pub fn LoadStringW(
    sys: &mut dyn System,
    hInstance: HINSTANCE,
    uID: u32,
    lpBuffer: u32,
    cchBufferMax: u32,
) -> u32 {
    // TODO: merge with LoadStringA
    let Some(str) = find_string(sys, hInstance, uID) else {
        return 0;
    };
    let mem = sys.mem();
    if cchBufferMax == 0 {
        mem.put_pod::<u32>(lpBuffer, str.start);
        str.len() as u32
    } else {
        let dst = sys.mem().sub32_mut(lpBuffer, cchBufferMax * 2);
        let copy_len = std::cmp::min(dst.len() - 2, str.len()) as u32;
        dst[..copy_len as usize].copy_from_slice(mem.sub32(str.start, copy_len));
        dst.put_pod::<u16>(copy_len as u32, 0);
        copy_len as u32
    }
}

#[win32_derive::dllexport]
pub fn LoadMenuW(sys: &dyn System, hInstance: u32, lpMenuName: u32) -> HMENU {
    0
}

#[win32_derive::dllexport]
pub fn LoadAcceleratorsW(sys: &dyn System, hInstance: u32, lpTableName: u32) -> HMENU {
    0
}
