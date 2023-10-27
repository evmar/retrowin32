//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//!
//! This module implements Shims for non-emulated cpu case, using raw 32-bit memory.
//! See doc/x86-64.md for an overview.

use memory::Mem;

use crate::{shims::Shim, Machine};

#[derive(Default)]
pub struct Shims {
    shims: Vec<Shim>,
    sysenters_addr: u32,
}

impl Shims {
    pub fn new(mem: Mem, addr: u32) -> Self {
        for i in 0..mem.len() / 2 {
            mem.put::<u16>(i * 2, 0x340f);
        }

        Shims {
            shims: Default::default(),
            sysenters_addr: addr,
        }
    }

    /// HACK: we need a pointer to the Machine, but we get it so late we have to poke it in
    /// way after all the initialization happens...
    pub unsafe fn set_machine_hack(
        &mut self,
        machine: *mut Machine,
        unicorn: &mut unicorn_engine::Unicorn<'static, ()>,
    ) {
        unicorn
            .add_insn_sys_hook(
                unicorn_engine::InsnSysX86::SYSENTER,
                0,
                0xFFFF_FFFF,
                move |_u| {
                    let machine = &mut *machine;
                    Shims::handle_call(machine).unwrap();
                },
            )
            .unwrap();
    }

    pub fn add(&mut self, shim: Shim) -> u32 {
        let index = self.shims.len() as u32;
        let addr = self.sysenters_addr + index * 2;
        self.shims.push(shim);
        addr
    }

    pub fn add_todo(&mut self, _name: String) -> u32 {
        log::warn!("todo: register shim");
        0
    }

    pub fn handle_call(machine: &mut Machine) -> anyhow::Result<()> {
        let eip = machine
            .unicorn
            .reg_read(unicorn_engine::RegisterX86::EIP)
            .unwrap() as u32;
        let index = (eip - machine.shims.sysenters_addr) / 2;
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

        let next_eip = machine.mem().get::<u32>(esp);
        machine
            .unicorn
            .reg_write(
                unicorn_engine::RegisterX86::EIP,
                // -2 here because it appears unicorn increments eip unconditionally from the 2-byte sysenter
                next_eip as u64 - 2,
            )
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
    machine.unicorn.emu_start(func as u64, 0, 0, 0).unwrap();
    UnimplFuture {}
}
