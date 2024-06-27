use super::helpers::*;
use crate::CPU;
use iced_x86::Instruction;
use memory::{Extensions, Mem};

fn op1_mmm64(cpu: &mut CPU, mem: Mem, instr: &iced_x86::Instruction) -> u64 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => cpu.regs.get64(instr.op1_register()),
        iced_x86::OpKind::Memory => mem.get_pod::<u64>(x86_addr(cpu, instr)),
        _ => unreachable!(),
    }
}

fn op1_mmm32(cpu: &mut CPU, mem: Mem, instr: &iced_x86::Instruction) -> u32 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => cpu.regs.get64(instr.op1_register()) as u32,
        iced_x86::OpKind::Memory => mem.get_pod::<u32>(x86_addr(cpu, instr)),
        _ => unreachable!(),
    }
}

// Note: looking at compiler explorer, it appears splitting into an array, iterating the
// array, and reassembling the output all gets inlined by the compiler into the same code
// as doing it with a bunch of bitshift expressions.

trait Unpack<T> {
    fn unpack(self) -> T;
}

impl Unpack<[u32; 2]> for u64 {
    fn unpack(self) -> [u32; 2] {
        [(self >> 0) as u32, (self >> 32) as u32]
    }
}

impl Unpack<[i32; 2]> for u64 {
    fn unpack(self) -> [i32; 2] {
        [(self >> 0) as i32, (self >> 32) as i32]
    }
}

impl Unpack<[u16; 4]> for u64 {
    fn unpack(self) -> [u16; 4] {
        [
            (self >> 0) as u16,
            (self >> 16) as u16,
            (self >> 32) as u16,
            (self >> 48) as u16,
        ]
    }
}

impl Unpack<[i16; 4]> for u64 {
    fn unpack(self) -> [i16; 4] {
        let x: [u16; 4] = self.unpack();
        [x[0] as i16, x[1] as i16, x[2] as i16, x[3] as i16]
    }
}

impl Unpack<[u8; 8]> for u64 {
    fn unpack(self) -> [u8; 8] {
        self.to_le_bytes()
    }
}

impl Unpack<[u16; 2]> for u32 {
    fn unpack(self) -> [u16; 2] {
        [(self >> 0) as u16, (self >> 16) as u16]
    }
}

impl Unpack<[u8; 4]> for u32 {
    fn unpack(self) -> [u8; 4] {
        self.to_le_bytes()
    }
}

trait Pack {
    type Target;
    fn pack(self) -> Self::Target;
}

impl Pack for [u32; 2] {
    type Target = u64;
    fn pack(self) -> u64 {
        (self[0] as u64) | ((self[1] as u64) << 32)
    }
}

impl Pack for [u16; 4] {
    type Target = u64;
    fn pack(self) -> u64 {
        (self[0] as u64)
            | ((self[1] as u64) << 16)
            | ((self[2] as u64) << 32)
            | ((self[3] as u64) << 48)
    }
}

impl Pack for [u8; 8] {
    type Target = u64;
    fn pack(self) -> u64 {
        u64::from_le_bytes(self)
    }
}

impl Pack for [u16; 2] {
    type Target = u32;
    fn pack(self) -> u32 {
        (self[0] as u32) | ((self[1] as u32) << 16)
    }
}

impl Pack for [u8; 4] {
    type Target = u32;
    fn pack(self) -> u32 {
        u32::from_le_bytes(self)
    }
}

pub fn pxor_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| x ^ y);
}

pub fn movq_mmm64_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, _x| y);
}

pub fn movd_mm_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr) as u64; // zero extend
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
        let x: [u16; 2] = (x as u32).unpack(); // instr only uses low 32 bits of x
        let y: [u16; 2] = y.unpack();
        [x[0], y[0], x[1], y[1]].pack()
    });
}

pub fn punpcklbw_mm_mmm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm32(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [u8; 4] = (x as u32).unpack(); // instr only uses low 32 bits of x
        let y: [u8; 4] = y.unpack();
        [x[0], y[0], x[1], y[1], x[2], y[2], x[3], y[3]].pack()
    });
}

pub fn pmullw_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [i16; 4] = x.unpack();
        let y: [i16; 4] = y.unpack();
        let out: [u16; 4] = std::array::from_fn(|i| {
            let x = x[i] as i32;
            let y = y[i] as i32;
            (x * y) as u16
        });
        out.pack()
    });
}

