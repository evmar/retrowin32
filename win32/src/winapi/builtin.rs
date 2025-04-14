//! Built-in DLLs that are statically linked into the emulator.
//! Each winapi DLL contains a generated 'builtin.rs',
//! which joins the raw DLL bytes with emulator-side implementation.

use crate::shims::Shim;

pub struct BuiltinDLL {
    pub file_name: &'static str,
    /// The xth function in the DLL represents a call to shims[x].
    pub shims: &'static [Shim],
    /// Raw bytes of generated .dll.
    pub raw: &'static [u8],
}

pub const DLLS: [BuiltinDLL; 19] = [
    crate::winapi::advapi32::DLL,
    crate::winapi::bass::DLL,
    crate::winapi::comctl32::DLL,
    crate::winapi::ddraw::DLL,
    crate::winapi::dinput::DLL,
    crate::winapi::dsound::DLL,
    crate::winapi::gdi32::DLL,
    crate::winapi::kernel32::DLL,
    crate::winapi::ntdll::DLL,
    crate::winapi::ole32::DLL,
    crate::winapi::oleaut32::DLL,
    crate::winapi::shlwapi::DLL,
    crate::winapi::ucrtbase::DLL,
    crate::winapi::user32::DLL,
    crate::winapi::vcruntime140::DLL,
    crate::winapi::version::DLL,
    crate::winapi::wininet::DLL,
    crate::winapi::winmm::DLL,
    crate::winapi::retrowin32_test::DLL,
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

        "api-ms-win-core-processthreads-l1-1-2.dll" => "kernel32.dll",
        "api-ms-win-core-profile-l1-1-0.dll" => "kernel32.dll",
        "api-ms-win-core-sysinfo-l1-2-1.dll" => "kernel32.dll",
        "api-ms-win-core-synch-l1-2-0.dll" => "kernel32.dll",
        "api-ms-win-core-libraryloader-l1-2-0.dll" => "kernel32.dll",

        _ => return None,
    })
}
