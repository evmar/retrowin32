use std::collections::HashMap;

use crate::winapi::kernel32::STDOUT_HFILE;
use crate::{pe, winapi, X86};

/// Set up TEB, PEB, and other process info.
/// The FS register points at the TEB (thread info), which points at the PEB (process info).
fn init_teb_peb(x86: &mut X86) {
    let mapping = x86
        .state
        .kernel32
        .alloc(0x1000, "PEB/TEB".into(), &mut x86.mem);
    // Fill region with garbage so it's clearer when we access something we don't intend to.
    x86.mem[mapping.addr as usize..(mapping.addr + mapping.size) as usize].fill(0xde);

    let peb_addr = mapping.addr;
    let params_addr = mapping.addr + 0x100;
    let teb_addr = params_addr + 0x100;

    // PEB
    x86.write_u32(peb_addr + 0x10, params_addr);

    // RTL_USER_PROCESS_PARAMETERS
    // x86.write_u32(params_addr + 0x10, console_handle);
    // x86.write_u32(params_addr + 0x14, console_flags);
    // x86.write_u32(params_addr + 0x18, stdin);
    x86.write_u32(params_addr + 0x1c, STDOUT_HFILE);

    // TEB
    x86.write_u32(teb_addr + 0x18, teb_addr); // Confusing: it points to itself.
    x86.write_u32(teb_addr + 0x30, peb_addr);

    x86.state.kernel32.teb = teb_addr;
}

pub fn load_exe(x86: &mut X86, buf: &[u8]) -> anyhow::Result<HashMap<u32, String>> {
    let file = pe::parse(&buf)?;
    log::info!("{file:#x?}");

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

    init_teb_peb(x86);

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

    log::info!("mappings {:x?}", x86.state.kernel32.mappings);

    let imports_data = &file.opt_header.data_directory[1];
    let mut x = 0;
    let mut labels: HashMap<u32, String> = HashMap::new();
    pe::parse_imports(
        &mut x86.mem[base as usize..],
        imports_data.virtual_address as usize,
        |dll, sym, iat_addr| {
            // "Resolving" a given import just means recording that when we jump to
            // some particular magic address we should run some faked function.
            x += 1;
            // "fake IAT" => "FIAT" => "F1A7"
            let addr = 0xF1A7_0000 | x;
            x86.imports.insert(addr, winapi::resolve(dll, &sym));
            labels.insert(base + iat_addr, format!("{}@IAT", sym));
            labels.insert(addr, sym);
            addr
        },
    )?;

    let entry_point = base + file.opt_header.address_of_entry_point;
    x86.regs.eip = entry_point;

    Ok(labels)
}