pub fn pmulhw_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [i16; 4] = x.unpack();
        let y: [i16; 4] = y.unpack();
        let out: [u16; 4] = std::array::from_fn(|i| {
            let x = x[i] as i32;
            let y = y[i] as i32;
            ((x * y) >> 16) as u16
        });
        out.pack()
    });
}

pub fn psraw_mm_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [i16; 4] = x.unpack();
        let out: [u16; 4] = std::array::from_fn(|i| (x[i] >> y) as u16);
        out.pack()
    });
}

pub fn psrad_mm_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [i32; 2] = x.unpack();
        let out: [u32; 2] = std::array::from_fn(|i| (x[i] >> y) as u32);
        out.pack()
    });
}

pub fn psrlw_mm_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [u16; 4] = x.unpack();
        let out: [u16; 4] = std::array::from_fn(|i| x[i] >> y);
        out.pack()
    });
}

pub fn psrlq_mm_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    rm64_x(cpu, mem, instr, |_cpu, x| x >> y);
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
        let x: [i16; 4] = x.unpack();
        let y: [i16; 4] = y.unpack();
        let lo: [u8; 4] = std::array::from_fn(|i| saturate(x[i]));
        let hi: [u8; 4] = std::array::from_fn(|i| saturate(y[i]));
        [lo.pack(), hi.pack()].pack()
    });
}

pub fn emms(_cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    // This is supposed to reset the FPU tag word, but I don't have one of those
    // because I'm not yet clear on what it's actually used for...
}

pub fn psubusb_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [u8; 8] = x.unpack();
        let y: [u8; 8] = y.unpack();
        let out: [u8; 8] = std::array::from_fn(|i| x[i].saturating_sub(y[i]));
        out.pack()
    });
}

pub fn paddusb_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [u8; 8] = x.unpack();
        let y: [u8; 8] = y.unpack();
        let out: [u8; 8] = std::array::from_fn(|i| x[i].saturating_add(y[i]));
        out.pack()
    });
}

pub fn psllw_mm_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [u16; 4] = x.unpack();
        let out: [u16; 4] = std::array::from_fn(|i| x[i] << y);
        out.pack()
    });
}

pub fn paddsb_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [u8; 8] = x.unpack();
        let y: [u8; 8] = y.unpack();
        let out: [u8; 8] = std::array::from_fn(|i| (x[i] as i8).saturating_add(y[i] as i8) as u8);
        out.pack()
    });
}

pub fn paddw_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [u16; 4] = x.unpack();
        let y: [u16; 4] = y.unpack();
        let out: [u16; 4] = std::array::from_fn(|i| x[i].wrapping_add(y[i]));
        out.pack()
    });
}

pub fn paddd_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [u32; 2] = x.unpack();
        let y: [u32; 2] = y.unpack();
        let out: [u32; 2] = std::array::from_fn(|i| x[i].wrapping_add(y[i]));
        out.pack()
    });
}

pub fn paddsw_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [i16; 4] = x.unpack();
        let y: [i16; 4] = y.unpack();
        let out: [u16; 4] = std::array::from_fn(|i| x[i].saturating_add(y[i]) as u16);
        out.pack()
    });
}

pub fn pmaddwd_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [i16; 4] = x.unpack();
        let y: [i16; 4] = y.unpack();
        let prod: [i32; 4] = std::array::from_fn(|i| x[i] as i32 * y[i] as i32);
        let out: [u32; 2] = [(prod[1] + prod[0]) as u32, (prod[3] + prod[2]) as u32];
        out.pack()
    });
}

pub fn psubw_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [u16; 4] = x.unpack();
        let y: [u16; 4] = y.unpack();
        let out: [u16; 4] = std::array::from_fn(|i| x[i].wrapping_sub(y[i]));
        out.pack()
    });
}

pub fn pcmpeqb_mm_mmm64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_mmm64(cpu, mem, instr);
    rm64_x(cpu, mem, instr, |_cpu, x| {
        let x: [u8; 8] = x.unpack();
        let y: [u8; 8] = y.unpack();
        let out: [u8; 8] = std::array::from_fn(|i| if x[i] == y[i] { 0xFF } else { 0 });
        out.pack()
    });
}
