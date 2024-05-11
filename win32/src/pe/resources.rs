//! Windows resources section:
//! https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#the-rsrc-section
//!
//! Section contains a series of IMAGE_RESOURCE_DIRECTORY.
//! Each IMAGE_RESOURCE_DIRECTORY is a header followed by series of IMAGE_RESOURCE_DIRECTORY_ENTRY.
//! Each IMAGE_RESOURCE_DIRECTORY_ENTRY is a name or id and pointer to either
//! another IMAGE_RESOURCE_DIRECTORY or resource data in IMAGE_RESOURCE_DATA_ENTRY.
//! IMAGE_RESOURCE_DATA_ENTRY measures a span of data.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::winapi::types::{DWORD, WORD};
use memory::Extensions;
use std::{mem::size_of, ops::Range};

#[repr(C)]
#[derive(Debug, Clone)]
struct IMAGE_RESOURCE_DIRECTORY {
    Characteristics: DWORD,
    TimeDateStamp: DWORD,
    MajorVersion: WORD,
    MinorVersion: WORD,
    NumberOfNamedEntries: WORD,
    NumberOfIdEntries: WORD,
}
unsafe impl memory::Pod for IMAGE_RESOURCE_DIRECTORY {}

impl IMAGE_RESOURCE_DIRECTORY {
    fn entries(mem: &[u8]) -> impl Iterator<Item = IMAGE_RESOURCE_DIRECTORY_ENTRY> + '_ {
        let header = mem.get_pod::<IMAGE_RESOURCE_DIRECTORY>(0);
        let count = (header.NumberOfIdEntries + header.NumberOfNamedEntries) as u32;
        // Entries are in memory immediately after the directory.
        mem.iter_pod::<IMAGE_RESOURCE_DIRECTORY_ENTRY>(
            size_of::<IMAGE_RESOURCE_DIRECTORY>() as u32,
            count,
        )
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
struct IMAGE_RESOURCE_DIRECTORY_ENTRY {
    Name: DWORD,
    OffsetToData: DWORD,
}
unsafe impl memory::Pod for IMAGE_RESOURCE_DIRECTORY_ENTRY {}

/// Top-level dir entry.
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum RT {
    BITMAP = 2,
    STRING = 6,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResourceName<'a> {
    Name(&'a [u16]),
    Id(u32),
}
enum ResourceValue<'a> {
    Dir(&'a [u8]),
    Data(IMAGE_RESOURCE_DATA_ENTRY),
}
impl IMAGE_RESOURCE_DIRECTORY_ENTRY {
    fn name(&self) -> ResourceName {
        let val = self.Name;
        if val >> 31 == 0 {
            ResourceName::Id(val)
        } else {
            log::error!("unhandled name {val:x} in res dir");
            ResourceName::Name(&[])
        }
    }

    fn value<'a>(&self, section: &'a [u8]) -> ResourceValue<'a> {
        let is_directory = self.OffsetToData >> 31 == 1;
        let offset = self.OffsetToData & 0x7FFF_FFFF;
        if is_directory {
            ResourceValue::Dir(&section[offset as usize..])
        } else {
            ResourceValue::Data(section.get_pod::<IMAGE_RESOURCE_DATA_ENTRY>(offset))
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
struct IMAGE_RESOURCE_DATA_ENTRY {
    OffsetToData: DWORD,
    Size: DWORD,
    CodePage: DWORD,
    Reserved: DWORD,
}
unsafe impl memory::Pod for IMAGE_RESOURCE_DATA_ENTRY {}

/// Look up a resource by its type/id values.
/// Returns a the range within the image of the data.
pub fn find_resource(
    section: &[u8],
    query_type: ResourceName,
    query_id: ResourceName,
) -> Option<Range<u32>> {
    // Resources are structured as generic nested directories, but in practice there
    // are always exactly three levels with known semantics.
    let mut dir = IMAGE_RESOURCE_DIRECTORY::entries(section);

    let etype = dir.find(|entry| entry.name() == query_type)?;
    let mut dir = match etype.value(section) {
        ResourceValue::Dir(dir) => IMAGE_RESOURCE_DIRECTORY::entries(dir),
        _ => todo!(),
    };

    let eid = dir.find(|entry| entry.name() == query_id)?;
    let mut dir = match eid.value(section) {
        ResourceValue::Dir(dir) => IMAGE_RESOURCE_DIRECTORY::entries(dir),
        _ => todo!(),
    };

    let first = dir.next()?;
    if dir.next().is_some() {
        log::warn!("multiple res entries, picking first");
    }
    let data = match first.value(section) {
        ResourceValue::Data(data) => data,
        _ => todo!(),
    };
    Some(data.OffsetToData..data.OffsetToData + data.Size)
}
