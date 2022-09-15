use anyhow::bail;

mod pe;
mod reader;

use pe::*;

pub struct Registers {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,

    pub esp: u32,
    pub ebp: u32,
    pub esi: u32,
    pub edi: u32,

    pub eip: u32,

    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,
}
impl Registers {
    fn new() -> Self {
        Registers {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
            esp: 0,
            ebp: 0,
            esi: 0,
            edi: 0,
            eip: 0,
            cs: 0,
            ds: 0,
            es: 0,
            fs: 0,
            gs: 0,
            ss: 0,
        }
    }
}

struct X86 {
    mem: Vec<u8>,
    regs: Registers,
}
impl X86 {
    fn new() -> Self {
        X86 {
            mem: Vec::new(),
            regs: Registers::new(),
        }
    }
}

fn load_exe(x86: &mut X86, buf: &[u8]) -> anyhow::Result<()> {
    let file = parse(&buf)?;
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
            .contains(ImageSectionFlags::UNINITIALIZED_DATA)
        {
            x86.mem[dst..dst + size].copy_from_slice(&buf[src..(src + size)]);
        }
    }
    let entry_point = base + file.opt_header.address_of_entry_point;
    x86.regs.eip = entry_point;
    Ok(())
}

fn run() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let exe = match args.as_slice() {
        [_, exe] => exe,
        _ => bail!("specify path"),
    };

    let mut x86 = X86::new();
    let buf = std::fs::read(exe)?;
    load_exe(&mut x86, &buf)?;

    let decoder = iced_x86::Decoder::with_ip(
        32,
        &x86.mem[x86.regs.eip as usize..],
        x86.regs.eip as u64,
        iced_x86::DecoderOptions::NONE,
    );

    let mut i = 0;
    for instruction in decoder {
        print!("{:08X} ", instruction.ip());
        let start_index = instruction.ip() as usize;
        let instr_bytes = &x86.mem[start_index..start_index + instruction.len()];
        for b in instr_bytes.iter() {
            print!("{:02x}", b);
        }
        if instr_bytes.len() < 10 {
            for _ in 0..10 - instr_bytes.len() {
                print!("  ");
            }
        }
        println!(" {}", instruction);
        i += 1;
        if i > 20 {
            break;
        }
    }

    Ok(())
}

fn main() {
    run().unwrap();
}
