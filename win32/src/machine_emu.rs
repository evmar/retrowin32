//! Implements Machine using retrowin32's x86 emulator as found in the x86/ directory.

use crate::{
    machine::{MachineX, Status},
    shims::Shims,
};
use builtin_kernel32 as kernel32;
use memory::{Extensions, ExtensionsMut, Mem, MemImpl};
use std::collections::HashMap;
use win32_system::{dll::Handler, host, memory::Memory};
use win32_winapi::calling_convention::ABIReturn;

pub struct Emulator {
    pub status: Status,
    pub x86: x86::X86,
    pub shims: Shims,

    /// Places where we've patched out the instruction with an int3.
    /// The map values are the bytes from before the breakpoint.
    breakpoints: HashMap<u32, u8>,
}

pub type Machine = MachineX<Emulator>;

impl MachineX<Emulator> {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        let memory = Memory::new(MemImpl::new(256 << 20));

        Machine {
            emu: Emulator {
                status: Default::default(),
                x86: x86::X86::new(),
                shims: Default::default(),
                breakpoints: Default::default(),
            },
            memory,
            host,
            state: Default::default(),
            external_dlls: Default::default(),
        }
    }

    pub fn mem(&self) -> Mem<'_> {
        self.memory.mem()
    }

    pub fn start_exe(&mut self, cmdline: String, relocate: Option<Option<u32>>) {
        // Initialize the process heap.
        // We use this for misc allocations like per-thread TEBs,
        // so we need it to exist very early in process startup,
        // before even the exe is loaded.  This means we need to be
        // careful to not make it so big as to cover the memory region
        // that the exe will be loaded into.
        self.memory.create_process_heap();

        let retrowin32_syscall = b"\x0f\x34\xc3"; // sysenter; ret

        let exe_name = {
            let mut state = kernel32::get_state(self);
            state.init_process(&self.memory, retrowin32_syscall, cmdline);
            state.cmdline.exe_name()
        };

        let machine = self as *mut Machine;
        let cpu = self.emu.x86.new_cpu();
        cpu.call_async(Box::pin(async move {
            let machine: &mut MachineX<Emulator> = unsafe { &mut *machine };
            kernel32::loader::start_exe(machine, exe_name, relocate)
                .await
                .unwrap();
            0
        }));
    }

    pub fn single_step(&mut self) {
        self.emu.x86.single_step_next_block(self.memory.mem());
        self.run();
    }

    pub fn block(&mut self, wait: Option<u32>) -> impl std::future::Future<Output = ()> {
        self.emu.x86.cpu_mut().block(wait)
    }

    pub fn unblock_all(&mut self) {
        for cpu in self.emu.x86.cpus.iter_mut() {
            if matches!(
                cpu.state,
                x86::CPUState::Blocked(_) | x86::CPUState::DebugBreak
            ) {
                cpu.state = x86::CPUState::Running;
                self.emu.status = Status::Running;
            }
        }
    }

    pub fn unblock(&mut self) {
        let cpu = self.emu.x86.cpu_mut();
        if matches!(
            cpu.state,
            x86::CPUState::Blocked(_) | x86::CPUState::DebugBreak
        ) {
            cpu.state = x86::CPUState::Running;
            self.emu.status = Status::Running;
        }
    }

    pub fn run(&mut self) -> bool {
        self.emu.x86.schedule();
        match &self.emu.x86.cpu().state {
            x86::CPUState::Running => self.execute_block(),
            x86::CPUState::SysCall => self.syscall(),
            x86::CPUState::Blocked(wait) => {
                let wait = *wait;
                if self.host.block(wait) {
                    self.unblock();
                } else {
                    self.emu.status = Status::Blocked;
                }
            }
            x86::CPUState::Error(message) => {
                self.emu.status = Status::Error {
                    message: message.clone(),
                };
            }
            x86::CPUState::DebugBreak => {
                self.emu.status = Status::DebugBreak;
            }
            state => unimplemented!("{state:?}"),
        }
        self.emu.status.is_running()
    }

    fn execute_block(&mut self) {
        self.emu.x86.execute_block(self.memory.mem())
    }

    /// Update registers after a syscall; shared by sync and async codepaths.
    fn post_syscall(&mut self, ret: ABIReturn) {
        // In addition to passing return values, we attempt to clear other registers
        // to make execution traces easier to match.

        let regs = &mut self.emu.x86.cpu_mut().regs;
        match ret {
            ABIReturn::U32(ret) => {
                regs.set32(x86::Register::EAX, ret);
                regs.set32(x86::Register::ECX, 0);
                regs.set32(x86::Register::EDX, 0);
                // EBX: callee-saved
            }
            ABIReturn::U64(ret) => {
                regs.set32(x86::Register::EAX, ret as u32);
                regs.set32(x86::Register::ECX, 0);
                regs.set32(x86::Register::EDX, (ret >> 32) as u32);
                // EBX: callee-saved
            }
            ABIReturn::F64(ret) => {
                regs.set32(x86::Register::EAX, 0);
                regs.set32(x86::Register::ECX, 0);
                regs.set32(x86::Register::EDX, 0);
                // EBX: callee-saved
                self.emu.x86.cpu_mut().fpu.push(ret);
            }
        }
    }

    fn syscall(&mut self) {
        self.emu.x86.cpu_mut().state = x86::CPUState::Running;

        // See doc/shims.md for the state of the stack when we get here.

        let regs = &mut self.emu.x86.cpu_mut().regs;

        // stack[0] is the return address within the shim DLL, after the call instruction.
        // The 'call retrowin32_syscall' instruction is ff15+addr, for 6 bytes.
        // Subtract to find the address of the shim function.
        let esp = regs.get32(x86::Register::ESP);
        let call_len = 6;
        let shim_addr = self.memory.mem().get_pod::<u32>(esp) - call_len;

        let shim = match self.emu.shims.get(shim_addr) {
            Ok(shim) => shim,
            Err(name) => unimplemented!("{}", name),
        };

        let stack_args = esp + 8;
        match shim.func {
            Handler::Sync(func) => {
                let ret = unsafe { func(self, stack_args) };
                self.post_syscall(ret);
            }

            Handler::Async(func) => {
                let return_address = regs.eip;
                let machine: *mut Machine = self as *mut _;
                let future = Box::pin(async move {
                    let machine = unsafe { &mut *machine };
                    let ret = unsafe { func(machine, stack_args) }.await;
                    machine.post_syscall(ret);
                    return_address
                });
                self.emu.x86.cpu_mut().call_async(future);
            }
        }
    }

    pub async fn call_x86(&mut self, func: u32, args: Vec<u32>) -> u32 {
        self.emu
            .x86
            .cpu_mut()
            .call_x86(self.memory.mem(), func, args)
            .await
    }

    pub fn dump_state(&self, eip_offset: usize) {
        let cpu = self.emu.x86.cpu();
        x86::debug::dump_state(cpu, self.mem(), &self.memory.labels, eip_offset);
        println!("stack:");
        x86::debug::dump_stack(cpu, self.mem());
        x86::debug::dump_fpu_state(cpu);
    }

    /// Print a one-line trace of the current CPU state, intended for matching across emulator backends.
    pub fn print_trace(&self) {
        let regs = &self.emu.x86.cpu().regs;
        print!(
            "@{eip:x}\n  eax:{eax:x} ebx:{ebx:x} ecx:{ecx:x} edx:{edx:x} esi:{esi:x} edi:{edi:x} esp:{esp:x} ebp:{ebp:x}",
            eip = regs.eip,
            eax = regs.get32(x86::Register::EAX),
            ebx = regs.get32(x86::Register::EBX),
            ecx = regs.get32(x86::Register::ECX),
            edx = regs.get32(x86::Register::EDX),
            esi = regs.get32(x86::Register::ESI),
            edi = regs.get32(x86::Register::EDI),
            esp = regs.get32(x86::Register::ESP),
            ebp = regs.get32(x86::Register::EBP),
        );
        let st_top = self.emu.x86.cpu().fpu.st_top;
        if st_top != 8 {
            print!(" st_top:{st_top:x}");
        }
        println!();
    }

    /// Patch in an int3 over the instruction at that addr, backing up the current one.
    pub fn add_breakpoint(&mut self, addr: u32) -> bool {
        assert!(addr != 0);
        match self.emu.breakpoints.entry(addr) {
            std::collections::hash_map::Entry::Occupied(_) => false,
            std::collections::hash_map::Entry::Vacant(entry) => {
                let mem = self.memory.mem();
                entry.insert(mem.get_pod::<u8>(addr));
                mem.put_pod::<u8>(addr, 0xcc); // int3
                self.emu.x86.icache.clear_cache(addr);
                true
            }
        }
    }

    /// Undo an add_breakpoint().
    pub fn clear_breakpoint(&mut self, addr: u32) -> bool {
        match self.emu.breakpoints.remove(&addr) {
            Some(prev) => {
                self.emu.x86.icache.clear_cache(addr);
                self.mem().put_pod::<u8>(addr, prev);
                true
            }
            None => {
                log::warn!("clear_breakpoint({addr:x}) not set");
                false
            }
        }
    }

    pub fn teb_addr(&self) -> u32 {
        self.emu.x86.cpu().regs.fs_addr
    }

    pub fn new_thread_impl(
        &mut self,
        new_cpu: bool,
        stack_size: u32,
        start_addr: u32,
        args: &[u32],
    ) -> u32 {
        let thread = kernel32::create_thread(self, stack_size);
        let cpu = if new_cpu {
            self.emu.x86.new_cpu()
        } else {
            self.emu.x86.cpu_mut()
        };
        cpu.regs.set32(x86::Register::ESP, thread.stack_pointer);
        cpu.regs.set32(x86::Register::EBP, thread.stack_pointer);
        cpu.regs.fs_addr = thread.thread.teb;

        // Push the args in reverse order.
        // TODO: deduplicate this with call_x86 logic.
        for &arg in args.iter().rev() {
            x86::ops::push(cpu, self.memory.mem(), arg);
        }
        // Put a return address here so we crash more obviously if the thread
        // returns; in practice, it should be retrowin32_thread_main which
        // stops the thread first.
        x86::ops::push(cpu, self.memory.mem(), 0);
        cpu.regs.eip = start_addr;

        thread.thread.handle.to_raw()
    }

    pub fn exit_thread(&mut self, status: u32) {
        if self.emu.x86.cur_cpu == 0 {
            panic!("ExitThread called on main thread");
        }

        log::warn!(
            "thread on cpu {id} exiting with code {code}",
            code = status,
            id = self.emu.x86.cur_cpu
        );

        self.emu.x86.cpu_mut().state = x86::CPUState::Free;
    }

    pub fn exit(&mut self, exit_code: u32) {
        self.emu.status = Status::Exit(exit_code);
    }

    pub fn debug_break(&mut self) {
        self.emu.x86.cpu_mut().state = x86::CPUState::DebugBreak;
    }
}
