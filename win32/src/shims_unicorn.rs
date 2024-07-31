//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//!
//! This module implements Shims for use within the Unicorn CPU emulator.
//!
//! The basic approach is:
//! 1) reserve some page of memory as special for shims, called `hooks_base`
//! 2) use the address `hooks_base+x` to represent shim number x
//! 3) tell Unicorn to stop emulation whenever the page is hit,
//!    and use eip to compute which shim

use crate::shims::{BoxFuture, Handler};
use crate::{shims::Shim, Machine};
use memory::Extensions;
use std::future::Future;
use std::pin::Pin;
use unicorn_engine::RegisterX86;

#[derive(Default)]
pub struct Shims {
    shims: Vec<Result<&'static Shim, String>>,
    hooks_base: u32,
}

impl Shims {
    pub fn new(unicorn: &mut unicorn_engine::Unicorn<'static, ()>, hooks_base: u32) -> Self {
        unicorn
            .add_code_hook(
                hooks_base as u64,
                hooks_base as u64 + 0x1000,
                |unicorn, _addr, _size| {
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

        Shims {
            shims: Default::default(),
            hooks_base,
        }
    }

    pub fn add(&mut self, shim: Result<&'static Shim, String>) -> u32 {
        let index = self.shims.len() as u32;
        let addr = self.hooks_base + index;
        self.shims.push(shim);
        addr
    }

    pub fn get(&self, addr: u32) -> Option<Result<&'static Shim, &str>> {
        if (self.hooks_base..self.hooks_base + 0x1000).contains(&addr) {
            let index = (addr - self.hooks_base) as usize;
            self.shims.get(index).map(|result| match result {
                Ok(shim) => Ok(*shim),
                Err(name) => Err(name.as_str()),
            })
        } else {
            None
        }
    }
}

/// When eip==MAGIC_ADDR, the CPU executes futures (async tasks) rather than x86 code.
pub const MAGIC_ADDR: u32 = 0xFFFF_FFF0;

/// Handle a call to a shim.
pub fn handle_shim_call(
    machine: &mut Machine,
    shim: &'static Shim,
) -> Option<BoxFuture<()>> {
    let Shim {
        func,
        stack_consumed,
        ..
    } = *shim;
    match func {
        Handler::Sync(func) => {
            let esp = machine.emu.unicorn.reg_read(RegisterX86::ESP).unwrap() as u32;
            let ret = unsafe { func(machine, esp) };
            let ret_addr = machine.mem().get_pod::<u32>(esp);
            machine
                .emu
                .unicorn
                .reg_write(RegisterX86::EIP, ret_addr as u64)
                .unwrap();
            machine
                .emu
                .unicorn
                .reg_write(RegisterX86::ESP, (esp + stack_consumed + 4) as u64)
                .unwrap();
            machine
                .emu
                .unicorn
                .reg_write(RegisterX86::EAX, ret as u64)
                .unwrap();
            None
        }
        Handler::Async(func) => {
            let machine: *mut Machine = machine;
            Some(Box::pin(async move {
                let machine = unsafe { &mut *machine };
                let esp = machine.emu.unicorn.reg_read(RegisterX86::ESP).unwrap() as u32;
                let ret = unsafe { func(machine, esp) }.await;
                let ret_addr = machine.mem().get_pod::<u32>(esp);
                machine
                    .emu
                    .unicorn
                    .reg_write(RegisterX86::EIP, ret_addr as u64)
                    .unwrap();
                machine
                    .emu
                    .unicorn
                    .reg_write(RegisterX86::ESP, (esp + stack_consumed + 4) as u64)
                    .unwrap();
                machine
                    .emu
                    .unicorn
                    .reg_write(RegisterX86::EAX, ret as u64)
                    .unwrap();
            }))
        }
    }
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
        if machine.emu.unicorn.reg_read(RegisterX86::ESP).unwrap() as u32 == self.esp {
            std::task::Poll::Ready(machine.emu.unicorn.reg_read(RegisterX86::EAX).unwrap() as u32)
        } else {
            std::task::Poll::Pending
        }
    }
}

pub fn call_x86(machine: &mut Machine, func: u32, args: Vec<u32>) -> impl Future<Output = u32> {
    let mem = machine.emu.memory.mem();

    let esp = machine.emu.unicorn.reg_read(RegisterX86::ESP).unwrap() as u32;
    let mut new_esp = esp;
    for &arg in args.iter().rev() {
        new_esp -= 4;
        mem.put_pod::<u32>(new_esp, arg);
    }
    new_esp -= 4;
    mem.put_pod::<u32>(new_esp, MAGIC_ADDR);

    machine
        .emu
        .unicorn
        .reg_write(RegisterX86::ESP, new_esp as u64)
        .unwrap();
    machine
        .emu
        .unicorn
        .reg_write(RegisterX86::EIP, func as u64)
        .unwrap();

    UnicornFuture { machine, esp }
}
