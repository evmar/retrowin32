#![allow(non_snake_case)]

use super::{apply_relocs, IMAGE_DATA_DIRECTORY, IMAGE_SECTION_HEADER};
use crate::{
    machine::{Emulator, Machine},
    pe, winapi,
};
use memory::Mem;
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
            .sub(addr, buf.len() as u32)
            .as_mut_slice_todo()
            .copy_from_slice(buf);
    }
}

/// Copy the file header itself into memory, choosing a base address.
fn load_image(
    machine: &mut Machine,
    name: &str,
    file: &pe::File,
    buf: &[u8],
    relocate: bool,
) -> u32 {
    let addr = if relocate {
        machine
            .state
            .kernel32
            .mappings
            .find_space(file.opt_header.SizeOfImage)
    } else {
        file.opt_header.ImageBase
    };

    let first_page_size = std::cmp::min(buf.len(), 0x1000);
    map_memory(
        machine,
        winapi::kernel32::Mapping {
            addr,
            size: first_page_size as u32,
            desc: name.into(),
            flags: pe::ImageSectionFlags::MEM_READ,
        },
        Some(&buf[..first_page_size]),
    );

    addr
}

/// Load a PE section into memory.
fn load_section(
    machine: &mut Machine,
    name: &str,
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
        desc: format!("{name} {:?} ({:?})", sec.name(), flags),
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

    let image: Mem = unsafe { std::mem::transmute(machine.mem().slice(base..)) };
    let image = image.as_slice_todo();
    for dll_imports in pe::read_imports(imports_data.as_slice(image)) {
        let dll_name = dll_imports.image_name(image).to_ascii_lowercase();
        let hmodule = winapi::kernel32::LoadLibraryA(machine, Some(&dll_name));
        // TODO: missing dll should not be an possibility here, we should error instead.
        let mut dll = match hmodule.to_dll_index() {
            Some(index) => Some(&mut machine.state.kernel32.dlls[index]),
            None => None,
        };
        for (i, entry) in dll_imports.ilt(image).enumerate() {
            let sym = entry.as_import_symbol(image);
            let name = format!("{}!{}", dll_name, sym.to_string());
            let iat_addr = base + dll_imports.iat_offset() + (i as u32 * 4);
            machine.labels.insert(iat_addr, format!("{}@IAT", name));

            let resolved_addr = if let Some(dll) = dll.as_mut() {
                dll.resolve(sym, |shim| machine.emu.register(shim))
            } else {
                machine.emu.register(Err(format!("{name} not found")))
            };
            machine.labels.insert(resolved_addr, name);
            patches.push((iat_addr, resolved_addr));
        }
    }

    for (addr, target) in patches {
        machine.mem().put::<u32>(addr, target);
    }
}

fn load_pe(
    machine: &mut Machine,
    name: &str,
    buf: &[u8],
    file: &pe::File,
    relocate: bool,
) -> anyhow::Result<u32> {
    let base = load_image(machine, name, file, buf, relocate);

    for sec in file.sections.iter() {
        load_section(machine, name, base, buf, sec);
    }

    if relocate {
        if let Some(relocs) = file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::BASERELOC) {
            let image = machine.mem().slice(base..);
            apply_relocs(
                image,
                file.opt_header.ImageBase,
                base,
                relocs.as_slice(image.as_slice_todo()),
            );
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
    cmdline: String,
    relocate: bool,
) -> anyhow::Result<EXEFields> {
    let file = pe::parse(buf)?;

    let base = load_pe(machine, &cmdline, buf, &file, relocate)?;
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
    /// Function name => resolved address.
    pub names: HashMap<String, u32>,

    /// Function ordinal => resolved address.
    // TODO: ords could be a flat array once we have ordinals for all the builtin DLLs.
    pub ordinals: HashMap<u32, u32>,

    /// Address of DllMain() entry point.
    pub entry_point: u32,
}

pub fn load_dll(machine: &mut Machine, name: &str, buf: &[u8]) -> anyhow::Result<DLL> {
    let file = pe::parse(&buf)?;

    let base = load_pe(machine, name, buf, &file, true)?;
    let image = machine.mem().slice(base..).as_slice_todo();

    let entry_point = base + file.opt_header.AddressOfEntryPoint;
    let mut ordinals = HashMap::new();
    let mut names = HashMap::new();
    if let Some(dir) = file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::EXPORT) {
        let dir = pe::read_exports(dir.as_slice(image));
        for (i, addr) in dir.fns(image).enumerate() {
            let ord = dir.Base + i as u32;
            ordinals.insert(ord, base + addr);
        }
        for (name, i) in dir.names(image) {
            let ord = dir.Base + i as u32;
            let addr = *ordinals.get(&ord).unwrap();
            names.insert(name.to_string(), addr);
        }
    }

    Ok(DLL {
        ordinals,
        names,
        entry_point,
    })
}
