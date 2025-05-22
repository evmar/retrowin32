use crate::{CPU, ops::helpers::*, registers::Flags};
use iced_x86::Instruction;
use memory::Mem;

/// bt: Bit Test
pub fn bt_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm32(cpu, mem, instr).get();
    let y = op1_rm32(cpu, mem, instr) % 32;
    cpu.flags.set(Flags::CF, ((x >> y) & 1) != 0);
}

/// bt: Bit Test
pub fn bt_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm32(cpu, mem, instr).get();
    let y = instr.immediate8() % 32;
    cpu.flags.set(Flags::CF, ((x >> y) & 1) != 0);
}

/// bts: Bit Test and Set
pub fn bts_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr) % 32;
    let x = rm32(cpu, mem, instr);
    let mask = 1u32 << y;
    cpu.flags.set(Flags::CF, x.get() & mask != 0);
    x.set(x.get() | mask);
}

/// btr: Bit Test and Reset
pub fn btr_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8() % 32;
    let x = rm32(cpu, mem, instr);
    cpu.flags.set(Flags::CF, ((x.get() >> y) & 1) != 0);
    x.set(x.get() & !(1 << y))
}

/// bsf — Bit Scan Forward
pub fn bsf_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    cpu.flags.set(Flags::ZF, y == 0);
    for i in 0..32 {
        if y & (1 << i) != 0 {
            x.set(i);
            break;
        }
    }
}

/// bsr: Bit Scan Reverse
pub fn bsr_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    cpu.flags.set(Flags::ZF, y == 0);
    for i in (0..32).rev() {
        if y & (1 << i) != 0 {
            x.set(i);
            break;
        }
    }
}

/// tzcnt: Count the Number of Trailing Zero Bits
pub fn tzcnt_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    let count = y.trailing_zeros();
    cpu.flags.set(Flags::CF, count == 32);
    cpu.flags.set(Flags::ZF, count == 0);
    x.set(count);
}
