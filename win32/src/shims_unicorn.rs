//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//!
//! This module implements Shims for use within the Unicorn CPU emulator.
//!
//! System function calls eventually map to syscalls.
//! We hook syscalls via the Unicorn API to have them stop Unicorn
//! emulation, then use the current eip to look up which shim to call.

use crate::{shims::Shim, Machine};
use memory::Extensions;
use std::collections::HashMap;

pub fn retrowin32_syscall() -> &'static [u8] {
    // sysenter; ret
    b"\x0f\x34\xc3".as_slice()
}

#[derive(Default)]
pub struct Shims {
    shims: HashMap<u32, Result<&'static Shim, String>>,
}

impl Shims {
    pub fn new(unicorn: &mut unicorn_engine::Unicorn<'static, ()>) -> Self {
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
        unicorn
            .add_code_hook(0, 0xFFFF_FFFF, |_unicorn, addr, size| {
                println!("u {addr:x}+{size:x}");
            })
            .unwrap();

        Shims {
            shims: Default::default(),
        }
    }

    pub fn register(&mut self, addr: u32, shim: Result<&'static Shim, String>) {
        self.shims.insert(addr, shim);
    }
}

fn handle_shim_call(machine: &mut Machine) {
    // stack looks like:
    //   return address within foo.dll!Foo (*1*)  <- esp
    //   return address within foo.exe (*2*)
    //   arg1 to Foo
    //   arg2 to Foo
    let esp = machine
        .emu
        .unicorn
        .reg_read(unicorn_engine::RegisterX86::ESP)
        .unwrap() as u32;
    // Return address - 6 = address of foo.dll!Foo
    let addr = machine.emu.memory.mem().get_pod::<u32>(esp) - 6;
    let shim = match machine.emu.shims.shims.get(&addr) {
        Some(Ok(shim)) => shim,
        Some(Err(name)) => unimplemented!("shim call to {name}"),
        None => panic!("unknown import reference at {addr:x}"),
    };
    let name = shim.name;

    let func = shim.func;
    let ret = unsafe { func(machine, esp + 4) }; // address of *2* above

    machine
        .emu
        .unicorn
        .reg_write(unicorn_engine::RegisterX86::EAX, ret as u64)
        .unwrap();
}

pub async fn call_x86(machine: &mut Machine, func: u32, args: Vec<u32>) -> u32 {
    let mem = machine.emu.memory.mem();

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
        mem.put_pod::<u32>(esp, arg);
    }
    esp -= 4;
    mem.put_pod::<u32>(esp, ret_addr);

    machine
        .emu
        .unicorn
        .reg_write(unicorn_engine::RegisterX86::ESP, esp as u64)
        .unwrap();

    unicorn_loop(machine, func, ret_addr);

    machine
        .emu
        .unicorn
        .reg_read(unicorn_engine::RegisterX86::EAX)
        .unwrap() as u32
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
        handle_shim_call(machine);
    }
}
