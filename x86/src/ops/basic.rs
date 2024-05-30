use crate::{registers::Flags, x86::CPU, CPUState};
use iced_x86::{Instruction, Register};
use memory::{Extensions, Mem};

use super::helpers::*;

pub fn nop(_cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {}

pub fn enterd_imm16_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    push(cpu, mem, cpu.regs.get32(Register::EBP));
    cpu.regs.set32(Register::EBP, cpu.regs.get32(Register::ESP));
    *cpu.regs.get32_mut(Register::ESP) -= instr.immediate16() as u32;
}

pub fn leaved(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    cpu.regs.set32(Register::ESP, cpu.regs.get32(Register::EBP));
    let ebp = pop(cpu, mem);
    cpu.regs.set32(Register::EBP, ebp);
}

pub fn pushd_r16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
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

pub fn pushd_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    push(cpu, mem, instr.immediate8to32() as u32);
}

pub fn pushd_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    push(cpu, mem, instr.immediate32());
}

pub fn push_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    push(cpu, mem, cpu.regs.get32(instr.op0_register()));
}

pub fn push_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = rm32(cpu, mem, instr).get();
    push(cpu, mem, value);
}

pub fn push_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = rm16(cpu, mem, instr).get();
    push16(cpu, mem, value);
}

pub fn popd_r16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    // See discussion in pushd_r16.
    let value = pop(cpu, mem);
    cpu.regs.set16(instr.op0_register(), value as u16);
}

pub fn pop_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = pop(cpu, mem);
    let x = rm32(cpu, mem, instr);
    x.set(value);
}

pub fn pop_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = pop16(cpu, mem);
    let x = rm16(cpu, mem, instr);
    x.set(value);
}

pub fn mov_rm32_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm32(cpu, mem, instr);
    x.set(instr.immediate32());
}

pub fn mov_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = cpu.regs.get32(instr.op1_register());
    let x = rm32(cpu, mem, instr);
    x.set(value);
}

pub fn mov_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = op1_rm32(cpu, mem, instr);
    cpu.regs.set32(instr.op0_register(), value);
}

pub fn mov_r16_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = op1_rm16(cpu, mem, instr);
    cpu.regs.set16(instr.op0_register(), value);
}

pub fn mov_rm16_r16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get16(instr.op1_register());
    let x = rm16(cpu, mem, instr);
    x.set(y);
}

pub fn mov_rm16_imm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate16();
    let x = rm16(cpu, mem, instr);
    x.set(y);
}

pub fn mov_r8_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = op1_rm8(cpu, mem, instr);
    cpu.regs.set8(instr.op0_register(), value);
}

pub fn mov_rm8_r8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(instr.op1_register());
    let x = rm8(cpu, mem, instr);
    x.set(y);
}

pub fn mov_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm8(cpu, mem, instr);
    x.set(y);
}

pub fn mov_moffs8_al(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let addr = x86_addr(cpu, instr);
    mem.put::<u8>(addr, cpu.regs.get8(Register::AL));
}

pub fn mov_r32m16_sreg(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
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

pub fn movsx_r32_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm16(cpu, mem, instr) as i16 as u32;
    let x = rm32(cpu, mem, instr);
    x.set(y);
}

pub fn movsx_r32_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as i8 as u32;
    let x = rm32(cpu, mem, instr);
    x.set(y);
}

pub fn movsx_r16_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as i8 as u16;
    let x = rm16(cpu, mem, instr);
    x.set(y);
}

pub fn movzx_r32_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm16(cpu, mem, instr) as u32;
    let x = rm32(cpu, mem, instr);
    x.set(y);
}

pub fn movzx_r32_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as u32;
    let x = rm32(cpu, mem, instr);
    x.set(y);
}

pub fn movzx_r16_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as u16;
    let x = rm16(cpu, mem, instr);
    x.set(y);
}

pub fn cmovb_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    if cpu.flags.contains(Flags::CF) {
        x.set(y);
    }
}

pub fn cmovne_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    if !cpu.flags.contains(Flags::ZF) {
        x.set(y);
    }
}

pub fn xchg_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let r1 = instr.op1_register();
    let y = cpu.regs.get32(r1);
    let x = rm32(cpu, mem, instr);
    let tmp = x.get();
    x.set(y);
    cpu.regs.set32(r1, tmp);
}

pub fn xchg_rm16_r16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let r1 = instr.op1_register();
    let y = cpu.regs.get16(r1);
    let x = rm16(cpu, mem, instr);
    let tmp = x.get();
    x.set(y);
    cpu.regs.set16(r1, tmp);
}

pub fn xchg_rm8_r8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let r1 = instr.op1_register();
    let y = cpu.regs.get8(r1);
    let x = rm8(cpu, mem, instr);
    let tmp = x.get();
    x.set(y);
    cpu.regs.set8(r1, tmp);
}

pub fn cmpxchg_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get32(instr.op1_register());
    match instr.op0_kind() {
        iced_x86::OpKind::Register => todo!(),
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(cpu, instr);
            let x = mem.get_pod::<u32>(addr);
            if cpu.regs.get32(Register::EAX) == x {
                cpu.flags.insert(Flags::ZF);
                mem.put::<u32>(addr, y);
            } else {
                cpu.flags.remove(Flags::ZF);
                cpu.regs.set32(Register::EAX, y);
            }
        }
        _ => unreachable!(),
    };
}

