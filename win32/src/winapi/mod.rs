mod advapi32;
mod alloc;
mod bass;
mod bitmap;
mod builtin;
mod com;
pub mod ddraw;
pub mod dsound;
pub mod gdi32;
mod handle;
mod heap;
pub mod kernel32;
mod ntdll;
mod ole32;
mod oleaut32;
mod retrowin32_test;
mod stack_args;
pub mod types;
mod ucrtbase;
pub mod user32;
mod vcruntime140;
mod winmm;

macro_rules! vtable_entry {
    ($iface:ident $shims:ident $fn:ident todo) => {
        Err(format!(
            "{} vtable::{}",
            stringify!($iface),
            stringify!($fn)
        ))
    };
    ($iface:ident $shims:ident $fn:ident ok) => {
        Ok(&$shims::$fn)
    };
    ($iface:ident $shims:ident $fn:ident $shim:tt) => {
        Ok(&$shim)
    };
}
pub(crate) use vtable_entry;

macro_rules! vtable {
    ($iface:ident $shims:ident $($fn:ident $impl:tt,)*) => {
        #[repr(C)]
        struct Vtable {
            $($fn: u32),*
        }
        unsafe impl memory::Pod for Vtable {}
        impl Vtable {
            fn new(mut register: impl FnMut(Result<&'static $crate::shims::Shim, String>) -> u32) -> Self {
                Vtable {
                    $($fn: register($crate::winapi::vtable_entry!($iface $shims $fn $impl))),*
                }
            }
        }

        pub fn vtable(mem: memory::Mem, heap: &mut $crate::winapi::heap::Heap, register: impl FnMut(Result<&'static $crate::shims::Shim, String>) -> u32) -> u32 {
            let addr = heap.alloc(mem, std::mem::size_of::<Vtable>() as u32);
            let vtable = Vtable::new(register);
            *mem.view_mut::<Vtable>(addr) = vtable;
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

pub const DLLS: [builtin::BuiltinDLL; 14] = [
    builtin::advapi32::DLL,
    builtin::bass::DLL,
    builtin::ddraw::DLL,
    builtin::dsound::DLL,
    builtin::gdi32::DLL,
    builtin::kernel32::DLL,
    builtin::ntdll::DLL,
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
