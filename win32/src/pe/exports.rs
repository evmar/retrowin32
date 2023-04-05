#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::IMAGE_DATA_DIRECTORY;
use crate::machine::Machine;
use x86::Memory;

#[derive(Debug)]
#[repr(C)]
pub struct IMAGE_EXPORT_DIRECTORY {
    Characteristics: u32,
    TimeDateStamp: u32,
    MajorVersion: u16,
    MinorVersion: u16,
    Name: u32,
    Base: u32,
    NumberOfFunctions: u32,
    NumberOfNames: u32,
    AddressOfFunctions: u32,
    AddressOfNames: u32,
    AddressOfNameOrdinals: u32,
}
unsafe impl x86::Pod for IMAGE_EXPORT_DIRECTORY {}

impl IMAGE_EXPORT_DIRECTORY {
    #[allow(dead_code)]
    pub fn name<'a>(&self, image: &'a [u8]) -> &'a str {
        image[self.Name as usize..].read_strz()
    }
}

pub struct ExportIter<'a> {
    image: &'a [u8],
    ordinal_base: u32,
    fns: &'a [u32],
    names: &'a [u32],
    ords: &'a [u16],
    i: usize,
}

#[derive(Debug)]
pub struct Export<'a> {
    pub ordinal: u32,
    pub name: &'a str,
    pub addr: u32,
}

impl<'a> Iterator for ExportIter<'a> {
    type Item = Export<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.ords.len() {
            return None;
        }

        let ordinal = self.ords[self.i];
        let name = self.image[self.names[self.i] as usize..].read_strz();
        let addr = self.fns[ordinal as usize];
        self.i += 1;

        Some(Export {
            ordinal: self.ordinal_base + ordinal as u32,
            name,
            addr,
        })
    }
}

pub fn read_exports<'a>(
    machine: &'a Machine,
    base: u32,
    exports: &'a IMAGE_DATA_DIRECTORY,
) -> ExportIter<'a> {
    let image = &machine.x86.mem[base as usize..];
    let dir = exports.as_slice(image).view::<IMAGE_EXPORT_DIRECTORY>(0);
    let fns = image.view_n::<u32>(
        dir.AddressOfFunctions as usize,
        dir.NumberOfFunctions as usize,
    );
    let names = image.view_n::<u32>(dir.AddressOfNames as usize, dir.NumberOfNames as usize);
    let ords = image.view_n::<u16>(
        dir.AddressOfNameOrdinals as usize,
        dir.NumberOfNames as usize,
    );
    ExportIter {
        image,
        ordinal_base: dir.Base,
        fns,
        names,
        ords,
        i: 0,
    }
}
