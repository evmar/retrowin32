use iced_x86::Instruction;

use crate::{memory::Memory, registers::Flags, x86::X86};

pub fn nop(_x86: &mut X86, _instr: &Instruction) -> anyhow::Result<()> {
    Ok(())
}

pub fn enterd_imm16_imm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    x86.push(x86.regs.ebp);
    x86.regs.ebp = x86.regs.esp;
    x86.regs.esp -= instr.immediate16() as u32;
    Ok(())
}

pub fn leaved(x86: &mut X86, _instr: &Instruction) -> anyhow::Result<()> {
    x86.regs.esp = x86.regs.ebp;
    x86.regs.ebp = x86.pop();
    Ok(())
}

pub fn pushd_imm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    x86.push(instr.immediate8to32() as u32);
    Ok(())
}

pub fn pushd_imm32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    x86.push(instr.immediate32());
    Ok(())
}

pub fn push_r32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    x86.push(x86.regs.get32(instr.op0_register()));
    Ok(())
}

pub fn push_rm32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let value = x86.op0_rm32(instr);
    x86.push(value);
    Ok(())
}

pub fn push_rm16(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let value = x86.op0_rm16(instr);
    x86.push16(value);
    Ok(())
}

pub fn pop_rm32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let value = x86.pop();
    x86.rm32_x(instr, |_x86, _x| value);
    Ok(())
}

pub fn pop_rm16(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let value = x86.pop16();
    x86.rm16_x(instr, |_x86, _x| value);
    Ok(())
}

pub fn mov_rm32_imm32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    // mov dword ptr [x], y
    // TODO: why is this 'rm32' when there is an r32 variant just below?
    x86.rm32_x(instr, |_x86, _x| instr.immediate32());
    Ok(())
}

pub fn mov_r32_imm32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    x86.regs.set32(instr.op0_register(), instr.immediate32());
    Ok(())
}

pub fn mov_moffs32_eax(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    // mov [x],eax
    x86.write_u32(x86.addr(instr), x86.regs.eax);
    Ok(())
}

pub fn mov_eax_moffs32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    // mov eax,[x]
    x86.regs.eax = x86.read_u32(x86.addr(instr));
    Ok(())
}

pub fn mov_rm32_r32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let value = x86.regs.get32(instr.op1_register());
    x86.rm32_x(instr, |_x86, _x| value);
    Ok(())
}

pub fn mov_r32_rm32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    x86.regs.set32(instr.op0_register(), x86.op1_rm32(instr));
    Ok(())
}

pub fn mov_r16_rm16(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    x86.regs.set16(instr.op0_register(), x86.op1_rm16(instr));
    Ok(())
}

pub fn mov_rm16_r16(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let y = x86.regs.get16(instr.op1_register());
    x86.rm16_x(instr, |_x86, _x| y);
    Ok(())
}

pub fn mov_r8_rm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    x86.regs.set8(instr.op0_register(), x86.op1_rm8(instr));
    Ok(())
}

pub fn mov_rm8_r8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let y = x86.regs.get8(instr.op1_register());
    x86.rm8_x(instr, |_x86, _x| y);
    Ok(())
}

pub fn mov_rm8_imm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let y = instr.immediate8();
    x86.rm8_x(instr, |_x86, _x| y);
    Ok(())
}

pub fn movsx_r32_rm16(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let y = x86.op1_rm16(instr) as i16 as u32;
    x86.rm32_x(instr, |_x86, _x| y);
    Ok(())
}

pub fn movsx_r32_rm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let y = x86.op1_rm8(instr) as i8 as u32;
    x86.rm32_x(instr, |_x86, _x| y);
    Ok(())
}

pub fn movsx_r16_rm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let y = x86.op1_rm8(instr) as i8 as u16;
    x86.rm16_x(instr, |_x86, _x| y);
    Ok(())
}

pub fn movzx_r32_rm16(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let y = x86.op1_rm16(instr) as u32;
    x86.rm32_x(instr, |_x86, _x| y);
    Ok(())
}

pub fn movzx_r32_rm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let y = x86.op1_rm8(instr) as u32;
    x86.rm32_x(instr, |_x86, _x| y);
    Ok(())
}

pub fn movzx_r16_rm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let y = x86.op1_rm8(instr) as u16;
    x86.rm16_x(instr, |_x86, _x| y);
    Ok(())
}

