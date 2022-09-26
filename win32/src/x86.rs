use std::collections::HashMap;

use anyhow::bail;
use bitflags::bitflags;
use tsify::Tsify;

use crate::windows::AppState;

// Addresses from 0 up to this point cause panics if we access them.
// This helps catch implementation bugs earlier.
pub const NULL_POINTER_REGION_SIZE: u32 = 0x1000;

bitflags! {
    pub struct Flags: u32 {
        const ZF = 0x40;
    }
}

#[derive(Tsify)]
pub struct Registers {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,

    pub esp: u32,
    pub ebp: u32,
    pub esi: u32,
    pub edi: u32,

    pub eip: u32,

    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,

    pub flags: Flags,
}
impl Registers {
    fn new() -> Self {
        Registers {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
            esp: 0,
            ebp: 0,
            esi: 0,
            edi: 0,
            eip: 0,
            cs: 0,
            ds: 0,
            es: 0,
            fs: 0,
            gs: 0,
            ss: 0,
            flags: Flags::empty(),
        }
    }

    fn get32(&self, name: iced_x86::Register) -> u32 {
        match name {
            iced_x86::Register::EAX => self.eax,
            iced_x86::Register::EBX => self.ebx,
            iced_x86::Register::ECX => self.ecx,
            iced_x86::Register::EDX => self.edx,
            iced_x86::Register::ESP => self.esp,
            iced_x86::Register::EBP => self.ebp,
            iced_x86::Register::ESI => self.esi,
            iced_x86::Register::EDI => self.edi,
            _ => unreachable!("{name:?}"),
        }
    }
    fn get16(&self, name: iced_x86::Register) -> u16 {
        match name {
            iced_x86::Register::AX => self.eax as u16,
            iced_x86::Register::CX => self.ecx as u16,
            iced_x86::Register::DX => self.edx as u16,
            iced_x86::Register::BX => self.ebx as u16,
            iced_x86::Register::SP => self.esp as u16,
            iced_x86::Register::BP => self.ebp as u16,
            iced_x86::Register::SI => self.esi as u16,
            iced_x86::Register::DI => self.edi as u16,
            iced_x86::Register::ES => self.es,
            iced_x86::Register::CS => self.cs,
            iced_x86::Register::SS => self.ss,
            iced_x86::Register::DS => self.ds,
            iced_x86::Register::FS => self.fs,
            iced_x86::Register::GS => self.gs,
            _ => unreachable!(),
        }
    }
    fn get8(&self, name: iced_x86::Register) -> u8 {
        match name {
            iced_x86::Register::AL => self.eax as u8,
            iced_x86::Register::CL => self.ecx as u8,
            iced_x86::Register::DL => self.edx as u8,
            iced_x86::Register::BL => self.ebx as u8,
            iced_x86::Register::AH => (self.eax >> 8) as u8,
            iced_x86::Register::CH => (self.ecx >> 8) as u8,
            iced_x86::Register::DH => (self.edx >> 8) as u8,
            iced_x86::Register::BH => (self.ebx >> 8) as u8,
            _ => unreachable!("{name:?}"),
        }
    }

    fn set32(&mut self, name: iced_x86::Register, value: u32) {
        match name {
            iced_x86::Register::EAX => self.eax = value,
            iced_x86::Register::EBX => self.ebx = value,
            iced_x86::Register::ECX => self.ecx = value,
            iced_x86::Register::EDX => self.edx = value,
            iced_x86::Register::ESP => self.esp = value,
            iced_x86::Register::EBP => self.ebp = value,
            iced_x86::Register::ESI => self.esi = value,
            iced_x86::Register::EDI => self.edi = value,
            _ => unreachable!(),
        }
    }
}

pub trait OS {
    fn exit(&self, code: u32);
    fn write(&self, buf: &[u8]) -> usize;
}

pub struct X86<'a> {
    pub os: &'a dyn OS,
    pub mem: Vec<u8>,
    pub regs: Registers,
    pub imports: HashMap<u32, Option<fn(&mut X86)>>,
    pub state: AppState,
}
impl<'a> X86<'a> {
    pub fn new(os: &'a dyn OS) -> Self {
        let mut regs = Registers::new();
        regs.eax = 0xdeadbeea;
        regs.ebx = 0xdeadbeeb;
        regs.ecx = 0xdeadbeec;
        regs.edx = 0xdeadbeed;
        regs.esi = 0xdeadbe51;
        regs.edi = 0xdeadbed1;
        X86 {
            os,
            mem: Vec::new(),
            regs,
            imports: HashMap::new(),
            state: AppState::new(),
        }
    }

