#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::reader::Reader;
use anyhow::{anyhow, bail};
use bitflags::bitflags;
use memory::Extensions;

// https://docs.microsoft.com/en-us/previous-versions/ms809762(v=msdn.10)
// https://learn.microsoft.com/en-us/windows/win32/debug/pe-format

fn dos_header<'m>(r: &mut Reader<'m>) -> anyhow::Result<u32> {
    r.expect("MZ")?;
    r.skip(0x3a)?;
    Ok(r.read::<u32>())
}

#[derive(Debug)]
#[repr(C)]
pub struct IMAGE_FILE_HEADER {
    pub Machine: u16,
    pub NumberOfSections: u16,
    pub TimeDateStamp: u32,
    pub PointerToSymbolTable: u32,
    pub NumberOfSymbols: u32,
    pub SizeOfOptionalHeader: u16,
    pub Characteristics: IMAGE_FILE,
}
unsafe impl memory::Pod for IMAGE_FILE_HEADER {}

bitflags! {
    #[derive(Debug)]
    pub struct IMAGE_FILE: u16 {
        const RELOCS_STRIPPED = 0x0001;
        const EXECUTABLE_IMAGE = 0x0002;
        const LINE_NUMS_STRIPPED = 0x0004;
        const LOCAL_SYMS_STRIPPED = 0x0008;
        const AGGRESSIVE_WS_TRIM = 0x0010;
        const LARGE_ADDRESS_AWARE = 0x0020;
        const BYTES_REVERSED_LO = 0x0080;
        const _32BIT_MACHINE = 0x0100;
        const DEBUG_STRIPPED = 0x0200;
        const REMOVABLE_RUN_FROM_SWAP = 0x0400;
        const NET_RUN_FROM_SWAP = 0x0800;
        const SYSTEM = 0x1000;
        const DLL = 0x2000;
        const UP_SYSTEM_ONLY = 0x4000;
        const BYTES_REVERSED_HI = 0x8000;
        const _ = !0;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct IMAGE_OPTIONAL_HEADER32 {
    pub Magic: u16,
    pub MajorLinkerVersion: u8,
    pub MinorLinkerVersion: u8,
    pub SizeOfCode: u32,
    pub SizeOfInitializedData: u32,
    pub SizeOfUninitializedData: u32,
    pub AddressOfEntryPoint: u32,
    pub BaseOfCode: u32,
    pub BaseOfData: u32,
    pub ImageBase: u32,
    pub SectionAlignment: u32,
    pub FileAlignment: u32,
    pub MajorOperatingSystemVersion: u16,
    pub MinorOperatingSystemVersion: u16,
    pub MajorImageVersion: u16,
    pub MinorImageVersion: u16,
    pub MajorSubsystemVersion: u16,
    pub MinorSubsystemVersion: u16,
    pub Win32VersionValue: u32,
    pub SizeOfImage: u32,
    pub SizeOfHeaders: u32,
    pub CheckSum: u32,
    pub Subsystem: u16,
    pub DllCharacteristics: IMAGE_DLL_CHARACTERISTICS,
    pub SizeOfStackReserve: u32,
    pub SizeOfStackCommit: u32,
    pub SizeOfHeapReserve: u32,
    pub SizeOfHeapCommit: u32,
    pub LoaderFlags: u32,
    pub NumberOfRvaAndSizes: u32,
}
unsafe impl memory::Pod for IMAGE_OPTIONAL_HEADER32 {}

bitflags! {
    #[derive(Debug)]
    pub struct IMAGE_DLL_CHARACTERISTICS: u16 {
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
        const _ = !0;
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: u32,
    pub Size: u32,
}
unsafe impl memory::Pod for IMAGE_DATA_DIRECTORY {}
impl IMAGE_DATA_DIRECTORY {
    pub fn as_slice<'m>(&self, image: &'m [u8]) -> Option<&'m [u8]> {
        if self.VirtualAddress + self.Size > image.len() as u32 {
            return None;
        }
        Some(image.sub32(self.VirtualAddress, self.Size))
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum IMAGE_DIRECTORY_ENTRY {
    EXPORT = 0,
    IMPORT = 1,
    RESOURCE = 2,
    EXCEPTION = 3,
    SECURITY = 4,
    BASERELOC = 5,
    DEBUG = 6,
    COPYRIGHT = 7,
    GLOBALPTR = 8,
    TLS = 9,
    LOAD_CONFIG = 10,
    BOUND_IMPORT = 11,
    IAT = 12,
    DELAY_IMPORT = 13,
    COM_DESCRIPTOR = 14,
}

fn pe_header<'m>(r: &mut Reader<'m>) -> anyhow::Result<IMAGE_FILE_HEADER> {
    r.expect("PE\0\0")?;

    let header = r.read::<IMAGE_FILE_HEADER>();
    if header.Machine != 0x14c {
        bail!("bad machine {:?}", header.Machine);
    }
    Ok(header)
}

#[repr(C)]
#[derive(Debug, Default)]
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
    pub fn name(&self) -> Result<&str, std::str::Utf8Error> {
        let end = self
            .Name
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(self.Name.len());
        std::str::from_utf8(&self.Name[..end])
    }

    pub fn characteristics(&self) -> anyhow::Result<IMAGE_SCN> {
        IMAGE_SCN::from_bits(self.Characteristics)
            .ok_or_else(|| anyhow!("unhandled PE flags {:#x}", self.Characteristics))
    }
}

bitflags! {
    #[derive(Debug, serde::Serialize)]
    pub struct IMAGE_SCN: u32 {
        const CODE = 0x20;
        const INITIALIZED_DATA = 0x40;
        const UNINITIALIZED_DATA = 0x80;
        const MEM_DISCARDABLE = 0x2000000;
        const MEM_SHARED = 0x10000000;
        const MEM_EXECUTE = 0x20000000;
        const MEM_READ = 0x40000000;
        const MEM_WRITE = 0x80000000;

        // Alignment is found in this nybble:
        const ALIGN = 0x00F0_0000;
    }
}

impl IMAGE_SCN {
    pub fn align(&self) -> u32 {
        let value = (self.bits() & IMAGE_SCN::ALIGN.bits()) >> 20;
        // IMG_SCN_ALIGN_1BYTES = 1
        // IMG_SCN_ALIGN_2BYTES = 2
        // IMG_SCN_ALIGN_4BYTES = 4
        // etc.
        1 << (value - 1)
    }
}

#[derive(Debug)]
pub struct File {
    pub header: IMAGE_FILE_HEADER,
    pub opt_header: IMAGE_OPTIONAL_HEADER32,
    pub data_directory: Box<[IMAGE_DATA_DIRECTORY]>,
    pub sections: Box<[IMAGE_SECTION_HEADER]>,
}

impl File {
    pub fn get_data_directory(
        &self,
        entry: IMAGE_DIRECTORY_ENTRY,
    ) -> Option<&IMAGE_DATA_DIRECTORY> {
        let dir = self.data_directory.get(entry as usize)?;
        if dir.VirtualAddress == 0 {
            return None;
        }
        Some(dir)
    }
}

pub fn parse(buf: &[u8]) -> anyhow::Result<File> {
    let mut r = Reader::new(buf);

    let pe_header_ofs = dos_header(&mut r).map_err(|err| anyhow!("reading DOS header: {}", err))?;
    if pe_header_ofs as usize + std::mem::size_of::<IMAGE_FILE_HEADER>() >= buf.len() {
        bail!("invalid PE offset in DOS header, might be a DOS executable?");
    }
    r.seek(pe_header_ofs)
        .map_err(|err| anyhow!("seeking PE header {pe_header_ofs:x}: {}", err))?;

    let header = pe_header(&mut r).map_err(|err| anyhow!("reading PE header: {}", err))?;
    let opt_header = r.read::<IMAGE_OPTIONAL_HEADER32>();
    let data_directory = r
        .read_n::<IMAGE_DATA_DIRECTORY>(opt_header.NumberOfRvaAndSizes)
        .unwrap();
    let sections = r
        .read_n::<IMAGE_SECTION_HEADER>(header.NumberOfSections as u32)
        .unwrap();

    Ok(File {
        header,
        opt_header,
        data_directory,
        sections,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn dos_header() {
        let mut buf: Vec<u8> = Vec::new();
        buf.write(b"MZ").unwrap();
        buf.write(&[0; 0x3a]).unwrap();
        buf.write(&0xFFFFFFFFu32.to_le_bytes()).unwrap();
        assert!(parse(&buf).is_err()); // no crash
    }

    #[test]
    fn kkrunchy_header() {
        let mut header = IMAGE_SECTION_HEADER::default();
        header.Name = *b"kkrunchy";
        assert_eq!(header.name().unwrap(), "kkrunchy");
    }
}
