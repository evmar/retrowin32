//! Functions for common behaviors across all operations.

use crate::x86::X86;

pub fn rm32_x(x86: &mut X86, instr: &iced_x86::Instruction, op: impl FnOnce(&mut X86, u32) -> u32) {
    match instr.op0_kind() {
        iced_x86::OpKind::Register => {
            let reg = instr.op0_register();
            let x = x86.regs.get32(reg);
            let value = op(x86, x);
            x86.regs.set32(reg, value);
        }
        iced_x86::OpKind::Memory => {
            let addr = x86.addr(instr);
            let x = x86.read_u32(addr);
            let value = op(x86, x);
            x86.write_u32(addr, value);
        }
        _ => unimplemented!(),
    }
}

pub fn rm16_x(x86: &mut X86, instr: &iced_x86::Instruction, op: impl FnOnce(&mut X86, u16) -> u16) {
    match instr.op0_kind() {
        iced_x86::OpKind::Register => {
            let reg = instr.op0_register();
            let x = x86.regs.get16(reg);
            let value = op(x86, x);
            x86.regs.set16(reg, value);
        }
        iced_x86::OpKind::Memory => {
            let addr = x86.addr(instr);
            let x = x86.read_u16(addr);
            let value = op(x86, x);
            x86.write_u16(addr, value);
        }
        _ => unimplemented!(),
    }
}

pub fn rm8_x(x86: &mut X86, instr: &iced_x86::Instruction, op: impl FnOnce(&mut X86, u8) -> u8) {
    match instr.op0_kind() {
        iced_x86::OpKind::Register => {
            let reg = instr.op0_register();
            let x = x86.regs.get8(reg);
            let value = op(x86, x);
            x86.regs.set8(reg, value);
        }
        iced_x86::OpKind::Memory => {
            let addr = x86.addr(instr);
            let x = x86.read_u8(addr);
            let value = op(x86, x);
            x86.write_u8(addr, value);
        }
        _ => unimplemented!(),
    }
}

pub fn op0_rm32(x86: &mut X86, instr: &iced_x86::Instruction) -> u32 {
    match instr.op0_kind() {
        iced_x86::OpKind::Register => x86.regs.get32(instr.op0_register()),
        iced_x86::OpKind::Memory => x86.read_u32(x86.addr(instr)),
        _ => unreachable!(),
    }
}

pub fn op0_rm16(x86: &mut X86, instr: &iced_x86::Instruction) -> u16 {
    match instr.op0_kind() {
        iced_x86::OpKind::Register => x86.regs.get16(instr.op0_register()),
        iced_x86::OpKind::Memory => x86.read_u16(x86.addr(instr)),
        _ => unreachable!(),
    }
}

pub fn op0_rm8(x86: &mut X86, instr: &iced_x86::Instruction) -> u8 {
    match instr.op0_kind() {
        iced_x86::OpKind::Register => x86.regs.get8(instr.op0_register()),
        iced_x86::OpKind::Memory => x86.read_u8(x86.addr(instr)),
        _ => unreachable!(),
    }
}

pub fn op1_rm32(x86: &mut X86, instr: &iced_x86::Instruction) -> u32 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => x86.regs.get32(instr.op1_register()),
        iced_x86::OpKind::Memory => x86.read_u32(x86.addr(instr)),
        _ => unreachable!(),
    }
}

pub fn op1_rm16(x86: &mut X86, instr: &iced_x86::Instruction) -> u16 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => x86.regs.get16(instr.op1_register()),
        iced_x86::OpKind::Memory => x86.read_u16(x86.addr(instr)),
        _ => unreachable!(),
    }
}

pub fn op1_rm8(x86: &mut X86, instr: &iced_x86::Instruction) -> u8 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => x86.regs.get8(instr.op1_register()),
        iced_x86::OpKind::Memory => x86.read_u8(x86.addr(instr)),
        _ => unreachable!(),
    }
}
