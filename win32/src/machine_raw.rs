use crate::{
    host,
    machine::MachineX,
    shims::Shims,
    shims_raw::{self, call_sync, retrowin32_syscall},
};
use builtin_kernel32 as kernel32;
use memory::{Extensions, Mem, MemImpl};
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
        let fs = shims_raw::get_fs_addr_32();
        self.mem().get_pod::<u32>(fs + 0x18)
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
            shims_raw::init_thread(self.mem(), &thread);
        }
        0 // no thread id
    }

    pub fn exit_thread(&mut self, _status: u32) {
        todo!();
    }

    pub fn exit(&mut self, exit_code: u32) {
        self.host.exit(exit_code);
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
