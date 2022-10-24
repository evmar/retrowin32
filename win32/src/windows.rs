use std::collections::HashMap;

use crate::{pe, winapi, x86::X86};

pub fn load_exe(x86: &mut X86, buf: &[u8]) -> anyhow::Result<HashMap<u32, String>> {
    let file = pe::parse(&buf)?;

    let base = file.opt_header.ImageBase.get();
    x86.state.kernel32.image_base = base;
    x86.mem
        .resize((base + file.opt_header.SizeOfImage.get()) as usize, 0);
    log::info!(
        "image base {base:#x}, image total size {:#x}",
        x86.mem.len()
    );
    for sec in file.sections {
        let src = sec.PointerToRawData as usize;
        let dst = (base + sec.VirtualAddress) as usize;
        let size = sec.SizeOfRawData as usize;
        let flags = sec.characteristics()?;
        if !flags.contains(pe::ImageSectionFlags::UNINITIALIZED_DATA) {
            x86.mem[dst..dst + size].copy_from_slice(&buf[src..(src + size)]);
        }
        x86.state.kernel32.add_mapping(winapi::kernel32::Mapping {
            addr: dst as u32,
            size: size as u32,
            desc: format!("{:?} ({:?})", sec.name(), flags),
            flags,
        });
    }

    x86.state.kernel32.init(&mut x86.mem);

    let mut stack_size = file.opt_header.SizeOfStackReserve.get();
    // Zig reserves 16mb stacks, just truncate for now.
    if stack_size > 1 << 20 {
        log::warn!(
            "requested {}mb stack reserve, using 32kb instead",
            stack_size / (1 << 20)
        );
        stack_size = 32 << 10;
    }
    let stack = x86
        .state
        .kernel32
        .new_mapping(stack_size, "stack".into(), &mut x86.mem);
    let stack_end = stack.addr + stack.size - 4;
    x86.regs.esp = stack_end;
    x86.regs.ebp = stack_end;

    const IMAGE_DIRECTORY_ENTRY_IMPORT: usize = 1;
    let imports_data = &file.opt_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT];
    let mut labels: HashMap<u32, String> = HashMap::new();
    pe::parse_imports(
        &mut x86.mem[base as usize..],
        imports_data.VirtualAddress.get() as usize,
        |dll, sym, iat_addr| {
            labels.insert(base + iat_addr, format!("{}@IAT", sym));

            let entry = match winapi::resolve(dll, &sym) {
                Some(f) => Ok(f),
                None => Err(format!("unimplemented: {dll}!{sym}")),
            };
            let addr = x86.shims.add(entry);

            labels.insert(addr, sym.to_string());

            addr
        },
    )?;

    const IMAGE_DIRECTORY_ENTRY_RESOURCE: usize = 2;
    let res_data = &file.opt_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_RESOURCE];
    x86.state.user32.resources_base = res_data.VirtualAddress.get();

    let entry_point = base + file.opt_header.AddressOfEntryPoint.get();
    x86.regs.eip = entry_point;

    Ok(labels)
}
