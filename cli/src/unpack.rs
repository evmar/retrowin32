use std::{io::Write, ops::Range};

use memory::{Extensions, ExtensionsMut, Pod};
use win32::{Machine, Module, winapi::kernel32};

/// Load and run the exe until we've hit the "real" (unpacked) entry point.
/// This means eip is the real entry point, and all the exe's code/data is unpacked in memory.
fn run_to_entry_point(machine: &mut Machine, unpack_at: u32) -> anyhow::Result<()> {
    // Run until we hit the initial entry point, which means load the exe and its dlls.
    machine.break_on_startup();
    while machine.run() {}
    if machine.status != win32::Status::DebugBreak {
        machine.dump_state(0);
        anyhow::bail!("failed to break on startup");
    }
    machine.unblock();

    // Run until we hit the unpack point -- let the exe's main() unpack the real exe.
    machine.add_breakpoint(unpack_at);
    while machine.run() {}
    if machine.status != win32::Status::DebugBreak || machine.emu.x86.cpu().regs.eip != unpack_at {
        machine.dump_state(0);
        anyhow::bail!("failed to break on unpack-at point");
    }
    machine.clear_breakpoint(unpack_at);
    machine.unblock();

    // Step one instruction to advance to the real entry point.
    // (This may be a jmp/call to the unpacked exe's entry point, or a ret to it.)
    machine.single_step();
    if !machine.status.is_running() {
        machine.dump_state(0);
        anyhow::bail!("failed to step to entry point");
    }

    Ok(())
}

/// Construct the IDT for the unpacked exe.  This lists the DLLs and functions that are imported.
/// This also mutates the image's IAT to become an ILT.
fn construct_idt(
    machine: &Machine,
    image: &mut [u8],
    orig_iat: &pe::IMAGE_DATA_DIRECTORY,
) -> Box<[u8]> {
    let mut names = Vec::new();
    for imp in pe::read_imports(orig_iat.as_slice(image).unwrap()) {
        names.push((imp.Name, imp.image_name(image).to_ascii_lowercase()));
    }

    let mut buf = Vec::new();

    for module in kernel32::get_state(machine).modules.values() {
        if module.name == "retrowin32.dll" {
            continue;
        }
        if !module.name.ends_with(".dll") {
            continue;
        }
        let name_addr = names
            .iter()
            .find(|(_, name)| module.name.as_bytes() == name)
            .unwrap()
            .0;

        let range = find_iat(image, orig_iat, module).unwrap();
        iat_to_ilt(module, image, range.clone());

        let desc = pe::IMAGE_IMPORT_DESCRIPTOR {
            OriginalFirstThunk: 0,
            TimeDateStamp: 0,
            ForwarderChain: 0,
            Name: name_addr,
            FirstThunk: range.start,
        };
        buf.write(Pod::as_bytes(&desc)).ok();
    }

    buf.into()
}

/// Find the IAT in the unpacked exe by searching for the known addresses of the functions that
/// were loaded with GetProcAddress().
/// The original IAT address range (of the packed outer executable) is used to filter out potential
/// irrelevant duplicates.
fn find_iat(
    image: &[u8],
    orig_iat: &pe::IMAGE_DATA_DIRECTORY,
    module: &Module,
) -> Option<Range<u32>> {
    if module.dynamic_imports.is_empty() {
        return None;
    }

    let orig_iat_range = orig_iat.as_range();

    let mut start = u32::MAX;
    let mut end = 0u32;

    for &addr in module.dynamic_imports.iter() {
        let mut loc = None;
        let addr_bytes = addr.to_le_bytes();
        for (ofs, _) in image
            .windows(4)
            .enumerate()
            .filter(|(_, w)| *w == addr_bytes)
        {
            let ofs = ofs as u32;
            if orig_iat_range.contains(&ofs) {
                // Skip matches found in original IAT; they must be in new IAT to matter.
                continue;
            }

            if let Some(prev) = loc {
                // TODO: it's possible some random memory location has the value; we could
                // filter these based on whether their locations are in the plausible range.
                todo!(
                    "{}: duplicate IAT match {:#x} vs {:#x}",
                    module.name,
                    prev,
                    ofs
                );
            }

            loc = Some(ofs);
        }

        let Some(loc) = loc else {
            log::warn!("{}: failed to find IAT entry for {:#x}", module.name, addr);
            // This can happen when e.g. msvcrt.dll's DllMain() calls GetProcAddress(), yikes.
            continue;
        };
        start = start.min(loc);
        end = end.max(loc);
    }

    if end == 0 {
        log::warn!("{}: failed to find IAT entry", module.name);
        return None;
    }
    // We expect to have found the entire IAT, so there should be an empty entry at the end.
    assert_eq!(image.get_pod::<u32>(end + 4), 0);

    Some(start..(end + 4))
}

