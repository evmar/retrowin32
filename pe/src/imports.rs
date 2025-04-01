#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use memory::Extensions;

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
#[derive(Debug, Default)]
#[repr(C)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    /// ILT
    OriginalFirstThunk: u32,
    TimeDateStamp: u32,
    ForwarderChain: u32,
    Name: u32,
    /// IAT
    FirstThunk: u32,
}
unsafe impl memory::Pod for IMAGE_IMPORT_DESCRIPTOR {}

impl IMAGE_IMPORT_DESCRIPTOR {
    pub fn image_name<'m>(&self, image: &'m [u8]) -> &'m [u8] {
        image.slicez(self.Name)
    }

    /// Return an iterator over entries in the ILT, which describe imported functions.
    pub fn ilt<'m>(&self, image: &'m [u8]) -> impl Iterator<Item = ILTEntry> + 'm {
        // Officially OriginalFirstThunk (ILT) should have all the data, but in one
        // executable they're all 0, possibly a Borland compiler thing.
        // Meanwhile, win2k's msvcrt.dll has invalid FirstThunk (IAT) data...
        let addr = if self.OriginalFirstThunk != 0 {
            self.OriginalFirstThunk
        } else {
            self.FirstThunk
        };

        // Import Lookup Table (section 6.4.2)
        image[addr as usize..]
            .into_iter_pod::<ILTEntry>()
            .take_while(|entry| entry.0 != 0)
    }

    /// Return an iterator over (IAT entry address, ILT entry) pairs.
    /// IAT addresses are relative to the image base.
    /// Caller should update IAT address with the address of the function referred to in the entry.
    pub fn iat_iter<'m>(&self, image: &'m [u8]) -> impl Iterator<Item = (u32, ILTEntry)> + 'm {
        let iat_addr = self.FirstThunk;
        let iat_iter = (0..).map(move |i| iat_addr + (i * 4));
        iat_iter.zip(self.ilt(image))
    }
}

pub fn read_imports<'m>(buf: &'m [u8]) -> impl Iterator<Item = IMAGE_IMPORT_DESCRIPTOR> + 'm {
    buf.into_iter_pod::<IMAGE_IMPORT_DESCRIPTOR>()
        .take_while(|desc| desc.Name != 0)
}

#[repr(transparent)]
pub struct ILTEntry(u32);
unsafe impl memory::Pod for ILTEntry {}

#[derive(Debug)]
pub enum ImportSymbol<'a> {
    Name(&'a [u8]),
    Ordinal(u32),
}
impl<'a> std::fmt::Display for ImportSymbol<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportSymbol::Name(name) => match name.try_ascii() {
                Some(name) => f.write_str(name),
                None => f.write_fmt(format_args!("{:?}", name)),
            },
            ImportSymbol::Ordinal(ord) => f.write_fmt(format_args!("{}", ord)),
        }
    }
}

impl ILTEntry {
    pub fn as_import_symbol(self, image: &[u8]) -> ImportSymbol {
        let entry = self.0;
        if entry & (1 << 31) != 0 {
            let ordinal = entry & 0xFFFF;
            ImportSymbol::Ordinal(ordinal)
        } else {
            // First two bytes at offset are hint/name table index, used to look up
            // the name faster in the DLL; we just skip them.
            let sym_name = image.slicez(entry + 2);
            ImportSymbol::Name(sym_name)
        }
    }
}
