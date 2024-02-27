mod advapi32;
mod alloc;
mod bass;
mod builtin;
pub mod ddraw;
pub mod dsound;
pub mod gdi32;
mod handle;
mod heap;
pub mod kernel32;
mod ole32;
mod oleaut32;
mod retrowin32_test;
mod stack_args;
mod str16;
pub mod types;
mod ucrtbase;
pub mod user32;
mod vcruntime140;
mod winmm;

macro_rules! vtable_entry {
    ($shims:expr, $module:ident $fn:ident todo) => {
        $shims.register(Err(format!("{}:{}", stringify!($module), stringify!($fn))))
    };
    ($shims:expr, $module:ident $fn:ident ok) => {
        $shims.register(Ok(&$module::$fn))
    };
    ($shims:expr, $module:ident $fn:ident $shim:tt) => {
        $shims.register(Ok(&$shim))
    };
}
pub(crate) use vtable_entry;

macro_rules! vtable {
    ($iface:ident $module:ident $($fn:ident $impl:tt,)*) => {
        #[repr(C)]
        struct Vtable {
            $($fn: DWORD),*
        }
        unsafe impl memory::Pod for Vtable {}
        impl Vtable {
            fn new(machine: &mut crate::Machine) -> Self {
                Vtable {
                    $($fn: $crate::winapi::vtable_entry!(machine.emu, $module $fn $impl).into()),*
                }
            }
        }

        pub fn vtable(state: &mut State, machine: &mut Machine) -> u32 {
            let addr = state.heap.alloc(machine.memory.mem(), std::mem::size_of::<Vtable>() as u32);
            let vtable = Vtable::new(machine);
            *machine.memory.mem().view_mut::<Vtable>(addr) = vtable;
            addr
        }
    };
}
pub(crate) use vtable;

#[derive(Debug)]
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

pub const DLLS: [builtin::BuiltinDLL; 13] = [
    builtin::advapi32::DLL,
    builtin::bass::DLL,
    builtin::ddraw::DLL,
    builtin::dsound::DLL,
    builtin::gdi32::DLL,
    builtin::kernel32::DLL,
    builtin::ole32::DLL,
    builtin::oleaut32::DLL,
    builtin::ucrtbase::DLL,
    builtin::user32::DLL,
    builtin::vcruntime140::DLL,
    builtin::winmm::DLL,
    builtin::retrowin32_test::DLL,
];

/// Maps a DLL "api set" alias to the underlying dll.
/// https://learn.microsoft.com/en-us/windows/win32/apiindex/api-set-loader-operation
pub fn apiset(name: &str) -> Option<&'static str> {
    Some(match name {
        "api-ms-win-crt-runtime-l1-1-0.dll" => "ucrtbase.dll",
        _ => return None,
    })
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
    pub fn new(kernel32: kernel32::State) -> Self {
        State {
            ddraw: ddraw::State::default(),
            dsound: dsound::State::default(),
            gdi32: gdi32::State::default(),
            kernel32,
            user32: user32::State::default(),
        }
    }
}