/// Overwrite the IAT entries with the addresses of the names of the functions they pointed to,
/// effectively reversing the ILT->IAT process done at load time.
fn iat_to_ilt(module: &Module, image: &mut [u8], iat: Range<u32>) {
    for iat_addr in iat.step_by(4) {
        let func_addr = image.get_pod::<u32>(iat_addr);
        // Find the name of the function exported at that address.
        let name = module.exports.name_from_addr(func_addr).unwrap();
        // Search the image for that name.  (We expect it to be present because it was passed to GetProcAddress...)
        let name_addr = (0..image.len() - name.len() - 1)
            .find(|&i| &image[i..][..name.len()] == name && image[i + name.len()] == 0)
            .unwrap() as u32;
        // Write the IAT entry with the address of the name, minus 2 because ILT entries have an (unused) 2-byte prefix.
        image.put_pod::<u32>(iat_addr, name_addr - 2);
    }
}

/// OutEXE stores some intermediate state of the exe we're trying to write.
/// In particular, we dynamically collect all the sections we intend to write out, because we
/// need to know how many sections we'll write to correctly compute offsets for the following
/// section data.
struct OutEXE {
    dos_header: Box<[u8]>,
    header: pe::IMAGE_NT_HEADERS32,
    data_dirs: Box<[pe::IMAGE_DATA_DIRECTORY]>,
    sections: Vec<OutEXESection>,
}

struct OutEXESection {
    name: String,
    file_addr: u32,
    virtual_addr: u32,
    data: Box<[u8]>,
}

impl OutEXE {
    fn from_image(image: &[u8]) -> anyhow::Result<Self> {
        let mut ofs = pe::parse::dos_header(image).unwrap();
        let dos_header = &image[..ofs as usize];
        let header = pe::parse::pe_header(image, &mut ofs).unwrap();
        let data_dirs = pe::parse::data_directory(&header, image, &mut ofs).unwrap();

        Ok(OutEXE {
            dos_header: dos_header.into(),
            header,
            data_dirs,
            sections: Default::default(),
        })
    }

    fn add_section(&mut self, name: String, virtual_addr: u32, data: Box<[u8]>) {
        let section = OutEXESection {
            name,
            file_addr: 0,
            virtual_addr,
            data,
        };
        self.sections.push(section);
    }

    fn layout_sections(&mut self) {
        self.header.FileHeader.NumberOfSections = self.sections.len() as u16;
        let mut ofs = self.dos_header.len() as u32
            + size_of::<pe::IMAGE_NT_HEADERS32>() as u32
            + self.data_dirs.len() as u32 * size_of::<pe::IMAGE_DATA_DIRECTORY>() as u32
            + self.sections.len() as u32 * size_of::<pe::IMAGE_SECTION_HEADER>() as u32;
        for section in self.sections.iter_mut() {
            section.file_addr = ofs;
            ofs += section.data.len() as u32;
        }
    }

