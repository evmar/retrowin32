//! kernel32 API without a better home.

use super::{teb_mut, WriteFile};
use crate::{
    winapi::{
        stack_args::{ArrayWithSize, ArrayWithSizeMut},
        types::*,
    },
    Machine,
};
use ::memory::{Extensions, Pod};
use bitflags::bitflags;

const TRACE_CONTEXT: &'static str = "kernel32/misc";

pub fn set_last_error(machine: &mut Machine, err: u32) {
    teb_mut(machine).LastErrorValue = err;
}

#[win32_derive::dllexport]
pub fn SetLastError(machine: &mut Machine, dwErrCode: u32) -> u32 {
    set_last_error(machine, dwErrCode);
    0 // unused
}

#[win32_derive::dllexport]
pub fn GetLastError(machine: &mut Machine) -> u32 {
    teb_mut(machine).LastErrorValue
}

#[win32_derive::dllexport]
pub fn ExitProcess(machine: &mut Machine, uExitCode: u32) -> u32 {
    machine.host.exit(uExitCode);
    // TODO: this is unsatisfying.
    // Maybe better is to generate a hlt instruction somewhere and jump to it?
    #[cfg(feature = "x86-emu")]
    {
        machine.emu.x86.cpu_mut().state = x86::CPUState::Exit(uExitCode);
    }
    0
}

#[win32_derive::dllexport]
pub fn GetACP(_machine: &mut Machine) -> u32 {
    1252 // windows-1252
}

#[win32_derive::dllexport]
pub fn IsValidCodePage(_machine: &mut Machine, CodePage: u32) -> bool {
    CodePage == 1252
}

#[win32_derive::dllexport]
pub fn GetCPInfo(_machine: &mut Machine, _CodePage: u32, _lpCPInfo: u32) -> u32 {
    0 // fail
}

