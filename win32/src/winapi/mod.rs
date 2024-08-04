use crate::machine::MemImpl;

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
mod version;
mod winmm;

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

pub const DLLS: [builtin::BuiltinDLL; 15] = [
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
    builtin::version::DLL,
    builtin::winmm::DLL,
    builtin::retrowin32_test::DLL,
];

pub fn dll_alias(name: &str) -> Option<&'static str> {
    Some(match name {
        "msvcrt.dll" => "ucrtbase.dll",
        _ => return None,
    })
}

/// Maps a DLL "api set" alias to the underlying dll.
/// https://learn.microsoft.com/en-us/windows/win32/apiindex/api-set-loader-operation
pub fn apiset(name: &str) -> Option<&'static str> {
    Some(match name {
        "api-ms-win-crt-heap-l1-1-0.dll" => "ucrtbase.dll",
        "api-ms-win-crt-locale-l1-1-0.dll" => "ucrtbase.dll",
        "api-ms-win-crt-runtime-l1-1-0.dll" => "ucrtbase.dll",
        "api-ms-win-crt-stdio-l1-1-0.dll" => "ucrtbase.dll",
        "api-ms-win-crt-string-l1-1-0.dll" => "ucrtbase.dll",
        _ => return None,
    })
}

pub struct State {
    scratch: heap::Heap,

    pub ddraw: ddraw::State,
    pub dsound: dsound::State,
    pub gdi32: gdi32::State,
    pub kernel32: kernel32::State,
    pub user32: user32::State,
}

impl State {
    pub fn new(mem: &mut MemImpl, mut kernel32: kernel32::State) -> Self {
        let scratch = kernel32.new_private_heap(mem, 0x1000, "winapi scratch".into());

        State {
            scratch,
            ddraw: ddraw::State::default(),
            dsound: dsound::State::default(),
            gdi32: gdi32::State::default(),
            kernel32,
            user32: user32::State::default(),
        }
    }
}
