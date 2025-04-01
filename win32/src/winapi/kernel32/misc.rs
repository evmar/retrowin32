//! kernel32 API without a better home.

use super::teb_mut;
use crate::{
    winapi::{
        kernel32::CURRENT_PROCESS_HANDLE,
        string::{BufWrite, BufWriteWide},
        ERROR, *,
    },
    Machine,
};
use ::memory::Pod;
use bitflags::bitflags;
use memory::ExtensionsMut;

pub type SECURITY_ATTRIBUTES = u32; // TODO

pub fn set_last_error(machine: &mut Machine, err: ERROR) {
    teb_mut(machine).LastErrorValue = err.into();
}

#[win32_derive::dllexport]
pub fn SetLastError(machine: &mut Machine, dwErrCode: Result<ERROR, u32>) {
    teb_mut(machine).LastErrorValue = dwErrCode.map_or_else(|err| err, |err| err as u32);
}

#[win32_derive::dllexport]
pub fn GetLastError(machine: &mut Machine) -> Result<ERROR, u32> {
    ERROR::try_from(teb_mut(machine).LastErrorValue)
}

#[win32_derive::dllexport]
pub fn ExitProcess(machine: &mut Machine, uExitCode: u32) {
    machine.exit(uExitCode);
}

#[win32_derive::dllexport]
pub fn TerminateProcess(_machine: &mut Machine, hProcess: u32, uExitCode: u32) -> bool {
    todo!();
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
    match feature.unwrap() {
        ProcessorFeature::FLOATING_POINT_PRECISION_ERRATA => {
            // We don't emulate floating point errors.
            false
        }
        feature => {
            log::warn!("IsProcessorFeaturePresent({feature:?}) unhandled, returning false");
            false
        }
    }
}

#[win32_derive::dllexport]
pub fn IsDebuggerPresent(_machine: &mut Machine) -> bool {
    true // Might cause a binary to log info via the debug API? Not sure.
}

