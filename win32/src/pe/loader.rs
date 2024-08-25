#![allow(non_snake_case)]

use super::{apply_relocs, IMAGE_DATA_DIRECTORY, IMAGE_SECTION_HEADER};
use crate::{machine::Machine, pe, winapi};
use memory::{Extensions, ExtensionsMut};
use std::collections::HashMap;

/// Create a memory mapping, optionally copying some data to it.
fn map_memory(machine: &mut Machine, mapping: winapi::kernel32::Mapping, buf: Option<&[u8]>) {
    let winapi::kernel32::Mapping { addr, size, .. } =
        *machine.state.kernel32.mappings.add(mapping);

    let memory_end = addr + size;
    if memory_end > machine.emu.memory.len() {
        panic!("not enough memory reserved");
    }

    if let Some(buf) = buf {
        machine
            .mem()
            .sub32_mut(addr, buf.len() as u32)
            .copy_from_slice(buf);
    }
}

/// Copy the file header itself into memory, choosing a base address.
fn load_image(
    machine: &mut Machine,
    filename: &str,
    file: &pe::File,
    buf: &[u8],
    relocate: Option<Option<u32>>,
) -> u32 {
    let addr = match relocate {
        Some(Some(addr)) => addr,
        Some(None) => machine
            .state
            .kernel32
            .mappings
            .find_space(file.opt_header.SizeOfImage),
        None => file.opt_header.ImageBase,
    };

    let first_page_size = std::cmp::min(buf.len(), 0x1000);
    map_memory(
        machine,
        winapi::kernel32::Mapping {
            addr,
            size: first_page_size as u32,
            desc: filename.into(),
            flags: pe::ImageSectionFlags::MEM_READ,
        },
        Some(&buf[..first_page_size]),
    );

    addr
}

/// Load a PE section into memory.
fn load_section(
    machine: &mut Machine,
    filename: &str,
    base: u32,
    buf: &[u8],
    sec: &IMAGE_SECTION_HEADER,
) {
    let mut src = sec.PointerToRawData as usize;
    if src == 1 {
        // Graphism (crinkler) hacks this as 1 but gets loaded as if it was zero.
        // TODO: something about alignment?  Maybe this section gets ignored?
        src = 0;
    }
    let dst = base + sec.VirtualAddress;
    // sec.SizeOfRawData is the amount of data in the file that should be copied to memory.
    // sec.VirtualSize is the in-memory size of the resulting section, which can be:
    // - greater than SizeOfRawData for sections that should be zero-filled (like uninitialized data),
    // - less than SizeOfRawData because SizeOfRawData is padded up to FileAlignment(!).

    let data_size = sec.SizeOfRawData;
    let flags = sec.characteristics().unwrap();

    // Decide whether t load the section contents from the file.
    // Note: kkrunchy-packed files have a single section marked
    // CODE | INITIALIZED_DATA | UNINITIALIZED_DATA | MEM_EXECUTE | MEM_READ | MEM_WRITE
    // so we ignore the UNINITIALIZED_DATA flag.
    let load_data = flags.contains(pe::ImageSectionFlags::CODE)
        || flags.contains(pe::ImageSectionFlags::INITIALIZED_DATA);

    let mapping = winapi::kernel32::Mapping {
        addr: dst as u32,
        size: sec.VirtualSize as u32,
        desc: format!(
            "{filename} {:?} ({:?})",
            sec.name().unwrap_or("[invalid]"),
            flags
        ),
        flags,
    };

    map_memory(
        machine,
        mapping,
        if load_data && data_size > 0 {
            Some(&buf[src..][..data_size as usize])
        } else {
            None
        },
    );
}

