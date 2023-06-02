use iced_x86::Instruction;

use crate::{memory::Memory, registers::Flags, x86::CPU, Mem};

use super::helpers::*;

pub fn nop(_cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) {}

pub fn enterd_imm16_imm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    push(cpu, mem, cpu.regs.ebp);
    cpu.regs.ebp = cpu.regs.esp;
    cpu.regs.esp -= instr.immediate16() as u32;
}

pub fn leaved(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) {
    cpu.regs.esp = cpu.regs.ebp;
    cpu.regs.ebp = pop(cpu, mem);
}

pub fn pushd_r16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    // Pushing segment registers is subtle:
    // "If the source operand is a segment register (16 bits) and [...]
    // the operand size is 32-bits, either a zero-extended value is pushed on
    // the stack or the segment selector is written on the stack using a 16-bit move.
    // For the last case, all recent Intel Core and Intel Atom processors perform a
    // 16-bit move, leaving the upper portion of the stack location unmodified."
    // tldr it's a 32-bit move in any normal case.
    let x = cpu.regs.get16(instr.op0_register());
    push(cpu, mem, x as u32);
}

pub fn pushd_imm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    push(cpu, mem, instr.immediate8to32() as u32);
}

pub fn pushd_imm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    push(cpu, mem, instr.immediate32());
}

pub fn push_r32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    push(cpu, mem, cpu.regs.get32(instr.op0_register()));
}

pub fn push_rm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let value = op0_rm32(cpu, mem, instr);
    push(cpu, mem, value);
}

pub fn push_rm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let value = op0_rm16(cpu, mem, instr);
    push16(cpu, mem, value);
}

pub fn popd_r16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    // See discussion in pushd_r16.
    let value = pop(cpu, mem);
    cpu.regs.set16(instr.op0_register(), value as u16);
}

pub fn pop_rm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let value = pop(cpu, mem);
    let (x, _flags) = rm32(cpu, mem, instr);
    *x = value;
}

pub fn pop_rm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let value = pop16(cpu, mem);
    let (x, _flags) = rm16(cpu, mem, instr);
    *x = value;
}

pub fn mov_rm32_imm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let (x, _flags) = rm32(cpu, mem, instr);
    *x = instr.immediate32();
}

pub fn mov_rm32_r32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let value = cpu.regs.get32(instr.op1_register());
    let (x, _flags) = rm32(cpu, mem, instr);
    *x = value;
}

pub fn mov_r32_rm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let value = op1_rm32(cpu, mem, instr);
    cpu.regs.set32(instr.op0_register(), value);
}

pub fn mov_r16_rm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let value = op1_rm16(cpu, mem, instr);
    cpu.regs.set16(instr.op0_register(), value);
}

pub fn mov_rm16_r16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = cpu.regs.get16(instr.op1_register());
    let (x, _flags) = rm16(cpu, mem, instr);
    *x = y;
}

pub fn mov_rm16_imm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = instr.immediate16();
    let (x, _flags) = rm16(cpu, mem, instr);
    *x = y;
}

pub fn mov_r8_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let value = op1_rm8(cpu, mem, instr);
    cpu.regs.set8(instr.op0_register(), value);
}

pub fn mov_rm8_r8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = cpu.regs.get8(instr.op1_register());
    let (x, _flags) = rm8(cpu, mem, instr);
    *x = y;
}

pub fn mov_rm8_imm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let (x, _flags) = rm8(cpu, mem, instr);
    *x = y;
}

pub fn mov_moffs8_al(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let addr = x86_addr(cpu, instr);
    mem.put::<u8>(addr, cpu.regs.eax as u8);
}

pub fn mov_r32m16_sreg(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    // This weirdly is either a 16-bit or 32-write, so we must match to determine.
    let y = cpu.regs.get16(instr.op1_register());
    match instr.op0_kind() {
        iced_x86::OpKind::Register => cpu.regs.set32(instr.op0_register(), y as u32),
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(cpu, instr);
            mem.put::<u16>(addr, y)
        }
        _ => unimplemented!(),
    }
}

pub fn mov_sreg_r32m16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    // This weirdly is either a 16-bit or 32-write, so we must match to determine.
    // TODO: this is supposed to do segment selector validation stuff.
    let y = match instr.op1_kind() {
        iced_x86::OpKind::Register => cpu.regs.get32(instr.op1_register()) as u16,
        iced_x86::OpKind::Memory => mem.get::<u16>(x86_addr(cpu, instr)),
        _ => unimplemented!(),
    };
    cpu.regs.set16(instr.op0_register(), y);
}

pub fn movsx_r32_rm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = op1_rm16(cpu, mem, instr) as i16 as u32;
    let (x, _flags) = rm32(cpu, mem, instr);
    *x = y;
}

pub fn movsx_r32_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as i8 as u32;
    let (x, _flags) = rm32(cpu, mem, instr);
    *x = y;
}

pub fn movsx_r16_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as i8 as u16;
    let (x, _flags) = rm16(cpu, mem, instr);
    *x = y;
}

pub fn movzx_r32_rm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = op1_rm16(cpu, mem, instr) as u32;
    let (x, _flags) = rm32(cpu, mem, instr);
    *x = y;
}

pub fn movzx_r32_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as u32;
    let (x, _flags) = rm32(cpu, mem, instr);
    *x = y;
}

pub fn movzx_r16_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as u16;
    let (x, _flags) = rm16(cpu, mem, instr);
    *x = y;
}

pub fn cmovb_r32_rm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let (x, flags) = rm32(cpu, mem, instr);
    if flags.contains(Flags::CF) {
        *x = y;
    }
}

