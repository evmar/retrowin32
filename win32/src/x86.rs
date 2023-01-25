use std::collections::HashMap;

use anyhow::bail;
use serde::ser::SerializeStruct;

use crate::{
    host, memory::Memory, ops, pe::ImageSectionFlags, registers::Registers, winapi,
    windows::load_exe,
};

/// Addresses from 0 up to this point cause panics if we access them.
/// This helps catch implementation bugs earlier.
pub const NULL_POINTER_REGION_SIZE: u32 = 0x1000;

/// Code that calls from x86 to the host will jump to addresses in this
/// magic range.
/// "fake IAT" => "FIAT" => "F1A7"
pub const SHIM_BASE: u32 = 0xF1A7_0000;

/// Jumps to memory address SHIM_BASE+x are interpreted as calling shims[x].
/// This is how emulated code calls out to hosting code for e.g. DLL imports.
pub struct Shims(Vec<Result<fn(&mut X86), String>>);
impl Shims {
    fn new() -> Self {
        Shims(Vec::new())
    }

    /// Returns the (fake) address of the registered function.
    pub fn add(&mut self, entry: Result<fn(&mut X86), String>) -> u32 {
        let id = SHIM_BASE | self.0.len() as u32;
        self.0.push(entry);
        id
    }

    pub fn get(&self, addr: u32) -> Option<&fn(&mut X86)> {
        let index = (addr & 0x0000_FFFF) as usize;
        let handler = self.0.get(index);
        match handler {
            Some(handler) => match handler {
                Ok(handler) => return Some(handler),
                Err(msg) => log::error!("{}", msg),
            },
            None => log::error!("unknown import reference at {:x}", addr),
        };
        None
    }
}

pub struct X86 {
    pub host: Box<dyn host::Host>,
    pub mem: Vec<u8>,
    pub regs: Registers,
    icache: InstrCache,
    pub shims: Shims,
    pub state: winapi::State,
    /// Toggled on by breakpoints/process exit.
    pub stopped: bool,
    pub crashed: Option<String>,
}
impl X86 {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        unsafe {
            ops::init_op_tab();
        }
        let mut regs = Registers::new();
        regs.eax = 0xdeadbeea;
        regs.ebx = 0xdeadbeeb;
        regs.ecx = 0xdeadbeec;
        regs.edx = 0xdeadbeed;
        regs.esi = 0xdeadbe51;
        regs.edi = 0xdeadbed1;
        X86 {
            host,
            mem: Vec::new(),
            regs,
            icache: InstrCache::new(),
            shims: Shims::new(),
            state: winapi::State::new(),
            stopped: false,
            crashed: None,
        }
    }

    fn crash(&mut self, msg: String) {
        self.crashed = Some(msg);
        self.stopped = true;
    }

    /// Check whether reading a T from mem[addr] would cause OOB, and crash() if so.
    fn check_oob<T>(&mut self, addr: u32) -> bool {
        if addr < NULL_POINTER_REGION_SIZE {
            self.crash(format!("crash: null pointer at {addr:#x}"));
            return true;
        }
        if addr as usize + std::mem::size_of::<T>() > self.mem.len() {
            self.crash(format!("crash: oob pointer at {addr:#x}"));
            return true;
        }
        false
    }

    pub fn write_u32(&mut self, addr: u32, value: u32) {
        if self.check_oob::<u32>(addr) {
            return;
        }
        self.mem.write_u32(addr, value);
    }
    pub fn write_u16(&mut self, addr: u32, value: u16) {
        if self.check_oob::<u16>(addr) {
            return;
        }
        let addr = addr as usize;
        // Safety: check_oob checked bounds.
        unsafe {
            *self.mem.get_unchecked_mut(addr) = value as u8;
            *self.mem.get_unchecked_mut(addr + 1) = (value >> 8) as u8;
        }
    }
    pub fn write_u8(&mut self, addr: u32, value: u8) {
        if self.check_oob::<u8>(addr) {
            return;
        }
        // Safety: check_oob checked bounds.
        unsafe { *self.mem.get_unchecked_mut(addr as usize) = value }
    }

    pub fn read_u32(&mut self, addr: u32) -> u32 {
        if self.check_oob::<u32>(addr) {
            return 0;
        }
        self.mem.read_u32(addr)
    }
    pub fn read_u16(&mut self, addr: u32) -> u16 {
        if self.check_oob::<u16>(addr) {
            return 0;
        }
        let offset = addr as usize;
        // Safety: check_oob checked bounds.
        unsafe {
            (*self.mem.get_unchecked(offset) as u16) << 0
                | (*self.mem.get_unchecked(offset + 1) as u16) << 8
        }
    }
    pub fn read_u8(&mut self, addr: u32) -> u8 {
        if self.check_oob::<u8>(addr) {
            return 0;
        }
        // Safety: check_oob checked bounds.
        unsafe { *self.mem.get_unchecked(addr as usize) }
    }

    pub fn jmp(&mut self, addr: u32) -> anyhow::Result<()> {
        if addr < 0x1000 {
            bail!("jmp to null page");
        }

        if addr & 0xFFFF_0000 == SHIM_BASE {
            let ret = ops::pop(self);
            let eip = self.regs.eip;
            let handler = self
                .shims
                .get(addr)
                .ok_or_else(|| anyhow::anyhow!("missing shim"))?;
            handler(self);
            if self.regs.eip != eip {
                return Ok(()); // handler set eip.
            }
            return self.jmp(ret);
        }

        self.regs.eip = addr;
        Ok(())
    }

    fn step(&mut self) -> anyhow::Result<bool> {
        let (ip, ref instr) = self.icache.instrs[self.icache.index];
        // Hack: ops don't mutate icache (yet) but Rust doesn't know that.
        let instr: &'static iced_x86::Instruction = unsafe { std::mem::transmute(instr) };
        self.regs.eip = instr.next_ip() as u32;
        match unsafe { ops::execute(self, instr) } {
            Err(err) => {
                // Point the debugger at the failed instruction.
                self.regs.eip = ip;
                Err(err)
            }
            Ok(_) => {
                if self.stopped {
                    self.regs.eip = ip;
                    if let Some(msg) = &self.crashed {
                        bail!(msg.clone());
                    }
                    self.stopped = false;
                    return Ok(false);
                }
                self.icache.index += 1;
                self.icache.jmp(&self.mem, self.regs.eip);
                Ok(true)
            }
        }
    }
}