fn patch_iat(machine: &mut Machine, base: u32, imports_data: &IMAGE_DATA_DIRECTORY) {
    // Traverse the ILT, gathering up addresses that need to be fixed up to point at
    // the relevant DLLs shims.
    let mut patches = Vec::new();

    let image: &[u8] = unsafe { std::mem::transmute(machine.mem().slice(base..)) };
    let section = match imports_data.as_slice(image) {
        None => return,
        Some(s) => s,
    };
    for dll_imports in pe::read_imports(section) {
        let dll_name = dll_imports.image_name(image).to_ascii_lowercase();
        let hmodule = winapi::kernel32::load_library(machine, &dll_name);
        // TODO: missing dll should not be an possibility here, we should error instead.
        let mut dll = machine.state.kernel32.dlls.get_mut(&hmodule);
        for (i, entry) in dll_imports.ilt(image).enumerate() {
            let sym = entry.as_import_symbol(image);
            let name = format!("{}!{}", dll_name, sym.to_string());
            let iat_addr = base + dll_imports.iat_offset() + (i as u32 * 4);
            machine.labels.insert(iat_addr, format!("{}@IAT", name));

            let resolved_addr = if let Some(dll) = dll.as_mut() {
                dll.resolve(&sym)
            } else {
                0
            };
            if resolved_addr != 0 {
                machine.labels.insert(resolved_addr, name);
            }
            patches.push((iat_addr, resolved_addr));
        }
    }

    for (addr, target) in patches {
        machine.mem().put_pod::<u32>(addr, target);
    }
}

fn load_pe(
    machine: &mut Machine,
    filename: &str,
    buf: &[u8],
    file: &pe::File,
    relocate: Option<Option<u32>>,
) -> anyhow::Result<u32> {
    let base = load_image(machine, filename, file, buf, relocate);

    for sec in file.sections.iter() {
        load_section(machine, filename, base, buf, sec);
    }

    if base != file.opt_header.ImageBase {
        if let Some(relocs) = file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::BASERELOC) {
            let image = machine.mem().slice(base..);
            if let Some(sec) = relocs.as_slice(image) {
                apply_relocs(image, file.opt_header.ImageBase, base, sec);
            }
        }
    }

    if let Some(imports) = file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::IMPORT) {
        patch_iat(machine, base, imports);
    }

    Ok(base)
}

pub struct EXEFields {
    pub entry_point: u32,
    pub stack_size: u32,
}

pub fn load_exe(
    machine: &mut Machine,
    buf: &[u8],
    filename: &str,
    relocate: Option<Option<u32>>,
) -> anyhow::Result<EXEFields> {
    let file = pe::parse(buf)?;

    let base = load_pe(machine, filename, buf, &file, relocate)?;
    machine.state.kernel32.image_base = base;

    if let Some(res_data) = file
        .data_directory
        .get(pe::IMAGE_DIRECTORY_ENTRY::RESOURCE as usize)
    {
        machine.state.kernel32.resources = res_data.clone();
    }

    let entry_point = base + file.opt_header.AddressOfEntryPoint;

    let addrs = EXEFields {
        entry_point,
        stack_size: file.opt_header.SizeOfStackReserve,
    };
    Ok(addrs)
}

#[derive(Debug)]
pub struct DLL {
    /// Image base address.
    pub base: u32,

    /// Function name => resolved address.
    pub names: HashMap<String, u32>,

    /// fns[ordinal - ordinal base] => resolved address.
    pub ordinal_base: u32,
    pub fns: Vec<u32>,

    pub resources: Option<IMAGE_DATA_DIRECTORY>,

    /// Address of DllMain() entry point.
    pub entry_point: Option<u32>,
}

pub fn load_dll(machine: &mut Machine, filename: &str, buf: &[u8]) -> anyhow::Result<DLL> {
    let file = pe::parse(&buf)?;

    let base = load_pe(machine, filename, buf, &file, Some(None))?;
    let image = machine.mem().slice(base..);

    let entry_point = if file.opt_header.AddressOfEntryPoint != 0 {
        Some(base + file.opt_header.AddressOfEntryPoint)
    } else {
        None
    };
    let mut ordinal_base = 1;
    let mut fns = Vec::new();
    let mut names = HashMap::new();
    if let Some(dir) = file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::EXPORT) {
        let section = dir
            .as_slice(image)
            .ok_or_else(|| anyhow::anyhow!("invalid exports"))?;
        let dir = pe::read_exports(section);
        ordinal_base = dir.Base;
        for addr in dir.fns(image) {
            fns.push(base + addr);
        }
        for (name, i) in dir.names(image) {
            names.insert(name.to_string(), fns[i as usize]);
        }
    }

    let resources = file
        .data_directory
        .get(pe::IMAGE_DIRECTORY_ENTRY::RESOURCE as usize)
        .cloned();

    Ok(DLL {
        base,
        names,
        ordinal_base,
        fns,
        resources,
        entry_point,
    })
}
