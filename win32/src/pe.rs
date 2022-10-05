#![allow(non_snake_case)]

use crate::{
    memory::{self, Memory, DWORD, WORD},
    reader::{read_strz, Reader},
    x86::write_u32,
};
use anyhow::{anyhow, bail};
use bitflags::bitflags;

// https://docs.microsoft.com/en-us/previous-versions/ms809762(v=msdn.10)
// https://learn.microsoft.com/en-us/windows/win32/debug/pe-format

fn dos_header(r: &mut Reader) -> anyhow::Result<u32> {
    r.expect("MZ")?;
    r.skip(0x3a)?;
    Ok(r.u32()?)
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
    log::info!("header {:?}", header);
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
    pub fn name(&self) -> String {
        self.Name.read_strz()
    }
    pub fn characteristics(&self) -> anyhow::Result<ImageSectionFlags> {
        ImageSectionFlags::from_bits(self.Characteristics)
            .ok_or_else(|| anyhow!("bad flags {:#x}", self.Characteristics))
    }
}

bitflags! {
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
    mut resolve: impl FnMut(&str, String, u32) -> u32,
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
            let entry = iat_reader.u32()?;
            if entry == 0 {
                break;
            }
            if entry & (1 << 31) != 0 {
                let ordinal = entry & 0xFFFF;
                log::warn!("TODO ordinal {}:{}", dll_name, ordinal);
            } else {
                // First two bytes at offset are hint/name table index, used to look up
                // the name faster in the DLL; we just skip them.
                let sym_name = read_strz(&mem[(entry + 2) as usize..]);
                let target = resolve(&dll_name, sym_name, addr);
                patches.push((addr, target));
            }
        }
    }

    for (addr, target) in patches {
        write_u32(mem, addr, target);
    }

    Ok(())
}
