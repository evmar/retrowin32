use crate::{
    gdt::create_gdt,
    host,
    machine::{MachineX, Status},
    shims::Shims,
};
use builtin_kernel32 as kernel32;
use memory::{Extensions, ExtensionsMut, Mem, MemImpl};
use std::{collections::HashMap, future::Future, pin::Pin};
use unicorn_engine::{
    RegisterX86, Unicorn, X86Mmr,
    unicorn_const::{Arch, Mode, Permission},
};
use win32_system::{dll::Handler, memory::Memory};
use win32_winapi::calling_convention::ABIReturn;

/// When eip==MAGIC_ADDR, the CPU executes futures (async tasks) rather than x86 code.
/// This is a u64 only because Unicorn wants u64s for registers/addresses.
const MAGIC_ADDR: u64 = 0xFFFF_FFF0;

pub struct Emulator {
    pub unicorn: Unicorn<'static, ()>,
    pub shims: Shims,
    // For now, we only support one thread, so stash the singular TEB address here.
    teb_addr: u32,
    breakpoints: HashMap<u32, *mut core::ffi::c_void>,
    futures: Vec<Pin<Box<dyn Future<Output = ()>>>>,
}

pub type Machine = MachineX<Emulator>;

impl MachineX<Emulator> {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        let memory = Memory::new(MemImpl::new(32 << 20));

        let mut unicorn = Unicorn::new(Arch::X86, Mode::MODE_32).unwrap();
        unsafe {
            let offset = 0x1000; // Leave the first 4k of memory unmapped
            unicorn
                .mem_map_ptr(
                    offset as u64,
                    (memory.len() as usize) - offset,
                    Permission::ALL,
                    memory.imp.as_ptr().add(offset) as *mut _,
                )
                .unwrap();
        };

        unicorn
            .add_insn_sys_hook(
                unicorn_engine::InsnSysX86::SYSENTER,
                0,
                0xFFFF_FFFF,
                |unicorn| {
                    unicorn.emu_stop().unwrap();
                },
            )
            .unwrap();

        // Uncomment to trace every executed instruction:
        // unicorn
        //     .add_code_hook(0, 0xFFFF_FFFF, |_unicorn, addr, size| {
        //         println!("u {addr:x}+{size:x}");
        //     })
        //     .unwrap();

