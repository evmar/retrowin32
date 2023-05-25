use iced_x86::Instruction;

use crate::{memory::Memory, registers::Flags, x86::CPU, Mem, StepError, StepResult};

use super::helpers::*;

pub fn nop(_cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    Ok(())
}

pub fn enterd_imm16_imm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    push(cpu, mem, cpu.regs.ebp);
    cpu.regs.ebp = cpu.regs.esp;
    cpu.regs.esp -= instr.immediate16() as u32;
    Ok(())
}

pub fn leaved(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    cpu.regs.esp = cpu.regs.ebp;
    cpu.regs.ebp = pop(cpu, mem);
    Ok(())
}

pub fn pushd_r16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    // Pushing segment registers is subtle:
    // "If the source operand is a segment register (16 bits) and [...]
    // the operand size is 32-bits, either a zero-extended value is pushed on
    // the stack or the segment selector is written on the stack using a 16-bit move.
    // For the last case, all recent Intel Core and Intel Atom processors perform a
    // 16-bit move, leaving the upper portion of the stack location unmodified."
    // tldr it's a 32-bit move in any normal case.
    let x = cpu.regs.get16(instr.op0_register());
    push(cpu, mem, x as u32);
    Ok(())
}

pub fn pushd_imm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    push(cpu, mem, instr.immediate8to32() as u32);
    Ok(())
}

pub fn pushd_imm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    push(cpu, mem, instr.immediate32());
    Ok(())
}

pub fn push_r32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    push(cpu, mem, cpu.regs.get32(instr.op0_register()));
    Ok(())
}

pub fn push_rm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let value = op0_rm32(cpu, mem, instr);
    push(cpu, mem, value);
    Ok(())
}

pub fn push_rm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let value = op0_rm16(cpu, mem, instr);
    push16(cpu, mem, value);
    Ok(())
}

pub fn popd_r16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    // See discussion in pushd_r16.
    let value = pop(cpu, mem);
    cpu.regs.set16(instr.op0_register(), value as u16);
    Ok(())
}

pub fn pop_rm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let value = pop(cpu, mem);
    let (x, _flags) = rm32(cpu, mem, instr);
    *x = value;
    Ok(())
}

pub fn pop_rm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let value = pop16(cpu, mem);
    let (x, _flags) = rm16(cpu, mem, instr);
    *x = value;
    Ok(())
}

pub fn mov_rm32_imm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let (x, _flags) = rm32(cpu, mem, instr);
    *x = instr.immediate32();
    Ok(())
}

pub fn mov_rm32_r32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let value = cpu.regs.get32(instr.op1_register());
    let (x, _flags) = rm32(cpu, mem, instr);
    *x = value;
    Ok(())
}

pub fn mov_r32_rm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let value = op1_rm32(cpu, mem, instr);
    cpu.regs.set32(instr.op0_register(), value);
    Ok(())
}

pub fn mov_r16_rm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let value = op1_rm16(cpu, mem, instr);
    cpu.regs.set16(instr.op0_register(), value);
    Ok(())
}

pub fn mov_rm16_r16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = cpu.regs.get16(instr.op1_register());
    let (x, _flags) = rm16(cpu, mem, instr);
    *x = y;
    Ok(())
}

pub fn mov_rm16_imm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate16();
    let (x, _flags) = rm16(cpu, mem, instr);
    *x = y;
    Ok(())
}

pub fn mov_r8_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let value = op1_rm8(cpu, mem, instr);
    cpu.regs.set8(instr.op0_register(), value);
    Ok(())
}

pub fn mov_rm8_r8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = cpu.regs.get8(instr.op1_register());
    let (x, _flags) = rm8(cpu, mem, instr);
    *x = y;
    Ok(())
}

pub fn mov_rm8_imm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8();
    let (x, _flags) = rm8(cpu, mem, instr);
    *x = y;
    Ok(())
}

pub fn mov_moffs8_al(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let addr = x86_addr(cpu, instr);
    mem.write_u8(addr, cpu.regs.eax as u8);
    Ok(())
}

