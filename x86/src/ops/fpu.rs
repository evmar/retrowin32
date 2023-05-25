use iced_x86::Instruction;

use crate::{registers::FPUStatus, x86::CPU, Mem, Memory, StepResult};

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
fn fcom<T: std::cmp::PartialOrd>(cpu: &mut CPU, x: T, y: T) {
    if x > y {
        cpu.regs.fpu_status.set(FPUStatus::C3, false);
        cpu.regs.fpu_status.set(FPUStatus::C2, false);
        cpu.regs.fpu_status.set(FPUStatus::C0, false);
    } else if x < y {
        cpu.regs.fpu_status.set(FPUStatus::C3, false);
        cpu.regs.fpu_status.set(FPUStatus::C2, false);
        cpu.regs.fpu_status.set(FPUStatus::C0, true);
    } else if x == y {
        cpu.regs.fpu_status.set(FPUStatus::C3, true);
        cpu.regs.fpu_status.set(FPUStatus::C2, false);
        cpu.regs.fpu_status.set(FPUStatus::C0, false);
    } else {
        cpu.regs.fpu_status.set(FPUStatus::C3, true);
        cpu.regs.fpu_status.set(FPUStatus::C2, true);
        cpu.regs.fpu_status.set(FPUStatus::C0, true);
    };
}

pub fn fld1(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    cpu.regs.st_top -= 1;
    *cpu.regs.st_top() = 1.0;
    Ok(())
}

pub fn fldz(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    cpu.regs.st_top -= 1;
    *cpu.regs.st_top() = 0.0;
    Ok(())
}

pub fn fldpi(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    cpu.regs.st_top -= 1;
    *cpu.regs.st_top() = std::f64::consts::PI;
    Ok(())
}

pub fn fld_sti(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = *cpu.regs.getst(instr.op0_register());
    cpu.regs.st_top -= 1;
    *cpu.regs.st_top() = y;
    Ok(())
}

pub fn fld_m64fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    cpu.regs.st_top -= 1;
    *cpu.regs.st_top() = read_f64(mem, x86_addr(cpu, instr));
    Ok(())
}

pub fn fld_m32fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    cpu.regs.st_top -= 1;
    *cpu.regs.st_top() = read_f32(mem, x86_addr(cpu, instr)) as f64;
    Ok(())
}

pub fn fild_m64int(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    cpu.regs.st_top -= 1;
    *cpu.regs.st_top() = read_f64(mem, x86_addr(cpu, instr));
    Ok(())
}

pub fn fild_m32int(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    cpu.regs.st_top -= 1;
    *cpu.regs.st_top() = mem.read_u32(x86_addr(cpu, instr)) as i32 as f64;
    Ok(())
}

pub fn fild_m16int(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    cpu.regs.st_top -= 1;
    *cpu.regs.st_top() = mem.read_u16(x86_addr(cpu, instr)) as i16 as f64;
    Ok(())
}

pub fn fst_m64fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let f = *cpu.regs.st_top();
    write_f64(mem, x86_addr(cpu, instr), f);
    Ok(())
}

pub fn fst_m32fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let f = *cpu.regs.st_top();
    let addr = x86_addr(cpu, instr);
    mem.write_u32(addr, (f as f32).to_bits());
    Ok(())
}

pub fn fstp_m64fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let f = *cpu.regs.st_top();
    write_f64(mem, x86_addr(cpu, instr), f);
    cpu.regs.st_top += 1;
    Ok(())
}

pub fn fstp_m32fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let f = *cpu.regs.st_top();
    let addr = x86_addr(cpu, instr);
    mem.write_u32(addr, (f as f32).to_bits());
    cpu.regs.st_top += 1;
    Ok(())
}

pub fn fstp_sti(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let f = *cpu.regs.st_top();
    *cpu.regs.getst(instr.op0_register()) = f;
    cpu.regs.st_top += 1;
    Ok(())
}

pub fn fistp_m64int(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let f = *cpu.regs.st_top();
    let addr = x86_addr(cpu, instr);
    *mem.view_mut::<u64>(addr) = f.round() as i64 as u64;
    cpu.regs.st_top += 1;
    Ok(())
}

pub fn fistp_m32int(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let f = *cpu.regs.st_top();
    let addr = x86_addr(cpu, instr);
    mem.write_u32(addr, f as i32 as u32);
    cpu.regs.st_top += 1;
    Ok(())
}

pub fn fchs(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    *cpu.regs.st_top() = -*cpu.regs.st_top();
    Ok(())
}

pub fn fcos(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let reg = cpu.regs.st_top();
    *reg = reg.cos();
    Ok(())
}
pub fn fsin(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let reg = cpu.regs.st_top();
    *reg = reg.sin();
    Ok(())
}
pub fn fpatan(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let x = *cpu.regs.st_top();
    cpu.regs.st_top += 1;
    let reg = cpu.regs.st_top();
    *reg = reg.atan2(x);
    Ok(())
}

