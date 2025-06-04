use crate::{
    host,
    machine::{MachineX, Status},
    shims::Shims,
    shims_raw::{self, call_sync, retrowin32_syscall},
};
use builtin_kernel32 as kernel32;
use memory::{Mem, MemImpl};
use win32_system::memory::Memory;

#[derive(Default)]
pub struct Emulator {
    pub shims: Shims,
}

pub type Machine = MachineX<Emulator>;

impl MachineX<Emulator> {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        let memory = Memory::new(MemImpl::default());
        Shims::init();

        Machine {
            emu: Default::default(),
            memory,
            host,
            state: Default::default(),
            external_dlls: Default::default(),
            status: Default::default(),
        }
    }

    pub fn mem(&self) -> Mem {
        self.memory.mem()
    }

    pub fn start_exe(&mut self, cmdline: String, relocate: Option<Option<u32>>) {
        self.memory.create_process_heap();

        let exe_name = {
            let mut state = kernel32::get_state(self);
            state.init_process(&self.memory, &retrowin32_syscall(), cmdline);
            state.cmdline.exe_name()
        };

        let start = std::pin::pin!(kernel32::loader::start_exe(self, exe_name, relocate));
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

        let thread = kernel32::create_thread(self, stack_size);
        unsafe {
            assert!(thread.thread.teb != 0);

            #[cfg(not(target_os = "linux"))]
            {
                // Set up fs to point at the TEB.
                // NOTE: OSX seems extremely sensitive to the values used here, where like
                // using a span size that is not exactly 0xFFF causes the entry to be rejected.
                let fs = crate::ldt::add_entry(thread.thread.teb, 0xFFF, false);
                std::arch::asm!(
                    "mov fs, {fs:x}",
                    fs = in(reg) fs
                );
            }

            #[cfg(target_os = "linux")]
            {
                shims_raw::init_context32(self.mem(), &thread);
            }
        }
        0 // no thread id
    }

    pub fn exit_thread(&mut self, _status: u32) {
        todo!();
    }

    pub fn exit(&mut self, exit_code: u32) {
        self.status = Status::Exit(exit_code);
    }

    pub async fn block(&mut self, wait: Option<u32>) {
        if !self.host.block(wait) {
            panic!("host must support blocking");
        }
    }

    pub fn debug_break(&mut self) {
        unsafe {
            std::arch::asm!("int3");
        }
    }
}
