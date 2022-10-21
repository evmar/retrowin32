use std::fmt::Write;

use iced_x86::{Formatter, IntelFormatter};
use tsify::Tsify;

#[derive(Tsify, serde::Serialize)]
pub struct CodePart {
    pub kind: String,
    pub text: String,
}

#[derive(Tsify, serde::Serialize)]
pub struct Instruction {
    pub addr: u32,
    pub bytes: String,
    pub code: Vec<CodePart>,
    pub ops: Vec<String>,
}

struct FormatterOutput {
    code: Vec<CodePart>,
}
impl iced_x86::FormatterOutput for FormatterOutput {
    fn write(&mut self, text: &str, kind: iced_x86::FormatterTextKind) {
        self.code.push(CodePart {
            kind: format!("{:?}", kind),
            text: text.to_string(),
        })
    }
}

pub fn disassemble(mem: &[u8], addr: u32) -> Vec<Instruction> {
    if addr as usize >= mem.len() {
        return Vec::new();
    }
    let decoder = iced_x86::Decoder::with_ip(
        32,
        &mem[addr as usize..],
        addr as u64,
        iced_x86::DecoderOptions::NONE,
    );
    let mut formatter = IntelFormatter::new();

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

        let mut output = FormatterOutput { code: Vec::new() };
        formatter.format(&instruction, &mut output);

        instrs.push(Instruction {
            addr: instruction.ip() as u32,
            bytes,
            code: output.code,
            ops: instruction.op_kinds().map(|k| format!("{:?}", k)).collect(),
        });
        i += 1;
        if i > 20 {
            break;
        }
    }
    instrs
}