pub fn fsqrt(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let reg = cpu.regs.st_top();
    *reg = reg.sqrt();
    Ok(())
}

pub fn fadd_m64fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f64(mem, x86_addr(cpu, instr));
    *cpu.regs.st_top() += y;
    Ok(())
}

pub fn fadd_m32fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f32(mem, x86_addr(cpu, instr)) as f64;
    *cpu.regs.st_top() += y;
    Ok(())
}

pub fn faddp_sti_st0(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = *cpu.regs.getst(instr.op1_register());
    let x = cpu.regs.getst(instr.op0_register());
    *x += y;
    cpu.regs.st_top += 1;
    Ok(())
}

pub fn fsub_m32fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f32(mem, x86_addr(cpu, instr)) as f64;
    let x = cpu.regs.st_top();
    *x = *x - y;
    Ok(())
}

pub fn fsub_st0_sti(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = *cpu.regs.getst(instr.op1_register());
    let x = cpu.regs.getst(instr.op0_register());
    *x *= y;
    Ok(())
}

pub fn fsubr_m64fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f64(mem, x86_addr(cpu, instr));
    let x = cpu.regs.st_top();
    *x = y - *x;
    Ok(())
}

pub fn fsubr_m32fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f32(mem, x86_addr(cpu, instr)) as f64;
    let x = cpu.regs.st_top();
    *x = y - *x;
    Ok(())
}

pub fn fmul_m64fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f64(mem, x86_addr(cpu, instr));
    *cpu.regs.st_top() *= y;
    Ok(())
}

pub fn fmul_m32fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f32(mem, x86_addr(cpu, instr)) as f64;
    *cpu.regs.st_top() *= y;
    Ok(())
}

pub fn fimul_m32int(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = mem.read_u32(x86_addr(cpu, instr)) as f64;
    *cpu.regs.st_top() *= y;
    Ok(())
}

pub fn fmul_sti_sti(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = *cpu.regs.getst(instr.op1_register());
    let x = cpu.regs.getst(instr.op0_register());
    *x *= y;
    Ok(())
}

pub fn fmulp_sti_st0(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = *cpu.regs.st_top();
    let x = cpu.regs.getst(instr.op0_register());
    *x *= y;
    cpu.regs.st_top += 1;
    Ok(())
}

pub fn fdivp_sti_st0(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = *cpu.regs.st_top();
    let x = cpu.regs.getst(instr.op0_register());
    *x = *x / y;
    cpu.regs.st_top += 1;
    Ok(())
}

pub fn fdivrp_sti_st0(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = *cpu.regs.st_top();
    let x = cpu.regs.getst(instr.op0_register());
    *x = y / *x;
    cpu.regs.st_top += 1;
    Ok(())
}

pub fn fdiv_m64fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f64(mem, x86_addr(cpu, instr));
    *cpu.regs.st_top() /= y;
    Ok(())
}

pub fn fdiv_m32fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = read_f32(mem, x86_addr(cpu, instr)) as f64;
    *cpu.regs.st_top() /= y;
    Ok(())
}

pub fn fidiv_m32int(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = mem.read_u32(x86_addr(cpu, instr)) as f64;
    *cpu.regs.st_top() /= y;
    Ok(())
}

pub fn fidivr_m32int(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let y = mem.read_u32(x86_addr(cpu, instr)) as f64;
    *cpu.regs.st_top() = y / *cpu.regs.st_top();
    Ok(())
}

pub fn fxch_st0_sti(cpu: &mut CPU, _mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    cpu.regs.st_swap(instr.op0_register(), instr.op1_register());
    Ok(())
}

pub fn fcomp_m32fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let x = *cpu.regs.st_top();
    let y = read_f32(mem, x86_addr(cpu, instr)) as f64;
    fcom(cpu, x, y);
    cpu.regs.st_top += 1;
    Ok(())
}

pub fn fcomp_m64fp(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    let x = *cpu.regs.st_top();
    let y = read_f64(mem, x86_addr(cpu, instr));
    fcom(cpu, x, y);
    cpu.regs.st_top += 1;
    Ok(())
}

pub fn frndint(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    let x = cpu.regs.st_top();
    *x = x.round();
    Ok(())
}

pub fn fnstsw_ax(cpu: &mut CPU, _mem: &mut Mem, _instr: &Instruction) -> StepResult<()> {
    // TODO: does this need stack top in it?
    cpu.regs.eax = cpu.regs.fpu_status.bits() as u32;
    Ok(())
}

pub fn fnstcw_m2byte(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    // TODO: control word
    let cw = 0x37u16; // default value
    let addr = x86_addr(cpu, instr);
    mem.write_u16(addr, cw);
    Ok(())
}

pub fn fldcw_m2byte(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) -> StepResult<()> {
    // TODO: control word
    mem.read_u16(x86_addr(cpu, instr));
    Ok(())
}
