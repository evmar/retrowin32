#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::winapi::types::{DWORD, WORD};
use std::mem::size_of;
use x86::Memory;

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
    _header: &'a IMAGE_RESOURCE_DIRECTORY,
    entries: &'a [IMAGE_RESOURCE_DIRECTORY_ENTRY],
}

impl<'a> ImageResourceDirectory<'a> {
    fn read(mem: &'a [u8], ofs: u32) -> ImageResourceDirectory<'a> {
        let header = mem.view::<IMAGE_RESOURCE_DIRECTORY>(ofs);
        let count = (header.NumberOfIdEntries + header.NumberOfNamedEntries) as usize;
        // Entries are in memory immediately after the directory.
        let entries = mem.view_n::<IMAGE_RESOURCE_DIRECTORY_ENTRY>(
            ofs as usize + size_of::<IMAGE_RESOURCE_DIRECTORY>(),
            count,
        );
        ImageResourceDirectory {
            _header: header,
            entries,
        }
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
impl IMAGE_RESOURCE_DIRECTORY_ENTRY {
    fn name(&self) -> ResourceName {
        let val = self.Name;
        if val >> 31 == 0 {
            ResourceName::Id(val)
        } else {
            ResourceName::Name(format!("unhandled name {val:x} in res dir"))
        }
    }
    fn is_directory(&self) -> bool {
        self.OffsetToData >> 31 == 1
    }
    fn offset(&self) -> u32 {
        self.OffsetToData & 0x7FFF_FFFF
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

pub fn get_resource(
    mem: &[u8],
    section_base: u32,
    query_type: u32,
    query_id: u32,
) -> Option<&[u8]> {
    let section = &mem[section_base as usize..];

    // Resources are structured as generic nested directories, but in practice there
    // are always exactly three levels with known semantics.
    let dir = ImageResourceDirectory::read(section, 0);
    for type_entry in dir.entries {
        assert!(type_entry.is_directory());
        match type_entry.name() {
            ResourceName::Id(id) if id == query_type => {}
            _ => continue,
        };
        let dir = ImageResourceDirectory::read(section, type_entry.offset());
        for id_entry in dir.entries {
            assert!(id_entry.is_directory());
            match id_entry.name() {
                ResourceName::Id(id) if id == query_id => {}
                _ => continue,
            };
            let dir = ImageResourceDirectory::read(section, id_entry.offset());
            if dir.entries.len() > 1 {
                log::warn!("multiple res entries, picking first");
            }
            for lang_entry in dir.entries {
                assert!(!lang_entry.is_directory());
                let data = section.view::<IMAGE_RESOURCE_DATA_ENTRY>(lang_entry.offset());
                let ofs = data.OffsetToData as usize;
                let len = data.Size as usize;
                let buf = &mem[ofs..ofs + len];
                return Some(buf);
            }
        }
    }
    None
}
