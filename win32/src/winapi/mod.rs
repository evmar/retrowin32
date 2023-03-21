use crate::machine::Machine;

mod alloc;
mod bass;
pub mod ddraw;
mod dll;
pub mod dsound;
pub mod gdi32;
pub mod kernel32;
mod ole32;
mod oleaut32;
mod stack_args;
pub mod types;
pub mod user32;
mod winmm;

macro_rules! vtable_entry {
    ($shims:ident $fn:ident ok) => {
        Some($shims::$fn)
    };
    ($shims:ident $fn:ident todo) => {
        None
    };
    ($shims:ident $fn:ident $impl:tt) => {
        Some($impl)
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
            fn new(shims: &mut crate::shims::Shims) -> Self {
                Vtable {
                    $($fn: shims.add(stringify!($fn).into(), $crate::winapi::vtable_entry!($shims $fn $status)).into()),*
                }
            }
        }

        pub fn vtable(state: &mut State, machine: &mut Machine) -> u32 {
            let addr = machine.state.kernel32.get_heap(&mut machine.x86.mem, state.hheap).unwrap().alloc(
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

pub struct BuiltinDLL {
    file_name: &'static str,
    resolve: fn(&ImportSymbol) -> Option<fn(&mut Machine)>,
}

pub const DLLS: [BuiltinDLL; 9] = [
    dll::bass::DLL,
    dll::ddraw::DLL,
    dll::dsound::DLL,
    dll::gdi32::DLL,
    dll::kernel32::DLL,
    dll::ole32::DLL,
    dll::oleaut32::DLL,
    dll::user32::DLL,
    dll::winmm::DLL,
];

pub fn resolve(file_name: &str, sym: &ImportSymbol) -> Option<fn(&mut Machine)> {
    // TODO: no support for ordinals yet in dll generation machinery.
    if file_name == dll::dsound::DLL.file_name {
        match *sym {
            ImportSymbol::Ordinal(1) => return Some(dll::dsound::DirectSoundCreate),
            _ => {}
        }
    }

    let dll = DLLS.iter().find(|&dll| dll.file_name == file_name)?;
    (dll.resolve)(sym)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct State {
    #[serde(skip)] // TODO
    pub ddraw: ddraw::State,
    #[serde(skip)] // TODO
    pub dsound: dsound::State,
    #[serde(skip)] // TODO
    pub gdi32: gdi32::State,
    pub kernel32: kernel32::State,
    #[serde(skip)] // TODO
    pub user32: user32::State,
}

impl State {
    pub fn new() -> Self {
        State {
            ddraw: ddraw::State::default(),
            dsound: dsound::State::default(),
            gdi32: gdi32::State::default(),
            kernel32: kernel32::State::new(),
            user32: user32::State::default(),
        }
    }
}
