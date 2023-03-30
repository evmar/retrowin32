#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::{
    reader::Reader,
    winapi::{types::DWORD, ImportSymbol},
};
use x86::Memory;

#[derive(Debug)]
#[repr(C)]
struct IMAGE_IMPORT_DESCRIPTOR {
    OriginalFirstThunk: DWORD,
    TimeDateStamp: DWORD,
    ForwarderChain: DWORD,
    Name: DWORD,
    FirstThunk: DWORD,
}
unsafe impl x86::Pod for IMAGE_IMPORT_DESCRIPTOR {}

/// mem: memory starting at image base
/// addr: address of imports table relative to mem start
/// resolve: map an import name to the address we will jump to for it
pub fn parse_imports(
    mem: &mut [u8],
    addr: usize,
    mut resolve: impl FnMut(&str, ImportSymbol, u32) -> u32,
) -> anyhow::Result<()> {
    // http://sandsprite.com/CodeStuff/Understanding_imports.html
    let mut r = Reader::new(mem);
    r.seek(addr)?;
    let mut patches = Vec::new();
    loop {
        let descriptor = r.read::<IMAGE_IMPORT_DESCRIPTOR>();
        if descriptor.Name == 0 {
            break;
        }
        let dll_name = mem[descriptor.Name as usize..]
            .read_strz()
            .to_ascii_lowercase();

        // Officially original_first_thunk should be an array that contains pointers to
        // IMAGE_IMPORT_BY_NAME entries, but in my sample executable they're all 0.
        // Peering Inside the PE claims this is some difference between compilers, yikes.

        // Code calling a DLL looks like:
        //   call [XXX]
        // where XXX is the address of an entry in the IAT:
        //   IAT: [
        //      AAA,
        //      BBB,  <- XXX might point here
        //   ]
        // On load, each IAT entry points to the function name (as parsed below).
        // The loader is supposed to overwrite the IAT with the addresses to the loaded DLL,
        // but we instead just record the IAT addresses to remap them.
        let mut iat_reader = Reader::new(&mem[descriptor.FirstThunk as usize..]);
        loop {
            let addr = descriptor.FirstThunk + iat_reader.pos as u32;
            let entry = *iat_reader.read::<DWORD>();
            if entry == 0 {
                break;
            }
            let symbol = if entry & (1 << 31) != 0 {
                let ordinal = entry & 0xFFFF;
                ImportSymbol::Ordinal(ordinal)
            } else {
                // First two bytes at offset are hint/name table index, used to look up
                // the name faster in the DLL; we just skip them.
                let sym_name = mem[(entry + 2) as usize..].read_strz();
                ImportSymbol::Name(sym_name)
            };
            let target = resolve(&dll_name, symbol, addr);
            patches.push((addr, target));
        }
    }

    for (addr, target) in patches {
        mem.write_u32(addr, target);
    }

    Ok(())
}
