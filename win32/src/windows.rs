use std::collections::HashMap;

use crate::{pe, winapi, x86::Machine};

pub fn load_exe(
    machine: &mut Machine,
    buf: &[u8],
    cmdline: String,
) -> anyhow::Result<HashMap<u32, String>> {
    let file = pe::parse(&buf)?;

    let base = file.opt_header.ImageBase;
    machine.state.kernel32.image_base = base;
    machine
        .x86
        .mem
        .resize((base + file.opt_header.SizeOfImage) as usize, 0);

    for sec in file.sections {
        let src = sec.PointerToRawData as usize;
        let dst = (base + sec.VirtualAddress) as usize;
        let size = sec.SizeOfRawData as usize;
        let flags = sec.characteristics()?;
        if !flags.contains(pe::ImageSectionFlags::UNINITIALIZED_DATA) {
            machine.x86.mem[dst..dst + size].copy_from_slice(&buf[src..(src + size)]);
        }
        machine
            .state
            .kernel32
            .mappings
            .add(winapi::kernel32::Mapping {
                addr: dst as u32,
                size: size as u32,
                desc: format!("{:?} ({:?})", sec.name(), flags),
                flags,
            });
    }

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

    const IMAGE_DIRECTORY_ENTRY_IMPORT: usize = 1;
    let imports_data = &file.opt_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT];
    let mut labels: HashMap<u32, String> = HashMap::new();
    pe::parse_imports(
        &mut machine.x86.mem[base as usize..],
        imports_data.VirtualAddress as usize,
        |dll, sym, iat_addr| {
            labels.insert(base + iat_addr, format!("{}@IAT", sym));

            let entry = match winapi::resolve(dll, &sym) {
                Some(f) => Ok(f),
                None => Err(format!("unimplemented: {dll}!{sym}")),
            };
            let addr = machine.x86.shims.add(entry);

            labels.insert(addr, sym.to_string());

            addr
        },
    )?;

    const IMAGE_DIRECTORY_ENTRY_RESOURCE: usize = 2;
    let res_data = &file.opt_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_RESOURCE];
    machine.state.user32.resources_base = res_data.VirtualAddress;

    let entry_point = base + file.opt_header.AddressOfEntryPoint;
    machine.x86.regs.eip = entry_point;

    Ok(labels)
}
