use crate::{Register, registers::Flags, x86::CPU};
use iced_x86::Instruction;
use memory::Mem;

use super::helpers::*;

/// call: Call Procedure
pub fn call(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    push(cpu, mem, cpu.regs.eip);
    cpu.jmp(mem, instr.near_branch32())
}

/// call: Call Procedure
pub fn call_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    // call dword ptr [addr]
    let target = rm32(cpu, mem, instr).get();
    push(cpu, mem, cpu.regs.eip);
    cpu.jmp(mem, target)
}

/// retn: Return From Procedure
pub fn retnd(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let addr = pop(cpu, mem);
    cpu.jmp(mem, addr)
}

/// retn: Return From Procedure
pub fn retnd_imm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let addr = pop(cpu, mem);
    cpu.jmp(mem, addr);
    *cpu.regs.get32_mut(Register::ESP) += instr.immediate16() as u32;
}

/// jmp: Jump
pub fn jmp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    cpu.jmp(mem, instr.near_branch32())
}

/// jmp: Jump
pub fn jmp_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let target = rm32(cpu, mem, instr).get();
    cpu.jmp(mem, target)
}

/// ja: Jump if Condition Is Met
pub fn ja(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::CF) && !cpu.flags.contains(Flags::ZF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// jae: Jump if Condition Is Met
pub fn jae(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::CF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// j: Jump if Condition Is Met
pub fn jb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::CF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// jbe: Jump if Condition Is Met
pub fn jbe(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::CF) || cpu.flags.contains(Flags::ZF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// je: Jump if Condition Is Met
pub fn je(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::ZF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// jecxz: Jump if Condition Is Met
pub fn jecxz(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.regs.get32(Register::ECX) == 0 {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// j: Jump if Condition Is Met
pub fn jne(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::ZF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// jno: Jump if Condition Is Met
pub fn jno(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::OF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// jns: Jump if Condition Is Met
pub fn jns(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::SF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// jo: Jump if Condition Is Met
pub fn jo(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::OF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// jp: Jump if Condition Is Met
pub fn jp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::PF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// jg: Jump if Condition Is Met
pub fn jg(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if !cpu.flags.contains(Flags::ZF)
        && (cpu.flags.contains(Flags::SF) == cpu.flags.contains(Flags::OF))
    {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// jge: Jump if Condition Is Met
pub fn jge(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::SF) == cpu.flags.contains(Flags::OF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// jle: Jump if Condition Is Met
pub fn jle(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::ZF)
        || (cpu.flags.contains(Flags::SF) != cpu.flags.contains(Flags::OF))
    {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// jl: Jump if Condition Is Met
pub fn jl(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::SF) != cpu.flags.contains(Flags::OF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// js: Jump if Condition Is Met
pub fn js(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    if cpu.flags.contains(Flags::SF) {
        cpu.jmp(mem, instr.near_branch32());
    }
}

/// loop: Loop According to ECX Counter
pub fn loop_(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let ecx = cpu.regs.get32_mut(Register::ECX);
    *ecx -= 1;
    if *ecx != 0 {
        cpu.jmp(mem, instr.near_branch32());
    }
}
