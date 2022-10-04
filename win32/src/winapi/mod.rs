use crate::x86;
use crate::X86;

pub mod ddraw;
pub mod gdi32;
pub mod kernel32;
pub mod user32;

struct DWORD([u8; 4]);
impl DWORD {
    fn set(&mut self, val: u32) {
        self.0[0] = val as u8;
        self.0[1] = (val >> 8) as u8;
        self.0[2] = (val >> 16) as u8;
        self.0[3] = (val >> 24) as u8;
    }
}

// winapi is stdcall, which means args are right to left and callee-cleaned.
// The caller of winapi functions is responsible for pushing/popping the
// return address, because some callers actually 'jmp' directly.
//
// This macro generates a shim wrapper of a function, taking its
// input args off the stack and forwarding its return values via eax.
#[macro_export]
macro_rules! winapi_shim {
    (fn $name:ident($($param:ident: $type:ident),* $(,)?)) => {
        #[allow(non_snake_case)]
        pub fn $name(x86: &mut X86) {
            $(let $param: $type = x86.pop();)*
            x86.regs.eax = super::$name(x86, $($param),*);
        }
    }
}

// This macro runs a block of the above and also generates a resolve()
// function that maps symbol names to shims.
#[macro_export]
macro_rules! winapi {
    ($(fn $name:ident($($param:ident: $type:ident),* $(,)?);)*) => {
        mod shims {
            use super::X86;
            use crate::winapi_shim;
            $(winapi_shim!(fn $name($($param: $type),*));)*
        }

        pub fn resolve(name: &str) -> Option<fn(&mut X86)> {
            Some(match name {
                $(stringify!($name) => shims::$name,)*
                _ => return None,
            })
        }
    }
}

pub fn resolve(dll: &str, sym: &str) -> Option<fn(&mut X86)> {
    match dll {
        "ddraw.dll" => ddraw::resolve(sym),
        "gdi32.dll" => gdi32::resolve(sym),
        "kernel32.dll" => kernel32::resolve(sym),
        "user32.dll" => user32::resolve(sym),
        _ => None,
    }
    // .or_else(|| {
    //     log::warn!("unresolved symbol {}:{}", dll, sym);
    //     None
    // })
}

pub struct State {
    pub ddraw: ddraw::State,
    pub kernel32: kernel32::State,
    pub user32: user32::State,
}
impl State {
    pub fn new() -> Self {
        State {
            ddraw: ddraw::State::new(),
            kernel32: kernel32::State::new(),
            user32: user32::State::new(),
        }
    }
}