    pub fn write_u32(&mut self, offset: u32, value: u32) {
        if offset < NULL_POINTER_REGION_SIZE {
            panic!("null pointer write at {offset:#x}");
        }
        let offset = offset as usize;
        self.mem[offset] = (value >> 0) as u8;
        self.mem[offset + 1] = (value >> 8) as u8;
        self.mem[offset + 2] = (value >> 16) as u8;
        self.mem[offset + 3] = (value >> 24) as u8;
    }

    pub fn read_u32(&self, offset: u32) -> u32 {
        if offset < NULL_POINTER_REGION_SIZE {
            panic!("null pointer read at {offset:#x}");
        }
        let offset = offset as usize;
        ((self.mem[offset] as u32) << 0)
            | ((self.mem[offset + 1] as u32) << 8)
            | ((self.mem[offset + 2] as u32) << 16)
            | ((self.mem[offset + 3] as u32) << 24)
    }
    pub fn read_u16(&self, offset: u32) -> u16 {
        if offset < NULL_POINTER_REGION_SIZE {
            panic!("null pointer read at {offset:#x}");
        }
        let offset = offset as usize;
        ((self.mem[offset] as u16) << 0) | ((self.mem[offset + 1] as u16) << 8)
    }
    pub fn read_u8(&self, offset: u32) -> u8 {
        if offset < NULL_POINTER_REGION_SIZE {
            panic!("null pointer read at {offset:#x}");
        }
        self.mem[offset as usize]
    }

    pub fn push(&mut self, value: u32) {
        self.regs.esp -= 4;
        self.write_u32(self.regs.esp, value);
    }

    pub fn pop(&mut self) -> u32 {
        let value = self.read_u32(self.regs.esp);
        self.regs.esp += 4;
        value
    }

    /// Compute the address found in instructions that reference memory, e.g.
    ///   mov [eax+03h],...
    fn addr(&self, instr: &iced_x86::Instruction) -> u32 {
        assert!(instr.memory_index_scale() == 1); // TODO
        let seg = if instr.segment_prefix() == iced_x86::Register::FS {
            self.state.teb
        } else {
            0
        };
        let base = if instr.memory_base() != iced_x86::Register::None {
            self.regs.get32(instr.memory_base())
        } else {
            0
        };
        let index = if instr.memory_index() != iced_x86::Register::None {
            self.regs.get32(instr.memory_index())
        } else {
            0
        };
        (seg + base + index).wrapping_add(instr.memory_displacement32())
    }

    fn op1_rm32(&self, instr: &iced_x86::Instruction) -> u32 {
        match instr.op1_kind() {
            iced_x86::OpKind::Register => self.regs.get32(instr.op1_register()),
            iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
            _ => unreachable!(),
        }
    }

    fn add(&mut self, x: u32, y: u32) -> u32 {
        // TODO flags.
        x.wrapping_add(y)
    }

    fn sub32(&mut self, x: u32, y: u32) -> u32 {
        let result = x.wrapping_sub(y);
        // XXX "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
        self.regs.flags.set(Flags::ZF, result == 0);
        result
    }
    fn sub16(&mut self, x: u16, y: u16) -> u16 {
        let result = x.wrapping_sub(y);
        // XXX "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
        self.regs.flags.set(Flags::ZF, result == 0);
        result
    }
    fn sub8(&mut self, x: u8, y: u8) -> u8 {
        let result = x.wrapping_sub(y);
        // XXX "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
        self.regs.flags.set(Flags::ZF, result == 0);
        result
    }

    fn and(&mut self, x: u32, y: u32) -> u32 {
        let result = x & y;
        // XXX More flags.
        self.regs.flags.set(Flags::ZF, result == 0);
        result
    }

