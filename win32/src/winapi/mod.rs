use crate::x86::X86;

pub mod ddraw;
mod dll;
pub mod gdi32;
pub mod kernel32;
pub mod user32;

// winapi is stdcall, which means args are right to left and callee-cleaned.
// The caller of winapi functions is responsible for pushing/popping the
// return address, because some callers actually 'jmp' directly.
//
// This macro generates shim wrappers of functions, taking their
// input args off the stack and forwarding their return values via eax.
#[macro_export]
macro_rules! winapi_shims {
    ($(fn $name:ident($($param:ident: $type:ident),* $(,)?);)*) => {
        pub mod shims {
            use crate::x86::X86;

            $(#[allow(non_snake_case)]
            pub fn $name(x86: &mut X86) {
                $(let $param: $type = x86.pop();)*
                x86.regs.eax = super::$name(x86, $($param),*);
            })*
        }
    }
}

pub fn resolve(dll: &str, sym: &str) -> Option<fn(&mut X86)> {
    match dll {
        "ddraw.dll" => dll::ddraw::resolve(sym),
        "gdi32.dll" => dll::gdi32::resolve(sym),
        "kernel32.dll" => dll::kernel32::resolve(sym),
        "user32.dll" => dll::user32::resolve(sym),
        _ => None,
    }
    // .or_else(|| {
    //     log::warn!("unresolved symbol {}:{}", dll, sym);
    //     None
    // })
}

pub struct State {
    pub ddraw: ddraw::State,
    pub gdi32: gdi32::State,
    pub kernel32: kernel32::State,
    pub user32: user32::State,
}
impl State {
    pub fn new() -> Self {
        State {
            ddraw: ddraw::State::new_empty(),
            gdi32: gdi32::State::new(),
            kernel32: kernel32::State::new(),
            user32: user32::State::new(),
        }
    }
}
