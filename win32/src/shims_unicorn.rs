//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//!
//! This module implements Shims for use within the Unicorn CPU emulator.
//!
//! The basic approach is:
//! 1) reserve some page of memory as special for shims, called `hooks_base`
//! 2) use the address `hooks_base+x` to represent shim number x
//! 3) tell Unicorn to stop emulation whenever the page is hit,
//!    and use eip to compute which shim

use crate::{shims::Shim, Machine};

#[derive(Default)]
pub struct Shims {
    shims: Vec<Result<&'static Shim, String>>,
    hooks_base: u32,
}

impl Shims {
    pub fn new(unicorn: &mut unicorn_engine::Unicorn<'static, ()>, addr: u32) -> Self {
        unicorn
            .add_code_hook(
                addr as u64,
                addr as u64 + 0x1000,
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
            hooks_base: addr,
        }
    }

    pub fn add(&mut self, shim: Result<&'static Shim, String>) -> u32 {
        let index = self.shims.len() as u32;
        let addr = self.hooks_base + index;
        self.shims.push(shim);
        addr
    }
}

/// Synchronously evaluate a Future, under the assumption that it is always immediately Ready.
#[allow(deref_nullptr)]
pub fn call_sync<T>(future: std::pin::Pin<&mut impl std::future::Future<Output = T>>) -> T {
    let context: &mut std::task::Context = unsafe { &mut *std::ptr::null_mut() };
    match future.poll(context) {
        std::task::Poll::Pending => unreachable!(),
        std::task::Poll::Ready(t) => t,
    }
}

pub struct UnimplFuture {}
impl std::future::Future for UnimplFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready(())
    }
}

/// Handle a call to a shim.  Expects eip to point somewhere within the hooks_base page.
fn handle_shim_call(machine: &mut Machine) -> u32 {
    let eip = machine
        .emu
        .unicorn
        .reg_read(unicorn_engine::RegisterX86::EIP)
        .unwrap() as u32;
    let index = eip - machine.emu.shims.hooks_base;
    let shim = match &machine.emu.shims.shims[index as usize] {
        Ok(shim) => shim,
        Err(name) => unimplemented!("shim call to {name}"),
    };

    let crate::shims::Shim {
        func,
        stack_consumed,
        ..
    } = *shim;

    let esp = machine
        .emu
        .unicorn
        .reg_read(unicorn_engine::RegisterX86::ESP)
        .unwrap() as u32;
    let ret = unsafe { func(machine, esp) };

    let ret_addr = machine.mem().get::<u32>(esp);
    machine
        .emu
        .unicorn
        .reg_write(unicorn_engine::RegisterX86::EIP, ret_addr as u64)
        .unwrap();
    machine
        .emu
        .unicorn
        .reg_write(
            unicorn_engine::RegisterX86::ESP,
            (esp + stack_consumed) as u64,
        )
        .unwrap();
    machine
        .emu
        .unicorn
        .reg_write(unicorn_engine::RegisterX86::EAX, ret as u64)
        .unwrap();

    ret_addr
}

pub fn call_x86(machine: &mut Machine, func: u32, args: Vec<u32>) -> UnimplFuture {
    let mem = machine.memory.mem();

    let ret_addr = machine
        .emu
        .unicorn
        .reg_read(unicorn_engine::RegisterX86::EIP)
        .unwrap() as u32;

    let mut esp = machine
        .emu
        .unicorn
        .reg_read(unicorn_engine::RegisterX86::ESP)
        .unwrap() as u32;
    for &arg in args.iter().rev() {
        esp -= 4;
        mem.put(esp, arg);
    }
    esp -= 4;
    mem.put(esp, ret_addr);

    machine
        .emu
        .unicorn
        .reg_write(unicorn_engine::RegisterX86::ESP, esp as u64)
        .unwrap();

    unicorn_loop(machine, func, ret_addr);

    UnimplFuture {}
}

/// Run emulation via machine.emu starting from eip=begin until eip==until is hit.
/// This is like unicorn.emu_start() but handles shim calls as well.
pub fn unicorn_loop(machine: &mut Machine, begin: u32, until: u32) {
    let mut eip = begin as u64;
    let until = until as u64;
    loop {
        machine.emu.unicorn.emu_start(eip, until, 0, 0).unwrap();
        eip = machine
            .emu
            .unicorn
            .reg_read(unicorn_engine::RegisterX86::EIP)
            .unwrap();
        if eip == until {
            return;
        }
        if (machine.emu.shims.hooks_base..machine.emu.shims.hooks_base + 0x1000)
            .contains(&(eip as u32))
        {
            eip = handle_shim_call(machine) as u64;
        }
    }
}