pub fn cmpxchg8b_m64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let addr = x86_addr(cpu, instr);
    let m64 = mem.get_pod::<u64>(addr);
    let test = get_edx_eax(cpu);
    if test == m64 {
        cpu.flags.insert(Flags::ZF);
        let val =
            ((cpu.regs.get32(Register::ECX) as u64) << 32) | (cpu.regs.get32(Register::EBX) as u64);
        mem.put::<u64>(addr, val);
    } else {
        cpu.flags.remove(Flags::ZF);
        set_edx_eax(cpu, m64);
    }
}

pub fn lea_r32_m(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    // lea eax,[esp+10h]
    cpu.regs.set32(instr.op0_register(), x86_addr(cpu, instr));
}

pub fn seta_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (!cpu.flags.contains(Flags::CF) && !cpu.flags.contains(Flags::ZF)) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setae_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = !cpu.flags.contains(Flags::CF) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setb_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = cpu.flags.contains(Flags::CF) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setbe_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (cpu.flags.contains(Flags::CF) || cpu.flags.contains(Flags::ZF)) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn sete_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = cpu.flags.contains(Flags::ZF) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setl_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (cpu.flags.contains(Flags::SF) != cpu.flags.contains(Flags::OF)) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setle_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (cpu.flags.contains(Flags::ZF)
        || (cpu.flags.contains(Flags::SF) != cpu.flags.contains(Flags::OF))) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setne_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = !cpu.flags.contains(Flags::ZF) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setge_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (cpu.flags.contains(Flags::SF) == cpu.flags.contains(Flags::OF)) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn pushad(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let esp = cpu.regs.get32(Register::ESP); // get before any pushes

    push(cpu, mem, cpu.regs.get32(Register::EAX));
    push(cpu, mem, cpu.regs.get32(Register::ECX));
    push(cpu, mem, cpu.regs.get32(Register::EDX));
    push(cpu, mem, cpu.regs.get32(Register::EBX));
    push(cpu, mem, esp);
    push(cpu, mem, cpu.regs.get32(Register::EBP));
    push(cpu, mem, cpu.regs.get32(Register::ESI));
    push(cpu, mem, cpu.regs.get32(Register::EDI));
}

pub fn popad(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let edi = pop(cpu, mem);
    cpu.regs.set32(Register::EDI, edi);
    let esi = pop(cpu, mem);
    cpu.regs.set32(Register::ESI, esi);
    let ebp = pop(cpu, mem);
    cpu.regs.set32(Register::EBP, ebp);
    pop(cpu, mem); // ignore esp
    let ebx = pop(cpu, mem);
    cpu.regs.set32(Register::EBX, ebx);
    let edx = pop(cpu, mem);
    cpu.regs.set32(Register::EDX, edx);
    let ecx = pop(cpu, mem);
    cpu.regs.set32(Register::ECX, ecx);
    let eax = pop(cpu, mem);
    cpu.regs.set32(Register::EAX, eax);
}

pub fn pushfd(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    push(cpu, mem, cpu.flags.bits());
}

pub fn pushfw(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let value = (cpu.flags.bits() & 0x0000_FFFF) as u16;
    push16(cpu, mem, value);
}

pub fn popfd(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let value = pop(cpu, mem);
    cpu.flags = Flags::from_bits(value).unwrap_or_else(|| panic!("invalid flags {:#x}", value));
}

pub fn popfw(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let prev = Flags::from_bits(cpu.flags.bits() & 0xFFFF_0000).unwrap();
    let new = Flags::from_bits(pop16(cpu, mem) as u32).unwrap();
    cpu.flags = prev.union(new);
}

pub fn sahf(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let ah = cpu.regs.get8(Register::AH);
    cpu.flags = Flags::from_bits((cpu.flags.bits() & 0xFFFF_FF00) | ah as u32).unwrap();
}

pub fn salc(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.regs.set8(
        iced_x86::Register::AL,
        if cpu.flags.contains(Flags::CF) {
            0xFF
        } else {
            0
        },
    );
}

pub fn std(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.insert(Flags::DF);
}

pub fn cld(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.remove(Flags::DF);
}

pub fn stc(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.insert(Flags::CF);
}

pub fn clc(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.remove(Flags::CF);
}

pub fn cmc(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.set(Flags::CF, !cpu.flags.contains(Flags::CF));
}

pub fn cwde(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let value = cpu.regs.get16(Register::AX) as i16 as i32;
    cpu.regs.set32(Register::EAX, value as u32);
}

pub fn cdq(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let edx = if cpu.regs.get32(Register::EAX) >> 31 == 0 {
        0
    } else {
        0xFFFF_FFFF
    };
    cpu.regs.set32(Register::EDX, edx);
}

pub fn int3(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    log::warn!("debugger interrupt");
    cpu.state = CPUState::Blocked(None);
    cpu.regs.eip -= 1;
}

pub fn bswap_r32(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
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

pub fn xlat_m8(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let addr = cpu.regs.get32(Register::EBX) + (cpu.regs.get8(Register::AL) as u32);
    cpu.regs.set8(Register::AL, mem.get_pod::<u8>(addr));
}

pub fn bts_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    let mask = 1u32 << y;
    cpu.flags.set(Flags::CF, x.get() & mask != 0);
    x.set(x.get() | mask);
}

pub fn tzcnt_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    let count = y.trailing_zeros();
    cpu.flags.set(Flags::CF, count == 32);
    cpu.flags.set(Flags::ZF, count == 0);
    x.set(count);
}