#[win32_derive::dllexport]
pub fn DebugBreak(_machine: &mut Machine) {
    todo!()
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
pub fn SetUnhandledExceptionFilter(_machine: &mut Machine, _lpTopLevelExceptionFilter: u32) -> u32 {
    0 // No current handler.
}

#[win32_derive::dllexport]
pub fn UnhandledExceptionFilter(_machine: &mut Machine, _exceptionInfo: u32) -> u32 {
    // "The process is being debugged, so the exception should be passed (as second chance) to the application's debugger."
    0 // EXCEPTION_CONTINUE_SEARCH
}

#[win32_derive::dllexport]
pub fn RaiseException(
    _machine: &mut Machine,
    dwExceptionCode: u32,
    dwExceptionFlags: u32,
    nNumberOfArguments: u32,
    lpArguments: u32,
) {
    todo!();
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
    #[derive(win32_derive::TryFromBitflags)]
    pub struct FormatMessageFlags: u32 {
        const FROM_STRING    = 0x00000400;
        const IGNORE_INSERTS = 0x00000200;
        const FROM_SYSTEM    = 0x00001000;

        // Low 8 bits can be used for line breaking width (!?).
        // Not sure if this makes bitflags do the right thing...
        const MAX_WIDTH_MASK = 0xFF;
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
        match ERROR::try_from(dwMessageId) {
            Ok(ERROR::FILE_NOT_FOUND) => "The system cannot find the file specified.",
            Ok(ERROR::OUT_OF_PAPER) => "The printer is out of paper.",
            err => todo!("system error {err:x?}"),
        }
    } else {
        todo!();
    };

    let buf = machine.mem().sub32_mut(lpBuffer, nSize);
    let len = BufWriteWide::from_mem(machine.mem(), lpBuffer, nSize)
        .write(msg)
        .unwrap();

    len as u32 - 1
}

#[win32_derive::dllexport]
pub fn CloseHandle(machine: &mut Machine, hObject: HFILE) -> bool {
    if machine.state.kernel32.files.remove(hObject).is_none() {
        log::debug!("CloseHandle({hObject:?}): unknown handle");
        set_last_error(machine, ERROR::INVALID_HANDLE);
        return false;
    }

    set_last_error(machine, ERROR::SUCCESS);
    true
}

#[win32_derive::dllexport]
pub fn GetSystemDirectoryA(machine: &mut Machine, lpBuffer: u32, uSize: u32) -> u32 {
    let path = "C:\\Windows\\System32";
    let path_bytes = path.as_bytes();
    if uSize < path_bytes.len() as u32 + 1 {
        return path_bytes.len() as u32 + 1;
    }
    set_last_error(machine, ERROR::SUCCESS);
    if lpBuffer != 0 {
        let buf = machine.mem().sub32_mut(lpBuffer, uSize);
        buf[..path_bytes.len()].copy_from_slice(path_bytes);
        buf[path_bytes.len()] = 0;
    }
    path_bytes.len() as u32
}

#[win32_derive::dllexport]
pub fn GetWindowsDirectoryA(machine: &mut Machine, lpBuffer: u32, uSize: u32) -> u32 {
    let path = "C:\\Windows";
    let path_bytes = path.as_bytes();
    set_last_error(machine, ERROR::SUCCESS);
    if uSize < path_bytes.len() as u32 + 1 {
        return path_bytes.len() as u32 + 1;
    }
    if lpBuffer != 0 {
        let buf = machine.mem().sub32_mut(lpBuffer, uSize);
        buf[..path_bytes.len()].copy_from_slice(path_bytes);
        buf[path_bytes.len()] = 0;
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
        let buf = machine.mem().sub32_mut(lpBuffer, nSize);
        buf[0] = 0;
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

#[win32_derive::dllexport]
pub fn RtlUnwind(
    _machine: &mut Machine,
    TargetFrame: u32,
    TargetIp: u32,
    ExceptionRecord: u32,
    ReturnValue: u32,
) {
    todo!();
}

#[win32_derive::dllexport]
pub fn CompareStringA(
    _machine: &mut Machine,
    Locale: u32,
    dwCmpFlags: u32,
    lpString1: u32,
    cchCount1: i32,
    lpString2: u32,
    cchCount2: i32,
) -> u32 /* COMPARESTRING_RESULT */ {
    todo!()
}

#[win32_derive::dllexport]
pub fn CompareStringW(
    _machine: &mut Machine,
    Locale: u32,
    dwCmpFlags: u32,
    lpString1: u32,
    cchCount1: i32,
    lpString2: u32,
    cchCount2: i32,
) -> u32 /* COMPARESTRING_RESULT */ {
    todo!()
}

#[win32_derive::dllexport]
pub fn DuplicateHandle(
    _machine: &mut Machine,
    hSourceProcessHandle: HANDLE<()>,
    hSourceHandle: HANDLE<()>,
    hTargetProcessHandle: HANDLE<()>,
    lpTargetHandle: Option<&mut HANDLE<()>>,
    dwDesiredAccess: u32,
    bInheritHandle: bool,
    dwOptions: u32, /* DUPLICATE_HANDLE_OPTIONS */
) -> bool {
    assert_eq!(hSourceProcessHandle, hTargetProcessHandle);
    assert_eq!(hSourceProcessHandle, CURRENT_PROCESS_HANDLE);
    log::warn!("DuplicateHandle: stub");
    *lpTargetHandle.unwrap() = hSourceHandle;
    true
}

type ATOM = u32;

#[win32_derive::dllexport]
pub fn GlobalAddAtomA(_machine: &mut Machine, lpString: Option<&str>) -> ATOM {
    log::warn!("GlobalAddAtomA: stub");
    0
}

#[win32_derive::dllexport]
pub fn Beep(_machine: &mut Machine, dwFreq: u32, dwDuration: u32) -> bool {
    todo!()
}

pub type SEM = u32; // TODO: SEM_*

#[win32_derive::dllexport]
pub fn SetErrorMode(_machine: &mut Machine, uMode: SEM) -> SEM {
    log::warn!("ignoring SetErrorMode({uMode:x?})");
    uMode
}

#[win32_derive::dllexport]
pub fn EncodePointer(_machine: &mut Machine, ptr: u32) -> u32 {
    ptr
}

#[win32_derive::dllexport]
pub fn DecodePointer(_machine: &mut Machine, ptr: u32) -> u32 {
    ptr
}