    fn write(&mut self, f: &mut std::fs::File) -> anyhow::Result<()> {
        f.write_all(self.dos_header.as_slice())?;
        f.write_all(Pod::as_bytes(&self.header))?;
        for dir in self.data_dirs.iter() {
            f.write_all(Pod::as_bytes(dir))?;
        }
        for section in self.sections.iter() {
            let header = pe::IMAGE_SECTION_HEADER {
                Name: format!("{}\0\0\0\0\0\0\0\0", section.name)[0..8]
                    .as_bytes()
                    .try_into()
                    .unwrap(),
                VirtualSize: section.data.len() as u32,
                VirtualAddress: section.virtual_addr,
                SizeOfRawData: section.data.len() as u32,
                PointerToRawData: section.file_addr,
                PointerToRelocations: 0,
                PointerToLinenumbers: 0,
                NumberOfRelocations: 0,
                NumberOfLinenumbers: 0,
                Characteristics: 0,
            };
            f.write_all(Pod::as_bytes(&header))?;
        }
        for section in self.sections.iter() {
            f.write_all(&*section.data)?;
        }
        Ok(())
    }
}

pub fn unpack(machine: &mut Machine, unpack_at: u32) -> anyhow::Result<()> {
    run_to_entry_point(machine, unpack_at)?;

    // Find the initial memory mapping where the exe was loaded.
    let image_base = kernel32::get_state(machine).image_base;
    let image_mapping = machine
        .memory
        .mappings
        .vec()
        .iter()
        .find(|m| m.addr == image_base)
        .expect("failed to find image base mapping");

    // Re-parse the in-memory PE header to gather info and rewrite the relevant parts.
    let mut out_exe = OutEXE::from_image(
        machine
            .memory
            .mem()
            .sub32(image_mapping.addr, image_mapping.size),
    )?;
    out_exe.header.OptionalHeader.AddressOfEntryPoint =
        machine.emu.x86.cpu().regs.eip - image_mapping.addr;

    // Find all the other memory mappings associated with the exe.
    let mappings = machine
        .memory
        .mappings
        .vec()
        .iter()
        .filter(|mapping| {
            mapping.module == image_mapping.module && mapping.addr != image_mapping.addr
        })
        .collect::<Vec<_>>();

    let image_end = mappings.iter().map(|m| m.addr + m.size).max().unwrap();
    let orig_iat = out_exe
        .data_dirs
        .get(pe::IMAGE_DIRECTORY_ENTRY::IMPORT as usize)
        .unwrap();
    let iat = construct_idt(
        machine,
        machine
            .memory
            .mem()
            .sub32_mut(image_mapping.addr, image_end - image_mapping.addr),
        orig_iat,
    );

    for mapping in mappings.iter() {
        out_exe.add_section(
            mapping.desc.clone(),
            mapping.addr - image_mapping.addr,
            machine
                .memory
                .mem()
                .sub32(mapping.addr, mapping.size)
                .into(),
        );
    }
    out_exe.add_section("rw32 iat".into(), image_end - image_mapping.addr, iat);

    out_exe.layout_sections();
    let iat_section = out_exe.sections.last().unwrap();

    for (i, dir) in out_exe.data_dirs.iter_mut().enumerate() {
        if dir.is_empty() {
            continue;
        }
        if i == pe::IMAGE_DIRECTORY_ENTRY::IMPORT as usize {
            *dir = pe::IMAGE_DATA_DIRECTORY {
                VirtualAddress: iat_section.virtual_addr,
                Size: iat_section.data.len() as u32,
            };
        } else if i == pe::IMAGE_DIRECTORY_ENTRY::BASERELOC as usize {
            *dir = pe::IMAGE_DATA_DIRECTORY::default();
        } else {
            log::info!("todo: update PE dir {}: {:#x?}", i, dir);
        }
    }

    let out_path = "unpacked.exe";
    {
        let mut f = std::fs::File::create(out_path)?;
        out_exe.write(&mut f)?;
    }
    log::info!("wrote exe to {}", out_path);

    Ok(())
}
