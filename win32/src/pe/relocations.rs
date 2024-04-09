#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use memory::{Extensions, Mem};

#[repr(C)]
#[derive(Clone)]
struct IMAGE_BASE_RELOCATION {
    VirtualAddress: u32,
    SizeOfBlock: u32,
}
unsafe impl memory::Pod for IMAGE_BASE_RELOCATION {}

pub fn apply_relocs(image: Mem, prev_base: u32, base: u32, mut relocs: &[u8]) {
    // monolife.exe has no IMAGE_DIRECTORY_ENTRY::BASERELOC, but does
    // have a .reloc section that is invalid (?).
    // Note: IMAGE_SECTION_HEADER itself also has some relocation-related fields
    // that appear to only apply to object files (?).

    while !relocs.is_empty() {
        let reloc = relocs.get_pod::<IMAGE_BASE_RELOCATION>(0);
        let body =
            &relocs[std::mem::size_of::<IMAGE_BASE_RELOCATION>()..reloc.SizeOfBlock as usize];
        for entry in body.into_iter_pod::<u16>() {
            let etype = entry >> 12;
            let ofs = entry & 0x0FFF;
            let addr = reloc.VirtualAddress + ofs as u32;
            match etype {
                0 => {} // skip
                3 => {
                    // IMAGE_REL_BASED_HIGHLOW, 32-bit adjustment
                    let reloc = image.view_mut::<u32>(addr);
                    *reloc -= prev_base;
                    *reloc += base;
                }
                _ => panic!("unhandled relocation type {etype}"),
            }
        }
        relocs = &relocs[reloc.SizeOfBlock as usize..];
    }
}