use crate::machine::Machine;

mod alloc;
pub mod ddraw;
mod dll;
pub mod dsound;
pub mod gdi32;
pub mod kernel32;
mod shims;
pub mod types;
pub mod user32;

// winapi is stdcall, which means args are right to left and callee-cleaned.
// The caller of winapi functions is responsible for pushing/popping the
// return address, because some callers actually 'jmp' directly.
//
// This macro generates shim wrappers of functions, taking their
// input args off the stack and forwarding their return values via eax.
macro_rules! winapi_shims {
    ($(fn $name:ident($($param:ident: $type:ty),* $(,)?);)*) => {
        #[allow(unused_imports)]
        pub mod shims {
            use crate::machine::Machine;
            use super::*;

            $(#[allow(non_snake_case)]
            pub fn $name(machine: &mut Machine) {
                $(let $param: $type = unsafe { crate::winapi::shims::from_x86(&mut machine.x86) };)*
                machine.x86.regs.eax = super::$name(machine, $($param),*);
            })*
        }
    }
}
pub(crate) use winapi_shims;

macro_rules! vtable_entry {
    ($shims:ident $fn:ident ok) => {
        Ok($shims::$fn)
    };
    ($shims:ident $fn:ident todo) => {
        Err(format!("unimplemented in vtable: {}", stringify!($fn)))
    };
    ($shims:ident $fn:ident $impl:tt) => {
        Ok($impl)
    };
}
pub(crate) use vtable_entry;

macro_rules! vtable {
    ($shims:ident $($fn:ident $status:tt,)*) => {
        #[repr(C)]
        struct Vtable {
            $($fn: DWORD),*
        }
        unsafe impl x86::Pod for Vtable {}
        impl Vtable {
            fn new(shims: &mut crate::machine::Shims) -> Self {
                Vtable {
                    $($fn: shims.add($crate::winapi::vtable_entry!($shims $fn $status)).into()),*
                }
            }
        }

        pub fn vtable(ddraw: &mut State, machine: &mut Machine) -> u32 {
            let addr = machine.state.kernel32.get_heap(&mut machine.x86.mem, ddraw.hheap).unwrap().alloc(
                std::mem::size_of::<Vtable>() as u32,
            );
            let vtable = machine.x86.mem.view_mut::<Vtable>(addr);
            *vtable = Vtable::new(&mut machine.shims);
            addr
        }
    };
}
pub(crate) use vtable;

pub enum ImportSymbol<'a> {
    Name(&'a str),
    Ordinal(u32),
}
impl<'a> std::fmt::Display for ImportSymbol<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportSymbol::Name(name) => f.write_str(name),
            ImportSymbol::Ordinal(ord) => f.write_fmt(format_args!("{}", ord)),
        }
    }
}

pub fn resolve(dll: &str, sym: &ImportSymbol) -> Option<fn(&mut Machine)> {
    match dll {
        "ddraw.dll" => dll::ddraw::resolve(sym),
        "dsound.dll" => {
            // TODO: no support for ordinals yet in dll generation machinery.
            match *sym {
                ImportSymbol::Ordinal(1) => return Some(dll::dsound::DirectSoundCreate),
                _ => {}
            };
            dll::dsound::resolve(sym)
        }
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
    pub dsound: dsound::State,
    pub gdi32: gdi32::State,
    pub kernel32: kernel32::State,
    pub user32: user32::State,
}
impl State {
    pub fn new() -> Self {
        State {
            ddraw: ddraw::State::new_empty(),
            dsound: dsound::State::new_empty(),
            gdi32: gdi32::State::new(),
            kernel32: kernel32::State::new(),
            user32: user32::State::new(),
        }
    }
}
