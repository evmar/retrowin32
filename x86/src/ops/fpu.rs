use iced_x86::Instruction;

use crate::{registers::FPUStatus, x86::X86, Mem, Memory, StepResult};

use super::helpers::*;

fn read_f32(mem: &mut Mem, addr: u32) -> f32 {
    f32::from_bits(mem.read_u32(addr))
}

pub fn read_f64(mem: &Mem, addr: u32) -> f64 {
    *mem.view::<f64>(addr)
}

pub fn write_f64(mem: &mut Mem, addr: u32, value: f64) {
    *mem.view_mut::<f64>(addr) = value;
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

pub fn fld1(x86: &mut X86, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = 1.0;
    Ok(())
}

pub fn fldz(x86: &mut X86, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = 0.0;
    Ok(())
}

pub fn fldpi(x86: &mut X86, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = std::f64::consts::PI;
    Ok(())
}

pub fn fld_sti(x86: &mut X86, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = *x86.regs.getst(instr.op0_register());
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = y;
    Ok(())
}

pub fn fld_m64fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = read_f64(mem, x86_addr(x86, instr));
    Ok(())
}

pub fn fld_m32fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = read_f32(mem, x86_addr(x86, instr)) as f64;
    Ok(())
}

pub fn fild_m64int(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = read_f64(mem, x86_addr(x86, instr));
    Ok(())
}

pub fn fild_m32int(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = mem.read_u32(x86_addr(x86, instr)) as i32 as f64;
    Ok(())
}

pub fn fild_m16int(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = mem.read_u16(x86_addr(x86, instr)) as i16 as f64;
    Ok(())
}

pub fn fst_m64fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let f = *x86.regs.st_top();
    write_f64(mem, x86_addr(x86, instr), f);
    Ok(())
}

pub fn fst_m32fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let f = *x86.regs.st_top();
    let addr = x86_addr(x86, instr);
    mem.write_u32(addr, (f as f32).to_bits());
    Ok(())
}

pub fn fstp_m64fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let f = *x86.regs.st_top();
    write_f64(mem, x86_addr(x86, instr), f);
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fstp_m32fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let f = *x86.regs.st_top();
    let addr = x86_addr(x86, instr);
    mem.write_u32(addr, (f as f32).to_bits());
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fstp_sti(x86: &mut X86, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let f = *x86.regs.st_top();
    *x86.regs.getst(instr.op0_register()) = f;
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fistp_m64int(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let f = *x86.regs.st_top();
    let addr = x86_addr(x86, instr);
    *mem.view_mut::<u64>(addr) = f.round() as i64 as u64;
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fistp_m32int(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let f = *x86.regs.st_top();
    let addr = x86_addr(x86, instr);
    mem.write_u32(addr, f as i32 as u32);
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fchs(x86: &mut X86, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    *x86.regs.st_top() = -*x86.regs.st_top();
    Ok(())
}

pub fn fcos(x86: &mut X86, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let reg = x86.regs.st_top();
    *reg = reg.cos();
    Ok(())
}
pub fn fsin(x86: &mut X86, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let reg = x86.regs.st_top();
    *reg = reg.sin();
    Ok(())
}
pub fn fpatan(x86: &mut X86, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let x = *x86.regs.st_top();
    x86.regs.st_top += 1;
    let reg = x86.regs.st_top();
    *reg = reg.atan2(x);
    Ok(())
}

pub fn fsqrt(x86: &mut X86, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let reg = x86.regs.st_top();
    *reg = reg.sqrt();
    Ok(())
}

pub fn fadd_m64fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f64(mem, x86_addr(x86, instr));
    *x86.regs.st_top() += y;
    Ok(())
}

pub fn fadd_m32fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f32(mem, x86_addr(x86, instr)) as f64;
    *x86.regs.st_top() += y;
    Ok(())
}

pub fn faddp_sti_st0(x86: &mut X86, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = *x86.regs.getst(instr.op1_register());
    let x = x86.regs.getst(instr.op0_register());
    *x += y;
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fsub_m32fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f32(mem, x86_addr(x86, instr)) as f64;
    let x = x86.regs.st_top();
    *x = *x - y;
    Ok(())
}

pub fn fsub_st0_sti(x86: &mut X86, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = *x86.regs.getst(instr.op1_register());
    let x = x86.regs.getst(instr.op0_register());
    *x *= y;
    Ok(())
}

pub fn fsubr_m64fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f64(mem, x86_addr(x86, instr));
    let x = x86.regs.st_top();
    *x = y - *x;
    Ok(())
}

