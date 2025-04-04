use crate::{registers::Flags, x86::CPU, CPUState};
use iced_x86::{Instruction, Register};
use memory::{Extensions, ExtensionsMut, Mem};

use super::helpers::*;

/// nop: No Operation
pub fn nop(_cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {}

/// enter: Make Stack Frame for Procedure Parameters
pub fn enterd_imm16_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    push(cpu, mem, cpu.regs.get32(Register::EBP));
    cpu.regs.set32(Register::EBP, cpu.regs.get32(Register::ESP));
    *cpu.regs.get32_mut(Register::ESP) -= instr.immediate16() as u32;
}

/// leave: High Level Procedure Exit
pub fn leaved(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    cpu.regs.set32(Register::ESP, cpu.regs.get32(Register::EBP));
    let ebp = pop(cpu, mem);
    cpu.regs.set32(Register::EBP, ebp);
}

/// push: Push Word, Doubleword, or Quadword Onto the Stack
pub fn pushd_r16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    // Pushing segment registers is subtle:
    // "If the source operand is a segment register (16 bits) and [...]
    // the operand size is 32-bits, either a zero-extended value is pushed on
    // the stack or the segment selector is written on the stack using a 16-bit move.
    // For the last case, all recent Intel Core and Intel Atom processors perform a
    // 16-bit move, leaving the upper portion of the stack location unmodified."
    // tldr it's a 32-bit move in any normal case.
    let x = cpu.regs.get16(instr.op0_register());
    push(cpu, mem, x as u32);
}

/// push: Push Word, Doubleword, or Quadword Onto the Stack
pub fn pushd_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    push(cpu, mem, instr.immediate8to32() as u32);
}

/// push: Push Word, Doubleword, or Quadword Onto the Stack
pub fn pushd_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    push(cpu, mem, instr.immediate32());
}

/// push: Push Word, Doubleword, or Quadword Onto the Stack
pub fn push_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    push(cpu, mem, cpu.regs.get32(instr.op0_register()));
}

/// push: Push Word, Doubleword, or Quadword Onto the Stack
pub fn push_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = rm32(cpu, mem, instr).get();
    push(cpu, mem, value);
}

/// push: Push Word, Doubleword, or Quadword Onto the Stack
pub fn push_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = rm16(cpu, mem, instr).get();
    push16(cpu, mem, value);
}

/// pop: Pop a Value From the Stack
pub fn popd_r16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    // See discussion in pushd_r16.
    let value = pop(cpu, mem);
    cpu.regs.set16(instr.op0_register(), value as u16);
}

/// pop: Pop a Value From the Stack
pub fn pop_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = pop(cpu, mem);
    let x = rm32(cpu, mem, instr);
    x.set(value);
}

/// pop: Pop a Value From the Stack
pub fn pop_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = pop16(cpu, mem);
    let x = rm16(cpu, mem, instr);
    x.set(value);
}

/// mov: Move
pub fn mov_rm32_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm32(cpu, mem, instr);
    x.set(instr.immediate32());
}

/// mov: Move
pub fn mov_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = cpu.regs.get32(instr.op1_register());
    let x = rm32(cpu, mem, instr);
    x.set(value);
}

/// mov: Move
pub fn mov_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = op1_rm32(cpu, mem, instr);
    cpu.regs.set32(instr.op0_register(), value);
}

/// mov: Move
pub fn mov_r16_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = op1_rm16(cpu, mem, instr);
    cpu.regs.set16(instr.op0_register(), value);
}

/// mov: Move
pub fn mov_rm16_r16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get16(instr.op1_register());
    let x = rm16(cpu, mem, instr);
    x.set(y);
}

/// mov: Move
pub fn mov_rm16_imm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate16();
    let x = rm16(cpu, mem, instr);
    x.set(y);
}

/// mov: Move
pub fn mov_r8_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = op1_rm8(cpu, mem, instr);
    cpu.regs.set8(instr.op0_register(), value);
}

/// mov: Move
pub fn mov_rm8_r8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(instr.op1_register());
    let x = rm8(cpu, mem, instr);
    x.set(y);
}

/// mov: Move
pub fn mov_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm8(cpu, mem, instr);
    x.set(y);
}

/// mov: Move
pub fn mov_moffs8_al(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let addr = x86_addr(cpu, instr);
    mem.put_pod::<u8>(addr, cpu.regs.get8(Register::AL));
}

