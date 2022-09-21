use std::collections::HashMap;

use anyhow::bail;
use bitflags::bitflags;
use tsify::Tsify;

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

    fn get(&self, name: iced_x86::Register) -> u32 {
        match name {
            iced_x86::Register::None => 0,
            iced_x86::Register::EAX => self.eax,
            iced_x86::Register::EBX => self.ebx,
            iced_x86::Register::ECX => self.ecx,
            iced_x86::Register::EDX => self.edx,
            iced_x86::Register::ESP => self.esp,
            iced_x86::Register::EBP => self.ebp,
            iced_x86::Register::ESI => self.esi,
            iced_x86::Register::EDI => self.edi,
            /*            iced_x86::Register::CS => self.cs,
            iced_x86::Register::DS => self.ds,
            iced_x86::Register::ES => self.es,
            iced_x86::Register::FS => self.fs,
            iced_x86::Register::SS => self.ss,
            iced_x86::Register::GS => self.gs, */
            _ => todo!(),
        }
    }
    fn set(&mut self, name: iced_x86::Register, value: u32) {
        match name {
            iced_x86::Register::EAX => self.eax = value,
            iced_x86::Register::EBX => self.ebx = value,
            iced_x86::Register::ECX => self.ecx = value,
            iced_x86::Register::EDX => self.edx = value,
            iced_x86::Register::ESP => self.esp = value,
            iced_x86::Register::EBP => self.ebp = value,
            iced_x86::Register::ESI => self.esi = value,
            iced_x86::Register::EDI => self.edi = value,
            /*            iced_x86::Register::CS => self.cs,
            iced_x86::Register::DS => self.ds,
            iced_x86::Register::ES => self.es,
            iced_x86::Register::FS => self.fs,
            iced_x86::Register::SS => self.ss,
            iced_x86::Register::GS => self.gs, */
            _ => todo!(),
        }
    }
}

pub struct X86 {
    pub mem: Vec<u8>,
    pub regs: Registers,
    // XXX PE base address, needed for winapi impls; we'll need some win32 system state bit.
    pub base: u32,
    pub imports: HashMap<u32, Option<fn(&mut X86)>>,
}
impl X86 {
    pub fn new() -> Self {
        let mut regs = Registers::new();
        regs.eax = 0xdeadbeea;
        regs.ebx = 0xdeadbeeb;
        regs.ecx = 0xdeadbeec;
        regs.edx = 0xdeadbeed;
        regs.esi = 0xdeadbe51;
        regs.edi = 0xdeadbed1;
        X86 {
            mem: Vec::new(),
            regs,
            base: 0,
            imports: HashMap::new(),
        }
    }

    fn write_u32(&mut self, offset: u32, value: u32) {
        let offset = offset as usize;
        self.mem[offset] = (value >> 0) as u8;
        self.mem[offset + 1] = (value >> 8) as u8;
        self.mem[offset + 2] = (value >> 16) as u8;
        self.mem[offset + 3] = (value >> 24) as u8;
    }

    pub fn read_u32(&self, offset: u32) -> u32 {
        let offset = offset as usize;
        ((self.mem[offset] as u32) << 0)
            | ((self.mem[offset + 1] as u32) << 8)
            | ((self.mem[offset + 2] as u32) << 16)
            | ((self.mem[offset + 3] as u32) << 24)
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
        self.regs
            .get(instr.memory_base())
            .wrapping_add(self.regs.get(instr.memory_index()))
            .wrapping_add(instr.memory_displacement32())
    }

    fn sub(&mut self, x: u32, y: u32) -> u32 {
        let result = x - y;
        // XXX "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
        self.regs.flags.set(Flags::ZF, result == 0);
        result
    }

    fn try_invoke_handler(&mut self, addr: u32) -> bool {
        let handler = match self.imports.get(&addr) {
            Some(handler) => handler,
            None => return false,
        };

        match handler {
            Some(handler) => handler(self),
            None => log::error!("unimplemented import: {:x}", addr),
        };
        true
    }

    fn run(&mut self, instr: &iced_x86::Instruction) -> anyhow::Result<()> {
        assert!(
            !instr.has_rep_prefix()
                && !instr.has_lock_prefix()
                && !instr.has_repe_prefix()
                && !instr.has_repne_prefix()
        );

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
                if !self.try_invoke_handler(target) {
                    self.push(self.regs.eip);
                    self.regs.eip = target;
                }
            }
            iced_x86::Code::Retnd => self.regs.eip = self.pop(),

