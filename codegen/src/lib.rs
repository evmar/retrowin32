use win32::pe::ImageSectionFlags;

#[derive(Debug, Clone, Copy)]
enum Register {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

fn globalidx(reg: Register) -> usize {
    match reg {
        Register::EAX => 0,
        Register::ECX => 1,
        Register::EDX => 2,
        Register::EBX => 3,
        Register::ESP => 4,
        Register::EBP => 5,
        Register::ESI => 6,
        Register::EDI => 7,
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

    fn i32_load(&mut self, offset: usize);
    fn i32_store(&mut self, offset: usize);

    fn i32_add(&mut self);
    fn i32_sub(&mut self);
    fn i32_mul(&mut self);

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

    fn i32_load(&mut self, offset: usize) {
        if offset > 0 {
            self.line(format!("i32.load offset=0x{offset:x}"));
        } else {
            self.line("i32.load");
        }
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

    fn i32_mul(&mut self) {
        self.line("i32.mul");
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

    fn x86_push(&mut self, f: impl FnOnce(&mut Self)) {
        // esp -= 4
        self.get_reg(Register::ESP);
        self.wasm.i32_const(4);
        self.wasm.i32_sub();
        self.set_reg(Register::ESP);

        // mem[esp] = reg
        self.get_reg(Register::ESP);
        f(self);
        self.wasm.i32_store(0);
    }

    fn x86_sub(&mut self) {
        self.wasm.i32_sub();
        // flags
    }

    fn addr(&mut self, instr: &iced_x86::Instruction) {
        let mut pushed = false;

        /* let seg = if instr.segment_prefix() == iced_x86::Register::FS {
            self.state.kernel32.teb
        } else {
            0
        };*/

        if instr.memory_base() != iced_x86::Register::None {
            self.get_reg(reg_from_iced(instr.memory_base()));
            if pushed {
                self.wasm.i32_add(); // + base
            } else {
                pushed = true;
            }
        };

        if instr.memory_index() != iced_x86::Register::None {
            self.get_reg(reg_from_iced(instr.memory_index()));
            self.wasm.i32_const(instr.memory_index_scale());
            self.wasm.i32_mul(); // index*scale
            if pushed {
                self.wasm.i32_add(); // + index
            } else {
                pushed = true;
            }
        }

        let disp = instr.memory_displacement32();
        if disp > 0 {
            self.wasm.i32_const(disp);

            if pushed {
                self.wasm.i32_add(); // + disp
            }
        }
    }

    fn get_rm32(&mut self, instr: &iced_x86::Instruction, idx: u32) {
        match instr.op_kind(idx) {
            iced_x86::OpKind::Register => self.get_reg(reg_from_iced(instr.op_register(idx))),
            iced_x86::OpKind::Memory => {
                self.addr(instr);
                self.wasm.i32_load(0);
            }
            _ => unreachable!(),
        }
    }

    fn set_rm32(&mut self, instr: &iced_x86::Instruction, idx: u32, f: impl FnOnce(&mut Self)) {
        match instr.op_kind(idx) {
            iced_x86::OpKind::Register => {
                f(self);
                self.set_reg(reg_from_iced(instr.op_register(idx)))
            }
            iced_x86::OpKind::Memory => {
                self.addr(instr);
                f(self);
                self.wasm.i32_store(0);
            }
            _ => unreachable!(),
        }
    }
}

fn disassemble(buf: &[u8], ip: u32) -> String {
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
                asm.x86_push(|asm| asm.get_rm32(&instr, 0));
            }
            iced_x86::Code::Pushd_imm32 => {
                asm.x86_push(|asm| asm.wasm.i32_const(instr.immediate32()));
            }
            iced_x86::Code::Pushd_imm8 => {
                asm.x86_push(|asm| asm.wasm.i32_const(instr.immediate8to32() as u32));
            }
            iced_x86::Code::Mov_r32_rm32 | iced_x86::Code::Mov_rm32_r32 => {
                asm.set_rm32(&instr, 0, |asm| asm.get_rm32(&instr, 1));
            }
            iced_x86::Code::Test_rm32_r32 => {
                asm.get_rm32(&instr, 0);
                asm.get_rm32(&instr, 1);
                asm.x86_sub();
                asm.wasm.drop(); // discard result of sub
            }
            iced_x86::Code::Call_rm32 => {
                // XXX push eip
                asm.get_rm32(&instr, 0);
                asm.wasm.call(0 /* indirect */);
                // XXX pop eip
            }
            code => {
                asm.wasm.comment(format!("todo: {:?}", code));
                break;
            }
        };
    }
    format!("{}\n", asm.wasm.lines.join("\n"))
}

pub fn load(buf: &[u8]) -> anyhow::Result<()> {
    let file = win32::pe::parse(buf)?;
    //println!("{file:#?}");

    let base = file.opt_header.ImageBase;

    let mut parts = Vec::new();
    for sec in file.sections {
        let flags = sec.characteristics()?;
        let src = sec.PointerToRawData as usize;
        let size = sec.SizeOfRawData as usize;
        let dst = (base + sec.VirtualAddress) as usize;
        if !flags.contains(win32::pe::ImageSectionFlags::UNINITIALIZED_DATA) {
            let mut str = String::new();
            for c in &buf[src..(src + size)] {
                str.push_str(&format!("\\{c:02x}"));
            }
            parts.push(format!("(data (offset i32.const {dst:#x}) \"{str}\")"));
        }
        if flags.contains(ImageSectionFlags::CODE) {
            let code = &buf[src..src + size];
            parts.push(format!(
                "(func (export \"run\") {})",
                disassemble(code, dst as u32)
            ));
        }
    }

    let globals = [
        Register::EAX,
        Register::ECX,
        Register::EDX,
        Register::EBX,
        Register::ESP,
        Register::EBP,
        Register::ESI,
        Register::EDI,
    ]
    .map(|reg| format!("(global (export \"{reg:?}\") (mut i32) (i32.const 0))"))
    .join("\n");

    println!(
        r#"
(module
    (import "host" "mem" (memory 1))
    (import "host" "icall" (func $icall (param i32)))
    {globals}
    {}
)"#,
        parts.join("\n")
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {}
}