    fn try_invoke_handler(&mut self, addr: u32) -> bool {
        let handler = match self.imports.get(&addr) {
            Some(&handler) => handler,
            None => return false,
        };

        let ret = self.pop();
        match handler {
            Some(handler) => handler(self),
            None => log::error!("unimplemented import: {:x}", addr),
        };
        self.regs.eip = ret;
        true
    }

    fn run(&mut self, instr: &iced_x86::Instruction) -> anyhow::Result<()> {
        self.regs.eip = instr.next_ip() as u32;
        match instr.code() {
            iced_x86::Code::Enterd_imm16_imm8 => {
                self.push(self.regs.ebp);
                self.regs.ebp = self.regs.esp;
                self.regs.esp -= instr.immediate16() as u32;
            }

            iced_x86::Code::Call_rel32_32 => {
                self.push(self.regs.eip);
                self.regs.eip = instr.near_branch32();
            }
            iced_x86::Code::Call_rm32 => {
                // call dword ptr [addr]
                assert!(instr.memory_index() == iced_x86::Register::None);
                let target = self.read_u32(self.addr(instr));
                self.push(self.regs.eip);
                if !self.try_invoke_handler(target) {
                    self.regs.eip = target;
                }
            }
            iced_x86::Code::Retnd => self.regs.eip = self.pop(),

            iced_x86::Code::Jmp_rel8_32 => {
                self.regs.eip = instr.near_branch32();
            }
            iced_x86::Code::Jmp_rel32_32 => {
                self.regs.eip = instr.near_branch32();
            }
            iced_x86::Code::Jmp_rm32 => {
                let target = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get32(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
                    _ => unreachable!(),
                };
                if !self.try_invoke_handler(target) {
                    self.regs.eip = target;
                }
            }

            iced_x86::Code::Je_rel8_32 => {
                if self.regs.flags.contains(Flags::ZF) {
                    self.regs.eip = instr.near_branch32();
                }
            }
            iced_x86::Code::Je_rel32_32 => {
                if self.regs.flags.contains(Flags::ZF) {
                    self.regs.eip = instr.near_branch32();
                }
            }
            iced_x86::Code::Jne_rel8_32 => {
                if !self.regs.flags.contains(Flags::ZF) {
                    self.regs.eip = instr.near_branch32();
                }
            }

            iced_x86::Code::Pushd_imm8 => self.push(instr.immediate8to32() as u32),
            iced_x86::Code::Pushd_imm32 => self.push(instr.immediate32()),
            iced_x86::Code::Push_r32 => self.push(self.regs.get32(instr.op0_register())),
            iced_x86::Code::Push_rm32 => {
                // push [eax+10h]
                let value = self.read_u32(self.addr(instr));
                self.push(value);
            }

            iced_x86::Code::Pop_r32 => {
                let value = self.pop();
                self.regs.set32(instr.op0_register(), value);
            }

            iced_x86::Code::Mov_rm32_imm32 => {
                // mov dword ptr [x], y
                // TODO: why is this 'rm32' when there is an r32 variant just below?
                assert!(instr.op0_kind() == iced_x86::OpKind::Memory);
                self.write_u32(self.addr(instr), instr.immediate32());
            }
            iced_x86::Code::Mov_r32_imm32 => {
                self.regs.set32(instr.op0_register(), instr.immediate32());
            }
            iced_x86::Code::Mov_moffs32_EAX => {
                // mov [x],eax
                self.write_u32(self.addr(instr), self.regs.eax);
            }
            iced_x86::Code::Mov_EAX_moffs32 => {
                // mov eax,[x]
                self.regs.eax = self.read_u32(self.addr(instr));
            }
            iced_x86::Code::Mov_rm32_r32 => {
                let value = self.regs.get32(instr.op1_register());
                match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.set32(instr.op0_register(), value),
                    iced_x86::OpKind::Memory => self.write_u32(self.addr(instr), value),
                    _ => unreachable!(),
                }
            }
            iced_x86::Code::Mov_r32_rm32 => {
                self.regs.set32(instr.op0_register(), self.op1_rm32(instr));
            }
            iced_x86::Code::Movsb_m8_m8 => {
                if !instr.has_rep_prefix() {
                    bail!("expected rep movsb");
                }
                let dst = self.regs.edi as usize;
                let src = self.regs.esi as usize;
                let count = self.regs.ecx as usize;
                self.mem.copy_within(src..src+count, dst);
                self.regs.edi += count as u32;
                self.regs.esi += count as u32;
                self.regs.ecx = 0;
            }

            iced_x86::Code::And_rm32_imm8 => {
                match instr.op0_kind() {
                    iced_x86::OpKind::Register => {
                        let reg = instr.op0_register();
                        assert!(instr.op1_kind() == iced_x86::OpKind::Immediate8to32);
                        let x = self.regs.get32(reg);
                        let y = instr.immediate8to32() as u32;
                        let value = self.and(x, y);
                        self.regs.set32(reg, value);
                    }
                    iced_x86::OpKind::Memory => {
                        let addr = self.addr(instr);
                        let x = self.read_u32(addr);
                        let y = instr.immediate8to32() as u32;
                        let value = self.and(x, y);
                        self.write_u32(addr, value);
                    }
                    _ => unreachable!(),
                };
            }
            iced_x86::Code::Xor_rm32_r32 => {
                assert!(instr.op0_kind() == iced_x86::OpKind::Register);
                let reg = instr.op0_register();
                self.regs.set32(
                    reg,
                    self.regs.get32(reg) ^ self.regs.get32(instr.op1_register()),
                );
            }

            iced_x86::Code::Add_r32_rm32 => {
                let reg = instr.op0_register();
                let value = self.add(self.regs.get32(reg), self.op1_rm32(&instr));
                self.regs.set32(reg, value);
            }
            iced_x86::Code::Add_rm32_imm8 => {
                let reg = instr.op0_register();
                let value = self.add(self.regs.get32(reg), instr.immediate8to32() as u32);
                self.regs.set32(reg, value);
            }

            iced_x86::Code::Sub_rm32_imm8 => {
                assert!(instr.op0_kind() == iced_x86::OpKind::Register);
                assert!(instr.op1_kind() == iced_x86::OpKind::Immediate8to32);
                let reg = instr.op0_register();
                self.regs
                    .set32(reg, self.regs.get32(reg) - instr.immediate8to32() as u32);
            }
            iced_x86::Code::Sub_rm32_imm32 => {
                assert!(instr.op0_kind() == iced_x86::OpKind::Register);
                let reg = instr.op0_register();
                let value = self.sub32(self.regs.get32(reg), instr.immediate32());
                self.regs.set32(reg, value);
            }
            iced_x86::Code::Sub_rm32_r32 => {
                assert!(instr.op0_kind() == iced_x86::OpKind::Register);
                let reg = instr.op0_register();
                let value = self.sub32(self.regs.get32(reg), self.regs.get32(instr.op1_register()));
                self.regs.set32(reg, value);
            }

            iced_x86::Code::Lea_r32_m => {
                // lea eax,[esp+10h]
                self.regs.set32(instr.op0_register(), self.addr(instr));
            }

            iced_x86::Code::Cmp_rm32_r32 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get32(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = self.regs.get32(instr.op1_register());
                self.sub32(x, y);
            }
            iced_x86::Code::Cmp_rm8_imm8 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get8(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u8(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = instr.immediate8();
                self.sub8(x, y);
            }

            iced_x86::Code::Test_rm32_r32 => match instr.op0_kind() {
                iced_x86::OpKind::Register => {
                    self.and(
                        self.regs.get32(instr.op0_register()),
                        self.regs.get32(instr.op1_register()),
                    );
                }
                iced_x86::OpKind::Memory => {
                    self.and(
                        self.read_u32(self.addr(instr)),
                        self.regs.get32(instr.op1_register()),
                    );
                }
                _ => unreachable!(),
            },
            iced_x86::Code::Test_rm16_r16 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get16(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u16(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = self.regs.get16(instr.op1_register());
                self.sub16(x, y);
            }

            code => {
                self.regs.eip -= instr.len() as u32;
                bail!("unhandled instruction {:?}", code);
            }
        }
        Ok(())
    }

    pub fn step(&mut self) -> anyhow::Result<()> {
        let mut decoder = iced_x86::Decoder::with_ip(
            32,
            &self.mem[self.regs.eip as usize..],
            self.regs.eip as u64,
            iced_x86::DecoderOptions::NONE,
        );
        self.run(&decoder.decode())
    }
}