pub fn mov_r32m16_sreg(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    // This weirdly is either a 16-bit or 32-write, so we must match to determine.
    let y = cpu.regs.get16(instr.op1_register());
    match instr.op0_kind() {
        iced_x86::OpKind::Register => cpu.regs.set32(instr.op0_register(), y as u32),
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(cpu, instr);
            mem.write_u16(addr, y)
        }
        _ => unimplemented!(),
    }
    Ok(())
}

pub fn mov_sreg_r32m16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    // This weirdly is either a 16-bit or 32-write, so we must match to determine.
    // TODO: this is supposed to do segment selector validation stuff.
    let y = match instr.op1_kind() {
        iced_x86::OpKind::Register => cpu.regs.get32(instr.op1_register()) as u16,
        iced_x86::OpKind::Memory => mem.read_u16(x86_addr(cpu, instr)),
        _ => unimplemented!(),
    };
    cpu.regs.set16(instr.op0_register(), y);
    Ok(())
}

pub fn movsx_r32_rm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm16(cpu, mem, instr) as i16 as u32;
    let (x, _flags) = rm32(cpu, mem, instr);
    *x = y;
    Ok(())
}

pub fn movsx_r32_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm8(cpu, mem, instr) as i8 as u32;
    let (x, _flags) = rm32(cpu, mem, instr);
    *x = y;
    Ok(())
}

pub fn movsx_r16_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm8(cpu, mem, instr) as i8 as u16;
    let (x, _flags) = rm16(cpu, mem, instr);
    *x = y;
    Ok(())
}

pub fn movzx_r32_rm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm16(cpu, mem, instr) as u32;
    let (x, _flags) = rm32(cpu, mem, instr);
    *x = y;
    Ok(())
}

pub fn movzx_r32_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm8(cpu, mem, instr) as u32;
    let (x, _flags) = rm32(cpu, mem, instr);
    *x = y;
    Ok(())
}

pub fn movzx_r16_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm8(cpu, mem, instr) as u16;
    let (x, _flags) = rm16(cpu, mem, instr);
    *x = y;
    Ok(())
}

pub fn cmovb_r32_rm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm32(cpu, mem, instr);
    let (x, flags) = rm32(cpu, mem, instr);
    if flags.contains(Flags::CF) {
        *x = y;
    }
    Ok(())
}

pub fn cmovne_r32_rm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm32(cpu, mem, instr);
    let (x, flags) = rm32(cpu, mem, instr);
    if !flags.contains(Flags::ZF) {
        *x = y;
    }
    Ok(())
}

pub fn xchg_rm32_r32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let r1 = instr.op1_register();
    let y = cpu.regs.get32(r1);
    let (x, _flags) = rm32(cpu, mem, instr);
    let tmp = *x;
    *x = y;
    cpu.regs.set32(r1, tmp);
    Ok(())
}

pub fn xchg_rm8_r8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let r1 = instr.op1_register();
    let y = cpu.regs.get8(r1);
    let (x, _flags) = rm8(cpu, mem, instr);
    let tmp = *x;
    *x = y;
    cpu.regs.set8(r1, tmp);
    Ok(())
}

pub fn cmpxchg_rm32_r32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = cpu.regs.get32(instr.op1_register());
    match instr.op0_kind() {
        iced_x86::OpKind::Register => todo!(),
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(cpu, instr);
            let x = mem.read_u32(addr);
            if cpu.regs.eax == x {
                mem.write_u32(addr, y);
            } else {
                cpu.regs.eax = y;
            }
        }
        _ => unreachable!(),
    };
    Ok(())
}

pub fn lea_r32_m(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    // lea eax,[esp+10h]
    cpu.regs.set32(instr.op0_register(), x86_addr(cpu, instr));
    Ok(())
}

pub fn seta_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let value = (!cpu.flags.contains(Flags::CF) && !cpu.flags.contains(Flags::ZF)) as u8;
    let (x, _flags) = rm8(cpu, mem, instr);
    *x = value;
    Ok(())
}

pub fn setb_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let value = cpu.flags.contains(Flags::CF) as u8;
    let (x, _flags) = rm8(cpu, mem, instr);
    *x = value;
    Ok(())
}

