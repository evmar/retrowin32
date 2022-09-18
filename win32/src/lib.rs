mod debug;
mod pe;
mod reader;
mod x86;

pub use debug::*;
pub use x86::X86;

pub fn load_exe(x86: &mut X86, buf: &[u8]) -> anyhow::Result<()> {
    let file = pe::parse(&buf)?;
    println!("{file:#?}");

    let base = file.opt_header.image_base;
    x86.mem
        .resize((base + file.opt_header.size_of_image) as usize, 0);
    println!(
        "image base {base:#x}, image total size {:#x}",
        x86.mem.len()
    );
    for sec in file.sections.iter() {
        let src = sec.pointer_to_raw_data as usize;
        let dst = (base + sec.virtual_address) as usize;
        let size = sec.size_of_raw_data as usize;
        println!(
            "sec {:?} at {dst:#x} size {size:#x} from {src:#x}",
            sec.name
        );
        if !sec
            .characteristics
            .contains(pe::ImageSectionFlags::UNINITIALIZED_DATA)
        {
            x86.mem[dst..dst + size].copy_from_slice(&buf[src..(src + size)]);
        }
    }
    let entry_point = base + file.opt_header.address_of_entry_point;
    x86.regs.eip = entry_point;

    // in debugger, initial stack was from 0xce000 + 0x12000
    // unclear where this comes from
    x86.regs.esp = 0xe0000;

    Ok(())
}
