use super::helpers::*;
use crate::{CPUState, registers::Flags, x86::CPU};
use iced_x86::{Instruction, Register};
use memory::{Extensions, ExtensionsMut, Mem};

/// nop: No Operation
pub fn nop(_cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {}

/// xchg: Exchange Register/Memory With Register
pub fn xchg_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let r1 = instr.op1_register();
    let y = cpu.regs.get32(r1);
    let x = rm32(cpu, mem, instr);
    let tmp = x.get();
    x.set(y);
    cpu.regs.set32(r1, tmp);
}

/// xchg: Exchange Register/Memory With Register
pub fn xchg_rm16_r16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let r1 = instr.op1_register();
    let y = cpu.regs.get16(r1);
    let x = rm16(cpu, mem, instr);
    let tmp = x.get();
    x.set(y);
    cpu.regs.set16(r1, tmp);
}

/// xchg: Exchange Register/Memory With Register
pub fn xchg_rm8_r8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let r1 = instr.op1_register();
    let y = cpu.regs.get8(r1);
    let x = rm8(cpu, mem, instr);
    let tmp = x.get();
    x.set(y);
    cpu.regs.set8(r1, tmp);
}

/// cmpxchg8b: Compare and Exchange Bytes
pub fn cmpxchg8b_m64(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let addr = x86_addr(cpu, instr);
    let prev = mem.get_pod::<u64>(addr);
    let test = get_edx_eax(cpu);
    if test == prev {
        cpu.flags.insert(Flags::ZF);
        let val =
            ((cpu.regs.get32(Register::ECX) as u64) << 32) | (cpu.regs.get32(Register::EBX) as u64);
        mem.put_pod::<u64>(addr, val);
    } else {
        cpu.flags.remove(Flags::ZF);
        set_edx_eax(cpu, prev);
    }
}

/// cmpxchg: Compare and Exchange
pub fn cmpxchg_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    assert_eq!(instr.op0_kind(), iced_x86::OpKind::Memory);

    let new = cpu.regs.get32(instr.op1_register());
    let prev = rm32(cpu, mem, instr);
    if cpu.regs.get32(Register::EAX) == prev.get() {
        cpu.flags.insert(Flags::ZF);
        prev.set(new);
    } else {
        cpu.flags.remove(Flags::ZF);
        cpu.regs.set32(Register::EAX, prev.get());
    }
}

/// cmpxchg: Compare and Exchange
pub fn cmpxchg_rm8_r8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    assert_eq!(instr.op0_kind(), iced_x86::OpKind::Memory);

    let new = op1_rm8(cpu, mem, instr);
    let prev = rm8(cpu, mem, instr);
    let test = cpu.regs.get8(Register::AL);
    if test == prev.get() {
        cpu.flags.insert(Flags::ZF);
        prev.set(new);
    } else {
        cpu.flags.remove(Flags::ZF);
        cpu.regs.set8(Register::AL, prev.get());
    }
}

/// lea: Load Effective Address
pub fn lea_r32_m(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    // lea eax,[esp+10h]
    cpu.regs.set32(instr.op0_register(), x86_addr(cpu, instr));
}

/// cwde: Convert Byte to Word/Convert Word to Doubleword/Convert Doubleword to Quadword
pub fn cwde(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let value = cpu.regs.get16(Register::AX) as i16 as i32;
    cpu.regs.set32(Register::EAX, value as u32);
}

/// cdq: Convert Word to Doubleword/Convert Doubleword to Quadword
pub fn cdq(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let edx = if cpu.regs.get32(Register::EAX) >> 31 == 0 {
        0
    } else {
        0xFFFF_FFFF
    };
    cpu.regs.set32(Register::EDX, edx);
}

/// int3: Call to Interrupt Procedure
pub fn int3(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    log::warn!("debugger interrupt");
    cpu.state = CPUState::DebugBreak;
    cpu.regs.eip -= 1;
}

/// ud2: Undefined Instruction
pub fn ud2(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.err("ud: undefined instruction".into());
    cpu.regs.eip -= 2;
}

/// sysenter: Fast System Call
pub fn sysenter(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.state = CPUState::SysCall;
}

/// bswap: Byte Swap
pub fn bswap_r32(cpu: &mut CPU, _mem: Mem, instr: &Instruction) {
    let reg = instr.op0_register();
    let val = cpu.regs.get32(reg);
    cpu.regs.set32(
        reg,
        ((val >> 24) & 0xFF) << 0
            | ((val >> 16) & 0xFF) << 8
            | ((val >> 8) & 0xFF) << 16
            | ((val >> 0) & 0xFF) << 24,
    );
}

/// xlat: Table Look-up Translation
pub fn xlat_m8(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let addr = cpu.regs.get32(Register::EBX) + (cpu.regs.get8(Register::AL) as u32);
    cpu.regs.set8(Register::AL, mem.get_pod::<u8>(addr));
}

/// rdtsc: Read Time-Stamp Counter
pub fn rdtsc(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let value = 0;
    set_edx_eax(cpu, value);
}
