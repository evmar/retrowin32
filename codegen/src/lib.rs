use iced_x86::Register;
use win32::pe::ImageSectionFlags;

fn globalidx(reg: Register) -> usize {
    match reg {
        Register::EAX => 1,
        Register::EBX => 2,
        Register::ECX => 3,
        Register::EDX => 4,
        Register::ESP => 5,
        Register::EBP => 6,
        Register::ESI => 7,
        Register::EDI => 8,
        _ => unreachable!(),
    }
}

struct Assembler {}
impl Assembler {
    fn push(&mut self, value: u32) {}

    fn global_get(&mut self, reg: Register) {}
    fn global_set(&mut self, reg: Register) {}

    fn i32_const(&mut self, val: u32) {}
    fn i32_store(&mut self, offset: usize) {}
    fn i32_add(&mut self) {}
}

fn disassemble(buf: &[u8], ip: u32) {
    let mut asm = Assembler {};
    let mut decoder =
        iced_x86::Decoder::with_ip(32, buf, ip as u64, iced_x86::DecoderOptions::NONE);
    while decoder.can_decode() {
        let instr = decoder.decode();
        println!("{}", instr);
        match instr.code() {
            iced_x86::Code::Push_r32 => {
                // mem[esp] = reg
                asm.global_get(instr.op0_register());
                asm.global_get(Register::ESP);
                asm.i32_store(0);

                // esp += 4
                asm.global_get(Register::ESP);
                asm.i32_const(4);
                asm.i32_add();
                asm.global_set(Register::ESP);
            }
            code => todo!("{:?}", code),
        };
    }
}

pub fn load(buf: &[u8]) -> anyhow::Result<()> {
    let file = win32::pe::parse(buf)?;
    //println!("{file:#?}");

    let base = file.opt_header.ImageBase;

    for sec in file.sections {
        let flags = sec.characteristics()?;
        if flags.contains(ImageSectionFlags::CODE) {
            let src = sec.PointerToRawData as usize;
            let size = sec.SizeOfRawData as usize;
            let dst = (base + sec.VirtualAddress) as usize;
            let code = &buf[src..src + size];
            disassemble(code, dst as u32);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
