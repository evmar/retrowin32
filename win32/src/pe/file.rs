#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::reader::Reader;
use crate::winapi::types::{DWORD, WORD};
use anyhow::{anyhow, bail};
use bitflags::bitflags;
use memory::Extensions;

// https://docs.microsoft.com/en-us/previous-versions/ms809762(v=msdn.10)
// https://learn.microsoft.com/en-us/windows/win32/debug/pe-format

fn dos_header<'m>(r: &mut Reader<'m>) -> anyhow::Result<u32> {
    r.expect("MZ")?;
    r.skip(0x3a)?;
    Ok(r.read::<DWORD>())
}

#[derive(Debug, Default, Clone)]
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
#[derive(Debug, Clone)]
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
}
unsafe impl memory::Pod for IMAGE_OPTIONAL_HEADER32 {}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: DWORD,
    pub Size: DWORD,
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
#[derive(Debug, Default, Clone)]
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
    pub fn characteristics(&self) -> anyhow::Result<ImageSectionFlags> {
        ImageSectionFlags::from_bits(self.Characteristics)
            .ok_or_else(|| anyhow!("unhandled PE flags {:#x}", self.Characteristics))
    }
}

bitflags! {
    #[derive(serde::Serialize, serde::Deserialize)]
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
