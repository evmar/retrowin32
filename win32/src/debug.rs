use std::fmt::Write;

#[derive(serde::Serialize)]
pub struct Instruction {
    pub addr: u32,
    pub bytes: String,
    pub code: String,
}

pub fn disassemble(mem: &[u8], addr: u32) -> Vec<Instruction> {
    let decoder = iced_x86::Decoder::with_ip(
        32,
        &mem[addr as usize..],
        addr as u64,
        iced_x86::DecoderOptions::NONE,
    );

    let mut instrs = Vec::new();
    let mut i = 0;
    for instruction in decoder {
        print!("{:08X} ", instruction.ip());
        let start_index = instruction.ip() as usize;
        let instr_bytes = &mem[start_index..start_index + instruction.len()];
        let mut bytes = String::new();
        for &b in instr_bytes.iter() {
            write!(&mut bytes, "{:02x}", b).unwrap();
        }
        instrs.push(Instruction {
            addr: instruction.ip() as u32,
            bytes,
            code: instruction.to_string(),
        });
        i += 1;
        if i > 20 {
            break;
        }
    }
    instrs
}