#[win32_derive::dllexport]
pub fn GetEnvironmentStrings(machine: &mut Machine) -> u32 {
    machine.state.kernel32.env
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsA(_machine: &mut Machine, _penv: u32) -> u32 {
    1 // success
}

#[win32_derive::dllexport]
pub fn GetEnvironmentStringsW(_machine: &mut Machine) -> u32 {
    // CRT startup appears to fallback on non-W version of this if it returns null.
    0
}

#[win32_derive::dllexport]
pub fn GetEnvironmentVariableA(
    _machine: &mut Machine,
    name: Option<&str>,
    buf: ArrayWithSize<u8>,
) -> bool {
    false
}

#[win32_derive::dllexport]
pub fn GetEnvironmentVariableW(
    _machine: &mut Machine,
    name: Option<&Str16>,
    buf: ArrayWithSize<u16>,
) -> bool {
    false
}

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum ProcessorFeature {
    FLOATING_POINT_PRECISION_ERRATA = 0,
    FLOATING_POINT_EMULATED = 1,
    COMPARE_EXCHANGE_DOUBLE = 2,
    MMX_INSTRUCTIONS_AVAILABLE = 3,
    PPC_MOVEMEM_64BIT_OK = 4,
    ALPHA_BYTE_INSTRUCTIONS = 5,
    XMMI_INSTRUCTIONS_AVAILABLE = 6,
    _3DNOW_INSTRUCTIONS_AVAILABLE = 7,
    RDTSC_INSTRUCTION_AVAILABLE = 8,
    PAE_ENABLED = 9,
    XMMI64_INSTRUCTIONS_AVAILABLE = 10,
    SSE_DAZ_MODE_AVAILABLE = 11,
    NX_ENABLED = 12,
    SSE3_INSTRUCTIONS_AVAILABLE = 13,
    COMPARE_EXCHANGE128 = 14,
    COMPARE64_EXCHANGE128 = 15,
    CHANNELS_ENABLED = 16,
    XSAVE_ENABLED = 17,
    ARM_VFP_32_REGISTERS_AVAILABLE = 18,
    ARM_NEON_INSTRUCTIONS_AVAILABLE = 19,
    SECOND_LEVEL_ADDRESS_TRANSLATION = 20,
    VIRT_FIRMWARE_ENABLED = 21,
    RDWRFSGSBASE_AVAILABLE = 22,
    FASTFAIL_AVAILABLE = 23,
    ARM_DIVIDE_INSTRUCTION_AVAILABLE = 24,
    ARM_64BIT_LOADSTORE_ATOMIC = 25,
    ARM_EXTERNAL_CACHE_AVAILABLE = 26,
    ARM_FMAC_INSTRUCTIONS_AVAILABLE = 27,
    RDRAND_INSTRUCTION_AVAILABLE = 28,
    ARM_V8_INSTRUCTIONS_AVAILABLE = 29,
    ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE = 30,
    ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE = 31,
    RDTSCP_INSTRUCTION_AVAILABLE = 32,
}

#[win32_derive::dllexport]
pub fn IsProcessorFeaturePresent(
    _machine: &mut Machine,
    feature: Result<ProcessorFeature, u32>,
) -> bool {
    log::warn!("IsProcessorFeaturePresent({feature:?}) => false");
    false
}

#[win32_derive::dllexport]
pub fn IsDebuggerPresent(_machine: &mut Machine) -> bool {
    true // Might cause a binary to log info via the debug API? Not sure.
}

#[win32_derive::dllexport]
pub fn GetCurrentProcessId(_machine: &mut Machine) -> u32 {
    1
}

#[win32_derive::dllexport]
pub fn GetVersion(_machine: &mut Machine) -> u32 {
    // Win95, version 4.0.
    (1 << 31) | 0x4
}

#[repr(C)]
#[derive(Debug)]
pub struct OSVERSIONINFO {
    dwOSVersionInfoSize: DWORD,
    dwMajorVersion: DWORD,
    dwMinorVersion: DWORD,
    dwBuildNumber: DWORD,
    dwPlatformId: DWORD,
    //szCSDVersion: [u8; 128],
}
unsafe impl Pod for OSVERSIONINFO {}

#[win32_derive::dllexport]
pub fn GetVersionExA(
    _machine: &mut Machine,
    lpVersionInformation: Option<&mut OSVERSIONINFO>,
) -> u32 {
    let info = lpVersionInformation.unwrap();
    if info.dwOSVersionInfoSize < std::mem::size_of::<OSVERSIONINFO>() as u32 {
        log::error!("GetVersionExA undersized buffer");
        return 0;
    }
    unsafe { info.clear_memory(info.dwOSVersionInfoSize) };

    info.dwMajorVersion = 6; // ? pulled from debugger
    info.dwPlatformId = 2 /* VER_PLATFORM_WIN32_NT */;

    1
}

#[win32_derive::dllexport]
pub fn SetHandleCount(_machine: &mut Machine, uNumber: u32) -> u32 {
    // "For Windows Win32 systems, this API has no effect."
    uNumber
}

#[win32_derive::dllexport]
pub fn OutputDebugStringA(_machine: &mut Machine, msg: Option<&str>) -> u32 {
    log::warn!("OutputDebugStringA: {:?}", msg);
    0
}

#[win32_derive::dllexport]
pub fn WriteConsoleA(
    _machine: &mut Machine,
    hConsoleOutput: HANDLE<()>,
    lpBuffer: ArrayWithSize<u8>,
    lpNumberOfCharsWritten: Option<&mut u32>,
    lpReserved: u32,
) -> bool {
    let msg = std::str::from_utf8(lpBuffer.unwrap()).unwrap();
    log::warn!("WriteConsoleA: {:?}", msg);
    if let Some(w) = lpNumberOfCharsWritten {
        *w = msg.len() as u32;
    }
    true // success
}

#[win32_derive::dllexport]
pub fn SetUnhandledExceptionFilter(_machine: &mut Machine, _lpTopLevelExceptionFilter: u32) -> u32 {
    0 // No current handler.
}

#[win32_derive::dllexport]
pub fn UnhandledExceptionFilter(_machine: &mut Machine, _exceptionInfo: u32) -> u32 {
    // "The process is being debugged, so the exception should be passed (as second chance) to the application's debugger."
    0 // EXCEPTION_CONTINUE_SEARCH
}

#[win32_derive::dllexport]
pub fn NtCurrentTeb(machine: &mut Machine) -> u32 {
    machine.state.kernel32.teb
}

// TODO: this has a bunch of synchronization magic that I haven't implemented,
// but I did at least make this struct the right size (128 bits).
#[repr(C)]
#[derive(Debug)]
pub struct SLIST_HEADER {
    Next: u32,
    todo: [u32; 3],
}
unsafe impl ::memory::Pod for SLIST_HEADER {}

#[win32_derive::dllexport]
pub fn InitializeSListHead(_machine: &mut Machine, ListHead: Option<&mut SLIST_HEADER>) -> u32 {
    ListHead.unwrap().Next = 0;
    0
}

/// Code pages
#[derive(Debug, win32_derive::TryFromEnum)]
pub enum CP {
    /// The system default Windows ANSI code page.
    ACP = 0,
    OEMCP = 1,
    WINDOWS_1252 = 1252,
    UTF8 = 65001,
}

#[win32_derive::dllexport]
pub fn MultiByteToWideChar(
    machine: &mut Machine,
    CodePage: Result<CP, u32>,
    dwFlags: u32,
    lpMultiByteStr: u32,
    cbMultiByte: i32,
    lpWideCharStr: ArrayWithSizeMut<u16>,
) -> u32 {
    match CodePage {
        Err(value) => unimplemented!("MultiByteToWideChar code page {value}"),
        _ => {} // treat all others as ansi for now
    }
    // TODO: dwFlags

    let input_len = match cbMultiByte {
        0 => return 0,                                               // TODO: invalid param
        -1 => machine.mem().slicez(lpMultiByteStr).len() as u32 + 1, // include nul
        len => len as u32,
    };

    let mut lpWideCharStr = lpWideCharStr.to_option();
    match lpWideCharStr {
        Some(buf) if buf.len() == 0 => lpWideCharStr = None,
        _ => (),
    };

    match lpWideCharStr {
        None => input_len,
        Some(buf) => {
            let input = machine.mem().sub(lpMultiByteStr, input_len);
            let mut len = 0;
            for (&c_in, c_out) in std::iter::zip(input.as_slice_todo(), buf) {
                if c_in > 0x7f {
                    unimplemented!("unicode");
                }
                *c_out = c_in as u16;
                len += 1;
            }
            len
        }
    }
}

#[win32_derive::dllexport]
pub fn WriteConsoleW(
    machine: &mut Machine,
    hConsoleOutput: HFILE,
    lpBuffer: ArrayWithSize<u16>,
    lpNumberOfCharsWritten: Option<&mut u32>,
    _lpReserved: u32,
) -> bool {
    let buf = Str16::from_buffer(lpBuffer.unwrap()).to_string();
    let mut bytes_written = 0;
    if !WriteFile(
        machine,
        hConsoleOutput,
        Some(buf.as_bytes()),
        Some(&mut bytes_written),
        0,
    ) {
        return false;
    }
    if let Some(chars_written) = lpNumberOfCharsWritten {
        *chars_written = bytes_written;
    }
    return bytes_written == buf.len() as u32;
}

#[win32_derive::dllexport]
pub fn SetPriorityClass(
    _machine: &mut Machine,
    hProcess: HANDLE<()>,
    dwPriorityClass: u32,
) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn AddVectoredExceptionHandler(_machine: &mut Machine, first: u32, handler: u32) -> u32 {
    handler // success
}

bitflags! {
    pub struct FormatMessageFlags: u32 {
        const FROM_STRING    = 0x00000400;
        const IGNORE_INSERTS = 0x00000200;
        const FROM_SYSTEM    = 0x00001000;

        // Low 8 bits can be used for line breaking width (!?).
        // Not sure if this makes bitflags do the right thing...
        const MAX_WIDTH_MASK = 0xFF;
    }
}
impl TryFrom<u32> for FormatMessageFlags {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FormatMessageFlags::from_bits(value).ok_or(value)
    }
}