/// mov: Move
pub fn mov_r32m16_sreg(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    // This weirdly is either a 16-bit or 32-write, so we must match to determine.
    let y = cpu.regs.get16(instr.op1_register());
    match instr.op0_kind() {
        iced_x86::OpKind::Register => cpu.regs.set32(instr.op0_register(), y as u32),
        iced_x86::OpKind::Memory => {
            let addr = x86_addr(cpu, instr);
            mem.put_pod::<u16>(addr, y)
        }
        _ => unimplemented!(),
    }
}

/// mov: Move
pub fn mov_sreg_r32m16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    // This weirdly is either a 16-bit or 32-write, so we must match to determine.
    // TODO: this is supposed to do segment selector validation stuff.
    let y = match instr.op1_kind() {
        iced_x86::OpKind::Register => cpu.regs.get32(instr.op1_register()) as u16,
        iced_x86::OpKind::Memory => mem.get_pod::<u16>(x86_addr(cpu, instr)),
        _ => unimplemented!(),
    };
    cpu.regs.set16(instr.op0_register(), y);
}

/// movsx: Move With Sign-Extension
pub fn movsx_r32_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm16(cpu, mem, instr) as i16 as u32;
    let x = rm32(cpu, mem, instr);
    x.set(y);
}

/// movsx: Move With Sign-Extension
pub fn movsx_r32_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as i8 as u32;
    let x = rm32(cpu, mem, instr);
    x.set(y);
}

/// movsx: Move With Sign-Extension
pub fn movsx_r16_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as i8 as u16;
    let x = rm16(cpu, mem, instr);
    x.set(y);
}

/// movzx: Move With Zero-Extend
pub fn movzx_r32_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm16(cpu, mem, instr) as u32;
    let x = rm32(cpu, mem, instr);
    x.set(y);
}

/// movzx: Move With Zero-Extend
pub fn movzx_r32_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as u32;
    let x = rm32(cpu, mem, instr);
    x.set(y);
}

/// movzx: Move With Zero-Extend
pub fn movzx_r16_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr) as u16;
    let x = rm16(cpu, mem, instr);
    x.set(y);
}

/// cmov: Conditional Move
pub fn cmovb_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    if cpu.flags.contains(Flags::CF) {
        x.set(y);
    }
}

/// cmov: Conditional Move
pub fn cmovne_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    if !cpu.flags.contains(Flags::ZF) {
        x.set(y);
    }
}

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

/// pushad: Push All General-Purpose Registers
pub fn pushad(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let esp = cpu.regs.get32(Register::ESP); // get before any pushes

    push(cpu, mem, cpu.regs.get32(Register::EAX));
    push(cpu, mem, cpu.regs.get32(Register::ECX));
    push(cpu, mem, cpu.regs.get32(Register::EDX));
    push(cpu, mem, cpu.regs.get32(Register::EBX));
    push(cpu, mem, esp);
    push(cpu, mem, cpu.regs.get32(Register::EBP));
    push(cpu, mem, cpu.regs.get32(Register::ESI));
    push(cpu, mem, cpu.regs.get32(Register::EDI));
}

/// popad: Pop All General-Purpose Registers
pub fn popad(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let edi = pop(cpu, mem);
    cpu.regs.set32(Register::EDI, edi);
    let esi = pop(cpu, mem);
    cpu.regs.set32(Register::ESI, esi);
    let ebp = pop(cpu, mem);
    cpu.regs.set32(Register::EBP, ebp);
    pop(cpu, mem); // ignore esp
    let ebx = pop(cpu, mem);
    cpu.regs.set32(Register::EBX, ebx);
    let edx = pop(cpu, mem);
    cpu.regs.set32(Register::EDX, edx);
    let ecx = pop(cpu, mem);
    cpu.regs.set32(Register::ECX, ecx);
    let eax = pop(cpu, mem);
    cpu.regs.set32(Register::EAX, eax);
}

/// cwde: Convert Byte to Word/Convert Word to Doubleword/Convert Doubleword toQuadword
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

/// bts: Bit Test and Set
pub fn bts_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr) % 32;
    let x = rm32(cpu, mem, instr);
    let mask = 1u32 << y;
    cpu.flags.set(Flags::CF, x.get() & mask != 0);
    x.set(x.get() | mask);
}

/// tzcnt: Count the Number of Trailing Zero Bits
pub fn tzcnt_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    let count = y.trailing_zeros();
    cpu.flags.set(Flags::CF, count == 32);
    cpu.flags.set(Flags::ZF, count == 0);
    x.set(count);
}

/// rdtsc: Read Time-Stamp Counter
pub fn rdtsc(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let value = 0;
    set_edx_eax(cpu, value);
}
