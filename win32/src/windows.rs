use serde::Serialize;
use tsify::Tsify;

use crate::winapi::STDOUT_HFILE;
use crate::x86;
use crate::{pe, winapi, X86};

#[derive(Debug, Tsify, Serialize)]
pub struct Mapping {
    pub addr: u32,
    pub size: u32,
    pub desc: String,
}

pub struct AppState {
    // Address image was loaded at.
    pub image_base: u32,
    // Address of TEB (FS register).
    pub teb: u32,
    pub mappings: Vec<Mapping>,
}
impl AppState {
    pub fn new() -> Self {
        let mappings = vec![Mapping {
            addr: 0,
            size: x86::NULL_POINTER_REGION_SIZE,
            desc: "avoid null pointers".into(),
        }];
        AppState {
            image_base: 0,
            teb: 0,
            mappings,
        }
    }

    fn add_mapping(&mut self, mapping: Mapping) {
        let pos = self
            .mappings
            .iter()
            .position(|m| m.addr > mapping.addr)
            .unwrap_or(self.mappings.len());
        if pos > 0 {
            let prev = &self.mappings[pos - 1];
            assert!(prev.addr + prev.size <= mapping.addr);
        }
        if pos < self.mappings.len() {
            let next = &self.mappings[pos];
            assert!(mapping.addr + mapping.size <= next.addr);
        }
        self.mappings.insert(pos, mapping);
    }

    fn alloc(&mut self, size: u32, desc: String) -> &Mapping {
        let mut end = 0;
        for (pos, mapping) in self.mappings.iter().enumerate() {
            let space = mapping.addr - end;
            if space > size {
                self.mappings.insert(
                    pos,
                    Mapping {
                        addr: end,
                        size,
                        desc,
                    },
                );
                return &self.mappings[pos];
            }
            end = mapping.addr + mapping.size + (0x1000 - 1) & !(0x1000 - 1);
        }
        panic!("alloc of {size:x} failed");
    }
}

/// Set up TEB, PEB, and other process info.
/// The FS register points at the TEB (thread info), which points at the PEB (process info).
fn init_teb_peb(x86: &mut X86) {
    let mapping = x86.state.alloc(0x1000, "PEB/TEB".into());
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

    x86.state.teb = teb_addr;
}

pub fn load_exe(x86: &mut X86, buf: &[u8]) -> anyhow::Result<()> {
    let file = pe::parse(&buf)?;
    log::info!("{file:#x?}");

    let base = file.opt_header.image_base;
    x86.state.image_base = file.opt_header.image_base;
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
        x86.state.add_mapping(Mapping {
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
    let stack = x86.state.alloc(stack_size, "stack".into());
    let stack_end = stack.addr + stack.size - 4;
    x86.regs.esp = stack_end;
    x86.regs.ebp = stack_end;

    log::info!("mappings {:x?}", x86.state.mappings);

    let imports_data = &file.opt_header.data_directory[1];
    let imports = pe::parse_imports(
        &x86.mem[(base as usize)..],
        &x86.mem[(base + imports_data.virtual_address) as usize
            ..(base + imports_data.virtual_address + imports_data.size) as usize],
    )?;
    log::info!("imports {:x?}", imports);
    for (&addr, sym) in imports.iter() {
        x86.imports.insert(addr, winapi::resolve(sym));
    }

    let entry_point = base + file.opt_header.address_of_entry_point;
    x86.regs.eip = entry_point;

    Ok(())
}
