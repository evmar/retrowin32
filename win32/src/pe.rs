#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::mem::size_of;

use crate::{
    memory::{self, Memory, DWORD, WORD},
    reader::Reader,
};
use anyhow::{anyhow, bail};
use bitflags::bitflags;

// https://docs.microsoft.com/en-us/previous-versions/ms809762(v=msdn.10)
// https://learn.microsoft.com/en-us/windows/win32/debug/pe-format

fn dos_header(r: &mut Reader) -> anyhow::Result<u32> {
    r.expect("MZ")?;
    r.skip(0x3a)?;
    Ok(r.view::<DWORD>().get())
}

#[derive(Debug)]
#[repr(C)]
pub struct IMAGE_FILE_HEADER {
    pub Machine: WORD,
    pub NumberOfSections: WORD,
    pub TimeDateStamp: DWORD,
    pub PointerToSymbolTable: DWORD,
    pub NumberOfSymbols: DWORD,
    pub SizeOfOptionalHeader: WORD,
    pub Characteristics: WORD,
}
unsafe impl memory::Pod for IMAGE_FILE_HEADER {}

bitflags! {
    pub struct DllCharacteristics: u16 {
        const HIGH_ENTROPY_VA = 0x0020;
        const DYNAMIC_BASE = 0x0040;
        const FORCE_INTEGRITY = 0x0080;
        const NX_COMPAT = 0x0100;
        const NO_ISOLATION = 0x0200;
        const NO_SEH = 0x0400;
        const NO_BIND = 0x0800;
        const APPCONTAINER = 0x1000;
        const WDM_DRIVER = 0x2000;
        const GUARD_CF = 0x4000;
        const TERMINAL_SERVER_AWARE = 0x8000;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct IMAGE_OPTIONAL_HEADER32 {
    pub Magic: WORD,
    pub MajorLinkerVersion: u8,
    pub MinorLinkerVersion: u8,
    pub SizeOfCode: DWORD,
    pub SizeOfInitializedData: DWORD,
    pub SizeOfUninitializedData: DWORD,
    pub AddressOfEntryPoint: DWORD,
    pub BaseOfCode: DWORD,
    pub BaseOfData: DWORD,
    pub ImageBase: DWORD,
    pub SectionAlignment: DWORD,
    pub FileAlignment: DWORD,
    pub MajorOperatingSystemVersion: WORD,
    pub MinorOperatingSystemVersion: WORD,
    pub MajorImageVersion: WORD,
    pub MinorImageVersion: WORD,
    pub MajorSubsystemVersion: WORD,
    pub MinorSubsystemVersion: WORD,
    pub Win32VersionValue: DWORD,
    pub SizeOfImage: DWORD,
    pub SizeOfHeaders: DWORD,
    pub CheckSum: DWORD,
    pub Subsystem: WORD,
    pub DllCharacteristics: WORD,
    pub SizeOfStackReserve: DWORD,
    pub SizeOfStackCommit: DWORD,
    pub SizeOfHeapReserve: DWORD,
    pub SizeOfHeapCommit: DWORD,
    pub LoaderFlags: DWORD,
    pub NumberOfRvaAndSizes: DWORD,
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
}
unsafe impl memory::Pod for IMAGE_OPTIONAL_HEADER32 {}

#[repr(C)]
#[derive(Debug)]
pub struct IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: DWORD,
    pub Size: DWORD,
}
unsafe impl memory::Pod for IMAGE_DATA_DIRECTORY {}

fn pe_header<'a>(r: &mut Reader<'a>) -> anyhow::Result<&'a IMAGE_FILE_HEADER> {
    r.expect("PE\0\0")?;

    let header: &'a IMAGE_FILE_HEADER = r.view::<IMAGE_FILE_HEADER>();
    if header.Machine.get() != 0x14c {
        bail!("bad machine {:?}", header.Machine);
    }
    Ok(header)
}