pub fn cmovne_r32_rm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let (x, flags) = rm32(cpu, mem, instr);
    if !flags.contains(Flags::ZF) {
        *x = y;
    }
}

pub fn xchg_rm32_r32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let r1 = instr.op1_register();
    let y = cpu.regs.get32(r1);
    let (x, _flags) = rm32(cpu, mem, instr);
    let tmp = *x;
    *x = y;
    cpu.regs.set32(r1, tmp);
}

pub fn xchg_rm8_r8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let r1 = instr.op1_register();
    let y = cpu.regs.get8(r1);
    let (x, _flags) = rm8(cpu, mem, instr);
    let tmp = *x;
    *x = y;
    cpu.regs.set8(r1, tmp);
}

pub fn cmpxchg_rm32_r32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = cpu.regs.get32(instr.op1_register());
    match instr.op0_kind() {
        iced_x86::OpKind::Register => todo!(),
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(cpu, instr);
            let x = mem.get::<u32>(addr);
            if cpu.regs.eax == x {
                mem.put::<u32>(addr, y);
            } else {
                cpu.regs.eax = y;
            }
        }
        _ => unreachable!(),
    };
}

pub fn lea_r32_m(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    // lea eax,[esp+10h]
    cpu.regs.set32(instr.op0_register(), x86_addr(cpu, instr));
}

pub fn seta_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let value = (!cpu.flags.contains(Flags::CF) && !cpu.flags.contains(Flags::ZF)) as u8;
    let (x, _flags) = rm8(cpu, mem, instr);
    *x = value;
}

pub fn setb_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let value = cpu.flags.contains(Flags::CF) as u8;
    let (x, _flags) = rm8(cpu, mem, instr);
    *x = value;
}

pub fn sete_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let value = cpu.flags.contains(Flags::ZF) as u8;
    let (x, _flags) = rm8(cpu, mem, instr);
    *x = value;
}

pub fn setne_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let value = !cpu.flags.contains(Flags::ZF) as u8;
    let (x, _flags) = rm8(cpu, mem, instr);
    *x = value;
}

pub fn setge_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let value = (cpu.flags.contains(Flags::ZF) == cpu.flags.contains(Flags::OF)) as u8;
    let (x, _flags) = rm8(cpu, mem, instr);
    *x = value;
}

pub fn pushad(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) {
    let esp = cpu.regs.esp;
    push(cpu, mem, cpu.regs.eax);
    push(cpu, mem, cpu.regs.ecx);
    push(cpu, mem, cpu.regs.edx);
    push(cpu, mem, cpu.regs.ebx);
    push(cpu, mem, esp);
    push(cpu, mem, cpu.regs.ebp);
    push(cpu, mem, cpu.regs.esi);
    push(cpu, mem, cpu.regs.edi);
}

pub fn popad(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) {
    cpu.regs.edi = pop(cpu, mem);
    cpu.regs.esi = pop(cpu, mem);
    cpu.regs.ebp = pop(cpu, mem);
    pop(cpu, mem); // ignore esp
    cpu.regs.ebx = pop(cpu, mem);
    cpu.regs.edx = pop(cpu, mem);
    cpu.regs.ecx = pop(cpu, mem);
    cpu.regs.eax = pop(cpu, mem);
}

pub fn pushfd(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) {
    push(cpu, mem, cpu.flags.bits());
}

pub fn pushfw(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) {
    let value = (cpu.flags.bits() & 0x0000_FFFF) as u16;
    push16(cpu, mem, value);
}

pub fn popfd(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) {
    let value = pop(cpu, mem);
    cpu.flags = Flags::from_bits(value).unwrap_or_else(|| panic!("invalid flags {:#x}", value));
}

pub fn popfw(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) {
    let prev = Flags::from_bits(cpu.flags.bits() & 0xFFFF_0000).unwrap();
    let new = Flags::from_bits(pop16(cpu, mem) as u32).unwrap();
    cpu.flags = prev.union(new);
}

pub fn sahf(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) {
    let ah = (cpu.regs.eax >> 8) as u8;
    cpu.flags = Flags::from_bits((cpu.flags.bits() & 0xFFFF_FF00) | ah as u32).unwrap();
}

pub fn salc(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) {
    cpu.regs.set8(
        iced_x86::Register::AL,
        if cpu.flags.contains(Flags::CF) {
            0xFF
        } else {
            0
        },
    );
}

pub fn std(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) {
    cpu.flags.insert(Flags::DF);
}

pub fn cld(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) {
    cpu.flags.remove(Flags::DF);
}

pub fn stc(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) {
    cpu.flags.insert(Flags::CF);
}

pub fn cmc(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) {
    cpu.flags.set(Flags::CF, !cpu.flags.contains(Flags::CF));
}

pub fn cwde(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) {
    cpu.regs.eax = cpu.regs.eax as i16 as i32 as u32;
}

pub fn cdq(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) {
    cpu.regs.edx = if cpu.regs.eax >> 31 == 0 {
        0
    } else {
        0xFFFF_FFFF
    };
}

pub fn int3(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) {
    cpu.state = Ok(false);
}

pub fn bswap_r32(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    let reg = instr.op0_register();
    let val = cpu.regs.get32(reg);
    cpu.regs.set32(
        reg,
        ((val >> 24) & 0xFF) << 0
            | ((val >> 16) & 0xFF) << 8
            | ((val >> 8) & 0xFF) << 16
            | ((val >> 0) & 0xFF) << 24,
    );
}

pub fn xlat_m8(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) {
    let addr = cpu.regs.ebx + (cpu.regs.eax & 0xFF);
    cpu.regs.set8(iced_x86::Register::AL, mem.get::<u8>(addr));
}
