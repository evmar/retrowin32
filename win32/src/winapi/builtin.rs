//! Built-in DLLs that are statically linked into the emulator.
//! Each winapi DLL contains a generated 'builtin.rs',
//! which joins the raw DLL bytes with emulator-side implementation.

use builtin_kernel32 as kernel32;
use win32_system::dll::{BuiltinDLL, DLLResolution};

pub const DLLS: [BuiltinDLL; 19] = [
    builtin_advapi32::DLL,
    builtin_bass::DLL,
    builtin_comctl32::DLL,
    builtin_ddraw::DLL,
    builtin_dinput::DLL,
    builtin_dsound::DLL,
    builtin_gdi32::DLL,
    builtin_kernel32::DLL,
    crate::winapi::ntdll::DLL,
    builtin_ole32::DLL,
    builtin_oleaut32::DLL,
    builtin_shlwapi::DLL,
    builtin_ucrtbase::DLL,
    builtin_user32::DLL,
    builtin_vcruntime140::DLL,
    builtin_version::DLL,
    builtin_wininet::DLL,
    builtin_winmm::DLL,
    builtin_retrowin32_test::DLL,
];

fn dll_alias(name: &str) -> Option<&'static str> {
    Some(match name {
        "msvcrt.dll" => "ucrtbase.dll",
        _ => return None,
    })
}

/// Maps a DLL "api set" alias to the underlying dll.
/// https://learn.microsoft.com/en-us/windows/win32/apiindex/api-set-loader-operation
fn apiset(name: &str) -> Option<&'static str> {
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

pub fn resolve_dll(external_dlls: &[String], filename: &str) -> DLLResolution {
    let mut filename = kernel32::loader::normalize_module_name(filename);
    if filename.starts_with("api-") {
        match apiset(&filename) {
            Some(name) => filename = name.to_string(),
            None => return DLLResolution::External(filename),
        }
    }

    let use_external = external_dlls.contains(&filename);
    if !use_external {
        if let Some(alias) = dll_alias(&filename) {
            filename = alias.to_string();
        }
        if let Some(builtin) = DLLS.iter().find(|&dll| dll.file_name == filename) {
            return DLLResolution::Builtin(builtin);
        }
    }

    DLLResolution::External(filename)
}
