//! Disassembler producing serde/JSON for use in displaying code in the debugger.

#![allow(non_snake_case)] // work around tsify generating lints

use iced_x86::{Formatter, IntelFormatter};
use memory::{Extensions, Mem};
use std::fmt::Write;

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[derive(serde::Serialize)]
pub struct CodePart {
    pub kind: String,
    pub text: String,
}

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[derive(serde::Serialize)]
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
        mem.slice(addr..),
        addr as u64,
        iced_x86::DecoderOptions::NONE,
    );
    let mut formatter = IntelFormatter::new();

    let mut instrs = Vec::new();
    for instruction in decoder.into_iter().take(limit) {
        let start_index = instruction.ip() as u32;
        let instr_bytes = mem.sub32(start_index, instruction.len() as u32);
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
    }
    instrs
}

/// Print the current CPU state and relevant context to stdout, useful when printf debugging.
/// eip_offset offsets eip backwards, useful when eip has already been advanced beyond an
/// instruction.  Call like `dump_state(..., instr.len())` to include the current instruction.
#[allow(unused)]
pub fn dump_state(cpu: &crate::CPU, mem: Mem, eip_offset: usize) {
    use iced_x86::Register::*;
    println!(
        "\
        eax {eax:08x}    eip {eip:08x}\n\
        ecx {ecx:08x}    esp {esp:08x}\n\
        edx {edx:08x}    ebp {ebp:08x}\n\
        ebx {ebx:08x}",
        eax = cpu.regs.get32(EAX),
        ecx = cpu.regs.get32(ECX),
        edx = cpu.regs.get32(EDX),
        ebx = cpu.regs.get32(EBX),
        eip = cpu.regs.eip,
        esp = cpu.regs.get32(ESP),
        ebp = cpu.regs.get32(EBP),
    );
    let instrs = disassemble(mem, cpu.regs.eip - eip_offset as u32, 5);
    for instr in instrs {
        print!("{:08x} {:10} ", instr.addr, instr.bytes);
        for part in &instr.code {
            print!("{}", part.text);
        }
        println!();
    }
}
