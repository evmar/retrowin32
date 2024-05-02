use super::helpers::*;
use crate::{fpu, x86::CPU};
use iced_x86::{Instruction, Register};
use memory::{Extensions, Mem};

pub fn finit(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.fpu.st_top = 8;
    cpu.fpu.status = fpu::Status::empty();
}

/// Compare two values and set floating-point comparison flags.
fn fcom<T: std::cmp::PartialOrd>(cpu: &mut CPU, x: T, y: T) {
    if x > y {
        cpu.fpu.status.set(fpu::Status::C3, false);
        cpu.fpu.status.set(fpu::Status::C2, false);
        cpu.fpu.status.set(fpu::Status::C0, false);
    } else if x < y {
        cpu.fpu.status.set(fpu::Status::C3, false);
        cpu.fpu.status.set(fpu::Status::C2, false);
        cpu.fpu.status.set(fpu::Status::C0, true);
    } else if x == y {
        cpu.fpu.status.set(fpu::Status::C3, true);
        cpu.fpu.status.set(fpu::Status::C2, false);
        cpu.fpu.status.set(fpu::Status::C0, false);
    } else {
        cpu.fpu.status.set(fpu::Status::C3, true);
        cpu.fpu.status.set(fpu::Status::C2, true);
        cpu.fpu.status.set(fpu::Status::C0, true);
    };
}

pub fn fld1(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.fpu.push(1.0);
}

pub fn fldz(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.fpu.push(0.0);
}

pub fn fldpi(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.fpu.push(std::f64::consts::PI);
}

pub fn fldl2e(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.fpu.push(std::f64::consts::LOG2_E);
}

pub fn fld_sti(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    let y = *cpu.fpu.get(instr.op0_register());
    cpu.fpu.push(y);
}

pub fn fld_m64fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    cpu.fpu.push(mem.get_pod::<f64>(x86_addr(cpu, instr)));
}

pub fn fld_m32fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    cpu.fpu
        .push(mem.get_pod::<f32>(x86_addr(cpu, instr)) as f64);
}

pub fn fild_m64int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    cpu.fpu
        .push(mem.get_pod::<u64>(x86_addr(cpu, instr)) as u64 as f64);
}

pub fn fild_m32int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    cpu.fpu
        .push(mem.get_pod::<u32>(x86_addr(cpu, instr)) as i32 as f64);
}

pub fn fild_m16int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    cpu.fpu
        .push(mem.get_pod::<u16>(x86_addr(cpu, instr)) as i16 as f64);
}

pub fn fst_m64fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let f = *cpu.fpu.st0();
    mem.put::<f64>(x86_addr(cpu, instr), f);
}

pub fn fst_m32fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let f = *cpu.fpu.st0();
    let addr = x86_addr(cpu, instr);
    mem.put::<u32>(addr, (f as f32).to_bits());
}

pub fn fstp_m64fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    fst_m64fp(cpu, mem, instr);
    cpu.fpu.pop();
}

pub fn fstp_m32fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    fst_m32fp(cpu, mem, instr);
    cpu.fpu.pop();
}

pub fn fstp_sti(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    let f = *cpu.fpu.st0();
    *cpu.fpu.get(instr.op0_register()) = f;
    cpu.fpu.pop();
}

pub fn fistp_m64int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let f = *cpu.fpu.st0();
    let addr = x86_addr(cpu, instr);
    *mem.view_mut::<u64>(addr) = f.round() as i64 as u64;
    cpu.fpu.pop();
}

pub fn fist_m32int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let f = *cpu.fpu.st0();
    let addr = x86_addr(cpu, instr);
    mem.put::<u32>(addr, f as i32 as u32);
}

pub fn fistp_m32int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    fist_m32int(cpu, mem, instr);
    cpu.fpu.pop();
}

pub fn fistp_m16int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let f = *cpu.fpu.st0();
    let addr = x86_addr(cpu, instr);
    mem.put::<u16>(addr, f as i16 as u16);
    cpu.fpu.pop();
}

