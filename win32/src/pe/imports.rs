#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::{
    reader::Reader,
    str16::expect_ascii,
    winapi::{types::DWORD, ImportSymbol},
};
use memory::{Extensions, Mem};

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

/// Import Directory Table (section 6.4.1)
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    /// ILT
    OriginalFirstThunk: DWORD,
    TimeDateStamp: DWORD,
    ForwarderChain: DWORD,
    Name: DWORD,
    /// IAT
    FirstThunk: DWORD,
}
unsafe impl memory::Pod for IMAGE_IMPORT_DESCRIPTOR {}

impl IMAGE_IMPORT_DESCRIPTOR {
    pub fn image_name<'a>(&self, image: Mem<'a>) -> &'a str {
        expect_ascii(image.slicez(self.Name))
    }

    pub fn ilt<'m>(&self, image: Mem<'m>) -> ILTITer<'m> {
        // Officially OriginalFirstThunk (ILT) should have all the data, but in one
        // executable they're all 0, possibly a Borland compiler thing.
        // Meanwhile, win2k's msvcrt.dll has invalid FirstThunk (IAT) data...
        let addr = if self.OriginalFirstThunk != 0 {
            self.OriginalFirstThunk
        } else {
            self.FirstThunk
        };
        ILTITer {
            mem: image.slice(addr..),
            ofs: 0,
        }
    }

    pub fn iat_offset(&self) -> u32 {
        self.FirstThunk
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

/// Import Lookup Table (section 6.4.2)
pub struct ILTITer<'m> {
    mem: Mem<'m>,
    ofs: u32,
}
impl<'m> Iterator for ILTITer<'m> {
    type Item = ILTEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.mem.get::<u32>(self.ofs);
        if entry == 0 {
            return None;
        }
        self.ofs += 4;
        Some(ILTEntry(entry))
    }
}

pub struct ILTEntry(u32);
impl ILTEntry {
    pub fn as_import_symbol(self, image: Mem) -> ImportSymbol {
        let entry = self.0;
        if entry & (1 << 31) != 0 {
            let ordinal = entry & 0xFFFF;
            ImportSymbol::Ordinal(ordinal)
        } else {
            // First two bytes at offset are hint/name table index, used to look up
            // the name faster in the DLL; we just skip them.
            let sym_name = expect_ascii(image.slicez(entry + 2));
            ImportSymbol::Name(sym_name)
        }
    }
}
