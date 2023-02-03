use std::collections::HashMap;

use anyhow::bail;
use x86::X86;

use crate::{host, pe::ImageSectionFlags, winapi, windows::load_exe};

/// Code that calls from x86 to the host will jump to addresses in this
/// magic range.
/// "fake IAT" => "FIAT" => "F1A7"
pub const SHIM_BASE: u32 = 0xF1A7_0000;

/// Jumps to memory address SHIM_BASE+x are interpreted as calling shims[x].
/// This is how emulated code calls out to hosting code for e.g. DLL imports.
pub struct Shims(Vec<Result<fn(&mut Machine), String>>);
impl Shims {
    fn new() -> Self {
        Shims(Vec::new())
    }

    /// Returns the (fake) address of the registered function.
    pub fn add(&mut self, entry: Result<fn(&mut Machine), String>) -> u32 {
        let id = SHIM_BASE | self.0.len() as u32;
        self.0.push(entry);
        id
    }

    pub fn get(&self, addr: u32) -> Option<&fn(&mut Machine)> {
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

pub struct Machine {
    pub x86: X86,
    pub host: Box<dyn host::Host>,
    pub state: winapi::State,
    pub shims: Shims,
}

impl Machine {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        Machine {
            x86: X86::new(),
            host,
            state: winapi::State::new(),
            shims: Shims::new(),
        }
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

/// Manages decoding and running instructions in an owned Machine.
pub struct Runner {
    pub machine: Machine,
    /// Total number of instructions executed.
    pub instr_count: usize,

    icache: InstrCache,

    /// Places to stop execution in a step_many() call.
    /// We unconditionally stop on these; the web frontend manages things like
    /// enabling/disabling breakpoints.  The map values are the instruction
    /// from before the breakpoint.
    breakpoints: HashMap<u32, iced_x86::Instruction>,
}
impl Runner {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        Runner {
            machine: Machine::new(host),
            instr_count: 0,
            icache: InstrCache::new(),
            breakpoints: HashMap::new(),
        }
    }

    pub fn load_exe(
        &mut self,
        buf: &[u8],
        cmdline: String,
    ) -> anyhow::Result<HashMap<u32, String>> {
        let labels = load_exe(&mut self.machine, buf, cmdline)?;

        // Disassemble the 'code' section.
        let mapping = self
            .machine
            .state
            .kernel32
            .mappings
            .vec()
            .iter()
            .find(|mapping| mapping.flags.contains(ImageSectionFlags::CODE))
            .ok_or_else(|| anyhow::anyhow!("no code section"))?;
        let section =
            &self.machine.x86.mem[mapping.addr as usize..(mapping.addr + mapping.size) as usize];
        self.icache.disassemble(section, mapping.addr);
        self.icache
            .jmp(&self.machine.x86.mem, self.machine.x86.regs.eip);

        Ok(labels)
    }

    /// Patch in an int3 over the instruction at that addr, backing up the current one.
    pub fn add_breakpoint(&mut self, addr: u32) -> anyhow::Result<()> {
        let mut int3 = iced_x86::Instruction::with(iced_x86::Code::Int3);
        // The instruction needs a length/next_ip so the execution machinery doesn't lose its location.
        int3.set_len(1);
        int3.set_next_ip(addr as u64 + 1);
        let prev = self.icache.patch(addr, int3)?;
        self.breakpoints.insert(addr, prev);
        Ok(())
    }

    /// Undo an add_breakpoint().
    pub fn clear_breakpoint(&mut self, addr: u32) -> anyhow::Result<()> {
        let prev = self.breakpoints.remove(&addr).unwrap();
        self.icache.patch(addr, prev)?;
        Ok(())
    }

    /// If eip points at a shim address, call the handler and update eip.
    fn check_shim_call(&mut self) -> anyhow::Result<()> {
        if self.machine.x86.regs.eip & 0xFFFF_0000 != SHIM_BASE {
            return Ok(());
        }
        let ret = x86::ops::pop(&mut self.machine.x86);
        let handler = self
            .machine
            .shims
            .get(self.machine.x86.regs.eip)
            .ok_or_else(|| anyhow::anyhow!("missing shim"))?;
        handler(&mut self.machine);
        x86::ops::x86_jmp(&mut self.machine.x86, ret).map_err(|err| anyhow::anyhow!(err))
    }

    // Single-step execution.  Returns Ok(false) if we stopped.
    pub fn step(&mut self) -> anyhow::Result<bool> {
        self.instr_count += 1;

        let (prev_ip, ref instr) = self.icache.instrs[self.icache.index];
        let next_ip = instr.next_ip() as u32;
        // Need to update eip before executing because instructions like 'call' will push eip onto the stack.
        self.machine.x86.regs.eip = next_ip;
        match self.machine.x86.run(instr) {
            Err(err) => {
                // Point the debugger at the failed instruction.
                self.machine.x86.regs.eip = prev_ip;
                bail!(err)
            }
            Ok(_) => {
                if self.machine.x86.stopped {
                    self.machine.x86.regs.eip = prev_ip;
                    if let Some(msg) = &self.machine.x86.crashed {
                        bail!(msg.clone());
                    }
                    self.machine.x86.stopped = false;
                    return Ok(false);
                }
                if self.machine.x86.regs.eip == next_ip {
                    self.icache.index += 1;
                } else {
                    self.check_shim_call()?;
                    // Instruction changed eip.  Update icache to match.
                    self.icache
                        .jmp(&self.machine.x86.mem, self.machine.x86.regs.eip);
                }
                Ok(true)
            }
        }
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

    pub fn load_snapshot(&mut self, snap: x86::Snapshot) {
        self.machine.x86.load_snapshot(snap);
        self.icache
            .jmp(&self.machine.x86.mem, self.machine.x86.regs.eip);
    }
}
