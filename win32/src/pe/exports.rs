#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::IMAGE_DATA_DIRECTORY;
use crate::machine::Machine;
use x86::{Mem, Memory};

#[derive(Debug)]
#[repr(C)]
pub struct IMAGE_EXPORT_DIRECTORY {
    Characteristics: u32,
    TimeDateStamp: u32,
    MajorVersion: u16,
    MinorVersion: u16,
    Name: u32,
    /// Ordinal offset. Symbol DLL@x => functions[Base + x].
    pub Base: u32,
    NumberOfFunctions: u32,
    NumberOfNames: u32,
    AddressOfFunctions: u32,
    AddressOfNames: u32,
    AddressOfNameOrdinals: u32,
}
unsafe impl x86::Pod for IMAGE_EXPORT_DIRECTORY {}

impl IMAGE_EXPORT_DIRECTORY {
    #[allow(dead_code)]
    pub fn name<'a>(&self, image: &'a Mem) -> &'a str {
        image.slice(self.Name..).read_strz()
    }

    pub fn fns<'a>(&self, image: &'a Mem) -> &'a [u32] {
        image.view_n::<u32>(
            self.AddressOfFunctions as usize,
            self.NumberOfFunctions as usize,
        )
    }

    pub fn names<'a>(&self, image: &'a Mem) -> impl Iterator<Item = (&'a str, u16)> {
        let names = image.view_n::<u32>(self.AddressOfNames as usize, self.NumberOfNames as usize);
        let ords = image.view_n::<u16>(
            self.AddressOfNameOrdinals as usize,
            self.NumberOfNames as usize,
        );

        let ni = names.iter().map(|&addr| image.slice(addr..).read_strz());
        let oi = ords.iter().copied();
        ni.zip(oi)
    }
}

pub fn read_exports<'a>(
    machine: &'a Machine,
    base: u32,
    exports: &'a IMAGE_DATA_DIRECTORY,
) -> &'a IMAGE_EXPORT_DIRECTORY {
    let image = &machine.x86.mem.slice(base..);
    exports.as_mem(image).view::<IMAGE_EXPORT_DIRECTORY>(0)
}
