use crate::{host, pe, shims::Shims, winapi};
use memory::{Mem, MemImpl};
use std::collections::HashMap;

pub struct LoadedAddrs {
    pub entry_point: u32,
    pub stack_pointer: u32,
}

/// Integrates the X86 CPU emulator with the Windows OS support.
pub struct Machine {
    #[cfg(feature = "x86-emu")]
    pub x86: x86::X86,
    #[cfg(feature = "x86-unicorn")]
    pub unicorn: unicorn_engine::Unicorn<'static, ()>,
    pub memory: MemImpl,
    pub host: Box<dyn host::Host>,
    pub state: winapi::State,
    pub shims: Shims,
    pub labels: HashMap<u32, String>,
}

impl Machine {
    pub fn new(host: Box<dyn host::Host>, cmdline: String) -> Self {
        #[cfg(not(feature = "x86-unicorn"))]
        let mut memory = MemImpl::default();

        #[cfg(feature = "x86-unicorn")]
        let mut memory = MemImpl::new(32 << 20);

        let mut kernel32 = winapi::kernel32::State::new(&mut memory, cmdline);

        #[cfg(feature = "x86-emu")]
        let shims = {
            kernel32 = kernel32;
            Shims::new()
        };
        #[cfg(feature = "x86-64")]
        let shims = {
            let mapping =
                kernel32
                    .mappings
                    .alloc(0x4000, "shims x64 trampoline".into(), &mut memory);
            Shims::new(
                &mut kernel32.ldt,
                mapping.addr as u64 as *mut u8,
                mapping.size,
            )
        };

        #[cfg(feature = "x86-unicorn")]
        let unicorn = {
            let mut unicorn = unicorn_engine::Unicorn::new(
                unicorn_engine::unicorn_const::Arch::X86,
                unicorn_engine::unicorn_const::Mode::MODE_32,
            )
            .unwrap();
            unsafe {
                unicorn
                    .mem_map_ptr(
                        0,
                        memory.len() as usize,
                        unicorn_engine::unicorn_const::Permission::ALL,
                        memory.ptr() as *mut _,
                    )
                    .unwrap();
            };
            unicorn
        };

        #[cfg(feature = "x86-unicorn")]
        let shims = {
            let mapping = kernel32
                .mappings
                .alloc(0x1000, "syscalls".into(), &mut memory);
            Shims::new(
                memory
                    .mem()
                    .slice(mapping.addr..mapping.addr + mapping.size),
                mapping.addr,
            )
        };

        let state = winapi::State::new(kernel32);

        Machine {
            #[cfg(feature = "x86-emu")]
            x86: x86::X86::new(),
            #[cfg(feature = "x86-unicorn")]
            unicorn,
            memory,
            host,
            state,
            shims,
            labels: HashMap::new(),
        }
    }

    pub fn mem(&self) -> Mem {
        self.memory.mem()
    }

    /// Initialize a memory mapping for the stack and return the initial stack pointer.
    fn setup_stack(&mut self, stack_size: u32) -> u32 {
        let stack =
            self.state
                .kernel32
                .mappings
                .alloc(stack_size, "stack".into(), &mut self.memory);
        let stack_pointer = stack.addr + stack.size - 4;

        #[cfg(feature = "x86-emu")]
        {
            self.x86.cpu.regs.esp = stack_pointer;
            self.x86.cpu.regs.ebp = stack_pointer;
        }

        #[cfg(feature = "x86-unicorn")]
        {
            // TODO: put this init somewhere better.
            self.unicorn
                .reg_write(unicorn_engine::RegisterX86::ESP, stack_pointer as u64)
                .unwrap();
            self.unicorn
                .reg_write(unicorn_engine::RegisterX86::EBP, stack_pointer as u64)
                .unwrap();
        }

        stack_pointer
    }

