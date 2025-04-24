#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::c_str;
use memory::Extensions;

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
unsafe impl memory::Pod for IMAGE_EXPORT_DIRECTORY {}

impl IMAGE_EXPORT_DIRECTORY {
    #[allow(dead_code)]
    pub fn name<'a>(&self, image: &'a [u8]) -> &'a [u8] {
        c_str(&image[self.Name as usize..])
    }

    /// Returns an iterator of function addresses in ordinal order.
    pub fn fns<'a>(&self, image: &'a [u8]) -> impl Iterator<Item = u32> {
        image.iter_pod::<u32>(self.AddressOfFunctions, self.NumberOfFunctions)
    }

    /// Returns an iterator of (name, index) pairs, where index is an index into fn()s.
    pub fn names<'a>(&self, image: &'a [u8]) -> impl Iterator<Item = (&'a [u8], u16)> {
        let names = image.iter_pod::<u32>(self.AddressOfNames, self.NumberOfNames);
        let ords = image.iter_pod::<u16>(self.AddressOfNameOrdinals, self.NumberOfNames);

        let ni = names.map(move |addr| c_str(&image[addr as usize..]));
        ni.zip(ords)
    }
}

pub fn read_exports(section: &[u8]) -> IMAGE_EXPORT_DIRECTORY {
    section.get_pod::<IMAGE_EXPORT_DIRECTORY>(0)
}