#[repr(C)]
#[derive(Debug)]
pub struct IMAGE_SECTION_HEADER {
    pub Name: [u8; 8],
    pub VirtualSize: u32,
    pub VirtualAddress: u32,
    pub SizeOfRawData: u32,
    pub PointerToRawData: u32,
    pub PointerToRelocations: u32,
    pub PointerToLinenumbers: u32,
    pub NumberOfRelocations: u16,
    pub NumberOfLinenumbers: u16,
    pub Characteristics: u32,
}
unsafe impl memory::Pod for IMAGE_SECTION_HEADER {}
impl IMAGE_SECTION_HEADER {
    pub fn name(&self) -> &str {
        self.Name.read_strz()
    }
    pub fn characteristics(&self) -> anyhow::Result<ImageSectionFlags> {
        ImageSectionFlags::from_bits(self.Characteristics)
            .ok_or_else(|| anyhow!("bad flags {:#x}", self.Characteristics))
    }
}

bitflags! {
    #[derive(serde::Serialize)]
    pub struct ImageSectionFlags: u32 {
        const CODE = 0x20;
        const INITIALIZED_DATA = 0x40;
        const UNINITIALIZED_DATA = 0x80;
        const MEM_DISCARDABLE = 0x02000000;
        const MEM_EXECUTE = 0x20000000;
        const MEM_READ = 0x40000000;
        const MEM_WRITE = 0x80000000;
    }
}

fn read_section<'a>(r: &mut Reader<'a>) -> &'a IMAGE_SECTION_HEADER {
    r.view::<IMAGE_SECTION_HEADER>()
}

#[derive(Debug)]
pub struct File<'a> {
    pub header: &'a IMAGE_FILE_HEADER,
    pub opt_header: &'a IMAGE_OPTIONAL_HEADER32,
    pub sections: Vec<&'a IMAGE_SECTION_HEADER>,
}

pub fn parse(buf: &[u8]) -> anyhow::Result<File> {
    let mut r = Reader::new(buf);

    let ofs = dos_header(&mut r)?;
    r.seek(ofs as usize)?;

    let mut file = File {
        header: pe_header(&mut r)?,
        opt_header: r.view::<IMAGE_OPTIONAL_HEADER32>(),
        sections: Vec::new(),
    };

    for _ in 0..file.header.NumberOfSections.get() {
        file.sections.push(read_section(&mut r));
    }

    Ok(file)
}

#[derive(Debug)]
#[repr(C)]
struct IMAGE_IMPORT_DESCRIPTOR {
    OriginalFirstThunk: DWORD,
    TimeDateStamp: DWORD,
    ForwarderChain: DWORD,
    Name: DWORD,
    FirstThunk: DWORD,
}
unsafe impl memory::Pod for IMAGE_IMPORT_DESCRIPTOR {}

/// mem: memory starting at image base
/// addr: address of imports table relative to mem start
/// resolve: map an import name to the address we will jump to for it
pub fn parse_imports(
    mem: &mut [u8],
    addr: usize,
    mut resolve: impl FnMut(&str, &str, u32) -> u32,
) -> anyhow::Result<()> {
    // http://sandsprite.com/CodeStuff/Understanding_imports.html
    let mut r = Reader::new(mem);
    r.seek(addr)?;
    let mut patches = Vec::new();
    loop {
        let descriptor = r.view::<IMAGE_IMPORT_DESCRIPTOR>();
        if descriptor.Name.get() == 0 {
            break;
        }
        let dll_name = mem[descriptor.Name.get() as usize..]
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
        let mut iat_reader = Reader::new(&mem[descriptor.FirstThunk.get() as usize..]);
        loop {
            let addr = descriptor.FirstThunk.get() + iat_reader.pos as u32;
            let entry = iat_reader.view::<DWORD>().get();
            if entry == 0 {
                break;
            }
            if entry & (1 << 31) != 0 {
                let ordinal = entry & 0xFFFF;
                log::warn!("TODO ordinal {}:{}", dll_name, ordinal);
            } else {
                // First two bytes at offset are hint/name table index, used to look up
                // the name faster in the DLL; we just skip them.
                let sym_name = mem[(entry + 2) as usize..].read_strz();
                let target = resolve(&dll_name, sym_name, addr);
                patches.push((addr, target));
            }
        }
    }

    for (addr, target) in patches {
        mem.write_u32(addr, target);
    }

    Ok(())
}

