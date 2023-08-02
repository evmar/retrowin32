#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::{
    reader::Reader,
    winapi::{types::DWORD, ImportSymbol},
};
use x86::Mem;

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

#[derive(Clone, Debug, Default)]
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
    pub fn image_name<'a>(&self, image: Mem<'a>) -> &'a str {
        image.slicez(self.Name).unwrap().to_ascii()
    }

    pub fn entries<'m>(&self, image: Mem<'m>) -> ILTITer<'m> {
        let mut r = Reader::new(image);
        // Officially OriginalFirstThunk should be an array that contains pointers to
        // IMAGE_IMPORT_BY_NAME entries, but in my sample executable they're all 0.
        // FirstThunk has the same pointer though.
        // Peering Inside the PE claims this is some difference between compilers, yikes.
        // I guess one is the IAT and one the ILT (?).
        r.seek(self.FirstThunk).unwrap();
        ILTITer { r }
    }
}

pub struct IDTIter<'m> {
    /// r.buf points at IDT
    r: Reader<'m>,
}
impl<'m> Iterator for IDTIter<'m> {
    type Item = IMAGE_IMPORT_DESCRIPTOR;

    fn next(&mut self) -> Option<Self::Item> {
        // On some executables the IDT is unaligned (at offset 0x200f!) so we must
        // clone here rather than just taking a view on the existing bytes.
        let descriptor = self.r.read_unaligned::<IMAGE_IMPORT_DESCRIPTOR>();
        if descriptor.Name == 0 {
            return None;
        }
        Some(descriptor)
    }
}

pub fn read_imports<'m>(buf: Mem<'m>) -> IDTIter<'m> {
    IDTIter {
        r: Reader::new(buf),
    }
}

pub struct ILTITer<'m> {
    /// r.buf must point at image, not ILT, because entries can refer to image-relative addresses.
    r: Reader<'m>,
}
impl<'m> Iterator for ILTITer<'m> {
    type Item = (ImportSymbol<'m>, u32);

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
            let sym_name = self.r.buf.slicez(entry + 2).unwrap().to_ascii();
            ImportSymbol::Name(sym_name)
        };
        Some((symbol, addr))
    }
}