    /// Initialize segment registers.  In particular, we need FS to point at the kernel32 TEB.
    fn setup_segments(&mut self) {
        #[cfg(feature = "x86-emu")]
        {
            self.x86.cpu.regs.fs_addr = self.state.kernel32.teb;
        }

        #[cfg(feature = "x86-unicorn")]
        {
            // To be able to set FS, we need to create a GDT, which then requires entries
            // for the code and data segments too.
            // https://scoding.de/setting-global-descriptor-table-unicorn
            // https://github.com/unicorn-engine/unicorn/blob/master/samples/sample_x86_32_gdt_and_seg_regs.c
            let (addr, cs, ds, fs, ss) = self.state.kernel32.create_gdt(self.memory.mem());

            let gdtr = unicorn_engine::X86Mmr {
                selector: 0, // unused
                base: addr as u64,
                limit: 5 * 8,
                flags: 0, // unused
            };
            // Gross: need gdtr as a slice to pass to reg_write_long.
            let gdtr_slice = unsafe {
                std::slice::from_raw_parts(
                    &gdtr as *const _ as *const u8,
                    std::mem::size_of::<unicorn_engine::X86Mmr>(),
                )
            };

            self.unicorn
                .reg_write_long(unicorn_engine::RegisterX86::GDTR, gdtr_slice)
                .unwrap();
            self.unicorn
                .reg_write(unicorn_engine::RegisterX86::CS, cs as u64)
                .unwrap();
            self.unicorn
                .reg_write(unicorn_engine::RegisterX86::DS, ds as u64)
                .unwrap();
            self.unicorn
                .reg_write(unicorn_engine::RegisterX86::SS, ss as u64)
                .unwrap();
            self.unicorn
                .reg_write(unicorn_engine::RegisterX86::FS, fs as u64)
                .unwrap();
        }
    }

    #[allow(non_snake_case)]
    pub fn load_exe(
        &mut self,
        buf: &[u8],
        cmdline: String,
        relocate: bool,
    ) -> anyhow::Result<LoadedAddrs> {
        let exe = pe::load_exe(self, buf, cmdline, relocate)?;

        let stack_pointer = self.setup_stack(exe.stack_size);
        self.setup_segments();

        #[cfg(feature = "x86-emu")]
        {
            // To make CPU traces match more closely, set up some registers to what their
            // initial values appear to be from looking in a debugger.
            self.x86.cpu.regs.ecx = exe.entry_point;
            self.x86.cpu.regs.edx = exe.entry_point;
            self.x86.cpu.regs.esi = exe.entry_point;
            self.x86.cpu.regs.edi = exe.entry_point;

            let mut dll_mains = Vec::new();
            for dll in &self.state.kernel32.dlls {
                if dll.dll.entry_point != 0 {
                    dll_mains.push(dll.dll.entry_point);
                }
            }

            if dll_mains.is_empty() {
                self.x86.cpu.regs.eip = exe.entry_point;
            } else {
                // Invoke any DllMains then jump to the entry point.

                let m = self as *mut Machine;
                crate::shims::become_async(
                    self,
                    Box::pin(async move {
                        let machine = unsafe { &mut *m };
                        for dll_main in dll_mains {
                            log::info!("invoking dllmain {:x}", dll_main);
                            let hInstance = 0u32; // TODO
                            let fdwReason = 1u32; // DLL_PROCESS_ATTACH
                            let lpvReserved = 0u32;
                            crate::shims::call_x86(
                                machine,
                                dll_main,
                                vec![hInstance, fdwReason, lpvReserved],
                            )
                            .await;
                        }
                        machine.x86.cpu.regs.eip = exe.entry_point;
                    }),
                );
            };
        }

        Ok(LoadedAddrs {
            entry_point: exe.entry_point,
            stack_pointer,
        })
    }

    /// If eip points at a shim address, call the handler and update eip.
    #[cfg(feature = "x86-emu")]
    fn check_shim_call(&mut self) -> anyhow::Result<bool> {
        if self.x86.cpu.regs.eip & 0xFFFF_0000 != crate::shims_emu::SHIM_BASE {
            return Ok(false);
        }
        let crate::shims::Shim {
            func,
            stack_consumed,
            is_async,
            ..
        } = *self.shims.get(self.x86.cpu.regs.eip);
        let ret = unsafe { func(self, self.x86.cpu.regs.esp) };
        if !is_async {
            self.x86.cpu.regs.eip = self.mem().get::<u32>(self.x86.cpu.regs.esp);
            self.x86.cpu.regs.esp += stack_consumed;
            self.x86.cpu.regs.eax = ret;
        } else {
            // Async handler will manage the return address etc.
        }
        Ok(true)
    }

    // Execute one basic block.  Returns Ok(false) if we stopped early.
    #[cfg(feature = "x86-emu")]
    pub fn execute_block(&mut self) -> anyhow::Result<bool> {
        if self.check_shim_call()? {
            // Treat any shim call as a single block.
            return Ok(true);
        }
        self.x86
            .execute_block(self.memory.mem())
            .map_err(|err| anyhow::anyhow!(err))
    }

    #[cfg(feature = "x86-emu")]
    pub fn single_step(&mut self) -> anyhow::Result<()> {
        if self.check_shim_call()? {
            // Treat any shim call as a single block.
            return Ok(());
        }
        self.x86
            .single_step(self.memory.mem())
            .map_err(|err| anyhow::anyhow!(err))
    }
}