#[win32_derive::dllexport]
pub fn FormatMessageW(
    machine: &mut Machine,
    dwFlags: Result<FormatMessageFlags, u32>,
    lpSource: u32,
    dwMessageId: u32,
    dwLanguageId: u32,
    lpBuffer: u32,
    nSize: u32,
    args: u32,
) -> u32 {
    // Note args is a va_list*, not a va_list!

    let flags = dwFlags.unwrap();

    if dwLanguageId != 0 {
        todo!();
    }
    let msg = if flags.contains(FormatMessageFlags::FROM_SYSTEM) {
        match dwMessageId {
            0x1c => "The printer is out of paper.",
            id => todo!("system message {:x}", id),
        }
    } else {
        todo!();
    };

    let buf: &mut [u16] = unsafe {
        let mem = machine.mem().sub(lpBuffer, nSize).as_mut_slice_todo();
        std::slice::from_raw_parts_mut(mem.as_mut_ptr() as *mut _, mem.len() / 2)
    };
    let msgw = String16::from(msg);

    buf[..msgw.len()].copy_from_slice(msgw.buf());
    buf[msgw.len()] = 0;

    msgw.len() as u32
}

#[win32_derive::dllexport]
pub fn CloseHandle(machine: &mut Machine, hObject: HFILE) -> bool {
    if machine.state.kernel32.files.remove(hObject).is_none() {
        log::warn!("CloseHandle({hObject:?}): unknown handle");
        set_last_error(machine, ERROR_INVALID_HANDLE);
        return false;
    }

    set_last_error(machine, ERROR_SUCCESS);
    true
}