#[repr(C)]
#[derive(Debug)]
struct IMAGE_RESOURCE_DIRECTORY {
    Characteristics: DWORD,
    TimeDateStamp: DWORD,
    MajorVersion: WORD,
    MinorVersion: WORD,
    NumberOfNamedEntries: WORD,
    NumberOfIdEntries: WORD,
}
unsafe impl memory::Pod for IMAGE_RESOURCE_DIRECTORY {}
impl IMAGE_RESOURCE_DIRECTORY {
    fn entries(&self, mem: &[u8]) -> &[IMAGE_RESOURCE_DIRECTORY_ENTRY] {
        let count = (self.NumberOfIdEntries.get() + self.NumberOfNamedEntries.get()) as usize;
        // Entries are in memory immediately after the directory.
        let mem = &mem[size_of::<IMAGE_RESOURCE_DIRECTORY>()..];
        let mem = &mem[0..(count * size_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY>()) as usize];
        unsafe {
            std::slice::from_raw_parts(mem.as_ptr() as *const IMAGE_RESOURCE_DIRECTORY_ENTRY, count)
        }
    }
}

#[repr(C)]
#[derive(Debug)]
struct IMAGE_RESOURCE_DIRECTORY_ENTRY {
    Name: DWORD,
    OffsetToData: DWORD,
}
unsafe impl memory::Pod for IMAGE_RESOURCE_DIRECTORY_ENTRY {}

/// Top-level dir entry.
pub const RT_BITMAP: u32 = 2;

#[derive(Debug)]
enum ResourceName {
    Name(String),
    Id(u32),
}
impl IMAGE_RESOURCE_DIRECTORY_ENTRY {
    fn name(&self) -> ResourceName {
        let val = self.Name.get();
        if val >> 31 == 0 {
            ResourceName::Id(val)
        } else {
            ResourceName::Name(format!("unhandled name {val:x} in res dir"))
        }
    }
    fn is_directory(&self) -> bool {
        self.OffsetToData.get() >> 31 == 1
    }
    fn offset(&self) -> u32 {
        self.OffsetToData.get() & 0x7FFF_FFFF
    }
}

#[repr(C)]
#[derive(Debug)]
struct IMAGE_RESOURCE_DATA_ENTRY {
    OffsetToData: DWORD,
    Size: DWORD,
    CodePage: DWORD,
    Reserved: DWORD,
}
unsafe impl memory::Pod for IMAGE_RESOURCE_DATA_ENTRY {}

pub fn get_resource(
    mem: &[u8],
    section_base: u32,
    query_type: u32,
    query_id: u32,
) -> Option<&[u8]> {
    let section = &mem[section_base as usize..];

    // Resources are structured as generic nested directories, but in practice there
    // are always exactly three levels with known semantics.
    let dir = section.view::<IMAGE_RESOURCE_DIRECTORY>(0);
    for type_entry in dir.entries(section) {
        assert!(type_entry.is_directory());
        match type_entry.name() {
            ResourceName::Id(id) if id == query_type => {}
            _ => continue,
        };
        let dir = section.view::<IMAGE_RESOURCE_DIRECTORY>(type_entry.offset());
        for id_entry in dir.entries(&section[type_entry.offset() as usize..]) {
            assert!(id_entry.is_directory());
            match id_entry.name() {
                ResourceName::Id(id) if id == query_id => {}
                _ => continue,
            };
            let dir = section.view::<IMAGE_RESOURCE_DIRECTORY>(id_entry.offset());
            let entries = dir.entries(&section[id_entry.offset() as usize..]);
            if entries.len() > 1 {
                log::warn!("multiple res entries, picking first");
            }
            for lang_entry in entries {
                assert!(!lang_entry.is_directory());
                let data = section.view::<IMAGE_RESOURCE_DATA_ENTRY>(lang_entry.offset());
                let ofs = data.OffsetToData.get() as usize;
                let len = data.Size.get() as usize;
                let buf = &mem[ofs..ofs + len];
                return Some(buf);
            }
        }
    }
    None
}
