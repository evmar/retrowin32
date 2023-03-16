use crate::machine::Machine;

/// Code that calls from x86 to the host will jump to addresses in this
/// magic range.
/// "fake IAT" => "FIAT" => "F1A7"
pub const SHIM_BASE: u32 = 0xF1A7_0000;

struct Shim {
    name: String,
    handler: Option<fn(&mut Machine)>,
}

/// Jumps to memory address SHIM_BASE+x are interpreted as calling shims[x].
/// This is how emulated code calls out to hosting code for e.g. DLL imports.
pub struct Shims {
    shims: Vec<Shim>,
}

impl Shims {
    pub fn new() -> Self {
        Shims { shims: Vec::new() }
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
}
