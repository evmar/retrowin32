use win32::pe::ImageSectionFlags;

#[derive(Debug, Clone, Copy)]
enum Register {
    EAX,
    EBX,
    ECX,
    EDX,
    ESP,
    EBP,
    ESI,
    EDI,
}
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
    }
}
fn reg_from_iced(reg: iced_x86::Register) -> Register {
    match reg {
        iced_x86::Register::EAX => Register::EAX,
        iced_x86::Register::EBX => Register::EBX,
        iced_x86::Register::ECX => Register::ECX,
        iced_x86::Register::EDX => Register::EDX,
        iced_x86::Register::ESP => Register::ESP,
        iced_x86::Register::EBP => Register::EBP,
        iced_x86::Register::ESI => Register::ESI,
        iced_x86::Register::EDI => Register::EDI,
        _ => unreachable!("reg {:?}", reg),
    }
}

trait WAssembler {
    fn comment(&mut self, msg: impl Into<String>);
    fn inline_comment(&mut self, msg: impl Into<String>);

    fn global_get(&mut self, idx: usize);
    fn global_set(&mut self, idx: usize);

    fn i32_const(&mut self, val: u32);
    fn i32_store(&mut self, offset: usize);
    fn i32_add(&mut self);
    fn i32_sub(&mut self);

    fn call(&mut self, func: u32);
    fn drop(&mut self);
}
struct WATAssembler {
    lines: Vec<String>,
}
impl WATAssembler {
    fn new() -> Self {
        WATAssembler { lines: Vec::new() }
    }
    fn line(&mut self, str: impl Into<String>) {
        self.lines.push(str.into())
    }
}
impl WAssembler for WATAssembler {
    fn comment(&mut self, msg: impl Into<String>) {
        let msg = msg.into();
        self.line(format!(";; {msg}"));
    }
    fn inline_comment(&mut self, msg: impl Into<String>) {
        let msg = msg.into();
        let idx = self.lines.len() - 1;
        self.lines[idx].push_str(&format!(" ;; {msg}"));
    }

    fn global_get(&mut self, idx: usize) {
        self.line(format!("global.get {idx}"));
    }

    fn global_set(&mut self, idx: usize) {
        self.line(format!("global.set {idx}"));
    }

    fn i32_const(&mut self, val: u32) {
        self.line(format!("i32.const 0x{val:x}"));
    }

    fn i32_store(&mut self, offset: usize) {
        if offset > 0 {
            self.line(format!("i32.store offset=0x{offset:x}"));
        } else {
            self.line("i32.store");
        }
    }

    fn i32_add(&mut self) {
        self.line("i32.add");
    }

    fn i32_sub(&mut self) {
        self.line("i32.sub");
    }

    fn call(&mut self, func: u32) {
        self.line(format!("call {func}"));
    }

    fn drop(&mut self) {
        self.line("drop");
    }
}

struct X86Assembler<A: WAssembler> {
    wasm: A,
}
impl<A: WAssembler> X86Assembler<A> {
    fn get_reg(&mut self, reg: Register) {
        let idx = globalidx(reg);
        self.wasm.global_get(idx);
        self.wasm.inline_comment(&format!("{:?}", reg));
    }

    fn set_reg(&mut self, reg: Register) {
        let idx = globalidx(reg);
        self.wasm.global_set(idx);
        self.wasm.inline_comment(&format!("{:?}", reg));
    }

    fn x86_push(&mut self) {
        // mem[esp] = reg
        self.get_reg(Register::ESP);
        self.wasm.i32_store(0);

        // esp += 4
        self.get_reg(Register::ESP);
        self.wasm.i32_const(4);
        self.wasm.i32_add();
        self.set_reg(Register::ESP);
    }

    fn x86_sub(&mut self) {
        self.wasm.i32_sub();
        // flags
    }

    fn get_rm32(&mut self, instr: &iced_x86::Instruction, idx: u32) {
        match instr.op_kind(idx) {
            iced_x86::OpKind::Register => self.get_reg(reg_from_iced(instr.op_register(idx))),
            iced_x86::OpKind::Memory => {} //todo!(),
            _ => unreachable!(),
        }
    }

    fn set_rm32(&mut self, instr: &iced_x86::Instruction, idx: u32) {
        match instr.op_kind(idx) {
            iced_x86::OpKind::Register => self.set_reg(reg_from_iced(instr.op_register(idx))),
            iced_x86::OpKind::Memory => {} //todo!(),
            _ => unreachable!(),
        }
    }
}

fn disassemble(buf: &[u8], ip: u32) {
    let mut asm = X86Assembler {
        wasm: WATAssembler::new(),
    };
    let mut decoder =
        iced_x86::Decoder::with_ip(32, buf, ip as u64, iced_x86::DecoderOptions::NONE);
    while decoder.can_decode() {
        let instr = decoder.decode();
        asm.wasm.comment(&format!("{}", instr));
        match instr.code() {
            iced_x86::Code::Push_rm32 | iced_x86::Code::Push_r32 => {
                asm.get_rm32(&instr, 0);
                asm.x86_push();
            }
            iced_x86::Code::Pushd_imm32 => {
                asm.wasm.i32_const(instr.immediate32());
                asm.x86_push();
            }
            iced_x86::Code::Pushd_imm8 => {
                asm.wasm.i32_const(instr.immediate8to32() as u32);
                asm.x86_push();
            }
            iced_x86::Code::Mov_r32_rm32 | iced_x86::Code::Mov_rm32_r32 => {
                asm.get_rm32(&instr, 1);
                asm.set_rm32(&instr, 0);
            }
            iced_x86::Code::Test_rm32_r32 => {
                asm.get_rm32(&instr, 0);
                asm.get_rm32(&instr, 1);
                asm.x86_sub();
                asm.wasm.drop(); // discard result of sub
            }
            iced_x86::Code::Call_rm32 => {
                asm.get_rm32(&instr, 0);
                asm.wasm.call(1 /* xxx */);
            }
            code => {
                asm.wasm.comment(format!("todo: {:?}", code));
                break;
            }
        };
    }
    println!("{}\n", asm.wasm.lines.join("\n"));
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
    //use super::*;

    #[test]
    fn it_works() {}
}