        Machine {
            status: Default::default(),
            emu: Emulator {
                unicorn,
                shims: Shims::default(),
                teb_addr: 0,
                breakpoints: Default::default(),
                futures: Default::default(),
            },
            memory,
            host,
            state: Default::default(),
            external_dlls: Default::default(),
        }
    }

    pub fn mem(&self) -> Mem {
        self.memory.mem()
    }

    pub fn new_thread_impl(
        &mut self,
        new_cpu: bool,
        stack_size: u32,
        start_addr: u32,
        args: &[u32],
    ) -> u32 {
        // We only support one thread for now, so assert these parameters.
        assert_eq!(new_cpu, false);
        assert_eq!(start_addr, 0);
        assert_eq!(args.len(), 0);

        let thread = kernel32::create_thread(self, stack_size);
        self.emu
            .unicorn
            .reg_write(RegisterX86::ESP, thread.stack_pointer as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::EBP, thread.stack_pointer as u64)
            .unwrap();

        self.emu.teb_addr = thread.thread.teb;

        // We create the whole GDT here, including the FS segment.
        // If we supported multiple threads we'd need to figure out how to adjust GDT here instead.
        self.setup_segments(self.emu.teb_addr);

        thread.thread.handle.to_raw()
    }

    /// Initialize segment registers.  In particular, we need FS to point at the kernel32 TEB.
    fn setup_segments(&mut self, fs_addr: u32) {
        // To be able to set FS, we need to create a GDT, which then requires entries
        // for the code and data segments too.
        // https://scoding.de/setting-global-descriptor-table-unicorn
        // https://github.com/unicorn-engine/unicorn/blob/master/samples/sample_x86_32_gdt_and_seg_regs.c
        let gdt = create_gdt(&self.memory, fs_addr);

        let gdtr = X86Mmr {
            selector: 0, // unused
            base: gdt.addr as u64,
            limit: 5 * 8,
            flags: 0, // unused
        };
        // Gross: need gdtr as a slice to pass to reg_write_long.
        let gdtr_slice = unsafe {
            std::slice::from_raw_parts(
                &gdtr as *const _ as *const u8,
                std::mem::size_of::<X86Mmr>(),
            )
        };

        self.emu
            .unicorn
            .reg_write_long(RegisterX86::GDTR, gdtr_slice)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::CS, gdt.cs as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::DS, gdt.ds as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::SS, gdt.ss as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::FS, gdt.fs as u64)
            .unwrap();
    }

    pub fn start_exe(&mut self, cmdline: String, relocate: Option<Option<u32>>) {
        self.memory.create_process_heap();

        let retrowin32_syscall = b"\x0f\x34\xc3".as_slice(); // sysenter; ret

        let exe_name = {
            let mut state = kernel32::get_state(self);
            state.init_process(&self.memory, retrowin32_syscall, cmdline);
            state.cmdline.exe_name()
        };

        let machine = self as *mut Machine;
        self.call_async(Box::pin(async move {
            let machine: &mut MachineX<Emulator> = unsafe { &mut *machine };
            kernel32::loader::start_exe(machine, exe_name, relocate)
                .await
                .unwrap();
        }));
    }

    fn syscall(&mut self) {
        // See machine_emu's syscall() for a more detailed explanation of how this works.
        // It is the same here except we poke Unicorn registers instead of Emulator registers.

        let eip = self.emu.unicorn.reg_read(RegisterX86::EIP).unwrap();
        assert!(eip != MAGIC_ADDR); // sanity check

        let esp = self.emu.unicorn.reg_read(RegisterX86::ESP).unwrap() as u32;
        assert!(esp != 0);
        let shim_addr = self.memory.mem().get_pod::<u32>(esp) - 6;
        assert!(shim_addr != 0);
        let shim = match self.emu.shims.get(shim_addr) {
            Ok(shim) => shim,
            Err(name) => unimplemented!("shim call to {name}"),
        };

        let stack_args = esp + 8;
        match shim.func {
            Handler::Sync(func) => {
                let ret = unsafe { func(self, stack_args) };
                self.post_syscall(ret);
            }
            Handler::Async(func) => {
                let return_address = eip;
                let machine = self as *mut Machine;
                let future = Box::pin(async move {
                    let machine: &mut Machine = unsafe { &mut *machine };
                    let ret = unsafe { func(machine, stack_args).await };
                    machine.post_syscall(ret);
                    machine
                        .emu
                        .unicorn
                        .reg_write(RegisterX86::EIP, return_address as u64)
                        .unwrap();
                });
                self.call_async(future);
            }
        };
    }

    /// Update registers after a syscall; shared by sync and async codepaths.
    fn post_syscall(&mut self, ret: ABIReturn) {
        let (eax, edx) = match ret {
            ABIReturn::U32(ret) => (ret, 0),
            ABIReturn::U64(ret) => (ret as u32, (ret >> 32) as u32),
            ABIReturn::F64(_) => todo!(),
        };
        self.emu
            .unicorn
            .reg_write(RegisterX86::EAX, eax as u64)
            .unwrap();
        self.emu.unicorn.reg_write(RegisterX86::ECX, 0).unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::EDX, edx as u64)
            .unwrap();
        // ebx: callee-saved
    }

    /// Set up the CPU such that we are making an x86->async call, enqueuing a Future.
    /// It resolves to the return address of the call.
    fn call_async(&mut self, future: Pin<Box<dyn Future<Output = ()>>>) {
        self.emu
            .unicorn
            .reg_write(RegisterX86::EIP, MAGIC_ADDR)
            .unwrap();
        self.emu.futures.push(future);
    }

    /// Poll the current future, removing it from the queue if it's done.
    fn async_executor(&mut self) {
        let future = self.emu.futures.last_mut().unwrap();
        // TODO: we don't use the waker at all.  Rust doesn't like us passing a random null pointer
        // here but it seems like nothing accesses it(?).
        //let c = unsafe { std::task::Context::from_waker(&Waker::from_raw(std::task::RawWaker::)) };
        #[allow(deref_nullptr)]
        let context: &mut std::task::Context = unsafe { &mut *std::ptr::null_mut() };
        let poll = future.as_mut().poll(context);
        match poll {
            std::task::Poll::Ready(_) => {
                self.emu.futures.pop();
            }
            std::task::Poll::Pending => {}
        }
    }

    /// Set up the CPU as if a function was just called with arguments,
    /// with return address and arguments on the stack.
    pub fn call_x86(&mut self, func: u32, args: Vec<u32>) -> impl Future<Output = u32> {
        let orig_esp = self.emu.unicorn.reg_read(RegisterX86::ESP).unwrap() as u32;

        let mem = self.memory.mem();

        let ret_addr = MAGIC_ADDR as u32; //self.emu.unicorn.reg_read(RegisterX86::EIP).unwrap() as u32;
        let mut esp = orig_esp;
        for &arg in args.iter().rev() {
            esp -= 4;
            mem.put_pod::<u32>(esp, arg);
        }
        esp -= 4;
        mem.put_pod::<u32>(esp, ret_addr);

        self.emu
            .unicorn
            .reg_write(RegisterX86::ESP, esp as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::EIP, func as u64)
            .unwrap();

        UnicornFuture {
            machine: self,
            esp: orig_esp,
        }
    }

    pub fn run(&mut self) -> &Status {
        let mut eip: u64;
        while self.status.is_running() {
            // self.print_trace();
            eip = self.emu.unicorn.reg_read(RegisterX86::EIP).unwrap();
            if let Err(err) = self.emu.unicorn.emu_start(eip, MAGIC_ADDR, 0, 0) {
                self.status = Status::Error {
                    message: format!("unicorn: {:?}", err),
                };
                break;
            }
            eip = self.emu.unicorn.reg_read(RegisterX86::EIP).unwrap();

            // There are two reasons Unicorn might have stopped:
            // either we hit the magic address, or we hit a system call.
            if eip == MAGIC_ADDR as u64 {
                self.async_executor();
            } else {
                self.syscall();
            }
        }
        &self.status
    }

    pub fn add_breakpoint(&mut self, addr: u32) -> bool {
        match self.emu.breakpoints.entry(addr) {
            std::collections::hash_map::Entry::Occupied(_) => {
                log::warn!("machine_unicorn: breakpoint already set at {:#x}", addr);
                false
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                let hook = self
                    .emu
                    .unicorn
                    .add_code_hook(addr as u64, (addr + 1) as u64, |u, addr, _size| {
                        log::debug!("machine_unicorn: breakpoint hit at {:#x}", addr);
                        u.emu_stop().unwrap()
                    })
                    .unwrap();
                entry.insert(hook);
                true
            }
        }
    }

    pub fn clear_breakpoint(&mut self, addr: u32) -> bool {
        if let Some(hook) = self.emu.breakpoints.remove(&addr) {
            self.emu.unicorn.remove_hook(hook).unwrap();
            true
        } else {
            log::warn!("machine_unicorn: no breakpoint at {:#x}", addr);
            false
        }
    }

    pub fn dump_stack(&self) {
        let esp = self.emu.unicorn.reg_read(RegisterX86::ESP).unwrap() as u32;
        for addr in ((esp - 0x8)..(esp + 0x18)).step_by(4) {
            let extra = if addr == esp { " <- esp" } else { "" };
            println!(
                "{:08x} {:08x}{extra}",
                addr,
                self.mem().get_pod::<u32>(addr)
            );
        }
    }

    /// Print a one-line trace of the current CPU state, intended for matching across emulator backends.
    pub fn print_trace(&self) {
        use RegisterX86::*;
        print!(
            "@{eip:x}\n  eax:{eax:x} ebx:{ebx:x} ecx:{ecx:x} edx:{edx:x} esi:{esi:x} edi:{edi:x} esp:{esp:x} ebp:{ebp:x}",
            eax = self.emu.unicorn.reg_read(EAX).unwrap() as u32,
            ecx = self.emu.unicorn.reg_read(ECX).unwrap() as u32,
            edx = self.emu.unicorn.reg_read(EDX).unwrap() as u32,
            ebx = self.emu.unicorn.reg_read(EBX).unwrap() as u32,
            esi = self.emu.unicorn.reg_read(ESI).unwrap() as u32,
            edi = self.emu.unicorn.reg_read(EDI).unwrap() as u32,
            esp = self.emu.unicorn.reg_read(ESP).unwrap() as u32,
            ebp = self.emu.unicorn.reg_read(EBP).unwrap() as u32,
            eip = self.emu.unicorn.reg_read(EIP).unwrap() as u32,
        );
        // TODO: st_top
        println!();
    }

    fn dump_regs(&self) {
        use RegisterX86::*;
        println!(
            "\
            eax {eax:08x}    esi {esi:08x}     eip {eip:08x}\n\
            ecx {ecx:08x}    edi {edi:08x}\n\
            edx {edx:08x}    esp {esp:08x}\n\
            ebx {ebx:08x}    ebp {ebp:08x}",
            eax = self.emu.unicorn.reg_read(EAX).unwrap() as u32,
            ecx = self.emu.unicorn.reg_read(ECX).unwrap() as u32,
            edx = self.emu.unicorn.reg_read(EDX).unwrap() as u32,
            ebx = self.emu.unicorn.reg_read(EBX).unwrap() as u32,
            esi = self.emu.unicorn.reg_read(ESI).unwrap() as u32,
            edi = self.emu.unicorn.reg_read(EDI).unwrap() as u32,
            esp = self.emu.unicorn.reg_read(ESP).unwrap() as u32,
            ebp = self.emu.unicorn.reg_read(EBP).unwrap() as u32,
            eip = self.emu.unicorn.reg_read(EIP).unwrap() as u32,
        );
    }

    pub fn dump_state(&self) {
        self.dump_regs();
        println!("stack:");
        self.dump_stack();
    }

    pub fn exit(&mut self, exit_code: u32) {
        self.status = Status::Exit(exit_code);
    }

    pub fn teb_addr(&self) -> u32 {
        self.emu.teb_addr
    }
    pub async fn block(&mut self, _wait: Option<u32>) {
        todo!()
    }
    pub fn exit_thread(&mut self, _status: u32) {}
    pub fn debug_break(&mut self) {}
}

struct UnicornFuture {
    // We assume the machine is around for the duration of the future execution.
    // https://github.com/rust-lang/futures-rs/issues/316
    machine: *mut Machine,
    esp: u32,
}
impl Future for UnicornFuture {
    type Output = u32;

    fn poll(
        mut self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let machine = unsafe { &mut *self.machine };
        let esp = machine.emu.unicorn.reg_read(RegisterX86::ESP).unwrap() as u32;
        if esp == self.esp {
            std::task::Poll::Ready(machine.emu.unicorn.reg_read(RegisterX86::EAX).unwrap() as u32)
        } else {
            std::task::Poll::Pending
        }
    }
}
