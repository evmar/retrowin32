#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::IMAGE_DATA_DIRECTORY;
use crate::str16::expect_ascii;
use memory::Mem;

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
    pub fn name<'a>(&self, image: Mem<'a>) -> &'a str {
        expect_ascii(image.slicez(self.Name))
    }

    pub fn fns<'a>(&self, image: Mem<'a>) -> &'a [u32] {
        image.view_n::<u32>(self.AddressOfFunctions, self.NumberOfFunctions)
    }

    pub fn names<'a>(&self, image: Mem<'a>) -> impl Iterator<Item = (&'a str, u16)> {
        let names = image.view_n::<u32>(self.AddressOfNames, self.NumberOfNames);
        let ords = image.view_n::<u16>(self.AddressOfNameOrdinals, self.NumberOfNames);

        let ni = names
            .iter()
            .map(move |&addr| expect_ascii(image.slicez(addr)));
        let oi = ords.iter().copied();
        ni.zip(oi)
    }
}

pub fn read_exports<'a>(
    mem: Mem<'a>,
    base: u32,
    exports: &IMAGE_DATA_DIRECTORY,
) -> &'a IMAGE_EXPORT_DIRECTORY {
    let image = mem.slice(base..);
    exports.as_mem(image).view::<IMAGE_EXPORT_DIRECTORY>(0)
}