#[win32_derive::dllexport]
pub fn GetSystemDirectoryA(machine: &mut Machine, lpBuffer: u32, uSize: u32) -> u32 {
    let path = "C:\\Windows\\System32";
    let path_bytes = path.as_bytes();
    if uSize < path_bytes.len() as u32 + 1 {
        return path_bytes.len() as u32 + 1;
    }
    set_last_error(machine, ERROR_SUCCESS);
    if lpBuffer != 0 {
        let mem = machine.mem().sub(lpBuffer, uSize).as_mut_slice_todo();
        mem[..path_bytes.len()].copy_from_slice(path_bytes);
        mem[path_bytes.len()] = 0;
    }
    path_bytes.len() as u32
}

#[win32_derive::dllexport]
pub fn GetWindowsDirectoryA(machine: &mut Machine, lpBuffer: u32, uSize: u32) -> u32 {
    let path = "C:\\Windows";
    let path_bytes = path.as_bytes();
    set_last_error(machine, ERROR_SUCCESS);
    if uSize < path_bytes.len() as u32 + 1 {
        return path_bytes.len() as u32 + 1;
    }
    if lpBuffer != 0 {
        let mem = machine.mem().sub(lpBuffer, uSize).as_mut_slice_todo();
        mem[..path_bytes.len()].copy_from_slice(path_bytes);
        mem[path_bytes.len()] = 0;
    }
    path_bytes.len() as u32
}

#[win32_derive::dllexport]
pub fn FormatMessageA(
    machine: &mut Machine,
    dwFlags: u32,
    lpSource: u32,
    dwMessageId: u32,
    dwLanguageId: u32,
    lpBuffer: u32,
    nSize: u32,
    args: u32,
) -> u32 {
    log::warn!("FormatMessageA: stub");
    if lpBuffer != 0 && nSize > 0 {
        let mem = machine.mem().sub(lpBuffer, nSize).as_mut_slice_todo();
        mem[0] = 0;
    }
    0
}

#[win32_derive::dllexport]
pub fn MulDiv(_machine: &mut Machine, nNumber: i32, nNumerator: i32, nDenominator: i32) -> i32 {
    if nDenominator == 0 {
        return -1;
    }

    let mut nMultiplicand = nNumber;
    let mut nDivisor = nDenominator;

    if nDivisor < 0 {
        nMultiplicand = -nMultiplicand;
        nDivisor = -nDivisor;
    }

    let result: i64;
    if (nMultiplicand < 0 && nNumerator < 0) || (nMultiplicand >= 0 && nNumerator >= 0) {
        result = (nMultiplicand as i64).saturating_mul(nNumerator as i64) + (nDivisor / 2) as i64;
    } else {
        result = (nMultiplicand as i64).saturating_mul(nNumerator as i64) - (nDivisor / 2) as i64;
    }

    let result = result / nDivisor as i64;

    if result > i32::MAX as i64 || result < i32::MIN as i64 {
        return -1;
    }

    result as i32
}
