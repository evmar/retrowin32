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

use memory::{str16::Str16, Extensions};
use std::{mem::size_of, ops::Range};

#[repr(C)]
#[derive(Debug, Clone)]
struct IMAGE_RESOURCE_DIRECTORY {
    Characteristics: u32,
    TimeDateStamp: u32,
    MajorVersion: u16,
    MinorVersion: u16,
    NumberOfNamedEntries: u16,
    NumberOfIdEntries: u16,
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
    Name: u32,
    OffsetToData: u32,
}
unsafe impl memory::Pod for IMAGE_RESOURCE_DIRECTORY_ENTRY {}

/// Top-level dir entry.
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum RT {
    CURSOR = 1,
    BITMAP = 2,
    ICON = 3,
    STRING = 6,
}

#[derive(Debug, Eq)]
pub enum ResourceName<'a> {
    Name(&'a Str16),
    Id(u32),
}

impl<'a> PartialEq for ResourceName<'a> {
    // Case insensitive comparison is used for string names
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Name(name), Self::Name(name_other)) => {
                name.to_string().to_ascii_lowercase() == name_other.to_string().to_ascii_lowercase()
            }
            (Self::Id(id), Self::Id(id_other)) => id == id_other,
            _ => false,
        }
    }
}

enum ResourceValue<'a> {
    Dir(&'a [u8]),
    Data(IMAGE_RESOURCE_DATA_ENTRY),
}

impl IMAGE_RESOURCE_DIRECTORY_ENTRY {
    fn name<'a>(&self, section: &'a [u8]) -> ResourceName<'a> {
        let is_id = self.Name >> 31 == 0;
        let val = self.Name & 0x7FFF_FFFF;
        if is_id {
            ResourceName::Id(val)
        } else {
            let len = section.get_pod::<u16>(val);
            let name = Str16::from_bytes(section.sub32(val + 2, len as u32 * 2));
            ResourceName::Name(name)
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
    OffsetToData: u32,
    Size: u32,
    CodePage: u32,
    Reserved: u32,
}
unsafe impl memory::Pod for IMAGE_RESOURCE_DATA_ENTRY {}

/// Look up a resource by its type/id values.
/// Returns the memory range within the image of the data.
pub fn find_resource(
    section: &[u8],
    query_type: ResourceName,
    query_id: ResourceName,
) -> Option<Range<u32>> {
    // Resources are structured as generic nested directories, but in practice there
    // are always exactly three levels with known semantics.
    let mut dir = IMAGE_RESOURCE_DIRECTORY::entries(section);

    let etype = dir.find(|entry| entry.name(section) == query_type)?;
    let mut dir = match etype.value(section) {
        ResourceValue::Dir(dir) => IMAGE_RESOURCE_DIRECTORY::entries(dir),
        _ => todo!(),
    };

    let eid = dir.find(|entry| entry.name(section) == query_id)?;
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
    Some(data.OffsetToData..(data.OffsetToData + data.Size))
}
