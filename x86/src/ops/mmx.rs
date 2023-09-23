use super::helpers::*;
use crate::CPU;
use iced_x86::Instruction;
use memory::Mem;

fn op1_mmm64(cpu: &mut CPU, mem: Mem, instr: &iced_x86::Instruction) -> u64 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => cpu.regs.get64(instr.op1_register()),
        iced_x86::OpKind::Memory => mem.get::<u64>(x86_addr(cpu, instr)),
        _ => unreachable!(),
    }
}

fn op1_mmm32(cpu: &mut CPU, mem: Mem, instr: &iced_x86::Instruction) -> u32 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => cpu.regs.get64(instr.op1_register()) as u32,
        iced_x86::OpKind::Memory => mem.get::<u32>(x86_addr(cpu, instr)),
        _ => unreachable!(),
    }
}

pub fn pxor_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| x ^ y);
}

pub fn movq_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, _x| y);
}

pub fn movd_mm_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr) as u64;
    rm64_x(cpu, mem, instr, |_cpu, _x| y);
}

pub fn movd_rm32_mm(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get64(instr.op1_register()) as u32;
    let x = rm32(cpu, mem, instr);
    x.set(y);
}

pub fn punpcklwd_mm_mmm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm32(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x = x as u32; // instr only uses low 32 bits of x
        (((x >> 16) as u16) as u64) << 48
            | (((y >> 16) as u16) as u64) << 32
            | (((x >> 0) as u16) as u64) << 16
            | (((y >> 0) as u16) as u64) << 0
    });
}

pub fn punpcklbw_mm_mmm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm32(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x = x as u32; // instr only uses low 32 bits of x
        (((y >> 24) & 0xFF) as u64) << 56
            | (((x >> 24) & 0xFF) as u64) << 48
            | (((y >> 16) & 0xFF) as u64) << 40
            | (((x >> 16) & 0xFF) as u64) << 32
            | (((y >> 8) & 0xFF) as u64) << 24
            | (((x >> 8) & 0xFF) as u64) << 16
            | (((y >> 0) & 0xFF) as u64) << 8
            | (((x >> 0) & 0xFF) as u64) << 0
    });
}

pub fn pmullw_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let t0 = (((x >> 0) & 0xFFFF) as i16 as u32) * (((y >> 0) & 0xFFFF) as i16 as u32);
        let t1 = (((x >> 16) & 0xFFFF) as i16 as u32) * (((y >> 16) & 0xFFFF) as i16 as u32);
        let t2 = (((x >> 32) & 0xFFFF) as i16 as u32) * (((y >> 32) & 0xFFFF) as i16 as u32);
        let t3 = (((x >> 48) & 0xFFFF) as i16 as u32) * (((y >> 48) & 0xFFFF) as i16 as u32);
        (t3 as u64) << 48 | (t2 as u64) << 32 | (t1 as u64) << 16 | (t0 as u64)
    });
}

pub fn psrlw_mm_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    rm64_x(cpu, mem, instr, |_cpu, x| {
        (((x >> 0) & 0xFFFF) >> y) << 0
            | (((x >> 16) & 0xFFFF) >> y) << 16
            | (((x >> 32) & 0xFFFF) >> y) << 32
            | (((x >> 48) & 0xFFFF) >> y) << 48
    });
}

pub fn packuswb_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    fn saturate(x: i16) -> u8 {
        if x < 0 {
            0
        } else if x > 0xFF {
            0xFF
        } else {
            x as u8
        }
    }
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        (saturate(((x >> 0) & 0xFFFF) as i16) as u64) << 0
            | (saturate(((x >> 16) & 0xFFFF) as i16) as u64) << 8
            | (saturate(((x >> 32) & 0xFFFF) as i16) as u64) << 16
            | (saturate(((x >> 48) & 0xFFFF) as i16) as u64) << 24
            | (saturate(((y >> 0) & 0xFFFF) as i16) as u64) << 32
            | (saturate(((y >> 16) & 0xFFFF) as i16) as u64) << 40
            | (saturate(((y >> 32) & 0xFFFF) as i16) as u64) << 48
            | (saturate(((y >> 48) & 0xFFFF) as i16) as u64) << 56
    });
}

pub fn emms(_cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    // This is supposed to reset the FPU tag word, but I don't have one of those
    // because I'm not yet clear on what it's actually used for...
}

pub fn psubusb_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    fn op(x: u8, y: u8) -> u8 {
        y.saturating_sub(x)
    }

    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        ((op((x >> 0) as u8, (y >> 0) as u8) as u64) << 0)
            | ((op((x >> 8) as u8, (y >> 8) as u8) as u64) << 8)
            | ((op((x >> 16) as u8, (y >> 16) as u8) as u64) << 16)
            | ((op((x >> 24) as u8, (y >> 24) as u8) as u64) << 24)
            | ((op((x >> 32) as u8, (y >> 32) as u8) as u64) << 32)
            | ((op((x >> 40) as u8, (y >> 40) as u8) as u64) << 40)
            | ((op((x >> 48) as u8, (y >> 48) as u8) as u64) << 48)
            | ((op((x >> 56) as u8, (y >> 56) as u8) as u64) << 56)
    });
}

pub fn paddusb_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    fn op(x: u8, y: u8) -> u8 {
        x.saturating_add(y)
    }

    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        ((op((x >> 0) as u8, (y >> 0) as u8) as u64) << 0)
            | ((op((x >> 8) as u8, (y >> 8) as u8) as u64) << 8)
            | ((op((x >> 16) as u8, (y >> 16) as u8) as u64) << 16)
            | ((op((x >> 24) as u8, (y >> 24) as u8) as u64) << 24)
            | ((op((x >> 32) as u8, (y >> 32) as u8) as u64) << 32)
            | ((op((x >> 40) as u8, (y >> 40) as u8) as u64) << 40)
            | ((op((x >> 48) as u8, (y >> 48) as u8) as u64) << 48)
            | ((op((x >> 56) as u8, (y >> 56) as u8) as u64) << 56)
    });
}
