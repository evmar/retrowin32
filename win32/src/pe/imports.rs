#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::{
    reader::Reader,
    winapi::{types::DWORD, ImportSymbol},
};
use x86::Memory;

// http://sandsprite.com/CodeStuff/Understanding_imports.html
//
// Code calling a DLL looks like:
//   call [XXX]
// where XXX is the address of an entry in the IAT:
//   IAT: [
//      AAA,
//      BBB,  <- XXX might point here
//   ]
// On load, each IAT entry points to the function name (as parsed below).
// The loader overwrites the IAT with the addresses to the loaded DLL.

#[derive(Debug)]
#[repr(C)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    OriginalFirstThunk: DWORD,
    TimeDateStamp: DWORD,
    ForwarderChain: DWORD,
    Name: DWORD,
    FirstThunk: DWORD,
}
unsafe impl x86::Pod for IMAGE_IMPORT_DESCRIPTOR {}

impl IMAGE_IMPORT_DESCRIPTOR {
    pub fn name<'a>(&self, image: &'a [u8]) -> &'a str {
        image[self.Name as usize..].read_strz()
    }

    pub fn entries<'a>(&self, image: &'a [u8]) -> ILTITer<'a> {
        let mut r = Reader::new(image);
        // Officially OriginalFirstThunk should be an array that contains pointers to
        // IMAGE_IMPORT_BY_NAME entries, but in my sample executable they're all 0.
        // FirstThunk has the same pointer though.
        // Peering Inside the PE claims this is some difference between compilers, yikes.
        // I guess one is the IAT and one the ILT (?).
        r.seek(self.FirstThunk as usize).unwrap();
        ILTITer { r }
    }
}

pub struct IDTIter<'a> {
    /// r.buf points at IDT
    r: Reader<'a>,
}
impl<'a> Iterator for IDTIter<'a> {
    type Item = &'a IMAGE_IMPORT_DESCRIPTOR;

    fn next(&mut self) -> Option<Self::Item> {
        let descriptor = self.r.read::<IMAGE_IMPORT_DESCRIPTOR>();
        if descriptor.Name == 0 {
            return None;
        }
        Some(descriptor)
    }
}

pub fn parse_dlls(buf: &[u8]) -> IDTIter {
    IDTIter {
        r: Reader::new(buf),
    }
}

pub struct ILTITer<'a> {
    /// r.buf points at image
    r: Reader<'a>,
}
impl<'a> Iterator for ILTITer<'a> {
    type Item = (ImportSymbol<'a>, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let addr = self.r.pos as u32;
        let entry = *self.r.read::<DWORD>();
        if entry == 0 {
            return None;
        }
        let symbol = if entry & (1 << 31) != 0 {
            let ordinal = entry & 0xFFFF;
            ImportSymbol::Ordinal(ordinal)
        } else {
            // First two bytes at offset are hint/name table index, used to look up
            // the name faster in the DLL; we just skip them.
            let sym_name = self.r.buf[(entry + 2) as usize..].read_strz();
            ImportSymbol::Name(sym_name)
        };
        Some((symbol, addr))
    }
}
