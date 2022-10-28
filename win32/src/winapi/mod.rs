use crate::x86::X86;

pub mod ddraw;
mod dll;
pub mod gdi32;
pub mod kernel32;
mod shims;
pub mod user32;

// winapi is stdcall, which means args are right to left and callee-cleaned.
// The caller of winapi functions is responsible for pushing/popping the
// return address, because some callers actually 'jmp' directly.
//
// This macro generates shim wrappers of functions, taking their
// input args off the stack and forwarding their return values via eax.
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
        unsafe impl crate::memory::Pod for Vtable {}
        impl Vtable {
            fn new(shims: &mut crate::x86::Shims) -> Self {
                Vtable {
                    $($fn: shims.add($crate::winapi::vtable_entry!($shims $fn $status)).into()),*
                }
            }
        }

        pub fn vtable(ddraw: &mut State, x86: &mut X86) -> u32 {
            let addr = ddraw.heap(&mut x86.state.kernel32).alloc(
                &mut x86.mem,
                std::mem::size_of::<Vtable>() as u32,
            );
            let vtable = x86.mem.view_mut::<Vtable>(addr);
            *vtable = Vtable::new(&mut x86.shims);
            addr
        }
    };
}
pub(crate) use vtable;

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
