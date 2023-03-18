use crate::machine::Machine;

/// Code that calls from x86 to the host will jump to addresses in this
/// magic range.
/// "fake IAT" => "FIAT" => "F1A7"
pub const SHIM_BASE: u32 = 0xF1A7_0000;

struct Shim {
    name: String,
    handler: Option<fn(&mut Machine)>,
}

/// Shims that want to call back into x86 code return one of these,
/// which is a closure that carries its state across multiple returns.
pub type ShimCallback = Box<dyn FnMut(&mut Machine) -> CallbackStep>;

/// Steps within a ShimCallback can either call into an x86 address or
/// return and clean up.
#[derive(Debug)]
pub enum CallbackStep {
    /// Call first arg with arguments on stack.
    Call(u32, Vec<u32>),
    /// Done, and return argument via eax.
    Done(u32),
}

/// Jumps to memory address SHIM_BASE+x are interpreted as calling shims[x].
/// This is how emulated code calls out to hosting code for e.g. DLL imports.
pub struct Shims {
    shims: Vec<Shim>,
    callback_fns: Vec<ShimCallback>,
    /// Address of callback_helper() shim entry point.
    callback_helper: u32,
}

fn callback_helper(machine: &mut Machine) {
    log::info!("in callback_helper esp:{:x}", machine.x86.regs.esp);
    let fn_id = 0; // TODO: get from stack.
    let mut callback: ShimCallback = Box::new(|_machine| unreachable!());
    std::mem::swap(
        &mut machine.shims.callback_fns[fn_id as usize],
        &mut callback,
    );
    let next = callback(machine);
    log::info!("next {:x?}", next);
    match next {
        CallbackStep::Call(addr, args) => {
            // Put the callback back in place.
            std::mem::swap(
                &mut callback,
                &mut machine.shims.callback_fns[fn_id as usize],
            );

            // Set things up so that we call this function next.
            for &arg in args.iter().rev() {
                x86::ops::push(&mut machine.x86, arg);
            }
            x86::ops::push(&mut machine.x86, machine.shims.callback_helper);
            machine.x86.regs.eip = addr;
        }
        CallbackStep::Done(ret) => {
            machine.x86.regs.eax = ret;
            machine.x86.regs.eip = machine.x86.read_u32(machine.x86.regs.esp);
            machine.x86.regs.esp -= 8; // return address + id
            log::warn!("TODO: cleanup shim callback");
        }
    }
}

pub fn push_callback(machine: &mut Machine, return_address: u32, callback: ShimCallback) {
    let id = machine.shims.add_callback(callback);
    if id != 0 {
        unimplemented!(); // TODO
    }
    x86::ops::push(&mut machine.x86, id);
    x86::ops::push(&mut machine.x86, return_address);
    machine.x86.regs.eip = machine.shims.callback_helper;
}

impl Shims {
    pub fn new() -> Self {
        let mut shims = Shims {
            shims: Vec::new(),
            callback_fns: Vec::new(),
            callback_helper: 0,
        };
        shims.callback_helper =
            shims.add("retrowin32 callback helper".into(), Some(callback_helper));
        shims
    }

    /// Returns the (fake) address of the registered function.
    pub fn add(&mut self, name: String, handler: Option<fn(&mut Machine)>) -> u32 {
        let id = SHIM_BASE | self.shims.len() as u32;
        self.shims.push(Shim { name, handler });
        id
    }

    pub fn get(&self, addr: u32) -> Option<&fn(&mut Machine)> {
        let index = (addr & 0x0000_FFFF) as usize;
        match self.shims.get(index) {
            Some(shim) => {
                if let Some(handler) = &shim.handler {
                    return Some(handler);
                }
                log::error!("unimplemented: {}", shim.name);
            }
            None => log::error!("unknown import reference at {:x}", addr),
        };
        None
    }

    pub fn lookup(&self, name: &str) -> Option<u32> {
        if let Some(idx) = self.shims.iter().position(|shim| shim.name == name) {
            return Some(SHIM_BASE | idx as u32);
        }
        None
    }

    pub fn add_callback(&mut self, callback: ShimCallback) -> u32 {
        self.callback_fns.push(callback);
        (self.callback_fns.len() - 1) as u32
    }
}
