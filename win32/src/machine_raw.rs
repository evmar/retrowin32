use crate::{
    host, ldt, loader,
    machine::{MachineX, Status},
    shims::Shims,
    shims_raw::{self, call_sync, retrowin32_syscall},
    winapi::{self, kernel32::CommandLine},
};
use memory::{Mem, MemImpl};
use win32_system::memory::Memory;

pub struct Emulator {
    pub shims: Shims,
}

pub type Machine = MachineX<Emulator>;

impl MachineX<Emulator> {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        let mut memory = Memory::new(MemImpl::default());
        let kernel32 = winapi::kernel32::State::new(&mut memory, &retrowin32_syscall());
        let shims = Shims::new();
        let state = winapi::State::new(kernel32);

        Machine {
            emu: Emulator { shims },
            memory,
            host,
            state,
            external_dlls: Default::default(),
            status: Default::default(),
        }
    }

    pub fn mem(&self) -> Mem {
        self.memory.mem()
    }

    pub fn start_exe(&mut self, cmdline: String, relocate: Option<Option<u32>>) {
        self.state
            .kernel32
            .init_process(self.memory.mem(), CommandLine::new(cmdline));
        let start = std::pin::pin!(loader::start_exe(self, relocate));
        call_sync(start).unwrap();
    }

    pub async fn call_x86(&mut self, func: u32, args: Vec<u32>) -> u32 {
        crate::shims_raw::call_x86(self, func, args).await
    }

    pub fn teb_addr(&self) -> u32 {
        let teb_addr: u32;
        #[cfg(target_arch = "x86_64")]
        unsafe {
            std::arch::asm!(
                "mov {teb_addr:e}, dword ptr fs:[0x18]",
                teb_addr = out(reg) teb_addr,
            );
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            teb_addr = 0;
        }
        teb_addr
    }

    pub fn new_thread_impl(
        &mut self,
        new_cpu: bool,
        stack_size: u32,
        start_addr: u32,
        args: &[u32],
    ) -> u32 {
        // We only use this to initialize the initial thread, so we don't support new_cpu
        // or start_addr.
        if new_cpu {
            todo!();
        }
        assert_eq!(start_addr, 0);
        let _ = args;

        let thread = winapi::kernel32::create_thread(self, stack_size);
        unsafe {
            shims_raw::set_stack32(thread.stack_pointer);
            assert!(thread.thread.teb != 0);

            // Set up fs to point at the TEB.
            // NOTE: OSX seems extremely sensitive to the values used here, where like
            // using a span size that is not exactly 0xFFF causes the entry to be rejected.
            let fs_sel = ldt::add_entry(thread.thread.teb, 0xFFF, false);
            std::arch::asm!(
                "mov fs, {fs_sel:x}",
                fs_sel = in(reg) fs_sel
            );
        }
        0
    }

    pub fn exit_thread(&mut self) {
        todo!();
    }

    pub fn exit(&mut self, exit_code: u32) {
        self.status = Status::Exit(exit_code);
    }
}