pub fn fchs(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    *cpu.fpu.st0() = -*cpu.fpu.st0();
}

pub fn fabs(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    *cpu.fpu.st0() = num_traits::abs(*cpu.fpu.st0());
}

pub fn fcos(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let reg = cpu.fpu.st0();
    *reg = reg.cos();
}
pub fn fsin(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let reg = cpu.fpu.st0();
    *reg = reg.sin();
}
pub fn fsincos(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let val = *cpu.fpu.st0();
    *cpu.fpu.st0() = val.sin();
    cpu.fpu.push(val.cos());
}
pub fn fpatan(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let x = *cpu.fpu.st0();
    cpu.fpu.pop();
    let reg = cpu.fpu.st0();
    *reg = reg.atan2(x);
}

pub fn fsqrt(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let reg = cpu.fpu.st0();
    *reg = reg.sqrt();
}

pub fn fadd_sti_sti(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    let y = *cpu.fpu.get(instr.op1_register());
    let x = cpu.fpu.get(instr.op0_register());
    *x += y;
}

pub fn fadd_m64fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<f64>(x86_addr(cpu, instr));
    *cpu.fpu.st0() += y;
}

pub fn fadd_m32fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<f32>(x86_addr(cpu, instr)) as f64;
    *cpu.fpu.st0() += y;
}

pub fn faddp_sti_sti(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    fadd_sti_sti(cpu, mem, instr);
    cpu.fpu.pop();
}

pub fn fiadd_m32int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<u32>(x86_addr(cpu, instr)) as i32 as f64;
    *cpu.fpu.st0() += y;
}

pub fn fiadd_m16int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<u16>(x86_addr(cpu, instr)) as f64;
    *cpu.fpu.st0() += y;
}

pub fn fsub_m64fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<f64>(x86_addr(cpu, instr));
    let x = cpu.fpu.st0();
    *x = *x - y;
}

pub fn fsub_m32fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<f32>(x86_addr(cpu, instr)) as f64;
    let x = cpu.fpu.st0();
    *x = *x - y;
}

pub fn fsub_sti_sti(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    let y = *cpu.fpu.get(instr.op1_register());
    let x = cpu.fpu.get(instr.op0_register());
    *x -= y;
}

pub fn fsubp_sti_sti(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    fsub_sti_sti(cpu, mem, instr);
    cpu.fpu.pop();
}

pub fn fisub_m32int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<u32>(x86_addr(cpu, instr)) as i32 as f64;
    let x = cpu.fpu.st0();
    *x = *x - y;
}

pub fn fsubr_m64fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<f64>(x86_addr(cpu, instr));
    let x = cpu.fpu.st0();
    *x = y - *x;
}

pub fn fsubr_m32fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<f32>(x86_addr(cpu, instr)) as f64;
    let x = cpu.fpu.st0();
    *x = y - *x;
}

pub fn fsubr_st0_sti(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    let y = *cpu.fpu.get(instr.op1_register());
    let x = cpu.fpu.st0();
    *x = y - *x;
}

pub fn fmul_m64fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<f64>(x86_addr(cpu, instr));
    *cpu.fpu.st0() *= y;
}

pub fn fmul_m32fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<f32>(x86_addr(cpu, instr)) as f64;
    *cpu.fpu.st0() *= y;
}

pub fn fimul_m32int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<u32>(x86_addr(cpu, instr)) as i32 as f64;
    *cpu.fpu.st0() *= y;
}

pub fn fimul_m16int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<u16>(x86_addr(cpu, instr)) as f64;
    *cpu.fpu.st0() *= y;
}

pub fn fmul_sti_sti(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    let y = *cpu.fpu.get(instr.op1_register());
    let x = cpu.fpu.get(instr.op0_register());
    *x *= y;
}

pub fn fmulp_sti_st0(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    let y = *cpu.fpu.st0();
    let x = cpu.fpu.get(instr.op0_register());
    *x *= y;
    cpu.fpu.pop();
}

