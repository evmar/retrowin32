use anyhow::bail;
use tsify::Tsify;

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
        }
    }

    fn get(&self, name: iced_x86::Register) -> u32 {
        match name {
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
}

pub struct X86 {
    pub mem: Vec<u8>,
    pub regs: Registers,
}
impl X86 {
    pub fn new() -> Self {
        X86 {
            mem: Vec::new(),
            regs: Registers::new(),
        }
    }

    fn write_u32(&mut self, offset: u32, value: u32) {
        let offset = offset as usize;
        self.mem[offset] = (value >> 0) as u8;
        self.mem[offset + 1] = (value >> 8) as u8;
        self.mem[offset + 2] = (value >> 16) as u8;
        self.mem[offset + 3] = (value >> 24) as u8;
    }

    fn push(&mut self, value: u32) {
        self.regs.esp -= 4;
        self.write_u32(self.regs.esp, value);
    }

    fn run(&mut self, instruction: &iced_x86::Instruction) -> anyhow::Result<()> {
        self.regs.eip = instruction.next_ip() as u32;
        match instruction.code() {
            iced_x86::Code::Call_rel32_32 => {
                self.push(self.regs.eip);
                self.regs.eip = instruction.near_branch32();
            }
            iced_x86::Code::Jmp_rel32_32 => {
                self.regs.eip = instruction.near_branch32();
            }
            iced_x86::Code::Pushd_imm8 => self.push(instruction.immediate8to32() as u32),
            iced_x86::Code::Push_r32 => self.push(self.regs.get(instruction.op_register(0))),
            iced_x86::Code::Mov_rm32_imm32 => {
                self.write_u32(
                    instruction.memory_displacement32(),
                    instruction.immediate32(),
                );
            }
            code => bail!("code {:?}", code),
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