pub fn sete_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let value = cpu.flags.contains(Flags::ZF) as u8;
    let (x, _flags) = rm8(cpu, mem, instr);
    *x = value;
    Ok(())
}

pub fn setne_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let value = !cpu.flags.contains(Flags::ZF) as u8;
    let (x, _flags) = rm8(cpu, mem, instr);
    *x = value;
    Ok(())
}

pub fn setge_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let value = (cpu.flags.contains(Flags::ZF) == cpu.flags.contains(Flags::OF)) as u8;
    let (x, _flags) = rm8(cpu, mem, instr);
    *x = value;
    Ok(())
}

pub fn pushad(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let esp = cpu.regs.esp;
    push(cpu, mem, cpu.regs.eax);
    push(cpu, mem, cpu.regs.ecx);
    push(cpu, mem, cpu.regs.edx);
    push(cpu, mem, cpu.regs.ebx);
    push(cpu, mem, esp);
    push(cpu, mem, cpu.regs.ebp);
    push(cpu, mem, cpu.regs.esi);
    push(cpu, mem, cpu.regs.edi);
    Ok(())
}

pub fn popad(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    cpu.regs.edi = pop(cpu, mem);
    cpu.regs.esi = pop(cpu, mem);
    cpu.regs.ebp = pop(cpu, mem);
    pop(cpu, mem); // ignore esp
    cpu.regs.ebx = pop(cpu, mem);
    cpu.regs.edx = pop(cpu, mem);
    cpu.regs.ecx = pop(cpu, mem);
    cpu.regs.eax = pop(cpu, mem);
    Ok(())
}

pub fn pushfd(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    push(cpu, mem, cpu.flags.bits());
    Ok(())
}

pub fn pushfw(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let value = (cpu.flags.bits() & 0x0000_FFFF) as u16;
    push16(cpu, mem, value);
    Ok(())
}

pub fn popfd(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let value = pop(cpu, mem);
    cpu.flags = Flags::from_bits(value).unwrap_or_else(|| panic!("invalid flags {:#x}", value));
    Ok(())
}

pub fn popfw(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let prev = Flags::from_bits(cpu.flags.bits() & 0xFFFF_0000).unwrap();
    let new = Flags::from_bits(pop16(cpu, mem) as u32).unwrap();
    cpu.flags = prev.union(new);
    Ok(())
}

pub fn sahf(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let ah = (cpu.regs.eax >> 8) as u8;
    cpu.flags = Flags::from_bits((cpu.flags.bits() & 0xFFFF_FF00) | ah as u32).unwrap();
    Ok(())
}

pub fn salc(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    cpu.regs.set8(
        iced_x86::Register::AL,
        if cpu.flags.contains(Flags::CF) {
            0xFF
        } else {
            0
        },
    );
    Ok(())
}

pub fn std(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    cpu.flags.insert(Flags::DF);
    Ok(())
}

pub fn cld(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    cpu.flags.remove(Flags::DF);
    Ok(())
}

pub fn stc(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    cpu.flags.insert(Flags::CF);
    Ok(())
}

pub fn cmc(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    cpu.flags.set(Flags::CF, !cpu.flags.contains(Flags::CF));
    Ok(())
}

pub fn cwde(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    cpu.regs.eax = cpu.regs.eax as i16 as i32 as u32;
    Ok(())
}

pub fn cdq(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    cpu.regs.edx = if cpu.regs.eax >> 31 == 0 {
        0
    } else {
        0xFFFF_FFFF
    };
    Ok(())
}

pub fn int3(_cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    Err(StepError::Interrupt)
}

pub fn bswap_r32(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let reg = instr.op0_register();
    let val = cpu.regs.get32(reg);
    cpu.regs.set32(
        reg,
        ((val >> 24) & 0xFF) << 0
            | ((val >> 16) & 0xFF) << 8
            | ((val >> 8) & 0xFF) << 16
            | ((val >> 0) & 0xFF) << 24,
    );
    Ok(())
}

pub fn xlat_m8(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let addr = cpu.regs.ebx + (cpu.regs.eax & 0xFF);
    cpu.regs.set8(iced_x86::Register::AL, *mem.view::<u8>(addr));
    Ok(())
}
