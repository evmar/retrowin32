//! Functions for common behaviors across all operations.

use crate::x86::CPU;
use memory::Mem;

// TODO: maybe there are no 64-bit memory reads needed (?)
pub fn rm64_x(
    cpu: &mut CPU,
    mem: Mem,
    instr: &iced_x86::Instruction,
    op: impl FnOnce(&mut CPU, u64) -> u64,
) {
    match instr.op0_kind() {
        iced_x86::OpKind::Register => {
            let reg = instr.op0_register();
            let x = cpu.regs.get64(reg);
            let value = op(cpu, x);
            cpu.regs.set64(reg, value);
        }
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(cpu, instr);
            let x = mem.get::<u64>(addr);
            let value = op(cpu, x);
            mem.put::<u64>(addr, value);
        }
        _ => unimplemented!(),
    }
}

/// Many x86 operations take an argument that is both read from and written to,
/// and which can refer to either a register or memory, e.g.
///   mov [...],3
///   and eax,4
/// This 'Arg' type wraps that left argument using a pointer internally.
/// This code was previously instead carefully using a (safe) mut reference instead,
/// but it turns out that memory accesses can be unaligned and Rust does not allow
/// references to unaligned memory.  It turns out a lot easier to not need to worry
/// about lifetimes anyway.
pub struct Arg<T>(*mut T);

impl<T> Arg<T> {
    pub fn get(&self) -> T {
        unsafe { std::ptr::read_unaligned(self.0) }
    }

    pub fn set(&self, val: T) {
        unsafe { std::ptr::write_unaligned(self.0, val) }
    }
}

pub fn rm32<'a>(cpu: &'a mut CPU, mem: Mem, instr: &iced_x86::Instruction) -> Arg<u32> {
    match instr.op0_kind() {
        iced_x86::OpKind::Register => {
            let reg = instr.op0_register();
            Arg(cpu.regs.get32_mut(reg))
        }
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(cpu, instr);
            Arg(mem.ptr_mut::<u32>(addr))
        }
        _ => unimplemented!(),
    }
}

pub fn rm16<'a>(cpu: &'a mut CPU, mem: Mem, instr: &iced_x86::Instruction) -> Arg<u16> {
    match instr.op0_kind() {
        iced_x86::OpKind::Register => {
            let reg = instr.op0_register();
            Arg(cpu.regs.get16_mut(reg))
        }
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(cpu, instr);
            Arg(mem.ptr_mut::<u16>(addr))
        }
        _ => unimplemented!(),
    }
}

pub fn rm8<'a>(cpu: &'a mut CPU, mem: Mem, instr: &iced_x86::Instruction) -> Arg<u8> {
    match instr.op0_kind() {
        iced_x86::OpKind::Register => {
            let reg = instr.op0_register();
            Arg(cpu.regs.get8_mut(reg))
        }
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(cpu, instr);
            Arg(mem.ptr_mut::<u8>(addr))
        }
        _ => unimplemented!(),
    }
}

pub fn op1_rm32(cpu: &mut CPU, mem: Mem, instr: &iced_x86::Instruction) -> u32 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => cpu.regs.get32(instr.op1_register()),
        iced_x86::OpKind::Memory => mem.get::<u32>(x86_addr(cpu, instr)),
        _ => unreachable!(),
    }
}

pub fn op1_rm16(cpu: &mut CPU, mem: Mem, instr: &iced_x86::Instruction) -> u16 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => cpu.regs.get16(instr.op1_register()),
        iced_x86::OpKind::Memory => mem.get::<u16>(x86_addr(cpu, instr)),
        _ => unreachable!(),
    }
}

pub fn op1_rm8(cpu: &mut CPU, mem: Mem, instr: &iced_x86::Instruction) -> u8 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => cpu.regs.get8(instr.op1_register()),
        iced_x86::OpKind::Memory => mem.get::<u8>(x86_addr(cpu, instr)),
        _ => unreachable!(),
    }
}

/// Push a u32 on the x86 stack.
pub fn push(cpu: &mut CPU, mem: Mem, value: u32) {
    cpu.regs.esp -= 4;
    mem.put::<u32>(cpu.regs.esp, value);
}

/// Push a u16 on the x86 stack.
pub fn push16(cpu: &mut CPU, mem: Mem, value: u16) {
    cpu.regs.esp -= 2;
    mem.put::<u16>(cpu.regs.esp, value);
}

/// Pop a u32 from the x86 stack.
pub fn pop(cpu: &mut CPU, mem: Mem) -> u32 {
    let value = mem.get::<u32>(cpu.regs.esp);
    cpu.regs.esp += 4;
    value
}

/// Pop a u16 from the x86 stack.
pub fn pop16(cpu: &mut CPU, mem: Mem) -> u16 {
    let value = mem.get::<u16>(cpu.regs.esp);
    cpu.regs.esp += 2;
    value
}

/// Compute the address found in instructions that reference memory, e.g.
///   mov [eax+03h],...
pub fn x86_addr(cpu: &CPU, instr: &iced_x86::Instruction) -> u32 {
    // TODO: see comments on regs.fs_addr.
    let seg = match instr.segment_prefix() {
        iced_x86::Register::FS => cpu.regs.fs_addr,
        _ => 0,
    };

    let base = if instr.memory_base() != iced_x86::Register::None {
        cpu.regs.get32(instr.memory_base())
    } else {
        0
    };
    let index = if instr.memory_index() != iced_x86::Register::None {
        cpu.regs
            .get32(instr.memory_index())
            .wrapping_mul(instr.memory_index_scale())
    } else {
        0
    };
    // In general these operations aren't written to wrap, but in some cases
    // the components are negative which is implemented in two's complement by
    // a wrapping add.
    seg.wrapping_add(base)
        .wrapping_add(index)
        .wrapping_add(instr.memory_displacement32())
}

pub fn x86_jmp(cpu: &mut CPU, addr: u32) {
    if addr < 0x1000 {
        cpu.state = Err("jmp to null page".into());
        return;
    }
    cpu.regs.eip = addr;
}
