use super::IMAGE_DATA_DIRECTORY;
use crate::{machine::Machine, pe, reader::Reader, winapi};
use x86::Memory;

fn patch_iat(machine: &mut Machine, base: u32, imports_data: &IMAGE_DATA_DIRECTORY) {
    // Traverse the ILT, gathering up addresses that need to be fixed up to point at
    // the relevant DLLs shims.
    let mut patches = Vec::new();

    let image = &machine.x86.mem[base as usize..];
    for dll in pe::parse_dlls(imports_data.as_slice(image)) {
        let dll_name = dll.name(image).to_ascii_lowercase();
        for (sym, iat_addr) in dll.entries(image) {
            let name = format!("{}!{}", dll_name, sym.to_string());
            machine
                .labels
                .insert(base + iat_addr, format!("{}@IAT", name));

            let handler = winapi::resolve(&dll_name, &sym);
            let resolved_addr = machine.shims.add(name.clone(), handler);
            machine.labels.insert(resolved_addr, name);

            patches.push((base + iat_addr, resolved_addr));
        }
    }

    for (addr, target) in patches {
        machine.x86.mem.write_u32(addr, target);
    }
}

fn apply_relocs(image: &mut [u8], prev_base: u32, base: u32, relocs: &IMAGE_DATA_DIRECTORY) {
    // monolife.exe has no IMAGE_DIRECTORY_ENTRY::BASERELOC, but does
    // have a .reloc section that is invalid (?).
    // Note: IMAGE_SECTION_HEADER itself also has some relocation-related fields
    // that appear to only apply to object files (?).

    // XXX want to iterate relocations in memory while writing updated relocations,
    // which requires two references to memory.
    let relocs = unsafe { std::mem::transmute(relocs.as_slice(image)) };
    let mut r = Reader::new(relocs);
    while !r.done() {
        let addr = *r.read::<u32>();
        let size = *r.read::<u32>() - 8; // -8 because size includes these two fields
        for _ in 0..(size / 2) {
            let entry = *r.read::<u16>();
            let etype = entry >> 12;
            let ofs = entry & 0x0FFF;
            match etype {
                0 => {} // skip
                3 => {
                    // 32-bit
                    let reloc = image.view_mut::<u32>(addr + ofs as u32);
                    *reloc -= prev_base;
                    *reloc += base;
                }
                _ => panic!("unhandled relocation type {etype}"),
            }
        }
    }
}

fn load_pe(
    machine: &mut Machine,
    buf: &[u8],
    file: &pe::File,
    relocate: bool,
) -> anyhow::Result<u32> {
    let memory_size = file.opt_header.SizeOfImage;
    if memory_size > 20 << 20 {
        // TODO: 5k_run.exe specifies SizeOfImage as like 1000mb, but then doesn't
        // end up using it.  We might need to figure out uncommitted memory to properly
        // load it.
        log::warn!(
            "file header requests {}mb of memory",
            memory_size / (1 << 20)
        );
    }

    let mapping = if relocate {
        machine
            .state
            .kernel32
            .mappings
            .alloc(memory_size, "load_pe".into(), &mut machine.x86.mem)
    } else {
        machine.state.kernel32.mappings.add(
            winapi::kernel32::Mapping {
                addr: file.opt_header.ImageBase,
                size: memory_size,
                desc: "load_pe".into(),
                flags: pe::ImageSectionFlags::MEM_READ,
            },
            false,
        )
    };

    // TODO: .alloc() ensures the memory exists, .add() doesn't.
    let memory_end = (mapping.addr + mapping.size) as usize;
    if memory_end > machine.x86.mem.len() {
        machine.x86.mem.resize(memory_end, 0);
    }
    let base = mapping.addr;

    // The first 0x1000 of the PE file itself is loaded at the base address.
    // I cannot find documentation of this but it is what I observe in a debugger,
    // and kkrunchy relies on file data found after the PE header but outside of any section.
    // TODO: possibly the whole file is loaded at this address and then sections are copied around?
    let size = 0x1000 as usize;
    machine.x86.mem[base as usize..][..size].copy_from_slice(&buf[..size]);

    for sec in file.sections {
        let mut src = sec.PointerToRawData as usize;
        if src == 1 {
            // Graphism (crinkler) hacks this as 1 but gets loaded as if it was zero.
            // TODO: something about alignment?  Maybe this section gets ignored?
            src = 0;
        }
        let dst = (base + sec.VirtualAddress) as usize;
        // sec.SizeOfRawData is the amount of data in the file that should be copied to memory.
        // sec.VirtualSize is the in-memory size of the resulting section, which can be:
        // - greater than SizeOfRawData for sections that should be zero-filled (like uninitialized data),
        // - less than SizeOfRawData because SizeOfRawData is padded up to FileAlignment(!).

        let data_size = sec.SizeOfRawData as usize;
        let flags = sec.characteristics()?;

        // Load the section contents from the file.
        // Note: kkrunchy-packed files have a single section marked
        // CODE | INITIALIZED_DATA | UNINITIALIZED_DATA | MEM_EXECUTE | MEM_READ | MEM_WRITE
        // so we ignore the UNINITIALIZED_DATA flag.
        let load_data = flags.contains(pe::ImageSectionFlags::CODE)
            || flags.contains(pe::ImageSectionFlags::INITIALIZED_DATA);
        if load_data && data_size > 0 {
            machine.x86.mem[dst..dst + data_size].copy_from_slice(&buf[src..(src + data_size)]);
        }
        machine.state.kernel32.mappings.add(
            winapi::kernel32::Mapping {
                addr: dst as u32,
                size: sec.VirtualSize as u32,
                desc: format!("{:?} ({:?})", sec.name(), flags),
                flags,
            },
            true,
        );
    }

    if relocate {
        if let Some(relocs) = file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::BASERELOC) {
            apply_relocs(
                &mut machine.x86.mem[base as usize..],
                file.opt_header.ImageBase,
                base,
                relocs,
            );
        }
    }

    if let Some(imports) = file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::IMPORT) {
        patch_iat(machine, base, imports);
    }

    if let Some(exports) = file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::EXPORT) {
        for export in pe::read_exports(machine, base, exports) {
            log::info!("export: {:x?}", export);
        }
    }

    Ok(base)
}

pub fn load_exe(machine: &mut Machine, buf: &[u8], cmdline: String) -> anyhow::Result<()> {
    let file = pe::parse(&buf)?;

    let base = load_pe(machine, buf, &file, false)?;
    machine.state.kernel32.image_base = base;

    machine.state.kernel32.init(&mut machine.x86.mem, cmdline);
    machine.x86.regs.fs_addr = machine.state.kernel32.teb;

    let mut stack_size = file.opt_header.SizeOfStackReserve;
    // Zig reserves 16mb stacks, just truncate for now.
    if stack_size > 1 << 20 {
        log::warn!(
            "requested {}mb stack reserve, using 32kb instead",
            stack_size / (1 << 20)
        );
        stack_size = 32 << 10;
    }
    let stack =
        machine
            .state
            .kernel32
            .mappings
            .alloc(stack_size, "stack".into(), &mut machine.x86.mem);
    let stack_end = stack.addr + stack.size - 4;
    machine.x86.regs.esp = stack_end;
    machine.x86.regs.ebp = stack_end;

    if let Some(res_data) = file
        .data_directory
        .get(pe::IMAGE_DIRECTORY_ENTRY::RESOURCE as usize)
    {
        machine.state.user32.resources_base = res_data.VirtualAddress;
    }

    let entry_point = base + file.opt_header.AddressOfEntryPoint;
    machine.x86.regs.eip = entry_point;

    Ok(())
}
