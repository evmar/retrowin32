//! PE image loading.

use crate::{machine::Machine, winapi};
use memory::{Extensions, ExtensionsMut};
use std::{collections::HashMap, path::Path};

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
            flags: pe::IMAGE_SCN::MEM_READ,
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
    sec: &pe::IMAGE_SECTION_HEADER,
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
    // - less than SizeOfRawData because SizeOfRawData is padded up to FileAlignment(!),
    // - and also seems to sometimes be zero, I guess in favor of SizeOfRawData.

    let mapping_size = sec.VirtualSize.max(sec.SizeOfRawData);
    let data_size = sec.SizeOfRawData;
    let flags = sec.characteristics().unwrap();

    // Decide whether t load the section contents from the file.
    // Note: kkrunchy-packed files have a single section marked
    // CODE | INITIALIZED_DATA | UNINITIALIZED_DATA | MEM_EXECUTE | MEM_READ | MEM_WRITE
    // so we ignore the UNINITIALIZED_DATA flag.
    // Another executable has its .bss section with non-zero SizeOfRawData, so we can't trust that
    // either.
    let load_data =
        flags.contains(pe::IMAGE_SCN::CODE) || flags.contains(pe::IMAGE_SCN::INITIALIZED_DATA);

    let mapping = winapi::kernel32::Mapping {
        addr: dst,
        size: mapping_size,
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

/// Load the exports section of a module, gathering addresses of exported symbols.
fn load_exports(image_base: u32, image: &[u8], exports_data: &[u8]) -> Exports {
    let dir = pe::read_exports(exports_data);

    let mut fns = Vec::new();
    for addr in dir.fns(image) {
        fns.push(image_base + addr);
    }

    let mut names = HashMap::new();
    for (name, i) in dir.names(image) {
        let addr = fns[i as usize];
        names.insert(name.to_vec(), addr);
    }

    Exports {
        ordinal_base: dir.Base,
        fns,
        names,
    }
}

fn patch_iat(machine: &mut Machine, base: u32, imports_addr: &pe::IMAGE_DATA_DIRECTORY) {
    // Traverse the ILT, gathering up addresses that need to be fixed up to point at
    // the correct targets.
    let mut patches: Vec<(u32, u32)> = Vec::new();

    let image: &[u8] = unsafe { std::mem::transmute(machine.mem().slice(base..)) };
    let Some(section) = imports_addr.as_slice(image) else {
        return;
    };
    for dll_imports in pe::read_imports(section) {
        let Some(dll_name) = dll_imports.image_name(image).try_ascii() else {
            log::warn!(
                "skipping non-ascii import {:?}",
                dll_imports.image_name(image)
            );
            continue;
        };
        let dll_name = dll_name.to_ascii_lowercase();
        let hmodule = winapi::kernel32::load_library(machine, &dll_name);
        let mut dll = machine.state.kernel32.dlls.get_mut(&hmodule);
        for (i, entry) in dll_imports.ilt(image).enumerate() {
            let sym = entry.as_import_symbol(image);
            let name = format!("{}!{}", dll_name, sym.to_string());
            let iat_addr = base + dll_imports.iat_offset() + (i as u32 * 4);
            machine.labels.insert(iat_addr, format!("{}@IAT", name));

            let resolved_addr = if let Some(dll) = dll.as_mut() {
                if let Some(sym) = dll.resolve(&sym) {
                    Some(sym)
                } else {
                    log::warn!("missing symbol {name}");
                    None
                }
            } else {
                None
            };

            let addr = resolved_addr.unwrap_or(0);
            patches.push((iat_addr, addr));
        }
    }

    for (addr, target) in patches {
        machine.mem().put_pod::<u32>(addr, target);
    }
}

/// Load an exe or dll, including resolving imports.
pub fn load_module(
    machine: &mut Machine,
    filename: &str,
    buf: &[u8],
    file: &pe::File,
    relocate: Option<Option<u32>>,
) -> anyhow::Result<Module> {
    let base = load_image(machine, filename, file, buf, relocate);

    for sec in file.sections.iter() {
        load_section(machine, filename, base, buf, sec);
    }

    if base != file.opt_header.ImageBase {
        if let Some(relocs) = file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::BASERELOC) {
            let image = machine.mem().slice(base..);
            if let Some(sec) = relocs.as_slice(image) {
                // Warning: apply_relocs wants to mutate arbitrary memory, which violates Rust aliasing rules.
                // It has started silently failing in release builds in the past...
                pe::apply_relocs(
                    file.opt_header.ImageBase,
                    base,
                    sec,
                    |addr| {
                        let addr = base + addr;
                        machine.mem().get_pod::<u32>(addr)
                    },
                    |addr, val| {
                        let addr = base + addr;
                        machine.mem().put_pod::<u32>(addr, val);
                    },
                );
            }
        }
    }

    // Gather exports before loading imports, because the import process may need to load
    // DLLs which reference exports from this module.
    let image = machine.emu.memory.mem().slice(base..);
    let exports = if let Some(dir) = file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::EXPORT) {
        let section = dir
            .as_slice(image)
            .ok_or_else(|| anyhow::anyhow!("invalid exports"))?;
        let exports = load_exports(base, image, section);
        for (name, &addr) in exports.names.iter() {
            if let Some(name) = name.try_ascii() {
                machine.labels.insert(addr, format!("{filename}!{name}"));
            }
        }
        exports
    } else {
        Exports::default()
    };

    if let Some(imports) = file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::IMPORT) {
        patch_iat(machine, base, imports);
    }

    let resources = file
        .data_directory
        .get(pe::IMAGE_DIRECTORY_ENTRY::RESOURCE as usize)
        .cloned();

    let entry_point = if file.opt_header.AddressOfEntryPoint == 0 {
        None
    } else {
        Some(base + file.opt_header.AddressOfEntryPoint)
    };

    Ok(Module {
        image_base: base,
        exports,
        resources,
        entry_point,
    })
}

pub struct EXEFields {
    pub entry_point: u32,
    pub stack_size: u32,
}

pub fn load_exe(
    machine: &mut Machine,
    buf: &[u8],
    path: &str,
    relocate: Option<Option<u32>>,
) -> anyhow::Result<EXEFields> {
    let file = pe::parse(buf)?;
    let path = Path::new(path);
    let filename = path.file_name().unwrap().to_string_lossy();
    let module = load_module(machine, &filename, buf, &file, relocate)?;
    machine.state.kernel32.image_base = module.image_base;

    if let Some(res_data) = module.resources {
        machine.state.kernel32.resources = res_data;
    }

    let addrs = EXEFields {
        entry_point: module.entry_point.unwrap(),
        stack_size: file.opt_header.SizeOfStackReserve,
    };
    Ok(addrs)
}

/// The result of loading an exe or DLL.
#[derive(Debug, Default)]
pub struct Module {
    /// Address where the module was loaded.
    pub image_base: u32,

    pub resources: Option<pe::IMAGE_DATA_DIRECTORY>,

    pub exports: Exports,

    pub entry_point: Option<u32>,
}

#[derive(Debug, Default)]
pub struct Exports {
    /// Exported function name => resolved address.
    pub names: HashMap<Vec<u8>, u32>,

    /// fns[ordinal - ordinal base] => resolved address.
    pub ordinal_base: u32,
    pub fns: Vec<u32>,
}

pub fn load_dll(machine: &mut Machine, filename: &str, buf: &[u8]) -> anyhow::Result<Module> {
    let file = pe::parse(buf)?;
    load_module(machine, filename, buf, &file, Some(None))
}
