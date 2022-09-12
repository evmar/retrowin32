use crate::reader::Reader;
use anyhow::{anyhow, bail};
use bitflags::bitflags;

// https://docs.microsoft.com/en-us/previous-versions/ms809762(v=msdn.10)

fn dos_header(r: &mut Reader) -> anyhow::Result<u32> {
    r.expect("MZ")?;
    r.skip(0x3a)?;
    Ok(r.u32()?)
}

#[derive(Debug)]
pub struct PEHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

#[derive(Debug)]
pub struct PEOptionalHeader {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: u32,
    pub image_base: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub check_sum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u32,
    pub size_of_stack_commit: u32,
    pub size_of_heap_reserve: u32,
    pub size_of_heap_commit: u32,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directory: [ImageDataDirectory; 16],
}

#[derive(Debug)]
pub struct ImageDataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

fn pe_header(r: &mut Reader) -> anyhow::Result<PEHeader> {
    r.expect("PE\0\0")?;
    let header = PEHeader {
        machine: r.u16()?,
        number_of_sections: r.u16()?,
        time_date_stamp: r.u32()?,
        pointer_to_symbol_table: r.u32()?,
        number_of_symbols: r.u32()?,
        size_of_optional_header: r.u16()?,
        characteristics: r.u16()?,
    };
    if header.machine != 0x14c {
        bail!("bad machine {:#x}", header.machine);
    }
    Ok(header)
}

fn data_directory(r: &mut Reader) -> anyhow::Result<ImageDataDirectory> {
    Ok(ImageDataDirectory {
        virtual_address: r.u32()?,
        size: r.u32()?,
    })
}

fn pe_opt_header(r: &mut Reader) -> anyhow::Result<PEOptionalHeader> {
    let opt_header: PEOptionalHeader = PEOptionalHeader {
        magic: r.u16()?,
        major_linker_version: r.u8()?,
        minor_linker_version: r.u8()?,
        size_of_code: r.u32()?,
        size_of_initialized_data: r.u32()?,
        size_of_uninitialized_data: r.u32()?,
        address_of_entry_point: r.u32()?,
        base_of_code: r.u32()?,
        base_of_data: r.u32()?,
        image_base: r.u32()?,
        section_alignment: r.u32()?,
        file_alignment: r.u32()?,
        major_operating_system_version: r.u16()?,
        minor_operating_system_version: r.u16()?,
        major_image_version: r.u16()?,
        minor_image_version: r.u16()?,
        major_subsystem_version: r.u16()?,
        minor_subsystem_version: r.u16()?,
        win32_version_value: r.u32()?,
        size_of_image: r.u32()?,
        size_of_headers: r.u32()?,
        check_sum: r.u32()?,
        subsystem: r.u16()?,
        dll_characteristics: r.u16()?,
        size_of_stack_reserve: r.u32()?,
        size_of_stack_commit: r.u32()?,
        size_of_heap_reserve: r.u32()?,
        size_of_heap_commit: r.u32()?,
        loader_flags: r.u32()?,
        number_of_rva_and_sizes: r.u32()?,
        data_directory: [
            data_directory(r)?,
            data_directory(r)?,
            data_directory(r)?,
            data_directory(r)?,
            data_directory(r)?,
            data_directory(r)?,
            data_directory(r)?,
            data_directory(r)?,
            data_directory(r)?,
            data_directory(r)?,
            data_directory(r)?,
            data_directory(r)?,
            data_directory(r)?,
            data_directory(r)?,
            data_directory(r)?,
            data_directory(r)?,
        ],
    };
    Ok(opt_header)
}

#[derive(Debug)]
pub struct ImageSectionHeader {
    pub name: String,
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: ImageSectionFlags,
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

fn read_section(r: &mut Reader) -> anyhow::Result<ImageSectionHeader> {
    Ok(ImageSectionHeader {
        name: r.str(8)?,
        virtual_size: r.u32()?,
        virtual_address: r.u32()?,
        size_of_raw_data: r.u32()?,
        pointer_to_raw_data: r.u32()?,
        pointer_to_relocations: r.u32()?,
        pointer_to_linenumbers: r.u32()?,
        number_of_relocations: r.u16()?,
        number_of_linenumbers: r.u16()?,
        characteristics: {
            let raw = r.u32()?;
            ImageSectionFlags::from_bits(raw).ok_or_else(|| anyhow!("bad flags {raw:#x}"))?
        },
    })
}

#[derive(Debug)]
pub struct File {
    pub header: PEHeader,
    pub opt_header: PEOptionalHeader,
    pub sections: Vec<ImageSectionHeader>,
}

pub fn parse(buf: &[u8]) -> anyhow::Result<File> {
    let mut r = Reader::new(buf);

    let ofs = dos_header(&mut r)?;
    r.seek(ofs as usize)?;

    let mut file = File {
        header: pe_header(&mut r)?,
        opt_header: pe_opt_header(&mut r)?,
        sections: Vec::new(),
    };

    for _ in 0..file.header.number_of_sections {
        file.sections.push(read_section(&mut r)?);
    }

    Ok(file)
}