pub fn f2xm1(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let x = cpu.fpu.st0();
    *x = 2.0_f64.powf(*x) - 1.0;
}

pub fn fscale(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let y = *cpu.fpu.get(iced_x86::Register::ST1);
    let x = cpu.fpu.st0();
    *x = *x * 2f64.powf(y.floor());
}

pub fn fdiv_m64fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<f64>(x86_addr(cpu, instr));
    *cpu.fpu.st0() /= y;
}

pub fn fdiv_m32fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<f32>(x86_addr(cpu, instr)) as f64;
    *cpu.fpu.st0() /= y;
}

pub fn fdiv_sti_sti(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    let y = *cpu.fpu.get(instr.op1_register());
    let x = cpu.fpu.get(instr.op0_register());
    *x = *x / y;
}

pub fn fdivp_sti_sti(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    fdiv_sti_sti(cpu, mem, instr);
    cpu.fpu.pop();
}

pub fn fidiv_m32int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<u32>(x86_addr(cpu, instr)) as i32 as f64;
    *cpu.fpu.st0() /= y;
}

pub fn fidiv_m16int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<u16>(x86_addr(cpu, instr)) as i16 as f64;
    *cpu.fpu.st0() /= y;
}

pub fn fdivr_m64fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<f64>(x86_addr(cpu, instr));
    let x = cpu.fpu.st0();
    *x = y / *x;
}

pub fn fdivr_m32fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<f32>(x86_addr(cpu, instr)) as i32 as f64;
    let x = cpu.fpu.st0();
    *x = y / *x;
}

pub fn fdivr_st0_sti(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    let y = *cpu.fpu.get(instr.op0_register());
    let x = cpu.fpu.st0();
    *x = y / *x;
}

pub fn fdivrp_sti_st0(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    let y = *cpu.fpu.st0();
    let x = cpu.fpu.get(instr.op0_register());
    *x = y / *x;
    cpu.fpu.pop();
}

pub fn fidivr_m32int(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = mem.get_pod::<u32>(x86_addr(cpu, instr)) as i32 as f64;
    let x = cpu.fpu.st0();
    *x = y / *x;
}

pub fn fprem(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let y = *cpu.fpu.get(iced_x86::Register::ST1);
    let x = cpu.fpu.st0();
    *x = *x % y;
}

pub fn fxch_st0_sti(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    cpu.fpu.swap(instr.op0_register(), instr.op1_register());
}

pub fn fcom_m32fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = *cpu.fpu.st0();
    let y = mem.get_pod::<f32>(x86_addr(cpu, instr)) as f64;
    fcom(cpu, x, y);
}

pub fn fcomp_m32fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    fcom_m32fp(cpu, mem, instr);
    cpu.fpu.pop();
}

pub fn fcomp_m64fp(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = *cpu.fpu.st0();
    let y = mem.get_pod::<f64>(x86_addr(cpu, instr));
    fcom(cpu, x, y);
    cpu.fpu.pop();
}

pub fn fcomp_st0_sti(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    let x = *cpu.fpu.st0();
    let y = *cpu.fpu.get(instr.op1_register());
    fcom(cpu, x, y);
    cpu.fpu.pop();
}

pub fn fucomp_st0_sti(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    fcomp_st0_sti(cpu, mem, instr);
    // TODO: raise the invalid-arithmetic-operand exception when appropriate.
}

pub fn frndint(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let x = cpu.fpu.st0();
    *x = x.round();
}

pub fn fnstsw_ax(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    // TODO: does this need stack top in it?
    cpu.regs.set32(Register::EAX, cpu.fpu.status.bits() as u32);
}

pub fn fnstcw_m2byte(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    // TODO: control word
    let cw = 0x37u16; // default value
    let addr = x86_addr(cpu, instr);
    mem.put::<u16>(addr, cw);
}

pub fn fldcw_m2byte(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    // TODO: control word
    mem.get_pod::<u16>(x86_addr(cpu, instr));
}
