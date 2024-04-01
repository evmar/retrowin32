//! Disassembler producing serde/JSON for use in displaying code in the debugger.

use iced_x86::{Formatter, IntelFormatter};
use memory::Mem;
use std::fmt::Write;
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

pub fn disassemble(mem: Mem, addr: u32, limit: usize) -> Vec<Instruction> {
    if addr >= mem.len() {
        return Vec::new();
    }
    let decoder = iced_x86::Decoder::with_ip(
        32,
        mem.slice(addr..).as_slice_todo(),
        addr as u64,
        iced_x86::DecoderOptions::NONE,
    );
    let mut formatter = IntelFormatter::new();

    let mut instrs = Vec::new();
    for instruction in decoder.into_iter().take(limit) {
        let start_index = instruction.ip() as u32;
        let instr_bytes = mem.sub(start_index, instruction.len() as u32);
        let mut bytes = String::new();
        for &b in instr_bytes.as_slice_todo().iter() {
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
    }
    instrs
}