            iced_x86::Code::Jmp_rel32_32 => {
                self.regs.eip = instr.near_branch32();
            }
            iced_x86::Code::Jmp_rm32 => {
                let target = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
                    _ => unreachable!(),
                };
                if self.try_invoke_handler(target) {
                    // Return from handler.
                    self.regs.eip = self.pop();
                } else {
                    self.regs.eip = target;
                }
            }

            iced_x86::Code::Je_rel8_32 => {
                if self.regs.flags.contains(Flags::ZF) {
                    self.regs.eip = instr.near_branch32();
                }
            }

            iced_x86::Code::Pushd_imm8 => self.push(instr.immediate8to32() as u32),
            iced_x86::Code::Pushd_imm32 => self.push(instr.immediate32()),
            iced_x86::Code::Push_r32 => self.push(self.regs.get(instr.op0_register())),
            iced_x86::Code::Push_rm32 => {
                // push [eax+10h]
                let value = self.read_u32(self.addr(instr));
                self.push(value);
            }

            iced_x86::Code::Pop_r32 => {
                let value = self.pop();
                self.regs.set(instr.op0_register(), value);
            }

            iced_x86::Code::Mov_rm32_imm32 => {
                // mov dword ptr [x], y
                // TODO: why is this 'rm32' when there is an r32 variant just below?
                assert!(instr.op0_kind() == iced_x86::OpKind::Memory);
                self.write_u32(self.addr(instr), instr.immediate32());
            }
            iced_x86::Code::Mov_r32_imm32 => {
                self.regs.set(instr.op0_register(), instr.immediate32());
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
                let value = self.regs.get(instr.op1_register());
                match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.set(instr.op0_register(), value),
                    iced_x86::OpKind::Memory => self.write_u32(self.addr(instr), value),
                    _ => unreachable!(),
                }
            }
            iced_x86::Code::Mov_r32_rm32 => {
                let value = match instr.op1_kind() {
                    iced_x86::OpKind::Register => self.regs.get(instr.op1_register()),
                    iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
                    _ => unreachable!(),
                };
                self.regs.set(instr.op0_register(), value);
            }

            iced_x86::Code::And_rm32_imm8 => {
                match instr.op0_kind() {
                    iced_x86::OpKind::Register => {
                        let reg = instr.op0_register();
                        assert!(instr.op1_kind() == iced_x86::OpKind::Immediate8to32);
                        self.regs
                            .set(reg, self.regs.get(reg) & instr.immediate8to32() as u32);
                    }
                    iced_x86::OpKind::Memory => {
                        let addr = self.addr(instr);
                        self.write_u32(addr, self.read_u32(addr) & instr.immediate8() as u32);
                    }
                    _ => unreachable!(),
                };
            }
            iced_x86::Code::Xor_rm32_r32 => {
                assert!(instr.op0_kind() == iced_x86::OpKind::Register);
                let reg = instr.op0_register();
                self.regs.set(
                    reg,
                    self.regs.get(reg) ^ self.regs.get(instr.op1_register()),
                );
            }

            iced_x86::Code::Sub_rm32_imm8 => {
                assert!(instr.op0_kind() == iced_x86::OpKind::Register);
                assert!(instr.op1_kind() == iced_x86::OpKind::Immediate8to32);
                let reg = instr.op0_register();
                self.regs
                    .set(reg, self.regs.get(reg) - instr.immediate8to32() as u32);
            }
            iced_x86::Code::Sub_rm32_imm32 => {
                assert!(instr.op0_kind() == iced_x86::OpKind::Register);
                let reg = instr.op0_register();
                let value = self.sub(self.regs.get(reg), instr.immediate32());
                self.regs.set(reg, value);
            }
            iced_x86::Code::Sub_rm32_r32 => {
                assert!(instr.op0_kind() == iced_x86::OpKind::Register);
                let reg = instr.op0_register();
                let value = self.sub(self.regs.get(reg), self.regs.get(instr.op1_register()));
                self.regs.set(reg, value);
            }

            iced_x86::Code::Lea_r32_m => {
                // lea eax,[esp+10h]
                self.regs.set(instr.op0_register(), self.addr(instr));
            }

            iced_x86::Code::Cmp_rm32_r32 => match instr.op0_kind() {
                iced_x86::OpKind::Register => {
                    self.sub(
                        self.regs.get(instr.op0_register()),
                        self.regs.get(instr.op1_register()),
                    );
                }
                iced_x86::OpKind::Memory => {
                    self.sub(
                        self.read_u32(self.addr(instr)),
                        self.regs.get(instr.op1_register()),
                    );
                }
                _ => unreachable!(),
            },

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
