//! Windows resources section:
//! https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#the-rsrc-section

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::winapi::types::{DWORD, WORD};
use std::mem::size_of;
use x86::Mem;

use super::IMAGE_DATA_DIRECTORY;

#[repr(C)]
#[derive(Debug)]
struct IMAGE_RESOURCE_DIRECTORY {
    Characteristics: DWORD,
    TimeDateStamp: DWORD,
    MajorVersion: WORD,
    MinorVersion: WORD,
    NumberOfNamedEntries: WORD,
    NumberOfIdEntries: WORD,
}
unsafe impl x86::Pod for IMAGE_RESOURCE_DIRECTORY {}

struct ImageResourceDirectory<'a> {
    entries: &'a [IMAGE_RESOURCE_DIRECTORY_ENTRY],
}

impl<'a> ImageResourceDirectory<'a> {
    fn read(mem: &'a Mem, ofs: u32) -> ImageResourceDirectory<'a> {
        let header = mem.view::<IMAGE_RESOURCE_DIRECTORY>(ofs);
        let count = (header.NumberOfIdEntries + header.NumberOfNamedEntries) as u32;
        // Entries are in memory immediately after the directory.
        let entries = mem.view_n::<IMAGE_RESOURCE_DIRECTORY_ENTRY>(
            ofs + size_of::<IMAGE_RESOURCE_DIRECTORY>() as u32,
            count,
        );
        ImageResourceDirectory { entries }
    }
}

#[repr(C)]
#[derive(Debug)]
struct IMAGE_RESOURCE_DIRECTORY_ENTRY {
    Name: DWORD,
    OffsetToData: DWORD,
}
unsafe impl x86::Pod for IMAGE_RESOURCE_DIRECTORY_ENTRY {}

/// Top-level dir entry.
pub const RT_BITMAP: u32 = 2;

#[derive(Debug)]
enum ResourceName {
    Name(String),
    Id(u32),
}
enum ResourceValue<'a> {
    Dir(ImageResourceDirectory<'a>),
    Data(&'a IMAGE_RESOURCE_DATA_ENTRY),
}
impl IMAGE_RESOURCE_DIRECTORY_ENTRY {
    fn name(&self) -> ResourceName {
        let val = self.Name;
        if val >> 31 == 0 {
            ResourceName::Id(val)
        } else {
            ResourceName::Name(format!("unhandled name {val:x} in res dir"))
        }
    }

    fn has_id(&self, query: u32) -> bool {
        match self.name() {
            ResourceName::Id(id) if id == query => true,
            _ => false,
        }
    }

    fn value<'a>(&self, section: &'a Mem) -> ResourceValue<'a> {
        let is_directory = self.OffsetToData >> 31 == 1;
        let offset = self.OffsetToData & 0x7FFF_FFFF;
        if is_directory {
            ResourceValue::Dir(ImageResourceDirectory::read(section, offset))
        } else {
            ResourceValue::Data(section.view::<IMAGE_RESOURCE_DATA_ENTRY>(offset))
        }
    }
}

#[repr(C)]
#[derive(Debug)]
struct IMAGE_RESOURCE_DATA_ENTRY {
    OffsetToData: DWORD,
    Size: DWORD,
    CodePage: DWORD,
    Reserved: DWORD,
}
unsafe impl x86::Pod for IMAGE_RESOURCE_DATA_ENTRY {}

/// Look up a resource by its type/id values.
pub fn get_resource<'a>(
    mem: &'a Mem,
    section: &IMAGE_DATA_DIRECTORY,
    query_type: u32,
    query_id: u32,
) -> Option<&'a Mem> {
    let section = mem.slice(section.VirtualAddress..).slice(..section.Size);

    // Resources are structured as generic nested directories, but in practice there
    // are always exactly three levels with known semantics.
    let dir = ImageResourceDirectory::read(section, 0);

    let etype = dir.entries.iter().find(|entry| entry.has_id(query_type))?;
    let dir = match etype.value(section) {
        ResourceValue::Dir(dir) => dir,
        _ => todo!(),
    };

    let eid = dir.entries.iter().find(|entry| entry.has_id(query_id))?;
    let dir = match eid.value(section) {
        ResourceValue::Dir(dir) => dir,
        _ => todo!(),
    };

    if dir.entries.len() > 1 {
        log::warn!("multiple res entries, picking first");
    }
    let data = match dir.entries.first()?.value(section) {
        ResourceValue::Data(data) => data,
        _ => todo!(),
    };
    let buf = mem.slice(data.OffsetToData..).slice(..data.Size);
    return Some(buf);
}
