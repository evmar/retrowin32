use iced_x86::Instruction;

use crate::{memory::Memory, registers::Flags, x86::X86, StepError, StepResult};

use super::helpers::*;

pub fn nop(_x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    Ok(())
}

pub fn enterd_imm16_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    push(x86, x86.regs.ebp);
    x86.regs.ebp = x86.regs.esp;
    x86.regs.esp -= instr.immediate16() as u32;
    Ok(())
}

pub fn leaved(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    x86.regs.esp = x86.regs.ebp;
    x86.regs.ebp = pop(x86);
    Ok(())
}

pub fn pushd_r16(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    // Pushing segment registers is subtle:
    // "If the source operand is a segment register (16 bits) and [...]
    // the operand size is 32-bits, either a zero-extended value is pushed on
    // the stack or the segment selector is written on the stack using a 16-bit move.
    // For the last case, all recent Intel Core and Intel Atom processors perform a
    // 16-bit move, leaving the upper portion of the stack location unmodified."
    // tldr it's a 32-bit move in any normal case.
    let x = x86.regs.get16(instr.op0_register());
    push(x86, x as u32);
    Ok(())
}

pub fn pushd_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    push(x86, instr.immediate8to32() as u32);
    Ok(())
}

pub fn pushd_imm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    push(x86, instr.immediate32());
    Ok(())
}

pub fn push_r32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    push(x86, x86.regs.get32(instr.op0_register()));
    Ok(())
}

pub fn push_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let value = op0_rm32(x86, instr);
    push(x86, value);
    Ok(())
}

pub fn push_rm16(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let value = op0_rm16(x86, instr);
    push16(x86, value);
    Ok(())
}

pub fn popd_r16(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    // See discussion in pushd_r16.
    let value = pop(x86);
    x86.regs.set16(instr.op0_register(), value as u16);
    Ok(())
}

pub fn pop_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let value = pop(x86);
    let (x, _flags) = rm32(x86, instr);
    *x = value;
    Ok(())
}

pub fn pop_rm16(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let value = pop16(x86);
    let (x, _flags) = rm16(x86, instr);
    *x = value;
    Ok(())
}

pub fn mov_rm32_imm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let (x, _flags) = rm32(x86, instr);
    *x = instr.immediate32();
    Ok(())
}

pub fn mov_rm32_r32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let value = x86.regs.get32(instr.op1_register());
    let (x, _flags) = rm32(x86, instr);
    *x = value;
    Ok(())
}

pub fn mov_r32_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let value = op1_rm32(x86, instr);
    x86.regs.set32(instr.op0_register(), value);
    Ok(())
}

pub fn mov_r16_rm16(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let value = op1_rm16(x86, instr);
    x86.regs.set16(instr.op0_register(), value);
    Ok(())
}

pub fn mov_rm16_r16(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = x86.regs.get16(instr.op1_register());
    let (x, _flags) = rm16(x86, instr);
    *x = y;
    Ok(())
}

pub fn mov_rm16_imm16(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate16();
    let (x, _flags) = rm16(x86, instr);
    *x = y;
    Ok(())
}

pub fn mov_r8_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let value = op1_rm8(x86, instr);
    x86.regs.set8(instr.op0_register(), value);
    Ok(())
}

pub fn mov_rm8_r8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = x86.regs.get8(instr.op1_register());
    let (x, _flags) = rm8(x86, instr);
    *x = y;
    Ok(())
}

pub fn mov_rm8_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8();
    let (x, _flags) = rm8(x86, instr);
    *x = y;
    Ok(())
}

pub fn mov_moffs8_al(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let addr = x86_addr(x86, instr);
    x86.mem.write_u8(addr, x86.regs.eax as u8);
    Ok(())
}

pub fn mov_r32m16_sreg(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    // This weirdly is either a 16-bit or 32-write, so we must match to determine.
    let y = x86.regs.get16(instr.op1_register());
    match instr.op0_kind() {
        iced_x86::OpKind::Register => x86.regs.set32(instr.op0_register(), y as u32),
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(x86, instr);
            x86.mem.write_u16(addr, y)
        }
        _ => unimplemented!(),
    }
    Ok(())
}

pub fn mov_sreg_r32m16(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    // This weirdly is either a 16-bit or 32-write, so we must match to determine.
    // TODO: this is supposed to do segment selector validation stuff.
    let y = match instr.op1_kind() {
        iced_x86::OpKind::Register => x86.regs.get32(instr.op1_register()) as u16,
        iced_x86::OpKind::Memory => x86.mem.read_u16(x86_addr(x86, instr)),
        _ => unimplemented!(),
    };
    x86.regs.set16(instr.op0_register(), y);
    Ok(())
}

pub fn movsx_r32_rm16(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm16(x86, instr) as i16 as u32;
    let (x, _flags) = rm32(x86, instr);
    *x = y;
    Ok(())
}

pub fn movsx_r32_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm8(x86, instr) as i8 as u32;
    let (x, _flags) = rm32(x86, instr);
    *x = y;
    Ok(())
}

pub fn movsx_r16_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm8(x86, instr) as i8 as u16;
    let (x, _flags) = rm16(x86, instr);
    *x = y;
    Ok(())
}

pub fn movzx_r32_rm16(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm16(x86, instr) as u32;
    let (x, _flags) = rm32(x86, instr);
    *x = y;
    Ok(())
}

pub fn movzx_r32_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm8(x86, instr) as u32;
    let (x, _flags) = rm32(x86, instr);
    *x = y;
    Ok(())
}

pub fn movzx_r16_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm8(x86, instr) as u16;
    let (x, _flags) = rm16(x86, instr);
    *x = y;
    Ok(())
}

