//! Functions for common behaviors across all operations.

use crate::{registers::Flags, x86::X86, Memory, StepError, StepResult, NULL_POINTER_REGION_SIZE};

pub fn read_u64(x86: &X86, addr: u32) -> u64 {
    if addr < NULL_POINTER_REGION_SIZE {
        panic!("null pointer read at {addr:#x}");
    }
    *x86.mem.view::<u64>(addr)
}

pub fn write_u64(x86: &mut X86, addr: u32, value: u64) {
    if addr < NULL_POINTER_REGION_SIZE {
        panic!("null pointer read at {addr:#x}");
    }
    *x86.mem.view_mut::<u64>(addr) = value;
}

// TODO: maybe there are no 64-bit memory reads needed (?)
pub fn rm64_x(x86: &mut X86, instr: &iced_x86::Instruction, op: impl FnOnce(&mut X86, u64) -> u64) {
    match instr.op0_kind() {
        iced_x86::OpKind::Register => {
            let reg = instr.op0_register();
            let x = x86.regs.get64(reg);
            let value = op(x86, x);
            x86.regs.set64(reg, value);
        }
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(x86, instr);
            let x = read_u64(x86, addr);
            let value = op(x86, x);
            write_u64(x86, addr, value);
        }
        _ => unimplemented!(),
    }
}

pub fn rm32<'a>(x86: &'a mut X86, instr: &iced_x86::Instruction) -> (&'a mut u32, &'a mut Flags) {
    let dest = match instr.op0_kind() {
        iced_x86::OpKind::Register => {
            let reg = instr.op0_register();
            x86.regs.get32_mut(reg)
        }
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(x86, instr);
            x86.mem.view_mut::<u32>(addr)
        }
        _ => unimplemented!(),
    };
    (dest, &mut x86.flags)
}

pub fn rm16<'a>(x86: &'a mut X86, instr: &iced_x86::Instruction) -> (&'a mut u16, &'a mut Flags) {
    let dest = match instr.op0_kind() {
        iced_x86::OpKind::Register => {
            let reg = instr.op0_register();
            x86.regs.get16_mut(reg)
        }
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(x86, instr);
            x86.mem.view_mut::<u16>(addr)
        }
        _ => unimplemented!(),
    };
    (dest, &mut x86.flags)
}

pub fn rm8<'a>(x86: &'a mut X86, instr: &iced_x86::Instruction) -> (&'a mut u8, &'a mut Flags) {
    let dest = match instr.op0_kind() {
        iced_x86::OpKind::Register => {
            let reg = instr.op0_register();
            x86.regs.get8_mut(reg)
        }
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(x86, instr);
            x86.mem.view_mut::<u8>(addr)
        }
        _ => unimplemented!(),
    };
    (dest, &mut x86.flags)
}

pub fn op0_rm32(x86: &mut X86, instr: &iced_x86::Instruction) -> u32 {
    match instr.op0_kind() {
        iced_x86::OpKind::Register => x86.regs.get32(instr.op0_register()),
        iced_x86::OpKind::Memory => x86.read_u32(x86_addr(x86, instr)),
        _ => unreachable!(),
    }
}

pub fn op0_rm16(x86: &mut X86, instr: &iced_x86::Instruction) -> u16 {
    match instr.op0_kind() {
        iced_x86::OpKind::Register => x86.regs.get16(instr.op0_register()),
        iced_x86::OpKind::Memory => x86.read_u16(x86_addr(x86, instr)),
        _ => unreachable!(),
    }
}

pub fn op0_rm8(x86: &mut X86, instr: &iced_x86::Instruction) -> u8 {
    match instr.op0_kind() {
        iced_x86::OpKind::Register => x86.regs.get8(instr.op0_register()),
        iced_x86::OpKind::Memory => x86.read_u8(x86_addr(x86, instr)),
        _ => unreachable!(),
    }
}

pub fn op1_rm32(x86: &mut X86, instr: &iced_x86::Instruction) -> u32 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => x86.regs.get32(instr.op1_register()),
        iced_x86::OpKind::Memory => x86.read_u32(x86_addr(x86, instr)),
        _ => unreachable!(),
    }
}

pub fn op1_rm16(x86: &mut X86, instr: &iced_x86::Instruction) -> u16 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => x86.regs.get16(instr.op1_register()),
        iced_x86::OpKind::Memory => x86.read_u16(x86_addr(x86, instr)),
        _ => unreachable!(),
    }
}

pub fn op1_rm8(x86: &mut X86, instr: &iced_x86::Instruction) -> u8 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => x86.regs.get8(instr.op1_register()),
        iced_x86::OpKind::Memory => x86.read_u8(x86_addr(x86, instr)),
        _ => unreachable!(),
    }
}

/// Push a u32 on the x86 stack.
pub fn push(x86: &mut X86, value: u32) {
    x86.regs.esp -= 4;
    x86.write_u32(x86.regs.esp, value);
}

/// Push a u16 on the x86 stack.
pub fn push16(x86: &mut X86, value: u16) {
    x86.regs.esp -= 2;
    x86.write_u16(x86.regs.esp, value);
}

/// Pop a u32 from the x86 stack.
pub fn pop(x86: &mut X86) -> u32 {
    let value = x86.read_u32(x86.regs.esp);
    x86.regs.esp += 4;
    value
}

/// Pop a u16 from the x86 stack.
pub fn pop16(x86: &mut X86) -> u16 {
    let value = x86.read_u16(x86.regs.esp);
    x86.regs.esp += 2;
    value
}

/// Compute the address found in instructions that reference memory, e.g.
///   mov [eax+03h],...
pub fn x86_addr(x86: &X86, instr: &iced_x86::Instruction) -> u32 {
    // TODO: see comments on regs.fs_addr.
    let seg = match instr.segment_prefix() {
        iced_x86::Register::FS => x86.regs.fs_addr,
        _ => 0,
    };

    let base = if instr.memory_base() != iced_x86::Register::None {
        x86.regs.get32(instr.memory_base())
    } else {
        0
    };
    let index = if instr.memory_index() != iced_x86::Register::None {
        x86.regs
            .get32(instr.memory_index())
            .wrapping_mul(instr.memory_index_scale())
    } else {
        0
    };
    // In general these operations aren't written to wrap, but in some cases
    // the components are negative which is implemented in two's complement by
    // a wrapping add.
    seg.wrapping_add(base)
        .wrapping_add(index)
        .wrapping_add(instr.memory_displacement32())
}

pub fn x86_jmp(x86: &mut X86, addr: u32) -> StepResult<()> {
    if addr < 0x1000 {
        return Err(StepError::Error("jmp to null page".into()));
    }
    x86.regs.eip = addr;
    Ok(())
}