impl serde::Serialize for X86 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("X86", 2)?;
        // TODO: serialize remaining state.
        state.serialize_field("mem", serde_bytes::Bytes::new(&self.mem))?;
        state.serialize_field("regs", &self.regs)?;
        state.end()
    }
}

pub struct Snapshot {
    mem: Vec<u8>,
    regs: Registers,
}

impl<'de> serde::Deserialize<'de> for Snapshot {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Snapshot;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct X86")
            }
            fn visit_seq<V: serde::de::SeqAccess<'de>>(
                self,
                mut seq: V,
            ) -> Result<Snapshot, V::Error> {
                let mem: serde_bytes::ByteBuf = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let regs = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                Ok(Snapshot {
                    mem: mem.into_vec(),
                    regs,
                })
            }
        }
        deserializer.deserialize_struct("X86", &["mem", "regs"], Visitor)
    }
}

/// Cache of decoded instructions, indexed by ip.
struct InstrCache {
    /// (ip, instruction) pairs of cached decoded instructions.
    instrs: Vec<(u32, iced_x86::Instruction)>,
    /// Current position within instrs.
    index: usize,
}

impl InstrCache {
    fn new() -> Self {
        InstrCache {
            instrs: Vec::new(),
            index: 0,
        }
    }

    fn disassemble(&mut self, buf: &[u8], ip: u32) {
        self.instrs.clear();
        let mut decoder =
            iced_x86::Decoder::with_ip(32, buf, ip as u64, iced_x86::DecoderOptions::NONE);
        while decoder.can_decode() {
            self.instrs.push((decoder.ip() as u32, decoder.decode()));
        }
    }

    /// Given an IP that wasn't found in the decoded instructions, re-decode starting at that
    /// address and patch in the new instructions in place of the previous ones.
    /// start is the index of where in the instruction table the patch will begin.
    fn repatch(&mut self, mem: &[u8], addr: u32, start: usize) {
        let mut decoder = iced_x86::Decoder::with_ip(
            32,
            &mem[addr as usize..],
            addr as u64,
            iced_x86::DecoderOptions::NONE,
        );
        let mut end = start;
        let mut patch = Vec::new();
        while decoder.can_decode() {
            let ip = decoder.ip() as u32;
            // Overwrite any instructions with conflicting ips.
            while ip > self.instrs[end].0 {
                end += 1;
            }
            if ip == self.instrs[end].0 {
                break;
            }
            patch.push((decoder.ip() as u32, decoder.decode()));
            if end - start > 100 {
                // We haven't hit this, just a defense against this going wrong.
                panic!("resync: oversized patch?");
            }
        }
        log::info!(
            "replacing [{:x}..{:x}] with {} instrs starting at {:x}",
            self.instrs[start].0,
            self.instrs[end].0,
            patch.len(),
            patch[0].0,
        );
        self.instrs.splice(start..end, patch);
    }