pub fn cmovb_r32_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm32(x86, instr);
    let (x, flags) = rm32(x86, instr);
    if flags.contains(Flags::CF) {
        *x = y;
    }
    Ok(())
}

pub fn cmovne_r32_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm32(x86, instr);
    let (x, flags) = rm32(x86, instr);
    if !flags.contains(Flags::ZF) {
        *x = y;
    }
    Ok(())
}

pub fn xchg_rm32_r32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let r1 = instr.op1_register();
    let y = x86.regs.get32(r1);
    let (x, _flags) = rm32(x86, instr);
    let tmp = *x;
    *x = y;
    x86.regs.set32(r1, tmp);
    Ok(())
}

pub fn xchg_rm8_r8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let r1 = instr.op1_register();
    let y = x86.regs.get8(r1);
    let (x, _flags) = rm8(x86, instr);
    let tmp = *x;
    *x = y;
    x86.regs.set8(r1, tmp);
    Ok(())
}

pub fn cmpxchg_rm32_r32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = x86.regs.get32(instr.op1_register());
    match instr.op0_kind() {
        iced_x86::OpKind::Register => todo!(),
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(x86, instr);
            let x = x86.mem.read_u32(addr);
            if x86.regs.eax == x {
                x86.mem.write_u32(addr, y);
            } else {
                x86.regs.eax = y;
            }
        }
        _ => unreachable!(),
    };
    Ok(())
}

pub fn lea_r32_m(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    // lea eax,[esp+10h]
    x86.regs.set32(instr.op0_register(), x86_addr(x86, instr));
    Ok(())
}

pub fn seta_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let value = (!x86.flags.contains(Flags::CF) && !x86.flags.contains(Flags::ZF)) as u8;
    let (x, _flags) = rm8(x86, instr);
    *x = value;
    Ok(())
}

pub fn setb_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let value = x86.flags.contains(Flags::CF) as u8;
    let (x, _flags) = rm8(x86, instr);
    *x = value;
    Ok(())
}

pub fn sete_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let value = x86.flags.contains(Flags::ZF) as u8;
    let (x, _flags) = rm8(x86, instr);
    *x = value;
    Ok(())
}

pub fn setne_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let value = !x86.flags.contains(Flags::ZF) as u8;
    let (x, _flags) = rm8(x86, instr);
    *x = value;
    Ok(())
}

pub fn setge_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let value = (x86.flags.contains(Flags::ZF) == x86.flags.contains(Flags::OF)) as u8;
    let (x, _flags) = rm8(x86, instr);
    *x = value;
    Ok(())
}

pub fn pushad(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    let esp = x86.regs.esp;
    push(x86, x86.regs.eax);
    push(x86, x86.regs.ecx);
    push(x86, x86.regs.edx);
    push(x86, x86.regs.ebx);
    push(x86, esp);
    push(x86, x86.regs.ebp);
    push(x86, x86.regs.esi);
    push(x86, x86.regs.edi);
    Ok(())
}

pub fn popad(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    x86.regs.edi = pop(x86);
    x86.regs.esi = pop(x86);
    x86.regs.ebp = pop(x86);
    pop(x86); // ignore esp
    x86.regs.ebx = pop(x86);
    x86.regs.edx = pop(x86);
    x86.regs.ecx = pop(x86);
    x86.regs.eax = pop(x86);
    Ok(())
}

pub fn pushfd(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    push(x86, x86.flags.bits());
    Ok(())
}

pub fn pushfw(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    let value = (x86.flags.bits() & 0x0000_FFFF) as u16;
    push16(x86, value);
    Ok(())
}

pub fn popfd(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    let value = pop(x86);
    x86.flags = Flags::from_bits(value).unwrap_or_else(|| panic!("invalid flags {:#x}", value));
    Ok(())
}

pub fn popfw(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    let prev = Flags::from_bits(x86.flags.bits() & 0xFFFF_0000).unwrap();
    let new = Flags::from_bits(pop16(x86) as u32).unwrap();
    x86.flags = prev.union(new);
    Ok(())
}

pub fn sahf(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    let ah = (x86.regs.eax >> 8) as u8;
    x86.flags = Flags::from_bits((x86.flags.bits() & 0xFFFF_FF00) | ah as u32).unwrap();
    Ok(())
}

pub fn salc(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    x86.regs.set8(
        iced_x86::Register::AL,
        if x86.flags.contains(Flags::CF) {
            0xFF
        } else {
            0
        },
    );
    Ok(())
}

pub fn std(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    x86.flags.insert(Flags::DF);
    Ok(())
}

pub fn cld(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    x86.flags.remove(Flags::DF);
    Ok(())
}

pub fn stc(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    x86.flags.insert(Flags::CF);
    Ok(())
}

pub fn cmc(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    x86.flags.set(Flags::CF, !x86.flags.contains(Flags::CF));
    Ok(())
}

pub fn cwde(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    x86.regs.eax = x86.regs.eax as i16 as i32 as u32;
    Ok(())
}

pub fn cdq(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    x86.regs.edx = if x86.regs.eax >> 31 == 0 {
        0
    } else {
        0xFFFF_FFFF
    };
    Ok(())
}

pub fn int3(_x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    Err(StepError::Interrupt)
}

pub fn bswap_r32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let reg = instr.op0_register();
    let val = x86.regs.get32(reg);
    x86.regs.set32(
        reg,
        ((val >> 24) & 0xFF) << 0
            | ((val >> 16) & 0xFF) << 8
            | ((val >> 8) & 0xFF) << 16
            | ((val >> 0) & 0xFF) << 24,
    );
    Ok(())
}

pub fn xlat_m8(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    let addr = x86.regs.ebx + (x86.regs.eax & 0xFF);
    x86.regs
        .set8(iced_x86::Register::AL, *x86.mem.view::<u8>(addr));
    Ok(())
}
