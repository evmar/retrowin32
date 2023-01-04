use iced_x86::Instruction;

use crate::x86::{FPUStatus, X86};

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
    }
}

pub fn fld1(x86: &mut X86, _instr: &Instruction) {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = 1.0;
    panic!("test");
}

pub fn fldz(x86: &mut X86, _instr: &Instruction) {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = 0.0;
}

pub fn fld_m64fp(x86: &mut X86, instr: &Instruction) {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = x86.read_f64(x86.addr(instr));
}

pub fn fld_m32fp(x86: &mut X86, instr: &Instruction) {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = x86.read_f32(x86.addr(instr)) as f64;
}

pub fn fild_m32int(x86: &mut X86, instr: &Instruction) {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = x86.read_u32(x86.addr(instr)) as i32 as f64;
}

pub fn fild_m16int(x86: &mut X86, instr: &Instruction) {
    x86.regs.st_top -= 1;
    *x86.regs.st_top() = x86.read_u16(x86.addr(instr)) as i16 as f64;
}

pub fn fst_m64fp(x86: &mut X86, instr: &Instruction) {
    let f = *x86.regs.st_top();
    x86.write_f64(x86.addr(instr), f);
}

pub fn fstp_m64fp(x86: &mut X86, instr: &Instruction) {
    let f = *x86.regs.st_top();
    x86.write_f64(x86.addr(instr), f);
    x86.regs.st_top += 1;
}

pub fn fstp_m32fp(x86: &mut X86, instr: &Instruction) {
    let f = *x86.regs.st_top();
    x86.write_u32(x86.addr(instr), (f as f32).to_bits());
    x86.regs.st_top += 1;
}

pub fn fistp_m64int(x86: &mut X86, instr: &Instruction) {
    let f = *x86.regs.st_top();
    let addr = x86.addr(instr) as usize;
    x86.mem[addr..addr + 8].copy_from_slice(&(f as i64).to_le_bytes());
    x86.regs.st_top += 1;
}

pub fn fistp_m32int(x86: &mut X86, instr: &Instruction) {
    let f = *x86.regs.st_top();
    x86.write_u32(x86.addr(instr), f as i32 as u32);
    x86.regs.st_top += 1;
}

pub fn fchs(x86: &mut X86, _instr: &Instruction) {
    *x86.regs.st_top() = -*x86.regs.st_top();
}

pub fn fcos(x86: &mut X86, _instr: &Instruction) {
    let reg = x86.regs.st_top();
    *reg = reg.cos();
}
pub fn fsin(x86: &mut X86, _instr: &Instruction) {
    let reg = x86.regs.st_top();
    *reg = reg.sin();
}
pub fn fpatan(x86: &mut X86, _instr: &Instruction) {
    let x = *x86.regs.st_top();
    x86.regs.st_top += 1;
    let reg = x86.regs.st_top();
    *reg = reg.atan2(x);
}

pub fn fsqrt(x86: &mut X86, _instr: &Instruction) {
    let reg = x86.regs.st_top();
    *reg = reg.sqrt();
}

pub fn fadd_m64fp(x86: &mut X86, instr: &Instruction) {
    let y = x86.read_f64(x86.addr(instr));
    *x86.regs.st_top() += y;
}

pub fn fadd_m32fp(x86: &mut X86, instr: &Instruction) {
    let y = x86.read_f32(x86.addr(instr)) as f64;
    *x86.regs.st_top() += y;
}

pub fn faddp_sti_st0(x86: &mut X86, instr: &Instruction) {
    let y = *x86.regs.getst(instr.op1_register());
    let x = x86.regs.getst(instr.op0_register());
    *x += y;
    x86.regs.st_top += 1;
}

pub fn fsub_m32fp(x86: &mut X86, instr: &Instruction) {
    let y = x86.read_f32(x86.addr(instr)) as f64;
    let x = x86.regs.st_top();
    *x = *x - y;
}

pub fn fsubr_m64fp(x86: &mut X86, instr: &Instruction) {
    let y = x86.read_f64(x86.addr(instr));
    let x = x86.regs.st_top();
    *x = y - *x;
}

pub fn fsubr_m32fp(x86: &mut X86, instr: &Instruction) {
    let y = x86.read_f32(x86.addr(instr)) as f64;
    let x = x86.regs.st_top();
    *x = y - *x;
}

pub fn fmul_m64fp(x86: &mut X86, instr: &Instruction) {
    let y = x86.read_f64(x86.addr(instr));
    *x86.regs.st_top() *= y;
}

pub fn fmul_m32fp(x86: &mut X86, instr: &Instruction) {
    let y = x86.read_f32(x86.addr(instr)) as f64;
    *x86.regs.st_top() *= y;
}

pub fn fmul_sti_sti(x86: &mut X86, instr: &Instruction) {
    let y = *x86.regs.getst(instr.op1_register());
    let x = x86.regs.getst(instr.op0_register());
    *x *= y;
}

pub fn fmulp_sti_st0(x86: &mut X86, instr: &Instruction) {
    let y = *x86.regs.st_top();
    let x = x86.regs.getst(instr.op0_register());
    *x *= y;
    x86.regs.st_top += 1;
}

pub fn fdivrp_sti_st0(x86: &mut X86, _instr: &Instruction) {
    let x = *x86.regs.st_top();
    x86.regs.st_top += 1;
    *x86.regs.st_top() /= x;
}

pub fn fdiv_m64fp(x86: &mut X86, instr: &Instruction) {
    let y = x86.read_f64(x86.addr(instr));
    *x86.regs.st_top() /= y;
}

pub fn fxch_st0_sti(x86: &mut X86, instr: &Instruction) {
    x86.regs.st_swap(instr.op0_register(), instr.op1_register());
}

pub fn fcomp_m32fp(x86: &mut X86, instr: &Instruction) {
    let x = *x86.regs.st_top();
    let y = x86.read_f32(x86.addr(instr)) as f64;
    fcom(x86, x, y);
    x86.regs.st_top += 1;
}

pub fn fcomp_m64fp(x86: &mut X86, instr: &Instruction) {
    let x = *x86.regs.st_top();
    let y = x86.read_f64(x86.addr(instr));
    fcom(x86, x, y);
    x86.regs.st_top += 1;
}

pub fn fnstsw_ax(x86: &mut X86, _instr: &Instruction) {
    // TODO: does this need stack top in it?
    x86.regs.eax = x86.regs.fpu_status.bits() as u32;
}

pub fn fnstcw_m2byte(x86: &mut X86, instr: &Instruction) {
    // TODO: control word
    let cw = 0x37u16; // default value
    x86.write_u16(x86.addr(instr), cw);
}

pub fn fldcw_m2byte(x86: &mut X86, instr: &Instruction) {
    // TODO: control word
    x86.read_u16(x86.addr(instr));
}
