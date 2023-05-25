use iced_x86::Instruction;

use crate::{registers::Flags, x86::CPU, Mem};

use super::helpers::*;

pub fn call(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    push(cpu, mem, cpu.regs.eip);
    x86_jmp(cpu, instr.near_branch32())
}

pub fn call_rm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    // call dword ptr [addr]
    let target = op0_rm32(cpu, mem, instr);
    push(cpu, mem, cpu.regs.eip);
    x86_jmp(cpu, target)
}

pub fn retnd(cpu: &mut CPU, mem: &mut Mem, _instr: &Instruction) {
    let addr = pop(cpu, mem);
    x86_jmp(cpu, addr)
}

pub fn retnd_imm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let addr = pop(cpu, mem);
    x86_jmp(cpu, addr);
    cpu.regs.esp += instr.immediate16() as u32;
}

pub fn jmp(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    x86_jmp(cpu, instr.near_branch32())
}

pub fn jmp_rm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let target = op0_rm32(cpu, mem, instr);
    x86_jmp(cpu, target)
}

pub fn ja(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::CF) && !cpu.flags.contains(Flags::ZF) {
        x86_jmp(cpu, instr.near_branch32());
    }
}

pub fn jae(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::CF) {
        x86_jmp(cpu, instr.near_branch32());
    }
}

pub fn jb(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::CF) {
        x86_jmp(cpu, instr.near_branch32());
    }
}

pub fn jbe(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::CF) || cpu.flags.contains(Flags::ZF) {
        x86_jmp(cpu, instr.near_branch32());
    }
}

pub fn je(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::ZF) {
        x86_jmp(cpu, instr.near_branch32());
    }
}

pub fn jecxz(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    if cpu.regs.ecx == 0 {
        x86_jmp(cpu, instr.near_branch32());
    }
}

pub fn jne(cpu: &mut CPU, _: &mut Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::ZF) {
        x86_jmp(cpu, instr.near_branch32());
    }
}

pub fn jns(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::SF) {
        x86_jmp(cpu, instr.near_branch32());
    }
}

pub fn jg(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::ZF)
        && (cpu.flags.contains(Flags::SF) == cpu.flags.contains(Flags::OF))
    {
        x86_jmp(cpu, instr.near_branch32());
    }
}

pub fn jge(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::SF) == cpu.flags.contains(Flags::OF) {
        x86_jmp(cpu, instr.near_branch32());
    }
}

pub fn jle(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::ZF)
        || (cpu.flags.contains(Flags::SF) != cpu.flags.contains(Flags::OF))
    {
        x86_jmp(cpu, instr.near_branch32());
    }
}

pub fn jl(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::SF) != cpu.flags.contains(Flags::OF) {
        x86_jmp(cpu, instr.near_branch32());
    }
}

pub fn js(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::SF) {
        x86_jmp(cpu, instr.near_branch32());
    }
}

pub fn loop_(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) {
    cpu.regs.ecx -= 1;
    if cpu.regs.ecx != 0 {
        x86_jmp(cpu, instr.near_branch32());
    }
}
