//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//!
//! This module implements Shims for non-emulated cpu case, using raw 32-bit memory.
//! See doc/x86-64.md for an overview.

use crate::{shims::Shim, Machine};

#[derive(Default)]
pub struct Shims {
    shims: Vec<Shim>,
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

    pub fn add(&mut self, shim: Shim) -> u32 {
        let index = self.shims.len() as u32;
        let addr = self.hooks_base + index;
        self.shims.push(shim);
        addr
    }

    pub fn add_todo(&mut self, name: String) -> u32 {
        log::warn!("todo: register shim for {name}");
        0
    }

    pub fn handle_call(machine: &mut Machine) -> anyhow::Result<()> {
        let eip = machine
            .unicorn
            .reg_read(unicorn_engine::RegisterX86::EIP)
            .unwrap() as u32;
        let index = eip - machine.shims.hooks_base;
        let shim = &machine.shims.shims[index as usize];

        let crate::shims::Shim {
            func,
            stack_consumed,
            ..
        } = *shim;

        let esp = machine
            .unicorn
            .reg_read(unicorn_engine::RegisterX86::ESP)
            .unwrap() as u32;
        let ret = unsafe { func(machine, esp) };

        let ret_addr = machine.mem().get::<u32>(esp);
        machine
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::EIP, ret_addr as u64)
            .unwrap();
        machine
            .unicorn
            .reg_write(
                unicorn_engine::RegisterX86::ESP,
                (esp + stack_consumed) as u64,
            )
            .unwrap();
        machine
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::EAX, ret as u64)
            .unwrap();
        Ok(())
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

pub fn call_x86(machine: &mut Machine, func: u32, args: Vec<u32>) -> UnimplFuture {
    let mem = machine.memory.mem();

    let ret_addr = machine
        .unicorn
        .reg_read(unicorn_engine::RegisterX86::EIP)
        .unwrap() as u32;

    let mut esp = machine
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
        .unicorn
        .reg_write(unicorn_engine::RegisterX86::ESP, esp as u64)
        .unwrap();

    unicorn_loop(machine, func, ret_addr);

    UnimplFuture {}
}

pub fn unicorn_loop(machine: &mut Machine, eip: u32, until: u32) {
    let mut eip = eip as u64;
    let until = until as u64;
    loop {
        println!("unicorn_loop {eip:x} {until:x}");
        machine.unicorn.emu_start(eip, until, 0, 0).unwrap();
        eip = machine
            .unicorn
            .reg_read(unicorn_engine::RegisterX86::EIP)
            .unwrap();
        if eip == until {
            println!("x eip => {eip:x}");
            return;
        }
        if (machine.shims.hooks_base..machine.shims.hooks_base + 0x1000).contains(&(eip as u32)) {
            Shims::handle_call(machine).unwrap();
            eip = machine
                .unicorn
                .reg_read(unicorn_engine::RegisterX86::EIP)
                .unwrap();
        } else {
            println!("x no hook eip => {eip:x}");
            break;
        }
    }
}
