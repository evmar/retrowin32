use std::collections::HashMap;

use crate::{pe, winapi, X86};

pub fn load_exe(x86: &mut X86, buf: &[u8]) -> anyhow::Result<HashMap<u32, String>> {
    let file = pe::parse(&buf)?;

    let base = file.opt_header.image_base;
    x86.state.kernel32.image_base = file.opt_header.image_base;
    x86.mem
        .resize((base + file.opt_header.size_of_image) as usize, 0);
    log::info!(
        "image base {base:#x}, image total size {:#x}",
        x86.mem.len()
    );
    for sec in file.sections {
        let src = sec.pointer_to_raw_data as usize;
        let dst = (base + sec.virtual_address) as usize;
        let size = sec.size_of_raw_data as usize;
        if !sec
            .characteristics
            .contains(pe::ImageSectionFlags::UNINITIALIZED_DATA)
        {
            x86.mem[dst..dst + size].copy_from_slice(&buf[src..(src + size)]);
        }
        x86.state.kernel32.add_mapping(winapi::kernel32::Mapping {
            addr: dst as u32,
            size: size as u32,
            desc: format!("{} ({:?})", sec.name, sec.characteristics),
        });
    }

    winapi::kernel32::init_teb_peb(x86);

    let mut stack_size = file.opt_header.size_of_stack_reserve;
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
        .alloc(stack_size, "stack".into(), &mut x86.mem);
    let stack_end = stack.addr + stack.size - 4;
    x86.regs.esp = stack_end;
    x86.regs.ebp = stack_end;

    let imports_data = &file.opt_header.data_directory[1];
    let mut labels: HashMap<u32, String> = HashMap::new();
    pe::parse_imports(
        &mut x86.mem[base as usize..],
        imports_data.virtual_address as usize,
        |dll, sym, iat_addr| {
            labels.insert(base + iat_addr, format!("{}@IAT", sym));

            let entry = match winapi::resolve(dll, &sym) {
                Some(f) => Ok(f),
                None => Err(format!("unimplemented: {dll}!{sym}")),
            };
            let addr = x86.shims.add(entry);

            labels.insert(addr, sym);

            addr
        },
    )?;

    let entry_point = base + file.opt_header.address_of_entry_point;
    x86.regs.eip = entry_point;

    Ok(labels)
}
