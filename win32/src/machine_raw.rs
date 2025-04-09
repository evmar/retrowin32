use crate::{
    host, loader,
    machine::{MachineX, Status},
    memory::Memory,
    shims::Shims,
    shims_raw::{self, retrowin32_syscall},
    winapi::{
        kernel32::CommandLine,
        {self},
    },
};
use memory::Mem;
use std::collections::HashMap;

#[derive(Default)]
pub struct RawMem {}

impl RawMem {
    pub fn mem(&self) -> Mem {
        Mem::from_ptrs(0 as *mut u8..(1 << 30) as *mut u8)
    }
    pub fn len(&self) -> u32 {
        0xFFFF_FFFF
    }
}

pub struct Emulator {
    pub shims: Shims,
}

pub type MemImpl = RawMem;
pub type Machine = MachineX<Emulator>;

impl MachineX<Emulator> {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        let mut memory = Memory::new(MemImpl::default());
        let kernel32 = winapi::kernel32::State::new(&mut memory, &retrowin32_syscall());
        let teb = 0; // TODO: set up teb here
        let shims = Shims::new(teb);
        let state = winapi::State::new(&mut memory, kernel32);

        Machine {
            emu: Emulator { shims },
            memory,
            host,
            state,
            labels: HashMap::new(),
            external_dlls: Default::default(),
            status: Default::default(),
        }
    }

    pub fn mem(&self) -> Mem {
        self.memory.mem()
    }

    #[allow(non_snake_case)]
    pub fn load_exe(
        &mut self,
        buf: &[u8],
        cmdline: String,
        relocate: Option<Option<u32>>,
    ) -> anyhow::Result<u32> {
        self.state
            .kernel32
            .init_process(self.memory.mem(), CommandLine::new(cmdline));
        let exe = loader::load_exe(self, buf, &self.state.kernel32.cmdline.exe_name(), relocate)?;

        // TODO: stack init now lives in load_exe->create_thread call.
        // let stack =
        //     self.memory
        //         .mappings
        //         .alloc(exe.stack_size, "stack".into(), &mut self.memory.imp);
        // let stack_pointer = stack.addr + stack.size - 4;
        // unsafe {
        //     shims_raw::set_stack32(stack_pointer);
        // }
        todo!();

        Ok(exe.entry_point)
    }

    pub async fn call_x86(&mut self, func: u32, args: Vec<u32>) -> u32 {
        crate::shims_raw::call_x86(self, func, args).await
    }

    /// Transfer control to the executable's entry point.
    /// Needs to switch code segments to enter compatibility mode, stacks, etc.
    #[inline(never)] // aid in debugging
    pub fn jump_to_entry_point(&mut self, entry_point: u32) {
        // Assert that our code was loaded in the 3-4gb memory range, which means
        // that calls from/to it can be managed with 32-bit pointers.
        // (This arrangement is set up by the linker flags.)
        let mem_3gb_range = 0xc000_0000u64..0x1_0000_0000u64;
        let fn_addr = &Self::jump_to_entry_point as *const _ as u64;
        assert!(mem_3gb_range.contains(&fn_addr));

        println!("entry point at {:x}, about to jump", entry_point);
        std::io::stdin().read_line(&mut String::new()).unwrap();

        let pin = std::pin::pin!(self.call_x86(entry_point, vec![]));
        crate::shims::call_sync(pin);
    }

    pub fn teb_addr(&self) -> u32 {
        todo!();
    }

    pub fn exit_thread(&mut self) {
        todo!();
    }

    pub fn exit(&mut self, exit_code: u32) {
        self.status = Status::Exit(exit_code);
    }
}
