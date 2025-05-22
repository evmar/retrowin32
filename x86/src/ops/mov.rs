use super::helpers::*;
use crate::{registers::Flags, x86::CPU};
use iced_x86::{Instruction, Register};
use memory::{Extensions, ExtensionsMut, Mem};

/// mov: Move
pub fn mov_rm32_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm32(cpu, mem, instr);
    x.set(instr.immediate32());
}

/// mov: Move
pub fn mov_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = cpu.regs.get32(instr.op1_register());
    let x = rm32(cpu, mem, instr);
    x.set(value);
}

/// mov: Move
pub fn mov_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = op1_rm32(cpu, mem, instr);
    cpu.regs.set32(instr.op0_register(), value);
}

/// mov: Move
pub fn mov_r16_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = op1_rm16(cpu, mem, instr);
    cpu.regs.set16(instr.op0_register(), value);
}

/// mov: Move
pub fn mov_rm16_r16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get16(instr.op1_register());
    let x = rm16(cpu, mem, instr);
    x.set(y);
}

/// mov: Move
pub fn mov_rm16_imm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate16();
    let x = rm16(cpu, mem, instr);
    x.set(y);
}

/// mov: Move
pub fn mov_r8_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = op1_rm8(cpu, mem, instr);
    cpu.regs.set8(instr.op0_register(), value);
}

/// mov: Move
pub fn mov_rm8_r8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(instr.op1_register());
    let x = rm8(cpu, mem, instr);
    x.set(y);
}

/// mov: Move
pub fn mov_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm8(cpu, mem, instr);
    x.set(y);
}

/// mov: Move
pub fn mov_moffs8_al(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let addr = x86_addr(cpu, instr);
    mem.put_pod::<u8>(addr, cpu.regs.get8(Register::AL));
}

/// mov: Move
pub fn mov_r32m16_sreg(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    // This weirdly is either a 16-bit or 32-write, so we must match to determine.
    let y = cpu.regs.get16(instr.op1_register());
    match instr.op0_kind() {
        iced_x86::OpKind::Register => cpu.regs.set32(instr.op0_register(), y as u32),
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(cpu, instr);
            mem.put_pod::<u16>(addr, y)
        }
        _ => unimplemented!(),
    }
}

/// mov: Move
pub fn mov_sreg_r32m16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    // This weirdly is either a 16-bit or 32-write, so we must match to determine.
    // TODO: this is supposed to do segment selector validation stuff.
    let y = match instr.op1_kind() {
        iced_x86::OpKind::Register => cpu.regs.get32(instr.op1_register()) as u16,
        iced_x86::OpKind::Memory => mem.get_pod::<u16>(x86_addr(cpu, instr)),
        _ => unimplemented!(),
    };
    cpu.regs.set16(instr.op0_register(), y);
}

/// movsx: Move With Sign-Extension
pub fn movsx_r32_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm16(cpu, mem, instr) as i16 as u32;
    let x = rm32(cpu, mem, instr);
    x.set(y);
}

/// movsx: Move With Sign-Extension
pub fn movsx_r32_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as i8 as u32;
    let x = rm32(cpu, mem, instr);
    x.set(y);
}

/// movsx: Move With Sign-Extension
pub fn movsx_r16_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as i8 as u16;
    let x = rm16(cpu, mem, instr);
    x.set(y);
}

/// movzx: Move With Zero-Extend
pub fn movzx_r32_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm16(cpu, mem, instr) as u32;
    let x = rm32(cpu, mem, instr);
    x.set(y);
}

/// movzx: Move With Zero-Extend
pub fn movzx_r32_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as u32;
    let x = rm32(cpu, mem, instr);
    x.set(y);
}

/// movzx: Move With Zero-Extend
pub fn movzx_r16_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as u16;
    let x = rm16(cpu, mem, instr);
    x.set(y);
}

/// cmov: Conditional Move
pub fn cmovb_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    if cpu.flags.contains(Flags::CF) {
        x.set(y);
    }
}

/// cmov: Conditional Move
pub fn cmove_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    if cpu.flags.contains(Flags::ZF) {
        x.set(y);
    }
}

/// cmov: Conditional Move
pub fn cmovne_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    if !cpu.flags.contains(Flags::ZF) {
        x.set(y);
    }
}