pub fn xchg_rm32_r32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let r1 = instr.op1_register();
    x86.rm32_x(instr, |x86, x| {
        let tmp = x86.regs.get32(r1);
        x86.regs.set32(r1, x);
        tmp
    });
    Ok(())
}

pub fn cmpxchg_rm32_r32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let y = x86.regs.get32(instr.op1_register());
    match instr.op0_kind() {
        iced_x86::OpKind::Register => todo!(),
        iced_x86::OpKind::Memory => {
            let addr = x86.addr(instr);
            let x = x86.mem.read_u32(addr);
            if x86.regs.eax == x {
                x86.mem.write_u32(addr, y);
            } else {
                x86.regs.eax = y;
            }
        }
        _ => unreachable!(),
    };
    Ok(())
}

pub fn lea_r32_m(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    // lea eax,[esp+10h]
    x86.regs.set32(instr.op0_register(), x86.addr(instr));
    Ok(())
}

pub fn sete_rm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let value = x86.regs.flags.contains(Flags::ZF) as u8;
    x86.rm8_x(instr, |_x86, _x| value);
    Ok(())
}

pub fn setne_rm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let value = !x86.regs.flags.contains(Flags::ZF) as u8;
    x86.rm8_x(instr, |_x86, _x| value);
    Ok(())
}

pub fn setge_rm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let value = (x86.regs.flags.contains(Flags::ZF) == x86.regs.flags.contains(Flags::OF)) as u8;
    x86.rm8_x(instr, |_x86, _x| value);
    Ok(())
}

pub fn pushad(x86: &mut X86, _instr: &Instruction) -> anyhow::Result<()> {
    let esp = x86.regs.esp;
    x86.push(x86.regs.eax);
    x86.push(x86.regs.ecx);
    x86.push(x86.regs.edx);
    x86.push(x86.regs.ebx);
    x86.push(esp);
    x86.push(x86.regs.ebp);
    x86.push(x86.regs.esi);
    x86.push(x86.regs.edi);
    Ok(())
}

pub fn popad(x86: &mut X86, _instr: &Instruction) -> anyhow::Result<()> {
    x86.regs.edi = x86.pop();
    x86.regs.esi = x86.pop();
    x86.regs.ebp = x86.pop();
    x86.pop(); // ignore esp
    x86.regs.ebx = x86.pop();
    x86.regs.edx = x86.pop();
    x86.regs.ecx = x86.pop();
    x86.regs.eax = x86.pop();
    Ok(())
}

pub fn pushfd(x86: &mut X86, _instr: &Instruction) -> anyhow::Result<()> {
    x86.push(x86.regs.flags.bits());
    Ok(())
}

pub fn pushfw(x86: &mut X86, _instr: &Instruction) -> anyhow::Result<()> {
    let value = (x86.regs.flags.bits() & 0x0000_FFFF) as u16;
    x86.push16(value);
    Ok(())
}

pub fn popfd(x86: &mut X86, _instr: &Instruction) -> anyhow::Result<()> {
    x86.regs.flags = Flags::from_bits(x86.pop()).unwrap();
    Ok(())
}

pub fn popfw(x86: &mut X86, _instr: &Instruction) -> anyhow::Result<()> {
    let prev = Flags::from_bits(x86.regs.flags.bits() & 0xFFFF_0000).unwrap();
    let new = Flags::from_bits(x86.pop16() as u32).unwrap();
    x86.regs.flags = prev.union(new);
    Ok(())
}

pub fn sahf(x86: &mut X86, _instr: &Instruction) -> anyhow::Result<()> {
    let ah = (x86.regs.eax >> 8) as u8;
    x86.regs.flags = Flags::from_bits((x86.regs.flags.bits() & 0xFFFF_FF00) | ah as u32).unwrap();
    Ok(())
}

pub fn std(x86: &mut X86, _instr: &Instruction) -> anyhow::Result<()> {
    x86.regs.flags.insert(Flags::DF);
    Ok(())
}

pub fn cld(x86: &mut X86, _instr: &Instruction) -> anyhow::Result<()> {
    x86.regs.flags.remove(Flags::DF);
    Ok(())
}

pub fn cwde(x86: &mut X86, _instr: &Instruction) -> anyhow::Result<()> {
    x86.regs.eax = x86.regs.eax as i16 as i32 as u32;
    Ok(())
}

pub fn cdq(x86: &mut X86, _instr: &Instruction) -> anyhow::Result<()> {
    x86.regs.edx = if x86.regs.eax >> 31 == 0 {
        0
    } else {
        0xFFFF_FFFF
    };
    Ok(())
}
