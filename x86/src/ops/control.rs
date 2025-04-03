use crate::{registers::Flags, x86::CPU, Register};
use iced_x86::Instruction;
use memory::Mem;

use super::helpers::*;

pub fn call(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    push(cpu, mem, cpu.regs.eip);
    cpu.jmp(mem, instr.near_branch32())
}

pub fn call_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    // call dword ptr [addr]
    let target = rm32(cpu, mem, instr).get();
    push(cpu, mem, cpu.regs.eip);
    cpu.jmp(mem, target)
}

pub fn retnd(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let addr = pop(cpu, mem);
    cpu.jmp(mem, addr)
}

pub fn retnd_imm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let addr = pop(cpu, mem);
    cpu.jmp(mem, addr);
    *cpu.regs.get32_mut(Register::ESP) += instr.immediate16() as u32;
}

pub fn jmp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    cpu.jmp(mem, instr.near_branch32())
}

pub fn jmp_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let target = rm32(cpu, mem, instr).get();
    cpu.jmp(mem, target)
}

pub fn ja(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::CF) && !cpu.flags.contains(Flags::ZF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn jae(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::CF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn jb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::CF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn jbe(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::CF) || cpu.flags.contains(Flags::ZF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn je(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::ZF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn jecxz(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.regs.get32(Register::ECX) == 0 {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn jne(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::ZF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn jno(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::OF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn jns(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::SF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn jp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    // Note: the decoder ensures we only see a jp directly after a sahf,
    // so we know the PF flag directly depends on ax.
    let pf = ((cpu.regs.get32(Register::EAX) >> 10) & 1) == 1;
    if pf {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn jg(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::ZF)
        && (cpu.flags.contains(Flags::SF) == cpu.flags.contains(Flags::OF))
    {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn jge(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::SF) == cpu.flags.contains(Flags::OF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn jle(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::ZF)
        || (cpu.flags.contains(Flags::SF) != cpu.flags.contains(Flags::OF))
    {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn jl(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::SF) != cpu.flags.contains(Flags::OF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn js(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::SF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

pub fn loop_(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let ecx = cpu.regs.get32_mut(Register::ECX);
    *ecx -= 1;
    if *ecx != 0 {
        cpu.jmp(mem, instr.near_branch32());
    }
}