pub fn fsubr_m32fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f32(mem, x86_addr(x86, instr)) as f64;
    let x = x86.regs.st_top();
    *x = y - *x;
    Ok(())
}

pub fn fmul_m64fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f64(mem, x86_addr(x86, instr));
    *x86.regs.st_top() *= y;
    Ok(())
}

pub fn fmul_m32fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f32(mem, x86_addr(x86, instr)) as f64;
    *x86.regs.st_top() *= y;
    Ok(())
}

pub fn fimul_m32int(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = mem.read_u32(x86_addr(x86, instr)) as f64;
    *x86.regs.st_top() *= y;
    Ok(())
}

pub fn fmul_sti_sti(x86: &mut X86, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = *x86.regs.getst(instr.op1_register());
    let x = x86.regs.getst(instr.op0_register());
    *x *= y;
    Ok(())
}

pub fn fmulp_sti_st0(x86: &mut X86, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = *x86.regs.st_top();
    let x = x86.regs.getst(instr.op0_register());
    *x *= y;
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fdivp_sti_st0(x86: &mut X86, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = *x86.regs.st_top();
    let x = x86.regs.getst(instr.op0_register());
    *x = *x / y;
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fdivrp_sti_st0(x86: &mut X86, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = *x86.regs.st_top();
    let x = x86.regs.getst(instr.op0_register());
    *x = y / *x;
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fdiv_m64fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f64(mem, x86_addr(x86, instr));
    *x86.regs.st_top() /= y;
    Ok(())
}

pub fn fdiv_m32fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f32(mem, x86_addr(x86, instr)) as f64;
    *x86.regs.st_top() /= y;
    Ok(())
}

pub fn fidiv_m32int(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = mem.read_u32(x86_addr(x86, instr)) as f64;
    *x86.regs.st_top() /= y;
    Ok(())
}

pub fn fidivr_m32int(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = mem.read_u32(x86_addr(x86, instr)) as f64;
    *x86.regs.st_top() = y / *x86.regs.st_top();
    Ok(())
}

pub fn fxch_st0_sti(x86: &mut X86, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    x86.regs.st_swap(instr.op0_register(), instr.op1_register());
    Ok(())
}

pub fn fcomp_m32fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let x = *x86.regs.st_top();
    let y = read_f32(mem, x86_addr(x86, instr)) as f64;
    fcom(x86, x, y);
    x86.regs.st_top += 1;
    Ok(())
}

pub fn fcomp_m64fp(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let x = *x86.regs.st_top();
    let y = read_f64(mem, x86_addr(x86, instr));
    fcom(x86, x, y);
    x86.regs.st_top += 1;
    Ok(())
}

pub fn frndint(x86: &mut X86, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let x = x86.regs.st_top();
    *x = x.round();
    Ok(())
}

pub fn fnstsw_ax(x86: &mut X86, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    // TODO: does this need stack top in it?
    x86.regs.eax = x86.regs.fpu_status.bits() as u32;
    Ok(())
}

pub fn fnstcw_m2byte(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    // TODO: control word
    let cw = 0x37u16; // default value
    let addr = x86_addr(x86, instr);
    mem.write_u16(addr, cw);
    Ok(())
}

pub fn fldcw_m2byte(x86: &mut X86, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    // TODO: control word
    mem.read_u16(x86_addr(x86, instr));
    Ok(())
}
