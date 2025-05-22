use crate::{ops::helpers::*, x86::CPU};
use iced_x86::{Instruction, Register};
use memory::Mem;

/// idiv: Signed Divide
pub fn idiv_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = get_edx_eax(cpu) as i64;
    let y = rm32(cpu, mem, instr).get() as i32 as i64;
    cpu.regs.set32(Register::EAX, (x / y) as i32 as u32);
    cpu.regs.set32(Register::EDX, (x % y) as i32 as u32);
}

/// idiv: Signed Divide
pub fn idiv_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = get_dx_ax(cpu) as i32;
    let y = rm16(cpu, mem, instr).get() as i16 as i32;
    let quotient = x / y;
    if quotient > 0x7FFF || quotient < -0x8000 {
        panic!("divide error");
    }
    cpu.regs.set16(Register::AX, quotient as i16 as u16);
    cpu.regs.set16(Register::DX, (x % y) as u16);
}

/// idiv: Signed Divide
pub fn idiv_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = cpu.regs.get16(Register::AX) as i16;
    let y = rm8(cpu, mem, instr).get() as i8 as i16;
    let quotient = x / y;
    if quotient > 0x7F || quotient < -0x80 {
        panic!("divide error");
    }
    let rem = x % y;
    cpu.regs
        .set16(Register::AX, ((rem << 8) as u16) | (quotient as i8 as u16));
}

/// div: Unsigned Divide
pub fn div_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = get_edx_eax(cpu);
    let y = rm32(cpu, mem, instr).get() as u64;
    cpu.regs.set32(Register::EAX, (x / y) as u32);
    cpu.regs.set32(Register::EDX, (x % y) as u32);
    // No flags.
}

/// div: Unsigned Divide
pub fn div_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = get_dx_ax(cpu);
    let y = rm16(cpu, mem, instr).get() as u32;
    cpu.regs.set32(Register::EAX, ((x / y) as u16) as u32);
    cpu.regs.set32(Register::EDX, ((x % y) as u16) as u32);
    // No flags.
}

/// div: Unsigned Divide
pub fn div_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = cpu.regs.get16(Register::AX);
    let y = rm8(cpu, mem, instr).get() as u16;
    cpu.regs
        .set32(Register::EAX, (((x % y) as u32) << 16) | ((x / y) as u32));
    // No flags.
}
