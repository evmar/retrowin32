use serde::Serialize;
use tsify::Tsify;

use crate::{pe, winapi, X86};

#[derive(Debug, Tsify, Serialize)]
pub struct Mapping {
    pub addr: u32,
    pub size: u32,
    pub desc: String,
}

pub struct AppState {
    pub image_base: u32,
    pub mappings: Vec<Mapping>,
}
impl AppState {
    pub fn new() -> Self {
        AppState {
            image_base: 0,
            mappings: Vec::new(),
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
}

pub fn load_exe(buf: &[u8]) -> anyhow::Result<X86> {
    let file = pe::parse(&buf)?;
    log::info!("{file:#x?}");

    let mut x86 = X86::new();
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

    // in debugger, initial stack was from 0xce000 + 0x12000
    // unclear where this comes from
    x86.regs.esp = 0xe0000;
    x86.regs.ebp = 0xe0000;

    Ok(x86)
}
