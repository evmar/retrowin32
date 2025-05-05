//! Built-in DLLs that are statically linked into the emulator.
//! Each winapi DLL contains a generated 'builtin.rs',
//! which joins the raw DLL bytes with emulator-side implementation.

use win32_system::dll::BuiltinDLL;

pub const DLLS: [BuiltinDLL; 19] = [
    builtin_advapi32::DLL,
    builtin_bass::DLL,
    crate::winapi::comctl32::DLL,
    crate::winapi::ddraw::DLL,
    builtin_dinput::DLL,
    builtin_dsound::DLL,
    crate::winapi::gdi32::DLL,
    crate::winapi::kernel32::DLL,
    crate::winapi::ntdll::DLL,
    builtin_ole32::DLL,
    builtin_oleaut32::DLL,
    builtin_shlwapi::DLL,
    crate::winapi::ucrtbase::DLL,
    crate::winapi::user32::DLL,
    builtin_vcruntime140::DLL,
    builtin_version::DLL,
    builtin_wininet::DLL,
    crate::winapi::winmm::DLL,
    builtin_retrowin32_test::DLL,
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
