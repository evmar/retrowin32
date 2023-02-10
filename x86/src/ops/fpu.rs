use iced_x86::Instruction;

use crate::{
    registers::FPUStatus,
    x86::{NULL_POINTER_REGION_SIZE, X86},
    StepResult,
};

use super::helpers::*;

fn read_f32(x86: &mut X86, addr: u32) -> f32 {
    f32::from_bits(x86.read_u32(addr))
}

pub fn read_f64(x86: &X86, addr: u32) -> f64 {
    if addr < NULL_POINTER_REGION_SIZE {
        panic!("null pointer read at {addr:#x}");
    }
    let addr = addr as usize;
    let n = u64::from_le_bytes(x86.mem[addr..addr + 8].try_into().unwrap());
    f64::from_bits(n)
}

pub fn write_f64(x86: &mut X86, addr: u32, value: f64) {
    if addr < NULL_POINTER_REGION_SIZE {
        panic!("null pointer read at {addr:#x}");
    }
    let addr = addr as usize;
    x86.mem[addr..addr + 8].copy_from_slice(&f64::to_le_bytes(value));
}

/// Compare two values and set floating-point comparison flags.
fn fcom<T: std::cmp::PartialOrd>(x86: &mut X86, x: T, y: T) {
    if x > y {
        x86.regs.fpu_status.set(FPUStatus::C3, false);
        x86.regs.fpu_status.set(FPUStatus::C2, false);
        x86.regs.fpu_status.set(FPUStatus::C0, false);
    } else if x < y {
        x86.regs.fpu_status.set(FPUStatus::C3, false);
        x86.regs.fpu_status.set(FPUStatus::C2, false);
        x86.regs.fpu_status.set(FPUStatus::C0, true);
    } else if x == y {
        x86.regs.fpu_status.set(FPUStatus::C3, true);
        x86.regs.fpu_status.set(FPUStatus::C2, false);
        x86.regs.fpu_status.set(FPUStatus::C0, false);
    } else {
        x86.regs.fpu_status.set(FPUStatus::C3, true);
        x86.regs.fpu_status.set(FPUStatus::C2, true);
        x86.regs.fpu_status.set(FPUStatus::C0, true);
    };
}

pub fn fld1(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = 1.0;
    Ok(())
}

pub fn fldz(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = 0.0;
    Ok(())
}

pub fn fld_m64fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = read_f64(x86, x86_addr(x86, instr));
    Ok(())
}

pub fn fld_m32fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = read_f32(x86, x86_addr(x86, instr)) as f64;
    Ok(())
}

pub fn fild_m32int(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = x86.read_u32(x86_addr(x86, instr)) as i32 as f64;
    Ok(())
}

pub fn fild_m16int(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = x86.read_u16(x86_addr(x86, instr)) as i16 as f64;
    Ok(())
}

pub fn fst_m64fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let f = *x86.regs.st_top();
    write_f64(x86, x86_addr(x86, instr), f);
    Ok(())
}

pub fn fst_m32fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let f = *x86.regs.st_top();
    x86.write_u32(x86_addr(x86, instr), (f as f32).to_bits());
    Ok(())
}

pub fn fstp_m64fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let f = *x86.regs.st_top();
    write_f64(x86, x86_addr(x86, instr), f);
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fstp_m32fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let f = *x86.regs.st_top();
    x86.write_u32(x86_addr(x86, instr), (f as f32).to_bits());
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fistp_m64int(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let f = *x86.regs.st_top();
    let addr = x86_addr(x86, instr) as usize;
    x86.mem[addr..addr + 8].copy_from_slice(&(f as i64).to_le_bytes());
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fistp_m32int(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let f = *x86.regs.st_top();
    x86.write_u32(x86_addr(x86, instr), f as i32 as u32);
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fchs(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    *x86.regs.st_top() = -*x86.regs.st_top();
    Ok(())
}

pub fn fcos(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    let reg = x86.regs.st_top();
    *reg = reg.cos();
    Ok(())
}
pub fn fsin(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    let reg = x86.regs.st_top();
    *reg = reg.sin();
    Ok(())
}
pub fn fpatan(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    let x = *x86.regs.st_top();
    x86.regs.st_top += 1;
    let reg = x86.regs.st_top();
    *reg = reg.atan2(x);
    Ok(())
}

pub fn fsqrt(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    let reg = x86.regs.st_top();
    *reg = reg.sqrt();
    Ok(())
}

pub fn fadd_m64fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = read_f64(x86, x86_addr(x86, instr));
    *x86.regs.st_top() += y;
    Ok(())
}

pub fn fadd_m32fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = read_f32(x86, x86_addr(x86, instr)) as f64;
    *x86.regs.st_top() += y;
    Ok(())
}

pub fn faddp_sti_st0(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = *x86.regs.getst(instr.op1_register());
    let x = x86.regs.getst(instr.op0_register());
    *x += y;
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fsub_m32fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = read_f32(x86, x86_addr(x86, instr)) as f64;
    let x = x86.regs.st_top();
    *x = *x - y;
    Ok(())
}

pub fn fsubr_m64fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = read_f64(x86, x86_addr(x86, instr));
    let x = x86.regs.st_top();
    *x = y - *x;
    Ok(())
}

pub fn fsubr_m32fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = read_f32(x86, x86_addr(x86, instr)) as f64;
    let x = x86.regs.st_top();
    *x = y - *x;
    Ok(())
}

pub fn fmul_m64fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = read_f64(x86, x86_addr(x86, instr));
    *x86.regs.st_top() *= y;
    Ok(())
}

pub fn fmul_m32fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = read_f32(x86, x86_addr(x86, instr)) as f64;
    *x86.regs.st_top() *= y;
    Ok(())
}

pub fn fmul_sti_sti(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = *x86.regs.getst(instr.op1_register());
    let x = x86.regs.getst(instr.op0_register());
    *x *= y;
    Ok(())
}

pub fn fmulp_sti_st0(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = *x86.regs.st_top();
    let x = x86.regs.getst(instr.op0_register());
    *x *= y;
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fdivrp_sti_st0(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    let x = *x86.regs.st_top();
    x86.regs.st_top += 1;
    *x86.regs.st_top() /= x;
    Ok(())
}

pub fn fdiv_m64fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = read_f64(x86, x86_addr(x86, instr));
    *x86.regs.st_top() /= y;
    Ok(())
}

pub fn fxch_st0_sti(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    x86.regs.st_swap(instr.op0_register(), instr.op1_register());
    Ok(())
}

pub fn fcomp_m32fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let x = *x86.regs.st_top();
    let y = read_f32(x86, x86_addr(x86, instr)) as f64;
    fcom(x86, x, y);
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fcomp_m64fp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let x = *x86.regs.st_top();
    let y = read_f64(x86, x86_addr(x86, instr));
    fcom(x86, x, y);
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fnstsw_ax(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    // TODO: does this need stack top in it?
    x86.regs.eax = x86.regs.fpu_status.bits() as u32;
    Ok(())
}

pub fn fnstcw_m2byte(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    // TODO: control word
    let cw = 0x37u16; // default value
    x86.write_u16(x86_addr(x86, instr), cw);
    Ok(())
}

pub fn fldcw_m2byte(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    // TODO: control word
    x86.read_u16(x86_addr(x86, instr));
    Ok(())
}