    fn ip_to_instr_index(&mut self, mem: &[u8], target_ip: u32) -> Option<usize> {
        match self.instrs.binary_search_by_key(&target_ip, |&(ip, _)| ip) {
            Ok(pos) => Some(pos),
            Err(pos) => {
                // We may hit this case if the disassembler gets desynchronized from the instruction
                // stream.  We need to re-disassemble or something in that case.
                let nearby = match self.instrs.get(pos) {
                    Some(&(ip, _)) => format!("nearby address {:x}", ip),
                    None => format!("address beyond decoded range"),
                };
                log::error!("invalid ip {:x}, desync? {}", target_ip, nearby);
                self.repatch(mem, target_ip, pos);
                self.ip_to_instr_index(mem, target_ip)
            }
        }
    }

    fn jmp(&mut self, mem: &[u8], target_ip: u32) {
        let (cur_ip, _) = self.instrs[self.index];
        if cur_ip == target_ip {
            return;
        }
        self.index = self.ip_to_instr_index(mem, target_ip).unwrap();
    }

    /// Replace the instruction found at a given ip, returning the previous instruction.
    fn patch(
        &mut self,
        addr: u32,
        instr: iced_x86::Instruction,
    ) -> anyhow::Result<iced_x86::Instruction> {
        let index = self
            .ip_to_instr_index(&[], addr)
            .ok_or_else(|| anyhow::anyhow!("couldn't map ip"))?;
        let prev = std::mem::replace(&mut self.instrs[index].1, instr);
        Ok(prev)
    }
}

/// Manages decoding and running instructions in an owned X86.
pub struct Runner {
    pub x86: X86,
    /// Total number of instructions executed.
    pub instr_count: usize,

    /// Places to stop execution in a step_many() call.
    /// We unconditionally stop on these; the web frontend manages things like
    /// enabling/disabling breakpoints.  The map values are the instruction
    /// from before the breakpoint.
    breakpoints: HashMap<u32, iced_x86::Instruction>,
}
impl Runner {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        Runner {
            x86: X86::new(host),
            instr_count: 0,
            breakpoints: HashMap::new(),
        }
    }

    pub fn load_exe(
        &mut self,
        buf: &[u8],
        cmdline: String,
    ) -> anyhow::Result<HashMap<u32, String>> {
        let labels = load_exe(&mut self.x86, buf, cmdline)?;

        // Disassemble the 'code' section.
        let mapping = self
            .x86
            .state
            .kernel32
            .mappings
            .iter()
            .find(|mapping| mapping.flags.contains(ImageSectionFlags::CODE))
            .ok_or_else(|| anyhow::anyhow!("no code section"))?;
        let section = &self.x86.mem[mapping.addr as usize..(mapping.addr + mapping.size) as usize];
        self.x86.icache.disassemble(section, mapping.addr);
        self.x86.icache.jmp(&self.x86.mem, self.x86.regs.eip);

        Ok(labels)
    }

    /// Patch in an int3 over the instruction at that addr, backing up the current one.
    pub fn add_breakpoint(&mut self, addr: u32) -> anyhow::Result<()> {
        let mut int3 = iced_x86::Instruction::with(iced_x86::Code::Int3);
        // The instruction needs a length/next_ip so the execution machinery doesn't lose its location.
        int3.set_len(1);
        int3.set_next_ip(addr as u64 + 1);
        let prev = self.x86.icache.patch(addr, int3)?;
        self.breakpoints.insert(addr, prev);
        Ok(())
    }

    /// Undo an add_breakpoint().
    pub fn clear_breakpoint(&mut self, addr: u32) -> anyhow::Result<()> {
        let prev = self.breakpoints.remove(&addr).unwrap();
        self.x86.icache.patch(addr, prev)?;
        Ok(())
    }

    // Single-step execution.  Returns Ok(false) if we stopped.
    pub fn step(&mut self) -> anyhow::Result<bool> {
        self.instr_count += 1;
        self.x86.step()
    }

    // Multi-step execution.  Returns Ok(false) on breakpoint.
    pub fn step_many(&mut self, count: usize) -> anyhow::Result<usize> {
        for i in 0..count {
            if !self.step()? {
                return Ok(i);
            }
        }
        Ok(count)
    }

    pub fn load_snapshot(&mut self, snap: Snapshot) {
        self.x86.mem = snap.mem;
        self.x86.regs = snap.regs;
        self.x86.icache.jmp(&self.x86.mem, self.x86.regs.eip);
    }
}
