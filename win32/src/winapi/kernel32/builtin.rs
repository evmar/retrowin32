#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::{
    shims::{Handler, Shim},
    winapi::builtin::BuiltinDLL,
};
mod wrappers {
    use crate::{
        machine::Machine,
        winapi::{self, stack_args::*, types::*},
    };
    use ::memory::Extensions;
    use winapi::kernel32::*;
    pub unsafe fn AcquireSRWLockExclusive(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/srw_lock") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/srw_lock",
                "AcquireSRWLockExclusive",
                &[("SRWLock", &SRWLock)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::AcquireSRWLockExclusive(machine, SRWLock);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::AcquireSRWLockExclusive_pos.0,
                winapi::kernel32::AcquireSRWLockExclusive_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn AcquireSRWLockShared(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/srw_lock") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/srw_lock",
                "AcquireSRWLockShared",
                &[("SRWLock", &SRWLock)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::AcquireSRWLockShared(machine, SRWLock);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::AcquireSRWLockShared_pos.0,
                winapi::kernel32::AcquireSRWLockShared_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn AddVectoredExceptionHandler(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let first = <u32>::from_stack(mem, stack_args + 0u32);
        let handler = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "AddVectoredExceptionHandler",
                &[("first", &first), ("handler", &handler)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::AddVectoredExceptionHandler(machine, first, handler);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::AddVectoredExceptionHandler_pos.0,
                winapi::kernel32::AddVectoredExceptionHandler_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn CloseHandle(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hObject = <HFILE>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "CloseHandle",
                &[("hObject", &hObject)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::CloseHandle(machine, hObject);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::CloseHandle_pos.0,
                winapi::kernel32::CloseHandle_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn CompareStringA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let Locale = <u32>::from_stack(mem, stack_args + 0u32);
        let dwCmpFlags = <u32>::from_stack(mem, stack_args + 4u32);
        let lpString1 = <u32>::from_stack(mem, stack_args + 8u32);
        let cchCount1 = <i32>::from_stack(mem, stack_args + 12u32);
        let lpString2 = <u32>::from_stack(mem, stack_args + 16u32);
        let cchCount2 = <i32>::from_stack(mem, stack_args + 20u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "CompareStringA",
                &[
                    ("Locale", &Locale),
                    ("dwCmpFlags", &dwCmpFlags),
                    ("lpString1", &lpString1),
                    ("cchCount1", &cchCount1),
                    ("lpString2", &lpString2),
                    ("cchCount2", &cchCount2),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::CompareStringA(
            machine, Locale, dwCmpFlags, lpString1, cchCount1, lpString2, cchCount2,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::CompareStringA_pos.0,
                winapi::kernel32::CompareStringA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn CompareStringW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let Locale = <u32>::from_stack(mem, stack_args + 0u32);
        let dwCmpFlags = <u32>::from_stack(mem, stack_args + 4u32);
        let lpString1 = <u32>::from_stack(mem, stack_args + 8u32);
        let cchCount1 = <i32>::from_stack(mem, stack_args + 12u32);
        let lpString2 = <u32>::from_stack(mem, stack_args + 16u32);
        let cchCount2 = <i32>::from_stack(mem, stack_args + 20u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "CompareStringW",
                &[
                    ("Locale", &Locale),
                    ("dwCmpFlags", &dwCmpFlags),
                    ("lpString1", &lpString1),
                    ("cchCount1", &cchCount1),
                    ("lpString2", &lpString2),
                    ("cchCount2", &cchCount2),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::CompareStringW(
            machine, Locale, dwCmpFlags, lpString1, cchCount1, lpString2, cchCount2,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::CompareStringW_pos.0,
                winapi::kernel32::CompareStringW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn CreateDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let lpSecurityAttributes = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "CreateDirectoryA",
                &[
                    ("lpPathName", &lpPathName),
                    ("lpSecurityAttributes", &lpSecurityAttributes),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::CreateDirectoryA(machine, lpPathName, lpSecurityAttributes);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::CreateDirectoryA_pos.0,
                winapi::kernel32::CreateDirectoryA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn CreateEventA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpEventAttributes = <u32>::from_stack(mem, stack_args + 0u32);
        let bManualReset = <bool>::from_stack(mem, stack_args + 4u32);
        let bInitialState = <bool>::from_stack(mem, stack_args + 8u32);
        let lpName = <Option<&str>>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/event") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/event",
                "CreateEventA",
                &[
                    ("lpEventAttributes", &lpEventAttributes),
                    ("bManualReset", &bManualReset),
                    ("bInitialState", &bInitialState),
                    ("lpName", &lpName),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::CreateEventA(
            machine,
            lpEventAttributes,
            bManualReset,
            bInitialState,
            lpName,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::CreateEventA_pos.0,
                winapi::kernel32::CreateEventA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn CreateFileA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let dwDesiredAccess = <u32>::from_stack(mem, stack_args + 4u32);
        let dwShareMode = <u32>::from_stack(mem, stack_args + 8u32);
        let lpSecurityAttributes = <u32>::from_stack(mem, stack_args + 12u32);
        let dwCreationDisposition =
            <Result<CreationDisposition, u32>>::from_stack(mem, stack_args + 16u32);
        let dwFlagsAndAttributes =
            <Result<FileAttribute, u32>>::from_stack(mem, stack_args + 20u32);
        let hTemplateFile = <HFILE>::from_stack(mem, stack_args + 24u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "CreateFileA",
                &[
                    ("lpFileName", &lpFileName),
                    ("dwDesiredAccess", &dwDesiredAccess),
                    ("dwShareMode", &dwShareMode),
                    ("lpSecurityAttributes", &lpSecurityAttributes),
                    ("dwCreationDisposition", &dwCreationDisposition),
                    ("dwFlagsAndAttributes", &dwFlagsAndAttributes),
                    ("hTemplateFile", &hTemplateFile),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::CreateFileA(
            machine,
            lpFileName,
            dwDesiredAccess,
            dwShareMode,
            lpSecurityAttributes,
            dwCreationDisposition,
            dwFlagsAndAttributes,
            hTemplateFile,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::CreateFileA_pos.0,
                winapi::kernel32::CreateFileA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn CreateFileW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
        let dwDesiredAccess = <u32>::from_stack(mem, stack_args + 4u32);
        let dwShareMode = <u32>::from_stack(mem, stack_args + 8u32);
        let lpSecurityAttributes = <u32>::from_stack(mem, stack_args + 12u32);
        let dwCreationDisposition =
            <Result<CreationDisposition, u32>>::from_stack(mem, stack_args + 16u32);
        let dwFlagsAndAttributes =
            <Result<FileAttribute, u32>>::from_stack(mem, stack_args + 20u32);
        let hTemplateFile = <HFILE>::from_stack(mem, stack_args + 24u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "CreateFileW",
                &[
                    ("lpFileName", &lpFileName),
                    ("dwDesiredAccess", &dwDesiredAccess),
                    ("dwShareMode", &dwShareMode),
                    ("lpSecurityAttributes", &lpSecurityAttributes),
                    ("dwCreationDisposition", &dwCreationDisposition),
                    ("dwFlagsAndAttributes", &dwFlagsAndAttributes),
                    ("hTemplateFile", &hTemplateFile),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::CreateFileW(
            machine,
            lpFileName,
            dwDesiredAccess,
            dwShareMode,
            lpSecurityAttributes,
            dwCreationDisposition,
            dwFlagsAndAttributes,
            hTemplateFile,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::CreateFileW_pos.0,
                winapi::kernel32::CreateFileW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn CreateProcessA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpApplicationName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let lpCommandLine = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let lpProcessAttributes =
            <Option<&mut SECURITY_ATTRIBUTES>>::from_stack(mem, stack_args + 8u32);
        let lpThreadAttributes =
            <Option<&mut SECURITY_ATTRIBUTES>>::from_stack(mem, stack_args + 12u32);
        let bInheritHandles = <bool>::from_stack(mem, stack_args + 16u32);
        let dwCreationFlags = <u32>::from_stack(mem, stack_args + 20u32);
        let lpEnvironment = <Option<&mut u8>>::from_stack(mem, stack_args + 24u32);
        let lpCurrentDirectory = <Option<&str>>::from_stack(mem, stack_args + 28u32);
        let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, stack_args + 32u32);
        let lpProcessInformation =
            <Option<&mut PROCESS_INFORMATION>>::from_stack(mem, stack_args + 36u32);
        let __trace_context = if crate::trace::enabled("kernel32/process") {
            Some(crate::trace::trace_begin(
                "kernel32/process",
                "CreateProcessA",
                &[
                    ("lpApplicationName", &lpApplicationName),
                    ("lpCommandLine", &lpCommandLine),
                    ("lpProcessAttributes", &lpProcessAttributes),
                    ("lpThreadAttributes", &lpThreadAttributes),
                    ("bInheritHandles", &bInheritHandles),
                    ("dwCreationFlags", &dwCreationFlags),
                    ("lpEnvironment", &lpEnvironment),
                    ("lpCurrentDirectory", &lpCurrentDirectory),
                    ("lpStartupInfo", &lpStartupInfo),
                    ("lpProcessInformation", &lpProcessInformation),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::CreateProcessA(
            machine,
            lpApplicationName,
            lpCommandLine,
            lpProcessAttributes,
            lpThreadAttributes,
            bInheritHandles,
            dwCreationFlags,
            lpEnvironment,
            lpCurrentDirectory,
            lpStartupInfo,
            lpProcessInformation,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::CreateProcessA_pos.0,
                winapi::kernel32::CreateProcessA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn CreateThread(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let lpThreadAttributes = <u32>::from_stack(mem, stack_args + 0u32);
        let dwStackSize = <u32>::from_stack(mem, stack_args + 4u32);
        let lpStartAddress = <u32>::from_stack(mem, stack_args + 8u32);
        let lpParameter = <u32>::from_stack(mem, stack_args + 12u32);
        let dwCreationFlags = <u32>::from_stack(mem, stack_args + 16u32);
        let lpThreadId = <u32>::from_stack(mem, stack_args + 20u32);
        let __trace_context = if crate::trace::enabled("kernel32/thread") {
            Some(crate::trace::trace_begin(
                "kernel32/thread",
                "CreateThread",
                &[
                    ("lpThreadAttributes", &lpThreadAttributes),
                    ("dwStackSize", &dwStackSize),
                    ("lpStartAddress", &lpStartAddress),
                    ("lpParameter", &lpParameter),
                    ("dwCreationFlags", &dwCreationFlags),
                    ("lpThreadId", &lpThreadId),
                ],
            ))
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::kernel32::CreateThread(
                machine,
                lpThreadAttributes,
                dwStackSize,
                lpStartAddress,
                lpParameter,
                dwCreationFlags,
                lpThreadId,
            )
            .await;
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::CreateThread_pos.0,
                    winapi::kernel32::CreateThread_pos.1,
                    &result,
                );
            }
            result.to_raw()
        })
    }
    pub unsafe fn DebugBreak(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "DebugBreak",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::DebugBreak(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::DebugBreak_pos.0,
                winapi::kernel32::DebugBreak_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn DeleteCriticalSection(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/critical_section") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/critical_section",
                "DeleteCriticalSection",
                &[("lpCriticalSection", &lpCriticalSection)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::DeleteCriticalSection(machine, lpCriticalSection);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::DeleteCriticalSection_pos.0,
                winapi::kernel32::DeleteCriticalSection_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn DeleteFileA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "DeleteFileA",
                &[("lpFileName", &lpFileName)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::DeleteFileA(machine, lpFileName);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::DeleteFileA_pos.0,
                winapi::kernel32::DeleteFileA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn DisableThreadLibraryCalls(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hLibModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/dll") {
            Some(crate::trace::trace_begin(
                "kernel32/dll",
                "DisableThreadLibraryCalls",
                &[("hLibModule", &hLibModule)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::DisableThreadLibraryCalls(machine, hLibModule);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::DisableThreadLibraryCalls_pos.0,
                winapi::kernel32::DisableThreadLibraryCalls_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn DuplicateHandle(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hSourceProcessHandle = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
        let hSourceHandle = <HANDLE<()>>::from_stack(mem, stack_args + 4u32);
        let hTargetProcessHandle = <HANDLE<()>>::from_stack(mem, stack_args + 8u32);
        let lpTargetHandle = <Option<&mut HANDLE<()>>>::from_stack(mem, stack_args + 12u32);
        let dwDesiredAccess = <u32>::from_stack(mem, stack_args + 16u32);
        let bInheritHandle = <bool>::from_stack(mem, stack_args + 20u32);
        let dwOptions = <u32>::from_stack(mem, stack_args + 24u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "DuplicateHandle",
                &[
                    ("hSourceProcessHandle", &hSourceProcessHandle),
                    ("hSourceHandle", &hSourceHandle),
                    ("hTargetProcessHandle", &hTargetProcessHandle),
                    ("lpTargetHandle", &lpTargetHandle),
                    ("dwDesiredAccess", &dwDesiredAccess),
                    ("bInheritHandle", &bInheritHandle),
                    ("dwOptions", &dwOptions),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::DuplicateHandle(
            machine,
            hSourceProcessHandle,
            hSourceHandle,
            hTargetProcessHandle,
            lpTargetHandle,
            dwDesiredAccess,
            bInheritHandle,
            dwOptions,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::DuplicateHandle_pos.0,
                winapi::kernel32::DuplicateHandle_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn EnterCriticalSection(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/critical_section") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/critical_section",
                "EnterCriticalSection",
                &[("lpCriticalSection", &lpCriticalSection)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::EnterCriticalSection(machine, lpCriticalSection);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::EnterCriticalSection_pos.0,
                winapi::kernel32::EnterCriticalSection_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn ExitProcess(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let uExitCode = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "ExitProcess",
                &[("uExitCode", &uExitCode)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::ExitProcess(machine, uExitCode);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::ExitProcess_pos.0,
                winapi::kernel32::ExitProcess_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn ExitThread(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let dwExitCode = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/thread") {
            Some(crate::trace::trace_begin(
                "kernel32/thread",
                "ExitThread",
                &[("dwExitCode", &dwExitCode)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::ExitThread(machine, dwExitCode);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::ExitThread_pos.0,
                winapi::kernel32::ExitThread_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn FileTimeToLocalFileTime(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpFileTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 0u32);
        let lpLocalFileTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/time") {
            Some(crate::trace::trace_begin(
                "kernel32/time",
                "FileTimeToLocalFileTime",
                &[
                    ("lpFileTime", &lpFileTime),
                    ("lpLocalFileTime", &lpLocalFileTime),
                ],
            ))
        } else {
            None
        };
        let result =
            winapi::kernel32::FileTimeToLocalFileTime(machine, lpFileTime, lpLocalFileTime);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::FileTimeToLocalFileTime_pos.0,
                winapi::kernel32::FileTimeToLocalFileTime_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn FileTimeToSystemTime(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpFileTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 0u32);
        let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/time") {
            Some(crate::trace::trace_begin(
                "kernel32/time",
                "FileTimeToSystemTime",
                &[("lpFileTime", &lpFileTime), ("lpSystemTime", &lpSystemTime)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::FileTimeToSystemTime(machine, lpFileTime, lpSystemTime);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::FileTimeToSystemTime_pos.0,
                winapi::kernel32::FileTimeToSystemTime_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn FindClose(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFindFile = <HFIND>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "FindClose",
                &[("hFindFile", &hFindFile)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::FindClose(machine, hFindFile);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::FindClose_pos.0,
                winapi::kernel32::FindClose_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn FindFirstFileA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let lpFindFileData = <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "FindFirstFileA",
                &[
                    ("lpFileName", &lpFileName),
                    ("lpFindFileData", &lpFindFileData),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::FindFirstFileA(machine, lpFileName, lpFindFileData);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::FindFirstFileA_pos.0,
                winapi::kernel32::FindFirstFileA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn FindNextFileA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFindFile = <HFIND>::from_stack(mem, stack_args + 0u32);
        let lpFindFileData = <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "FindNextFileA",
                &[
                    ("hFindFile", &hFindFile),
                    ("lpFindFileData", &lpFindFileData),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::FindNextFileA(machine, hFindFile, lpFindFileData);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::FindNextFileA_pos.0,
                winapi::kernel32::FindNextFileA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn FindResourceA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
        let lpName = <ResourceKey<&str>>::from_stack(mem, stack_args + 4u32);
        let lpType = <ResourceKey<&str>>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/resource") {
            Some(crate::trace::trace_begin(
                "kernel32/resource",
                "FindResourceA",
                &[
                    ("hModule", &hModule),
                    ("lpName", &lpName),
                    ("lpType", &lpType),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::FindResourceA(machine, hModule, lpName, lpType);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::FindResourceA_pos.0,
                winapi::kernel32::FindResourceA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn FindResourceW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
        let lpName = <ResourceKey<&Str16>>::from_stack(mem, stack_args + 4u32);
        let lpType = <ResourceKey<&Str16>>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/resource") {
            Some(crate::trace::trace_begin(
                "kernel32/resource",
                "FindResourceW",
                &[
                    ("hModule", &hModule),
                    ("lpName", &lpName),
                    ("lpType", &lpType),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::FindResourceW(machine, hModule, lpName, lpType);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::FindResourceW_pos.0,
                winapi::kernel32::FindResourceW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn FlushFileBuffers(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "FlushFileBuffers",
                &[("hFile", &hFile)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::FlushFileBuffers(machine, hFile);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::FlushFileBuffers_pos.0,
                winapi::kernel32::FlushFileBuffers_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn FormatMessageA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let dwFlags = <u32>::from_stack(mem, stack_args + 0u32);
        let lpSource = <u32>::from_stack(mem, stack_args + 4u32);
        let dwMessageId = <u32>::from_stack(mem, stack_args + 8u32);
        let dwLanguageId = <u32>::from_stack(mem, stack_args + 12u32);
        let lpBuffer = <u32>::from_stack(mem, stack_args + 16u32);
        let nSize = <u32>::from_stack(mem, stack_args + 20u32);
        let args = <u32>::from_stack(mem, stack_args + 24u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "FormatMessageA",
                &[
                    ("dwFlags", &dwFlags),
                    ("lpSource", &lpSource),
                    ("dwMessageId", &dwMessageId),
                    ("dwLanguageId", &dwLanguageId),
                    ("lpBuffer", &lpBuffer),
                    ("nSize", &nSize),
                    ("args", &args),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::FormatMessageA(
            machine,
            dwFlags,
            lpSource,
            dwMessageId,
            dwLanguageId,
            lpBuffer,
            nSize,
            args,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::FormatMessageA_pos.0,
                winapi::kernel32::FormatMessageA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn FormatMessageW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let dwFlags = <Result<FormatMessageFlags, u32>>::from_stack(mem, stack_args + 0u32);
        let lpSource = <u32>::from_stack(mem, stack_args + 4u32);
        let dwMessageId = <u32>::from_stack(mem, stack_args + 8u32);
        let dwLanguageId = <u32>::from_stack(mem, stack_args + 12u32);
        let lpBuffer = <u32>::from_stack(mem, stack_args + 16u32);
        let nSize = <u32>::from_stack(mem, stack_args + 20u32);
        let args = <u32>::from_stack(mem, stack_args + 24u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "FormatMessageW",
                &[
                    ("dwFlags", &dwFlags),
                    ("lpSource", &lpSource),
                    ("dwMessageId", &dwMessageId),
                    ("dwLanguageId", &dwLanguageId),
                    ("lpBuffer", &lpBuffer),
                    ("nSize", &nSize),
                    ("args", &args),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::FormatMessageW(
            machine,
            dwFlags,
            lpSource,
            dwMessageId,
            dwLanguageId,
            lpBuffer,
            nSize,
            args,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::FormatMessageW_pos.0,
                winapi::kernel32::FormatMessageW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn FreeEnvironmentStringsA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _penv = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/env") {
            Some(crate::trace::trace_begin(
                "kernel32/env",
                "FreeEnvironmentStringsA",
                &[("penv", &_penv)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::FreeEnvironmentStringsA(machine, _penv);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::FreeEnvironmentStringsA_pos.0,
                winapi::kernel32::FreeEnvironmentStringsA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn FreeEnvironmentStringsW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/env") {
            Some(crate::trace::trace_begin(
                "kernel32/env",
                "FreeEnvironmentStringsW",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::FreeEnvironmentStringsW(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::FreeEnvironmentStringsW_pos.0,
                winapi::kernel32::FreeEnvironmentStringsW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn FreeLibrary(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hLibModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/dll") {
            Some(crate::trace::trace_begin(
                "kernel32/dll",
                "FreeLibrary",
                &[("hLibModule", &hLibModule)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::FreeLibrary(machine, hLibModule);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::FreeLibrary_pos.0,
                winapi::kernel32::FreeLibrary_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetACP(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/nls") {
            Some(crate::trace::trace_begin("kernel32/nls", "GetACP", &[]))
        } else {
            None
        };
        let result = winapi::kernel32::GetACP(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetACP_pos.0,
                winapi::kernel32::GetACP_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetCPInfo(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _CodePage = <u32>::from_stack(mem, stack_args + 0u32);
        let _lpCPInfo = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/nls") {
            Some(crate::trace::trace_begin(
                "kernel32/nls",
                "GetCPInfo",
                &[("CodePage", &_CodePage), ("lpCPInfo", &_lpCPInfo)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetCPInfo_pos.0,
                winapi::kernel32::GetCPInfo_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetCommandLineA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/command_line") {
            Some(crate::trace::trace_begin(
                "kernel32/command_line",
                "GetCommandLineA",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetCommandLineA(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetCommandLineA_pos.0,
                winapi::kernel32::GetCommandLineA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetCommandLineW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/command_line") {
            Some(crate::trace::trace_begin(
                "kernel32/command_line",
                "GetCommandLineW",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetCommandLineW(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetCommandLineW_pos.0,
                winapi::kernel32::GetCommandLineW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetConsoleMode(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hConsoleHandle = <HFILE>::from_stack(mem, stack_args + 0u32);
        let lpMode = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "GetConsoleMode",
                &[("hConsoleHandle", &hConsoleHandle), ("lpMode", &lpMode)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetConsoleMode(machine, hConsoleHandle, lpMode);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetConsoleMode_pos.0,
                winapi::kernel32::GetConsoleMode_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetConsoleScreenBufferInfo(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _hConsoleOutput = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
        let lpConsoleScreenBufferInfo =
            <Option<&mut CONSOLE_SCREEN_BUFFER_INFO>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/console") {
            Some(crate::trace::trace_begin(
                "kernel32/console",
                "GetConsoleScreenBufferInfo",
                &[
                    ("hConsoleOutput", &_hConsoleOutput),
                    ("lpConsoleScreenBufferInfo", &lpConsoleScreenBufferInfo),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetConsoleScreenBufferInfo(
            machine,
            _hConsoleOutput,
            lpConsoleScreenBufferInfo,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetConsoleScreenBufferInfo_pos.0,
                winapi::kernel32::GetConsoleScreenBufferInfo_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetCurrentDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let nBufferLength = <u32>::from_stack(mem, stack_args + 0u32);
        let lpBuffer = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "GetCurrentDirectoryA",
                &[("nBufferLength", &nBufferLength), ("lpBuffer", &lpBuffer)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetCurrentDirectoryA(machine, nBufferLength, lpBuffer);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetCurrentDirectoryA_pos.0,
                winapi::kernel32::GetCurrentDirectoryA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetCurrentProcess(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/process") {
            Some(crate::trace::trace_begin(
                "kernel32/process",
                "GetCurrentProcess",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetCurrentProcess(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetCurrentProcess_pos.0,
                winapi::kernel32::GetCurrentProcess_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetCurrentProcessId(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "GetCurrentProcessId",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetCurrentProcessId(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetCurrentProcessId_pos.0,
                winapi::kernel32::GetCurrentProcessId_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetCurrentThread(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/thread") {
            Some(crate::trace::trace_begin(
                "kernel32/thread",
                "GetCurrentThread",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetCurrentThread(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetCurrentThread_pos.0,
                winapi::kernel32::GetCurrentThread_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetCurrentThreadId(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/thread") {
            Some(crate::trace::trace_begin(
                "kernel32/thread",
                "GetCurrentThreadId",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetCurrentThreadId(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetCurrentThreadId_pos.0,
                winapi::kernel32::GetCurrentThreadId_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetDiskFreeSpaceA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpRootPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let lpSectorsPerCluster = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let lpBytesPerSector = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
        let lpNumberOfFreeClusters = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
        let lpTotalNumberOfClusters = <Option<&mut u32>>::from_stack(mem, stack_args + 16u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "GetDiskFreeSpaceA",
                &[
                    ("lpRootPathName", &lpRootPathName),
                    ("lpSectorsPerCluster", &lpSectorsPerCluster),
                    ("lpBytesPerSector", &lpBytesPerSector),
                    ("lpNumberOfFreeClusters", &lpNumberOfFreeClusters),
                    ("lpTotalNumberOfClusters", &lpTotalNumberOfClusters),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetDiskFreeSpaceA(
            machine,
            lpRootPathName,
            lpSectorsPerCluster,
            lpBytesPerSector,
            lpNumberOfFreeClusters,
            lpTotalNumberOfClusters,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetDiskFreeSpaceA_pos.0,
                winapi::kernel32::GetDiskFreeSpaceA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetDriveTypeA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpRootPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "GetDriveTypeA",
                &[("lpRootPathName", &lpRootPathName)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetDriveTypeA(machine, lpRootPathName);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetDriveTypeA_pos.0,
                winapi::kernel32::GetDriveTypeA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetEnvironmentStrings(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/env") {
            Some(crate::trace::trace_begin(
                "kernel32/env",
                "GetEnvironmentStrings",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetEnvironmentStrings(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetEnvironmentStrings_pos.0,
                winapi::kernel32::GetEnvironmentStrings_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetEnvironmentStringsW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/env") {
            Some(crate::trace::trace_begin(
                "kernel32/env",
                "GetEnvironmentStringsW",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetEnvironmentStringsW(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetEnvironmentStringsW_pos.0,
                winapi::kernel32::GetEnvironmentStringsW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetEnvironmentVariableA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let name = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let buf = <ArrayWithSize<u8>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/env") {
            Some(crate::trace::trace_begin(
                "kernel32/env",
                "GetEnvironmentVariableA",
                &[("name", &name), ("buf", &buf)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetEnvironmentVariableA(machine, name, buf);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetEnvironmentVariableA_pos.0,
                winapi::kernel32::GetEnvironmentVariableA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetEnvironmentVariableW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let name = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
        let buf = <ArrayWithSize<u16>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/env") {
            Some(crate::trace::trace_begin(
                "kernel32/env",
                "GetEnvironmentVariableW",
                &[("name", &name), ("buf", &buf)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetEnvironmentVariableW(machine, name, buf);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetEnvironmentVariableW_pos.0,
                winapi::kernel32::GetEnvironmentVariableW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetExitCodeProcess(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hProcess = <HPROCESS>::from_stack(mem, stack_args + 0u32);
        let lpExitCode = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/process") {
            Some(crate::trace::trace_begin(
                "kernel32/process",
                "GetExitCodeProcess",
                &[("hProcess", &hProcess), ("lpExitCode", &lpExitCode)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetExitCodeProcess(machine, hProcess, lpExitCode);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetExitCodeProcess_pos.0,
                winapi::kernel32::GetExitCodeProcess_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetFileAttributesA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "GetFileAttributesA",
                &[("lpFileName", &lpFileName)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetFileAttributesA(machine, lpFileName);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetFileAttributesA_pos.0,
                winapi::kernel32::GetFileAttributesA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetFileInformationByHandle(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
        let lpFileInformation =
            <Option<&mut BY_HANDLE_FILE_INFORMATION>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "GetFileInformationByHandle",
                &[("hFile", &hFile), ("lpFileInformation", &lpFileInformation)],
            ))
        } else {
            None
        };
        let result =
            winapi::kernel32::GetFileInformationByHandle(machine, hFile, lpFileInformation);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetFileInformationByHandle_pos.0,
                winapi::kernel32::GetFileInformationByHandle_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetFileSize(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
        let lpFileSizeHigh = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "GetFileSize",
                &[("hFile", &hFile), ("lpFileSizeHigh", &lpFileSizeHigh)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetFileSize(machine, hFile, lpFileSizeHigh);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetFileSize_pos.0,
                winapi::kernel32::GetFileSize_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetFileTime(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
        let lpCreationTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 4u32);
        let lpLastAccessTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 8u32);
        let lpLastWriteTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "GetFileTime",
                &[
                    ("hFile", &hFile),
                    ("lpCreationTime", &lpCreationTime),
                    ("lpLastAccessTime", &lpLastAccessTime),
                    ("lpLastWriteTime", &lpLastWriteTime),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetFileTime(
            machine,
            hFile,
            lpCreationTime,
            lpLastAccessTime,
            lpLastWriteTime,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetFileTime_pos.0,
                winapi::kernel32::GetFileTime_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetFileType(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "GetFileType",
                &[("hFile", &hFile)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetFileType(machine, hFile);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetFileType_pos.0,
                winapi::kernel32::GetFileType_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetFullPathNameA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let nBufferLength = <u32>::from_stack(mem, stack_args + 4u32);
        let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
        let lpFilePart = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "GetFullPathNameA",
                &[
                    ("lpFileName", &lpFileName),
                    ("nBufferLength", &nBufferLength),
                    ("lpBuffer", &lpBuffer),
                    ("lpFilePart", &lpFilePart),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetFullPathNameA(
            machine,
            lpFileName,
            nBufferLength,
            lpBuffer,
            lpFilePart,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetFullPathNameA_pos.0,
                winapi::kernel32::GetFullPathNameA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetFullPathNameW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
        let nBufferLength = <u32>::from_stack(mem, stack_args + 4u32);
        let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
        let lpFilePart = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "GetFullPathNameW",
                &[
                    ("lpFileName", &lpFileName),
                    ("nBufferLength", &nBufferLength),
                    ("lpBuffer", &lpBuffer),
                    ("lpFilePart", &lpFilePart),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetFullPathNameW(
            machine,
            lpFileName,
            nBufferLength,
            lpBuffer,
            lpFilePart,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetFullPathNameW_pos.0,
                winapi::kernel32::GetFullPathNameW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetLastError(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "GetLastError",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetLastError(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetLastError_pos.0,
                winapi::kernel32::GetLastError_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetLocalTime(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/time") {
            Some(crate::trace::trace_begin(
                "kernel32/time",
                "GetLocalTime",
                &[("lpSystemTime", &lpSystemTime)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetLocalTime(machine, lpSystemTime);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetLocalTime_pos.0,
                winapi::kernel32::GetLocalTime_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetLogicalDrives(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "GetLogicalDrives",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetLogicalDrives(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetLogicalDrives_pos.0,
                winapi::kernel32::GetLogicalDrives_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetModuleFileNameA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
        let filename = <ArrayWithSizeMut<u8>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/dll") {
            Some(crate::trace::trace_begin(
                "kernel32/dll",
                "GetModuleFileNameA",
                &[("hModule", &hModule), ("filename", &filename)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetModuleFileNameA(machine, hModule, filename);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetModuleFileNameA_pos.0,
                winapi::kernel32::GetModuleFileNameA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetModuleFileNameW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
        let _lpFilename = <u32>::from_stack(mem, stack_args + 4u32);
        let _nSize = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/dll") {
            Some(crate::trace::trace_begin(
                "kernel32/dll",
                "GetModuleFileNameW",
                &[
                    ("hModule", &hModule),
                    ("lpFilename", &_lpFilename),
                    ("nSize", &_nSize),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetModuleFileNameW_pos.0,
                winapi::kernel32::GetModuleFileNameW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetModuleHandleA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpModuleName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/dll") {
            Some(crate::trace::trace_begin(
                "kernel32/dll",
                "GetModuleHandleA",
                &[("lpModuleName", &lpModuleName)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetModuleHandleA(machine, lpModuleName);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetModuleHandleA_pos.0,
                winapi::kernel32::GetModuleHandleA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetModuleHandleExW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let dwFlags = <u32>::from_stack(mem, stack_args + 0u32);
        let lpModuleName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
        let hModule = <Option<&mut HMODULE>>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/dll") {
            Some(crate::trace::trace_begin(
                "kernel32/dll",
                "GetModuleHandleExW",
                &[
                    ("dwFlags", &dwFlags),
                    ("lpModuleName", &lpModuleName),
                    ("hModule", &hModule),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetModuleHandleExW_pos.0,
                winapi::kernel32::GetModuleHandleExW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetModuleHandleW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpModuleName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/dll") {
            Some(crate::trace::trace_begin(
                "kernel32/dll",
                "GetModuleHandleW",
                &[("lpModuleName", &lpModuleName)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetModuleHandleW(machine, lpModuleName);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetModuleHandleW_pos.0,
                winapi::kernel32::GetModuleHandleW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetOEMCP(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/nls") {
            Some(crate::trace::trace_begin("kernel32/nls", "GetOEMCP", &[]))
        } else {
            None
        };
        let result = winapi::kernel32::GetOEMCP(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetOEMCP_pos.0,
                winapi::kernel32::GetOEMCP_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetPrivateProfileIntW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
        let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
        let nDefault = <u32>::from_stack(mem, stack_args + 8u32);
        let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/ini") {
            Some(crate::trace::trace_begin(
                "kernel32/ini",
                "GetPrivateProfileIntW",
                &[
                    ("lpAppName", &lpAppName),
                    ("lpKeyName", &lpKeyName),
                    ("nDefault", &nDefault),
                    ("lpFileName", &lpFileName),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetPrivateProfileIntW(
            machine, lpAppName, lpKeyName, nDefault, lpFileName,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetPrivateProfileIntW_pos.0,
                winapi::kernel32::GetPrivateProfileIntW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetPrivateProfileStringA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpAppName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let lpKeyName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let lpDefault = <Option<&str>>::from_stack(mem, stack_args + 8u32);
        let lpReturnedString = <Option<&str>>::from_stack(mem, stack_args + 12u32);
        let nSize = <u32>::from_stack(mem, stack_args + 16u32);
        let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 20u32);
        let __trace_context = if crate::trace::enabled("kernel32/ini") {
            Some(crate::trace::trace_begin(
                "kernel32/ini",
                "GetPrivateProfileStringA",
                &[
                    ("lpAppName", &lpAppName),
                    ("lpKeyName", &lpKeyName),
                    ("lpDefault", &lpDefault),
                    ("lpReturnedString", &lpReturnedString),
                    ("nSize", &nSize),
                    ("lpFileName", &lpFileName),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetPrivateProfileStringA(
            machine,
            lpAppName,
            lpKeyName,
            lpDefault,
            lpReturnedString,
            nSize,
            lpFileName,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetPrivateProfileStringA_pos.0,
                winapi::kernel32::GetPrivateProfileStringA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetPrivateProfileStringW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
        let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
        let lpDefault = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
        let lpReturnedString = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 12u32);
        let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 20u32);
        let __trace_context = if crate::trace::enabled("kernel32/ini") {
            Some(crate::trace::trace_begin(
                "kernel32/ini",
                "GetPrivateProfileStringW",
                &[
                    ("lpAppName", &lpAppName),
                    ("lpKeyName", &lpKeyName),
                    ("lpDefault", &lpDefault),
                    ("lpReturnedString", &lpReturnedString),
                    ("lpFileName", &lpFileName),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetPrivateProfileStringW(
            machine,
            lpAppName,
            lpKeyName,
            lpDefault,
            lpReturnedString,
            lpFileName,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetPrivateProfileStringW_pos.0,
                winapi::kernel32::GetPrivateProfileStringW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetProcAddress(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
        let lpProcName = <GetProcAddressArg>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/dll") {
            Some(crate::trace::trace_begin(
                "kernel32/dll",
                "GetProcAddress",
                &[("hModule", &hModule), ("lpProcName", &lpProcName)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetProcAddress(machine, hModule, lpProcName);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetProcAddress_pos.0,
                winapi::kernel32::GetProcAddress_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetProcessHeap(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "GetProcessHeap",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetProcessHeap(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetProcessHeap_pos.0,
                winapi::kernel32::GetProcessHeap_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetProfileIntW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
        let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
        let nDefault = <i32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/ini") {
            Some(crate::trace::trace_begin(
                "kernel32/ini",
                "GetProfileIntW",
                &[
                    ("lpAppName", &lpAppName),
                    ("lpKeyName", &lpKeyName),
                    ("nDefault", &nDefault),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetProfileIntW(machine, lpAppName, lpKeyName, nDefault);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetProfileIntW_pos.0,
                winapi::kernel32::GetProfileIntW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetProfileStringW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
        let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
        let lpDefault = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
        let lpReturnedString = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/ini") {
            Some(crate::trace::trace_begin(
                "kernel32/ini",
                "GetProfileStringW",
                &[
                    ("lpAppName", &lpAppName),
                    ("lpKeyName", &lpKeyName),
                    ("lpDefault", &lpDefault),
                    ("lpReturnedString", &lpReturnedString),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetProfileStringW(
            machine,
            lpAppName,
            lpKeyName,
            lpDefault,
            lpReturnedString,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetProfileStringW_pos.0,
                winapi::kernel32::GetProfileStringW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetStartupInfoA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/dll") {
            Some(crate::trace::trace_begin(
                "kernel32/dll",
                "GetStartupInfoA",
                &[("lpStartupInfo", &lpStartupInfo)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetStartupInfoA_pos.0,
                winapi::kernel32::GetStartupInfoA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetStartupInfoW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/dll") {
            Some(crate::trace::trace_begin(
                "kernel32/dll",
                "GetStartupInfoW",
                &[("lpStartupInfo", &lpStartupInfo)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetStartupInfoW_pos.0,
                winapi::kernel32::GetStartupInfoW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetStdHandle(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let nStdHandle = <Result<STD, u32>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "GetStdHandle",
                &[("nStdHandle", &nStdHandle)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetStdHandle(machine, nStdHandle);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetStdHandle_pos.0,
                winapi::kernel32::GetStdHandle_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetStringTypeA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let Locale = <LCID>::from_stack(mem, stack_args + 0u32);
        let dwInfoType = <u32>::from_stack(mem, stack_args + 4u32);
        let lpSrcStr = <u32>::from_stack(mem, stack_args + 8u32);
        let cchSrc = <i32>::from_stack(mem, stack_args + 12u32);
        let lpCharType = <Option<&mut u32>>::from_stack(mem, stack_args + 16u32);
        let __trace_context = if crate::trace::enabled("kernel32/nls") {
            Some(crate::trace::trace_begin(
                "kernel32/nls",
                "GetStringTypeA",
                &[
                    ("Locale", &Locale),
                    ("dwInfoType", &dwInfoType),
                    ("lpSrcStr", &lpSrcStr),
                    ("cchSrc", &cchSrc),
                    ("lpCharType", &lpCharType),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetStringTypeA(
            machine, Locale, dwInfoType, lpSrcStr, cchSrc, lpCharType,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetStringTypeA_pos.0,
                winapi::kernel32::GetStringTypeA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetStringTypeW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let dwInfoType = <u32>::from_stack(mem, stack_args + 0u32);
        let lpSrcStr = <u32>::from_stack(mem, stack_args + 4u32);
        let cchSrc = <i32>::from_stack(mem, stack_args + 8u32);
        let lpCharType = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/nls") {
            Some(crate::trace::trace_begin(
                "kernel32/nls",
                "GetStringTypeW",
                &[
                    ("dwInfoType", &dwInfoType),
                    ("lpSrcStr", &lpSrcStr),
                    ("cchSrc", &cchSrc),
                    ("lpCharType", &lpCharType),
                ],
            ))
        } else {
            None
        };
        let result =
            winapi::kernel32::GetStringTypeW(machine, dwInfoType, lpSrcStr, cchSrc, lpCharType);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetStringTypeW_pos.0,
                winapi::kernel32::GetStringTypeW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetSystemDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpBuffer = <u32>::from_stack(mem, stack_args + 0u32);
        let uSize = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "GetSystemDirectoryA",
                &[("lpBuffer", &lpBuffer), ("uSize", &uSize)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetSystemDirectoryA(machine, lpBuffer, uSize);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetSystemDirectoryA_pos.0,
                winapi::kernel32::GetSystemDirectoryA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetSystemTime(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/time") {
            Some(crate::trace::trace_begin(
                "kernel32/time",
                "GetSystemTime",
                &[("lpSystemTime", &lpSystemTime)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetSystemTime(machine, lpSystemTime);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetSystemTime_pos.0,
                winapi::kernel32::GetSystemTime_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetSystemTimeAsFileTime(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpSystemTimeAsFileTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/time") {
            Some(crate::trace::trace_begin(
                "kernel32/time",
                "GetSystemTimeAsFileTime",
                &[("lpSystemTimeAsFileTime", &lpSystemTimeAsFileTime)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetSystemTimeAsFileTime(machine, lpSystemTimeAsFileTime);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetSystemTimeAsFileTime_pos.0,
                winapi::kernel32::GetSystemTimeAsFileTime_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetThreadPriority(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/thread") {
            Some(crate::trace::trace_begin(
                "kernel32/thread",
                "GetThreadPriority",
                &[("hThread", &hThread)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetThreadPriority(machine, hThread);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetThreadPriority_pos.0,
                winapi::kernel32::GetThreadPriority_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetTickCount(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/time") {
            Some(crate::trace::trace_begin(
                "kernel32/time",
                "GetTickCount",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetTickCount(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetTickCount_pos.0,
                winapi::kernel32::GetTickCount_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetTimeZoneInformation(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpTimeZoneInformation =
            <Option<&mut TIME_ZONE_INFORMATION>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/time") {
            Some(crate::trace::trace_begin(
                "kernel32/time",
                "GetTimeZoneInformation",
                &[("lpTimeZoneInformation", &lpTimeZoneInformation)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetTimeZoneInformation(machine, lpTimeZoneInformation);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetTimeZoneInformation_pos.0,
                winapi::kernel32::GetTimeZoneInformation_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetVersion(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "GetVersion",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetVersion(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetVersion_pos.0,
                winapi::kernel32::GetVersion_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetVersionExA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpVersionInformation = <Option<&mut OSVERSIONINFO>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "GetVersionExA",
                &[("lpVersionInformation", &lpVersionInformation)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetVersionExA(machine, lpVersionInformation);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetVersionExA_pos.0,
                winapi::kernel32::GetVersionExA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GetWindowsDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpBuffer = <u32>::from_stack(mem, stack_args + 0u32);
        let uSize = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "GetWindowsDirectoryA",
                &[("lpBuffer", &lpBuffer), ("uSize", &uSize)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GetWindowsDirectoryA(machine, lpBuffer, uSize);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GetWindowsDirectoryA_pos.0,
                winapi::kernel32::GetWindowsDirectoryA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GlobalAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let uFlags = <GMEM>::from_stack(mem, stack_args + 0u32);
        let dwBytes = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "GlobalAlloc",
                &[("uFlags", &uFlags), ("dwBytes", &dwBytes)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GlobalAlloc(machine, uFlags, dwBytes);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GlobalAlloc_pos.0,
                winapi::kernel32::GlobalAlloc_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GlobalFlags(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hMem = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "GlobalFlags",
                &[("hMem", &hMem)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GlobalFlags(machine, hMem);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GlobalFlags_pos.0,
                winapi::kernel32::GlobalFlags_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GlobalFree(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hMem = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "GlobalFree",
                &[("hMem", &hMem)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GlobalFree(machine, hMem);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GlobalFree_pos.0,
                winapi::kernel32::GlobalFree_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn GlobalReAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hMem = <u32>::from_stack(mem, stack_args + 0u32);
        let dwBytes = <u32>::from_stack(mem, stack_args + 4u32);
        let uFlags = <GMEM>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "GlobalReAlloc",
                &[("hMem", &hMem), ("dwBytes", &dwBytes), ("uFlags", &uFlags)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::GlobalReAlloc(machine, hMem, dwBytes, uFlags);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::GlobalReAlloc_pos.0,
                winapi::kernel32::GlobalReAlloc_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn HeapAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
        let dwFlags = <Result<HeapAllocFlags, u32>>::from_stack(mem, stack_args + 4u32);
        let dwBytes = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "HeapAlloc",
                &[
                    ("hHeap", &hHeap),
                    ("dwFlags", &dwFlags),
                    ("dwBytes", &dwBytes),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::HeapAlloc_pos.0,
                winapi::kernel32::HeapAlloc_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn HeapCreate(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let flOptions = <Result<HeapCreateFlags, u32>>::from_stack(mem, stack_args + 0u32);
        let dwInitialSize = <u32>::from_stack(mem, stack_args + 4u32);
        let dwMaximumSize = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "HeapCreate",
                &[
                    ("flOptions", &flOptions),
                    ("dwInitialSize", &dwInitialSize),
                    ("dwMaximumSize", &dwMaximumSize),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, dwMaximumSize);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::HeapCreate_pos.0,
                winapi::kernel32::HeapCreate_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn HeapDestroy(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "HeapDestroy",
                &[("hHeap", &hHeap)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::HeapDestroy(machine, hHeap);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::HeapDestroy_pos.0,
                winapi::kernel32::HeapDestroy_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn HeapFree(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
        let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
        let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "HeapFree",
                &[("hHeap", &hHeap), ("dwFlags", &dwFlags), ("lpMem", &lpMem)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::HeapFree_pos.0,
                winapi::kernel32::HeapFree_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn HeapReAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
        let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
        let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
        let dwBytes = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "HeapReAlloc",
                &[
                    ("hHeap", &hHeap),
                    ("dwFlags", &dwFlags),
                    ("lpMem", &lpMem),
                    ("dwBytes", &dwBytes),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::HeapReAlloc_pos.0,
                winapi::kernel32::HeapReAlloc_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn HeapSetInformation(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let HeapHandle = <u32>::from_stack(mem, stack_args + 0u32);
        let HeapInformationClass = <u32>::from_stack(mem, stack_args + 4u32);
        let HeapInformation = <u32>::from_stack(mem, stack_args + 8u32);
        let HeapInformationLength = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "HeapSetInformation",
                &[
                    ("HeapHandle", &HeapHandle),
                    ("HeapInformationClass", &HeapInformationClass),
                    ("HeapInformation", &HeapInformation),
                    ("HeapInformationLength", &HeapInformationLength),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::HeapSetInformation(
            machine,
            HeapHandle,
            HeapInformationClass,
            HeapInformation,
            HeapInformationLength,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::HeapSetInformation_pos.0,
                winapi::kernel32::HeapSetInformation_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn HeapSize(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
        let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
        let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "HeapSize",
                &[("hHeap", &hHeap), ("dwFlags", &dwFlags), ("lpMem", &lpMem)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::HeapSize_pos.0,
                winapi::kernel32::HeapSize_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn HeapValidate(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
        let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
        let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "HeapValidate",
                &[("hHeap", &hHeap), ("dwFlags", &dwFlags), ("lpMem", &lpMem)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::HeapValidate(machine, hHeap, dwFlags, lpMem);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::HeapValidate_pos.0,
                winapi::kernel32::HeapValidate_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn InitOnceBeginInitialize(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, stack_args + 0u32);
        let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
        let fPending = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
        let lpContext = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/once") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/once",
                "InitOnceBeginInitialize",
                &[
                    ("lpInitOnce", &lpInitOnce),
                    ("dwFlags", &dwFlags),
                    ("fPending", &fPending),
                    ("lpContext", &lpContext),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::InitOnceBeginInitialize(
            machine, lpInitOnce, dwFlags, fPending, lpContext,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::InitOnceBeginInitialize_pos.0,
                winapi::kernel32::InitOnceBeginInitialize_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn InitOnceComplete(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, stack_args + 0u32);
        let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
        let lpContext = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/once") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/once",
                "InitOnceComplete",
                &[
                    ("lpInitOnce", &lpInitOnce),
                    ("dwFlags", &dwFlags),
                    ("lpContext", &lpContext),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::InitOnceComplete(machine, lpInitOnce, dwFlags, lpContext);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::InitOnceComplete_pos.0,
                winapi::kernel32::InitOnceComplete_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn InitializeCriticalSection(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/critical_section") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/critical_section",
                "InitializeCriticalSection",
                &[("lpCriticalSection", &lpCriticalSection)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::InitializeCriticalSection(machine, lpCriticalSection);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::InitializeCriticalSection_pos.0,
                winapi::kernel32::InitializeCriticalSection_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn InitializeCriticalSectionAndSpinCount(
        machine: &mut Machine,
        stack_args: u32,
    ) -> u32 {
        let mem = machine.mem().detach();
        let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
        let dwSpinCount = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/critical_section") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/critical_section",
                "InitializeCriticalSectionAndSpinCount",
                &[
                    ("lpCriticalSection", &lpCriticalSection),
                    ("dwSpinCount", &dwSpinCount),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::InitializeCriticalSectionAndSpinCount(
            machine,
            lpCriticalSection,
            dwSpinCount,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::InitializeCriticalSectionAndSpinCount_pos.0,
                winapi::kernel32::InitializeCriticalSectionAndSpinCount_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn InitializeCriticalSectionEx(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
        let dwSpinCount = <u32>::from_stack(mem, stack_args + 4u32);
        let flags = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/critical_section") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/critical_section",
                "InitializeCriticalSectionEx",
                &[
                    ("lpCriticalSection", &lpCriticalSection),
                    ("dwSpinCount", &dwSpinCount),
                    ("flags", &flags),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::InitializeCriticalSectionEx(
            machine,
            lpCriticalSection,
            dwSpinCount,
            flags,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::InitializeCriticalSectionEx_pos.0,
                winapi::kernel32::InitializeCriticalSectionEx_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn InitializeSListHead(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let ListHead = <Option<&mut SLIST_HEADER>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "InitializeSListHead",
                &[("ListHead", &ListHead)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::InitializeSListHead(machine, ListHead);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::InitializeSListHead_pos.0,
                winapi::kernel32::InitializeSListHead_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn InterlockedDecrement(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let addend = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/interlocked") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/interlocked",
                "InterlockedDecrement",
                &[("addend", &addend)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::InterlockedDecrement(machine, addend);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::InterlockedDecrement_pos.0,
                winapi::kernel32::InterlockedDecrement_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn InterlockedIncrement(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let addend = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/interlocked") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/interlocked",
                "InterlockedIncrement",
                &[("addend", &addend)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::InterlockedIncrement(machine, addend);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::InterlockedIncrement_pos.0,
                winapi::kernel32::InterlockedIncrement_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IsBadCodePtr(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpfn = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "IsBadCodePtr",
                &[("lpfn", &lpfn)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::IsBadCodePtr(machine, lpfn);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::IsBadCodePtr_pos.0,
                winapi::kernel32::IsBadCodePtr_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IsBadReadPtr(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lp = <u32>::from_stack(mem, stack_args + 0u32);
        let ucb = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "IsBadReadPtr",
                &[("lp", &lp), ("ucb", &ucb)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::IsBadReadPtr(machine, lp, ucb);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::IsBadReadPtr_pos.0,
                winapi::kernel32::IsBadReadPtr_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IsBadWritePtr(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lp = <u32>::from_stack(mem, stack_args + 0u32);
        let ucb = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "IsBadWritePtr",
                &[("lp", &lp), ("ucb", &ucb)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::IsBadWritePtr(machine, lp, ucb);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::IsBadWritePtr_pos.0,
                winapi::kernel32::IsBadWritePtr_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IsDBCSLeadByte(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _TestChar = <u8>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/nls") {
            Some(crate::trace::trace_begin(
                "kernel32/nls",
                "IsDBCSLeadByte",
                &[("TestChar", &_TestChar)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::IsDBCSLeadByte(machine, _TestChar);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::IsDBCSLeadByte_pos.0,
                winapi::kernel32::IsDBCSLeadByte_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IsDBCSLeadByteEx(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _TestChar = <u8>::from_stack(mem, stack_args + 0u32);
        let _CodePage = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/nls") {
            Some(crate::trace::trace_begin(
                "kernel32/nls",
                "IsDBCSLeadByteEx",
                &[("TestChar", &_TestChar), ("CodePage", &_CodePage)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::IsDBCSLeadByteEx(machine, _TestChar, _CodePage);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::IsDBCSLeadByteEx_pos.0,
                winapi::kernel32::IsDBCSLeadByteEx_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IsDebuggerPresent(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "IsDebuggerPresent",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::IsDebuggerPresent(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::IsDebuggerPresent_pos.0,
                winapi::kernel32::IsDebuggerPresent_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IsProcessorFeaturePresent(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let feature = <Result<ProcessorFeature, u32>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "IsProcessorFeaturePresent",
                &[("feature", &feature)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::IsProcessorFeaturePresent(machine, feature);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::IsProcessorFeaturePresent_pos.0,
                winapi::kernel32::IsProcessorFeaturePresent_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IsValidCodePage(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let CodePage = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/nls") {
            Some(crate::trace::trace_begin(
                "kernel32/nls",
                "IsValidCodePage",
                &[("CodePage", &CodePage)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::IsValidCodePage(machine, CodePage);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::IsValidCodePage_pos.0,
                winapi::kernel32::IsValidCodePage_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn LCMapStringA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let locale = <LCID>::from_stack(mem, stack_args + 0u32);
        let dwMapFlags = <u32>::from_stack(mem, stack_args + 4u32);
        let lpSrcStr = <u32>::from_stack(mem, stack_args + 8u32);
        let cchSrc = <i32>::from_stack(mem, stack_args + 12u32);
        let lpDestStr = <ArrayWithSizeMut<u8>>::from_stack(mem, stack_args + 16u32);
        let __trace_context = if crate::trace::enabled("kernel32/nls") {
            Some(crate::trace::trace_begin(
                "kernel32/nls",
                "LCMapStringA",
                &[
                    ("locale", &locale),
                    ("dwMapFlags", &dwMapFlags),
                    ("lpSrcStr", &lpSrcStr),
                    ("cchSrc", &cchSrc),
                    ("lpDestStr", &lpDestStr),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::LCMapStringA(
            machine, locale, dwMapFlags, lpSrcStr, cchSrc, lpDestStr,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::LCMapStringA_pos.0,
                winapi::kernel32::LCMapStringA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn LCMapStringW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let locale = <LCID>::from_stack(mem, stack_args + 0u32);
        let dwMapFlags = <u32>::from_stack(mem, stack_args + 4u32);
        let lpSrcStr = <u32>::from_stack(mem, stack_args + 8u32);
        let cchSrc = <i32>::from_stack(mem, stack_args + 12u32);
        let lpDestStr = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 16u32);
        let __trace_context = if crate::trace::enabled("kernel32/nls") {
            Some(crate::trace::trace_begin(
                "kernel32/nls",
                "LCMapStringW",
                &[
                    ("locale", &locale),
                    ("dwMapFlags", &dwMapFlags),
                    ("lpSrcStr", &lpSrcStr),
                    ("cchSrc", &cchSrc),
                    ("lpDestStr", &lpDestStr),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::LCMapStringW(
            machine, locale, dwMapFlags, lpSrcStr, cchSrc, lpDestStr,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::LCMapStringW_pos.0,
                winapi::kernel32::LCMapStringW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn LeaveCriticalSection(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/critical_section") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/critical_section",
                "LeaveCriticalSection",
                &[("lpCriticalSection", &lpCriticalSection)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::LeaveCriticalSection(machine, lpCriticalSection);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::LeaveCriticalSection_pos.0,
                winapi::kernel32::LeaveCriticalSection_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn LoadLibraryA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let filename = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/dll") {
            Some(crate::trace::trace_begin(
                "kernel32/dll",
                "LoadLibraryA",
                &[("filename", &filename)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::LoadLibraryA(machine, filename);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::LoadLibraryA_pos.0,
                winapi::kernel32::LoadLibraryA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn LoadLibraryExW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpLibFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
        let hFile = <HFILE>::from_stack(mem, stack_args + 4u32);
        let dwFlags = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/dll") {
            Some(crate::trace::trace_begin(
                "kernel32/dll",
                "LoadLibraryExW",
                &[
                    ("lpLibFileName", &lpLibFileName),
                    ("hFile", &hFile),
                    ("dwFlags", &dwFlags),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::LoadLibraryExW_pos.0,
                winapi::kernel32::LoadLibraryExW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn LoadResource(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
        let hResInfo = <HRSRC>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/resource") {
            Some(crate::trace::trace_begin(
                "kernel32/resource",
                "LoadResource",
                &[("hModule", &hModule), ("hResInfo", &hResInfo)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::LoadResource(machine, hModule, hResInfo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::LoadResource_pos.0,
                winapi::kernel32::LoadResource_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn LocalAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let uFlags = <GMEM>::from_stack(mem, stack_args + 0u32);
        let dwBytes = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "LocalAlloc",
                &[("uFlags", &uFlags), ("dwBytes", &dwBytes)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::LocalAlloc(machine, uFlags, dwBytes);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::LocalAlloc_pos.0,
                winapi::kernel32::LocalAlloc_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn LocalFree(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hMem = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "LocalFree",
                &[("hMem", &hMem)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::LocalFree(machine, hMem);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::LocalFree_pos.0,
                winapi::kernel32::LocalFree_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn LockResource(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hResData = <HRSRC>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/resource") {
            Some(crate::trace::trace_begin(
                "kernel32/resource",
                "LockResource",
                &[("hResData", &hResData)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::LockResource(machine, hResData);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::LockResource_pos.0,
                winapi::kernel32::LockResource_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn MoveFileA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpExistingFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let lpNewFileName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "MoveFileA",
                &[
                    ("lpExistingFileName", &lpExistingFileName),
                    ("lpNewFileName", &lpNewFileName),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::MoveFileA(machine, lpExistingFileName, lpNewFileName);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::MoveFileA_pos.0,
                winapi::kernel32::MoveFileA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn MulDiv(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let nNumber = <i32>::from_stack(mem, stack_args + 0u32);
        let nNumerator = <i32>::from_stack(mem, stack_args + 4u32);
        let nDenominator = <i32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "MulDiv",
                &[
                    ("nNumber", &nNumber),
                    ("nNumerator", &nNumerator),
                    ("nDenominator", &nDenominator),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::MulDiv(machine, nNumber, nNumerator, nDenominator);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::MulDiv_pos.0,
                winapi::kernel32::MulDiv_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn MultiByteToWideChar(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let CodePage = <Result<CP, u32>>::from_stack(mem, stack_args + 0u32);
        let dwFlags = <Result<MB, u32>>::from_stack(mem, stack_args + 4u32);
        let lpMultiByteStr = <u32>::from_stack(mem, stack_args + 8u32);
        let cbMultiByte = <i32>::from_stack(mem, stack_args + 12u32);
        let lpWideCharStr = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 16u32);
        let __trace_context = if crate::trace::enabled("kernel32/nls") {
            Some(crate::trace::trace_begin(
                "kernel32/nls",
                "MultiByteToWideChar",
                &[
                    ("CodePage", &CodePage),
                    ("dwFlags", &dwFlags),
                    ("lpMultiByteStr", &lpMultiByteStr),
                    ("cbMultiByte", &cbMultiByte),
                    ("lpWideCharStr", &lpWideCharStr),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::MultiByteToWideChar(
            machine,
            CodePage,
            dwFlags,
            lpMultiByteStr,
            cbMultiByte,
            lpWideCharStr,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::MultiByteToWideChar_pos.0,
                winapi::kernel32::MultiByteToWideChar_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn NtCurrentTeb(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/thread") {
            Some(crate::trace::trace_begin(
                "kernel32/thread",
                "NtCurrentTeb",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::NtCurrentTeb(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::NtCurrentTeb_pos.0,
                winapi::kernel32::NtCurrentTeb_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn OutputDebugStringA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let msg = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "OutputDebugStringA",
                &[("msg", &msg)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::OutputDebugStringA(machine, msg);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::OutputDebugStringA_pos.0,
                winapi::kernel32::OutputDebugStringA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn PulseEvent(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hEvent = <HEVENT>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/event") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/event",
                "PulseEvent",
                &[("hEvent", &hEvent)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::PulseEvent(machine, hEvent);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::PulseEvent_pos.0,
                winapi::kernel32::PulseEvent_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn QueryPerformanceCounter(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpPerformanceCount = <Option<&mut LARGE_INTEGER>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/time") {
            Some(crate::trace::trace_begin(
                "kernel32/time",
                "QueryPerformanceCounter",
                &[("lpPerformanceCount", &lpPerformanceCount)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::QueryPerformanceCounter(machine, lpPerformanceCount);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::QueryPerformanceCounter_pos.0,
                winapi::kernel32::QueryPerformanceCounter_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn QueryPerformanceFrequency(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpFrequency = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/time") {
            Some(crate::trace::trace_begin(
                "kernel32/time",
                "QueryPerformanceFrequency",
                &[("lpFrequency", &lpFrequency)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::QueryPerformanceFrequency(machine, lpFrequency);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::QueryPerformanceFrequency_pos.0,
                winapi::kernel32::QueryPerformanceFrequency_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn RaiseException(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let dwExceptionCode = <u32>::from_stack(mem, stack_args + 0u32);
        let dwExceptionFlags = <u32>::from_stack(mem, stack_args + 4u32);
        let nNumberOfArguments = <u32>::from_stack(mem, stack_args + 8u32);
        let lpArguments = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "RaiseException",
                &[
                    ("dwExceptionCode", &dwExceptionCode),
                    ("dwExceptionFlags", &dwExceptionFlags),
                    ("nNumberOfArguments", &nNumberOfArguments),
                    ("lpArguments", &lpArguments),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::RaiseException(
            machine,
            dwExceptionCode,
            dwExceptionFlags,
            nNumberOfArguments,
            lpArguments,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::RaiseException_pos.0,
                winapi::kernel32::RaiseException_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn ReadFile(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
        let lpBuffer = <ArrayWithSizeMut<u8>>::from_stack(mem, stack_args + 4u32);
        let lpNumberOfBytesRead = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
        let lpOverlapped = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "ReadFile",
                &[
                    ("hFile", &hFile),
                    ("lpBuffer", &lpBuffer),
                    ("lpNumberOfBytesRead", &lpNumberOfBytesRead),
                    ("lpOverlapped", &lpOverlapped),
                ],
            ))
        } else {
            None
        };
        let result =
            winapi::kernel32::ReadFile(machine, hFile, lpBuffer, lpNumberOfBytesRead, lpOverlapped);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::ReadFile_pos.0,
                winapi::kernel32::ReadFile_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn ReleaseSRWLockExclusive(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/srw_lock") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/srw_lock",
                "ReleaseSRWLockExclusive",
                &[("SRWLock", &SRWLock)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::ReleaseSRWLockExclusive(machine, SRWLock);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::ReleaseSRWLockExclusive_pos.0,
                winapi::kernel32::ReleaseSRWLockExclusive_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn ReleaseSRWLockShared(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/srw_lock") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/srw_lock",
                "ReleaseSRWLockShared",
                &[("SRWLock", &SRWLock)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::ReleaseSRWLockShared(machine, SRWLock);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::ReleaseSRWLockShared_pos.0,
                winapi::kernel32::ReleaseSRWLockShared_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn RemoveDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "RemoveDirectoryA",
                &[("lpPathName", &lpPathName)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::RemoveDirectoryA(machine, lpPathName);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::RemoveDirectoryA_pos.0,
                winapi::kernel32::RemoveDirectoryA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn ResetEvent(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hEvent = <HEVENT>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/event") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/event",
                "ResetEvent",
                &[("hEvent", &hEvent)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::ResetEvent(machine, hEvent);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::ResetEvent_pos.0,
                winapi::kernel32::ResetEvent_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn ResumeThread(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/thread") {
            Some(crate::trace::trace_begin(
                "kernel32/thread",
                "ResumeThread",
                &[("hThread", &hThread)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::ResumeThread(machine, hThread);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::ResumeThread_pos.0,
                winapi::kernel32::ResumeThread_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn RtlUnwind(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let TargetFrame = <u32>::from_stack(mem, stack_args + 0u32);
        let TargetIp = <u32>::from_stack(mem, stack_args + 4u32);
        let ExceptionRecord = <u32>::from_stack(mem, stack_args + 8u32);
        let ReturnValue = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "RtlUnwind",
                &[
                    ("TargetFrame", &TargetFrame),
                    ("TargetIp", &TargetIp),
                    ("ExceptionRecord", &ExceptionRecord),
                    ("ReturnValue", &ReturnValue),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::RtlUnwind(
            machine,
            TargetFrame,
            TargetIp,
            ExceptionRecord,
            ReturnValue,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::RtlUnwind_pos.0,
                winapi::kernel32::RtlUnwind_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetConsoleCtrlHandler(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _handlerRoutine = <DWORD>::from_stack(mem, stack_args + 0u32);
        let _add = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/console") {
            Some(crate::trace::trace_begin(
                "kernel32/console",
                "SetConsoleCtrlHandler",
                &[("handlerRoutine", &_handlerRoutine), ("add", &_add)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetConsoleCtrlHandler(machine, _handlerRoutine, _add);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetConsoleCtrlHandler_pos.0,
                winapi::kernel32::SetConsoleCtrlHandler_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetCurrentDirectoryA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "SetCurrentDirectoryA",
                &[("lpPathName", &lpPathName)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetCurrentDirectoryA(machine, lpPathName);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetCurrentDirectoryA_pos.0,
                winapi::kernel32::SetCurrentDirectoryA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetEndOfFile(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "SetEndOfFile",
                &[("hFile", &hFile)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetEndOfFile(machine, hFile);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetEndOfFile_pos.0,
                winapi::kernel32::SetEndOfFile_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetEnvironmentVariableA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let name = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let value = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/env") {
            Some(crate::trace::trace_begin(
                "kernel32/env",
                "SetEnvironmentVariableA",
                &[("name", &name), ("value", &value)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetEnvironmentVariableA(machine, name, value);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetEnvironmentVariableA_pos.0,
                winapi::kernel32::SetEnvironmentVariableA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetEvent(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hEvent = <HEVENT>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/event") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/event",
                "SetEvent",
                &[("hEvent", &hEvent)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetEvent(machine, hEvent);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetEvent_pos.0,
                winapi::kernel32::SetEvent_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetFileAttributesA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let dwFileAttributes = <Result<FileAttribute, u32>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "SetFileAttributesA",
                &[
                    ("lpFileName", &lpFileName),
                    ("dwFileAttributes", &dwFileAttributes),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetFileAttributesA(machine, lpFileName, dwFileAttributes);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetFileAttributesA_pos.0,
                winapi::kernel32::SetFileAttributesA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetFilePointer(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
        let lDistanceToMove = <i32>::from_stack(mem, stack_args + 4u32);
        let lpDistanceToMoveHigh = <Option<&mut i32>>::from_stack(mem, stack_args + 8u32);
        let dwMoveMethod = <Result<FILE, u32>>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "SetFilePointer",
                &[
                    ("hFile", &hFile),
                    ("lDistanceToMove", &lDistanceToMove),
                    ("lpDistanceToMoveHigh", &lpDistanceToMoveHigh),
                    ("dwMoveMethod", &dwMoveMethod),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetFilePointer(
            machine,
            hFile,
            lDistanceToMove,
            lpDistanceToMoveHigh,
            dwMoveMethod,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetFilePointer_pos.0,
                winapi::kernel32::SetFilePointer_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetFileTime(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
        let lpCreationTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 4u32);
        let lpLastAccessTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 8u32);
        let lpLastWriteTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "SetFileTime",
                &[
                    ("hFile", &hFile),
                    ("lpCreationTime", &lpCreationTime),
                    ("lpLastAccessTime", &lpLastAccessTime),
                    ("lpLastWriteTime", &lpLastWriteTime),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetFileTime(
            machine,
            hFile,
            lpCreationTime,
            lpLastAccessTime,
            lpLastWriteTime,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetFileTime_pos.0,
                winapi::kernel32::SetFileTime_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetHandleCount(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let uNumber = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "SetHandleCount",
                &[("uNumber", &uNumber)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetHandleCount(machine, uNumber);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetHandleCount_pos.0,
                winapi::kernel32::SetHandleCount_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetLastError(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let dwErrCode = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "SetLastError",
                &[("dwErrCode", &dwErrCode)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetLastError(machine, dwErrCode);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetLastError_pos.0,
                winapi::kernel32::SetLastError_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetPriorityClass(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hProcess = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
        let dwPriorityClass = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "SetPriorityClass",
                &[
                    ("hProcess", &hProcess),
                    ("dwPriorityClass", &dwPriorityClass),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetPriorityClass(machine, hProcess, dwPriorityClass);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetPriorityClass_pos.0,
                winapi::kernel32::SetPriorityClass_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetStdHandle(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let nStdHandle = <Result<STD, u32>>::from_stack(mem, stack_args + 0u32);
        let hHandle = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "SetStdHandle",
                &[("nStdHandle", &nStdHandle), ("hHandle", &hHandle)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetStdHandle(machine, nStdHandle, hHandle);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetStdHandle_pos.0,
                winapi::kernel32::SetStdHandle_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetThreadDescription(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
        let lpThreadDescription = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/thread") {
            Some(crate::trace::trace_begin(
                "kernel32/thread",
                "SetThreadDescription",
                &[
                    ("hThread", &hThread),
                    ("lpThreadDescription", &lpThreadDescription),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetThreadDescription(machine, hThread, lpThreadDescription);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetThreadDescription_pos.0,
                winapi::kernel32::SetThreadDescription_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetThreadPriority(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
        let nPriority = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/thread") {
            Some(crate::trace::trace_begin(
                "kernel32/thread",
                "SetThreadPriority",
                &[("hThread", &hThread), ("nPriority", &nPriority)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetThreadPriority(machine, hThread, nPriority);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetThreadPriority_pos.0,
                winapi::kernel32::SetThreadPriority_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetThreadStackGuarantee(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let StackSizeInBytes = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/thread") {
            Some(crate::trace::trace_begin(
                "kernel32/thread",
                "SetThreadStackGuarantee",
                &[("StackSizeInBytes", &StackSizeInBytes)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SetThreadStackGuarantee(machine, StackSizeInBytes);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetThreadStackGuarantee_pos.0,
                winapi::kernel32::SetThreadStackGuarantee_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SetUnhandledExceptionFilter(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _lpTopLevelExceptionFilter = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "SetUnhandledExceptionFilter",
                &[("lpTopLevelExceptionFilter", &_lpTopLevelExceptionFilter)],
            ))
        } else {
            None
        };
        let result =
            winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SetUnhandledExceptionFilter_pos.0,
                winapi::kernel32::SetUnhandledExceptionFilter_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn SizeofResource(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
        let hResInfo = <HRSRC>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/resource") {
            Some(crate::trace::trace_begin(
                "kernel32/resource",
                "SizeofResource",
                &[("hModule", &hModule), ("hResInfo", &hResInfo)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SizeofResource(machine, hModule, hResInfo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SizeofResource_pos.0,
                winapi::kernel32::SizeofResource_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn Sleep(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let dwMilliseconds = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/time") {
            Some(crate::trace::trace_begin(
                "kernel32/time",
                "Sleep",
                &[("dwMilliseconds", &dwMilliseconds)],
            ))
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::kernel32::Sleep(machine, dwMilliseconds).await;
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::Sleep_pos.0,
                    winapi::kernel32::Sleep_pos.1,
                    &result,
                );
            }
            result.to_raw()
        })
    }
    pub unsafe fn SystemTimeToFileTime(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpSystemTime = <Option<&SYSTEMTIME>>::from_stack(mem, stack_args + 0u32);
        let lpFileTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/time") {
            Some(crate::trace::trace_begin(
                "kernel32/time",
                "SystemTimeToFileTime",
                &[("lpSystemTime", &lpSystemTime), ("lpFileTime", &lpFileTime)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::SystemTimeToFileTime(machine, lpSystemTime, lpFileTime);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::SystemTimeToFileTime_pos.0,
                winapi::kernel32::SystemTimeToFileTime_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn TerminateProcess(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hProcess = <u32>::from_stack(mem, stack_args + 0u32);
        let uExitCode = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "TerminateProcess",
                &[("hProcess", &hProcess), ("uExitCode", &uExitCode)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::TerminateProcess(machine, hProcess, uExitCode);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::TerminateProcess_pos.0,
                winapi::kernel32::TerminateProcess_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn TlsAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("kernel32/thread") {
            Some(crate::trace::trace_begin(
                "kernel32/thread",
                "TlsAlloc",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::TlsAlloc(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::TlsAlloc_pos.0,
                winapi::kernel32::TlsAlloc_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn TlsFree(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let dwTlsIndex = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/thread") {
            Some(crate::trace::trace_begin(
                "kernel32/thread",
                "TlsFree",
                &[("dwTlsIndex", &dwTlsIndex)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::TlsFree(machine, dwTlsIndex);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::TlsFree_pos.0,
                winapi::kernel32::TlsFree_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn TlsGetValue(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let dwTlsIndex = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/thread") {
            Some(crate::trace::trace_begin(
                "kernel32/thread",
                "TlsGetValue",
                &[("dwTlsIndex", &dwTlsIndex)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::TlsGetValue(machine, dwTlsIndex);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::TlsGetValue_pos.0,
                winapi::kernel32::TlsGetValue_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn TlsSetValue(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let dwTlsIndex = <u32>::from_stack(mem, stack_args + 0u32);
        let lpTlsValue = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/thread") {
            Some(crate::trace::trace_begin(
                "kernel32/thread",
                "TlsSetValue",
                &[("dwTlsIndex", &dwTlsIndex), ("lpTlsValue", &lpTlsValue)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::TlsSetValue_pos.0,
                winapi::kernel32::TlsSetValue_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn TryAcquireSRWLockExclusive(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/srw_lock") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/srw_lock",
                "TryAcquireSRWLockExclusive",
                &[("SRWLock", &SRWLock)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::TryAcquireSRWLockExclusive(machine, SRWLock);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::TryAcquireSRWLockExclusive_pos.0,
                winapi::kernel32::TryAcquireSRWLockExclusive_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn UnhandledExceptionFilter(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let _exceptionInfo = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/misc") {
            Some(crate::trace::trace_begin(
                "kernel32/misc",
                "UnhandledExceptionFilter",
                &[("exceptionInfo", &_exceptionInfo)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::UnhandledExceptionFilter_pos.0,
                winapi::kernel32::UnhandledExceptionFilter_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn VirtualAlloc(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
        let dwSize = <u32>::from_stack(mem, stack_args + 4u32);
        let flAllocationType = <Result<MEM, u32>>::from_stack(mem, stack_args + 8u32);
        let flProtec = <Result<PAGE, u32>>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "VirtualAlloc",
                &[
                    ("lpAddress", &lpAddress),
                    ("dwSize", &dwSize),
                    ("flAllocationType", &flAllocationType),
                    ("flProtec", &flProtec),
                ],
            ))
        } else {
            None
        };
        let result =
            winapi::kernel32::VirtualAlloc(machine, lpAddress, dwSize, flAllocationType, flProtec);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::VirtualAlloc_pos.0,
                winapi::kernel32::VirtualAlloc_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn VirtualFree(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
        let dwSize = <u32>::from_stack(mem, stack_args + 4u32);
        let dwFreeType = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "VirtualFree",
                &[
                    ("lpAddress", &lpAddress),
                    ("dwSize", &dwSize),
                    ("dwFreeType", &dwFreeType),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::VirtualFree_pos.0,
                winapi::kernel32::VirtualFree_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn VirtualProtect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
        let dwSize = <u32>::from_stack(mem, stack_args + 4u32);
        let flNewProtect = <u32>::from_stack(mem, stack_args + 8u32);
        let lpflOldProtect = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "VirtualProtect",
                &[
                    ("lpAddress", &lpAddress),
                    ("dwSize", &dwSize),
                    ("flNewProtect", &flNewProtect),
                    ("lpflOldProtect", &lpflOldProtect),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::VirtualProtect(
            machine,
            lpAddress,
            dwSize,
            flNewProtect,
            lpflOldProtect,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::VirtualProtect_pos.0,
                winapi::kernel32::VirtualProtect_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn VirtualQuery(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
        let lpBuffer = <Option<&mut MEMORY_BASIC_INFORMATION>>::from_stack(mem, stack_args + 4u32);
        let dwLength = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/memory") {
            Some(crate::trace::trace_begin(
                "kernel32/memory",
                "VirtualQuery",
                &[
                    ("lpAddress", &lpAddress),
                    ("lpBuffer", &lpBuffer),
                    ("dwLength", &dwLength),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::VirtualQuery(machine, lpAddress, lpBuffer, dwLength);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::VirtualQuery_pos.0,
                winapi::kernel32::VirtualQuery_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn WaitForMultipleObjects(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let nCount = <u32>::from_stack(mem, stack_args + 0u32);
        let lpHandles = <u32>::from_stack(mem, stack_args + 4u32);
        let bWaitAll = <bool>::from_stack(mem, stack_args + 8u32);
        let dwMilliseconds = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/wait") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/wait",
                "WaitForMultipleObjects",
                &[
                    ("nCount", &nCount),
                    ("lpHandles", &lpHandles),
                    ("bWaitAll", &bWaitAll),
                    ("dwMilliseconds", &dwMilliseconds),
                ],
            ))
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::kernel32::WaitForMultipleObjects(
                machine,
                nCount,
                lpHandles,
                bWaitAll,
                dwMilliseconds,
            )
            .await;
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::WaitForMultipleObjects_pos.0,
                    winapi::kernel32::WaitForMultipleObjects_pos.1,
                    &result,
                );
            }
            result.to_raw()
        })
    }
    pub unsafe fn WaitForSingleObject(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let handle = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
        let dwMilliseconds = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/sync/wait") {
            Some(crate::trace::trace_begin(
                "kernel32/sync/wait",
                "WaitForSingleObject",
                &[("handle", &handle), ("dwMilliseconds", &dwMilliseconds)],
            ))
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result =
                winapi::kernel32::WaitForSingleObject(machine, handle, dwMilliseconds).await;
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::WaitForSingleObject_pos.0,
                    winapi::kernel32::WaitForSingleObject_pos.1,
                    &result,
                );
            }
            result.to_raw()
        })
    }
    pub unsafe fn WideCharToMultiByte(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let CodePage = <Result<CP, u32>>::from_stack(mem, stack_args + 0u32);
        let dwFlags = <Result<WC, u32>>::from_stack(mem, stack_args + 4u32);
        let lpWideCharStr = <u32>::from_stack(mem, stack_args + 8u32);
        let cchWideChar = <i32>::from_stack(mem, stack_args + 12u32);
        let lpMultiByteStr = <u32>::from_stack(mem, stack_args + 16u32);
        let cbMultiByte = <i32>::from_stack(mem, stack_args + 20u32);
        let lpUsedDefaultChar = <Option<&mut u32>>::from_stack(mem, stack_args + 24u32);
        let __trace_context = if crate::trace::enabled("kernel32/nls") {
            Some(crate::trace::trace_begin(
                "kernel32/nls",
                "WideCharToMultiByte",
                &[
                    ("CodePage", &CodePage),
                    ("dwFlags", &dwFlags),
                    ("lpWideCharStr", &lpWideCharStr),
                    ("cchWideChar", &cchWideChar),
                    ("lpMultiByteStr", &lpMultiByteStr),
                    ("cbMultiByte", &cbMultiByte),
                    ("lpUsedDefaultChar", &lpUsedDefaultChar),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::WideCharToMultiByte(
            machine,
            CodePage,
            dwFlags,
            lpWideCharStr,
            cchWideChar,
            lpMultiByteStr,
            cbMultiByte,
            lpUsedDefaultChar,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::WideCharToMultiByte_pos.0,
                winapi::kernel32::WideCharToMultiByte_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn WriteConsoleA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hConsoleOutput = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
        let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, stack_args + 4u32);
        let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
        let lpReserved = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_context = if crate::trace::enabled("kernel32/console") {
            Some(crate::trace::trace_begin(
                "kernel32/console",
                "WriteConsoleA",
                &[
                    ("hConsoleOutput", &hConsoleOutput),
                    ("lpBuffer", &lpBuffer),
                    ("lpNumberOfCharsWritten", &lpNumberOfCharsWritten),
                    ("lpReserved", &lpReserved),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::WriteConsoleA(
            machine,
            hConsoleOutput,
            lpBuffer,
            lpNumberOfCharsWritten,
            lpReserved,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::WriteConsoleA_pos.0,
                winapi::kernel32::WriteConsoleA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn WriteConsoleW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hConsoleOutput = <HFILE>::from_stack(mem, stack_args + 0u32);
        let lpBuffer = <ArrayWithSize<u16>>::from_stack(mem, stack_args + 4u32);
        let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
        let _lpReserved = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_context = if crate::trace::enabled("kernel32/console") {
            Some(crate::trace::trace_begin(
                "kernel32/console",
                "WriteConsoleW",
                &[
                    ("hConsoleOutput", &hConsoleOutput),
                    ("lpBuffer", &lpBuffer),
                    ("lpNumberOfCharsWritten", &lpNumberOfCharsWritten),
                    ("lpReserved", &_lpReserved),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::WriteConsoleW(
            machine,
            hConsoleOutput,
            lpBuffer,
            lpNumberOfCharsWritten,
            _lpReserved,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::WriteConsoleW_pos.0,
                winapi::kernel32::WriteConsoleW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn WriteFile(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
        let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, stack_args + 4u32);
        let lpNumberOfBytesWritten = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
        let lpOverlapped = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_context = if crate::trace::enabled("kernel32/file") {
            Some(crate::trace::trace_begin(
                "kernel32/file",
                "WriteFile",
                &[
                    ("hFile", &hFile),
                    ("lpBuffer", &lpBuffer),
                    ("lpNumberOfBytesWritten", &lpNumberOfBytesWritten),
                    ("lpOverlapped", &lpOverlapped),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::WriteFile(
            machine,
            hFile,
            lpBuffer,
            lpNumberOfBytesWritten,
            lpOverlapped,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::WriteFile_pos.0,
                winapi::kernel32::WriteFile_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn WritePrivateProfileStringA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpAppName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let lpKeyName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let lpString = <Option<&str>>::from_stack(mem, stack_args + 8u32);
        let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("kernel32/ini") {
            Some(crate::trace::trace_begin(
                "kernel32/ini",
                "WritePrivateProfileStringA",
                &[
                    ("lpAppName", &lpAppName),
                    ("lpKeyName", &lpKeyName),
                    ("lpString", &lpString),
                    ("lpFileName", &lpFileName),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::WritePrivateProfileStringA(
            machine, lpAppName, lpKeyName, lpString, lpFileName,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::WritePrivateProfileStringA_pos.0,
                winapi::kernel32::WritePrivateProfileStringA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn WriteProfileStringW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
        let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
        let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/ini") {
            Some(crate::trace::trace_begin(
                "kernel32/ini",
                "WriteProfileStringW",
                &[
                    ("lpAppName", &lpAppName),
                    ("lpKeyName", &lpKeyName),
                    ("lpString", &lpString),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::WriteProfileStringW(machine, lpAppName, lpKeyName, lpString);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::WriteProfileStringW_pos.0,
                winapi::kernel32::WriteProfileStringW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _lclose(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/file16") {
            Some(crate::trace::trace_begin(
                "kernel32/file16",
                "_lclose",
                &[("hFile", &hFile)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::_lclose(machine, hFile);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::_lclose_pos.0,
                winapi::kernel32::_lclose_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _llseek(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
        let lOffset = <i32>::from_stack(mem, stack_args + 4u32);
        let iOrigin = <i32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("kernel32/file16") {
            Some(crate::trace::trace_begin(
                "kernel32/file16",
                "_llseek",
                &[
                    ("hFile", &hFile),
                    ("lOffset", &lOffset),
                    ("iOrigin", &iOrigin),
                ],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::_llseek(machine, hFile, lOffset, iOrigin);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::_llseek_pos.0,
                winapi::kernel32::_llseek_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _lopen(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let iReadWrite = <i32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/file16") {
            Some(crate::trace::trace_begin(
                "kernel32/file16",
                "_lopen",
                &[("lpPathName", &lpPathName), ("iReadWrite", &iReadWrite)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::_lopen(machine, lpPathName, iReadWrite);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::_lopen_pos.0,
                winapi::kernel32::_lopen_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn _lread(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
        let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/file16") {
            Some(crate::trace::trace_begin(
                "kernel32/file16",
                "_lread",
                &[("hFile", &hFile), ("lpBuffer", &lpBuffer)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::_lread(machine, hFile, lpBuffer);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::_lread_pos.0,
                winapi::kernel32::_lread_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn lstrcmpiA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpString1 = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let lpString2 = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/libc") {
            Some(crate::trace::trace_begin(
                "kernel32/libc",
                "lstrcmpiA",
                &[("lpString1", &lpString1), ("lpString2", &lpString2)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::lstrcmpiA(machine, lpString1, lpString2);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::lstrcmpiA_pos.0,
                winapi::kernel32::lstrcmpiA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn lstrcpyA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpString1 = <u32>::from_stack(mem, stack_args + 0u32);
        let lpString2 = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/libc") {
            Some(crate::trace::trace_begin(
                "kernel32/libc",
                "lstrcpyA",
                &[("lpString1", &lpString1), ("lpString2", &lpString2)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::lstrcpyA(machine, lpString1, lpString2);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::lstrcpyA_pos.0,
                winapi::kernel32::lstrcpyA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn lstrcpyW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpString1 = <u32>::from_stack(mem, stack_args + 0u32);
        let lpString2 = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/libc") {
            Some(crate::trace::trace_begin(
                "kernel32/libc",
                "lstrcpyW",
                &[("lpString1", &lpString1), ("lpString2", &lpString2)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::lstrcpyW(machine, lpString1, lpString2);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::lstrcpyW_pos.0,
                winapi::kernel32::lstrcpyW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn lstrlenA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpString = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/libc") {
            Some(crate::trace::trace_begin(
                "kernel32/libc",
                "lstrlenA",
                &[("lpString", &lpString)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::lstrlenA(machine, lpString);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::lstrlenA_pos.0,
                winapi::kernel32::lstrlenA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn lstrlenW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/libc") {
            Some(crate::trace::trace_begin(
                "kernel32/libc",
                "lstrlenW",
                &[("lpString", &lpString)],
            ))
        } else {
            None
        };
        let result = winapi::kernel32::lstrlenW(machine, lpString);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::kernel32::lstrlenW_pos.0,
                winapi::kernel32::lstrlenW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn retrowin32_main(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let entry_point = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("kernel32/init") {
            Some(crate::trace::trace_begin(
                "kernel32/init",
                "retrowin32_main",
                &[("entry_point", &entry_point)],
            ))
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::kernel32::retrowin32_main(machine, entry_point).await;
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::retrowin32_main_pos.0,
                    winapi::kernel32::retrowin32_main_pos.1,
                    &result,
                );
            }
            result.to_raw()
        })
    }
    pub unsafe fn retrowin32_thread_main(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let entry_point = <u32>::from_stack(mem, stack_args + 0u32);
        let param = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("kernel32/init") {
            Some(crate::trace::trace_begin(
                "kernel32/init",
                "retrowin32_thread_main",
                &[("entry_point", &entry_point), ("param", &param)],
            ))
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result =
                winapi::kernel32::retrowin32_thread_main(machine, entry_point, param).await;
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::kernel32::retrowin32_thread_main_pos.0,
                    winapi::kernel32::retrowin32_thread_main_pos.1,
                    &result,
                );
            }
            result.to_raw()
        })
    }
}
const SHIMS: [Shim; 190usize] = [
    Shim {
        name: "AcquireSRWLockExclusive",
        func: Handler::Sync(wrappers::AcquireSRWLockExclusive),
    },
    Shim {
        name: "AcquireSRWLockShared",
        func: Handler::Sync(wrappers::AcquireSRWLockShared),
    },
    Shim {
        name: "AddVectoredExceptionHandler",
        func: Handler::Sync(wrappers::AddVectoredExceptionHandler),
    },
    Shim {
        name: "CloseHandle",
        func: Handler::Sync(wrappers::CloseHandle),
    },
    Shim {
        name: "CompareStringA",
        func: Handler::Sync(wrappers::CompareStringA),
    },
    Shim {
        name: "CompareStringW",
        func: Handler::Sync(wrappers::CompareStringW),
    },
    Shim {
        name: "CreateDirectoryA",
        func: Handler::Sync(wrappers::CreateDirectoryA),
    },
    Shim {
        name: "CreateEventA",
        func: Handler::Sync(wrappers::CreateEventA),
    },
    Shim {
        name: "CreateFileA",
        func: Handler::Sync(wrappers::CreateFileA),
    },
    Shim {
        name: "CreateFileW",
        func: Handler::Sync(wrappers::CreateFileW),
    },
    Shim {
        name: "CreateProcessA",
        func: Handler::Sync(wrappers::CreateProcessA),
    },
    Shim {
        name: "CreateThread",
        func: Handler::Async(wrappers::CreateThread),
    },
    Shim {
        name: "DebugBreak",
        func: Handler::Sync(wrappers::DebugBreak),
    },
    Shim {
        name: "DeleteCriticalSection",
        func: Handler::Sync(wrappers::DeleteCriticalSection),
    },
    Shim {
        name: "DeleteFileA",
        func: Handler::Sync(wrappers::DeleteFileA),
    },
    Shim {
        name: "DisableThreadLibraryCalls",
        func: Handler::Sync(wrappers::DisableThreadLibraryCalls),
    },
    Shim {
        name: "DuplicateHandle",
        func: Handler::Sync(wrappers::DuplicateHandle),
    },
    Shim {
        name: "EnterCriticalSection",
        func: Handler::Sync(wrappers::EnterCriticalSection),
    },
    Shim {
        name: "ExitProcess",
        func: Handler::Sync(wrappers::ExitProcess),
    },
    Shim {
        name: "ExitThread",
        func: Handler::Sync(wrappers::ExitThread),
    },
    Shim {
        name: "FileTimeToLocalFileTime",
        func: Handler::Sync(wrappers::FileTimeToLocalFileTime),
    },
    Shim {
        name: "FileTimeToSystemTime",
        func: Handler::Sync(wrappers::FileTimeToSystemTime),
    },
    Shim {
        name: "FindClose",
        func: Handler::Sync(wrappers::FindClose),
    },
    Shim {
        name: "FindFirstFileA",
        func: Handler::Sync(wrappers::FindFirstFileA),
    },
    Shim {
        name: "FindNextFileA",
        func: Handler::Sync(wrappers::FindNextFileA),
    },
    Shim {
        name: "FindResourceA",
        func: Handler::Sync(wrappers::FindResourceA),
    },
    Shim {
        name: "FindResourceW",
        func: Handler::Sync(wrappers::FindResourceW),
    },
    Shim {
        name: "FlushFileBuffers",
        func: Handler::Sync(wrappers::FlushFileBuffers),
    },
    Shim {
        name: "FormatMessageA",
        func: Handler::Sync(wrappers::FormatMessageA),
    },
    Shim {
        name: "FormatMessageW",
        func: Handler::Sync(wrappers::FormatMessageW),
    },
    Shim {
        name: "FreeEnvironmentStringsA",
        func: Handler::Sync(wrappers::FreeEnvironmentStringsA),
    },
    Shim {
        name: "FreeEnvironmentStringsW",
        func: Handler::Sync(wrappers::FreeEnvironmentStringsW),
    },
    Shim {
        name: "FreeLibrary",
        func: Handler::Sync(wrappers::FreeLibrary),
    },
    Shim {
        name: "GetACP",
        func: Handler::Sync(wrappers::GetACP),
    },
    Shim {
        name: "GetCPInfo",
        func: Handler::Sync(wrappers::GetCPInfo),
    },
    Shim {
        name: "GetCommandLineA",
        func: Handler::Sync(wrappers::GetCommandLineA),
    },
    Shim {
        name: "GetCommandLineW",
        func: Handler::Sync(wrappers::GetCommandLineW),
    },
    Shim {
        name: "GetConsoleMode",
        func: Handler::Sync(wrappers::GetConsoleMode),
    },
    Shim {
        name: "GetConsoleScreenBufferInfo",
        func: Handler::Sync(wrappers::GetConsoleScreenBufferInfo),
    },
    Shim {
        name: "GetCurrentDirectoryA",
        func: Handler::Sync(wrappers::GetCurrentDirectoryA),
    },
    Shim {
        name: "GetCurrentProcess",
        func: Handler::Sync(wrappers::GetCurrentProcess),
    },
    Shim {
        name: "GetCurrentProcessId",
        func: Handler::Sync(wrappers::GetCurrentProcessId),
    },
    Shim {
        name: "GetCurrentThread",
        func: Handler::Sync(wrappers::GetCurrentThread),
    },
    Shim {
        name: "GetCurrentThreadId",
        func: Handler::Sync(wrappers::GetCurrentThreadId),
    },
    Shim {
        name: "GetDiskFreeSpaceA",
        func: Handler::Sync(wrappers::GetDiskFreeSpaceA),
    },
    Shim {
        name: "GetDriveTypeA",
        func: Handler::Sync(wrappers::GetDriveTypeA),
    },
    Shim {
        name: "GetEnvironmentStrings",
        func: Handler::Sync(wrappers::GetEnvironmentStrings),
    },
    Shim {
        name: "GetEnvironmentStringsW",
        func: Handler::Sync(wrappers::GetEnvironmentStringsW),
    },
    Shim {
        name: "GetEnvironmentVariableA",
        func: Handler::Sync(wrappers::GetEnvironmentVariableA),
    },
    Shim {
        name: "GetEnvironmentVariableW",
        func: Handler::Sync(wrappers::GetEnvironmentVariableW),
    },
    Shim {
        name: "GetExitCodeProcess",
        func: Handler::Sync(wrappers::GetExitCodeProcess),
    },
    Shim {
        name: "GetFileAttributesA",
        func: Handler::Sync(wrappers::GetFileAttributesA),
    },
    Shim {
        name: "GetFileInformationByHandle",
        func: Handler::Sync(wrappers::GetFileInformationByHandle),
    },
    Shim {
        name: "GetFileSize",
        func: Handler::Sync(wrappers::GetFileSize),
    },
    Shim {
        name: "GetFileTime",
        func: Handler::Sync(wrappers::GetFileTime),
    },
    Shim {
        name: "GetFileType",
        func: Handler::Sync(wrappers::GetFileType),
    },
    Shim {
        name: "GetFullPathNameA",
        func: Handler::Sync(wrappers::GetFullPathNameA),
    },
    Shim {
        name: "GetFullPathNameW",
        func: Handler::Sync(wrappers::GetFullPathNameW),
    },
    Shim {
        name: "GetLastError",
        func: Handler::Sync(wrappers::GetLastError),
    },
    Shim {
        name: "GetLocalTime",
        func: Handler::Sync(wrappers::GetLocalTime),
    },
    Shim {
        name: "GetLogicalDrives",
        func: Handler::Sync(wrappers::GetLogicalDrives),
    },
    Shim {
        name: "GetModuleFileNameA",
        func: Handler::Sync(wrappers::GetModuleFileNameA),
    },
    Shim {
        name: "GetModuleFileNameW",
        func: Handler::Sync(wrappers::GetModuleFileNameW),
    },
    Shim {
        name: "GetModuleHandleA",
        func: Handler::Sync(wrappers::GetModuleHandleA),
    },
    Shim {
        name: "GetModuleHandleExW",
        func: Handler::Sync(wrappers::GetModuleHandleExW),
    },
    Shim {
        name: "GetModuleHandleW",
        func: Handler::Sync(wrappers::GetModuleHandleW),
    },
    Shim {
        name: "GetOEMCP",
        func: Handler::Sync(wrappers::GetOEMCP),
    },
    Shim {
        name: "GetPrivateProfileIntW",
        func: Handler::Sync(wrappers::GetPrivateProfileIntW),
    },
    Shim {
        name: "GetPrivateProfileStringA",
        func: Handler::Sync(wrappers::GetPrivateProfileStringA),
    },
    Shim {
        name: "GetPrivateProfileStringW",
        func: Handler::Sync(wrappers::GetPrivateProfileStringW),
    },
    Shim {
        name: "GetProcAddress",
        func: Handler::Sync(wrappers::GetProcAddress),
    },
    Shim {
        name: "GetProcessHeap",
        func: Handler::Sync(wrappers::GetProcessHeap),
    },
    Shim {
        name: "GetProfileIntW",
        func: Handler::Sync(wrappers::GetProfileIntW),
    },
    Shim {
        name: "GetProfileStringW",
        func: Handler::Sync(wrappers::GetProfileStringW),
    },
    Shim {
        name: "GetStartupInfoA",
        func: Handler::Sync(wrappers::GetStartupInfoA),
    },
    Shim {
        name: "GetStartupInfoW",
        func: Handler::Sync(wrappers::GetStartupInfoW),
    },
    Shim {
        name: "GetStdHandle",
        func: Handler::Sync(wrappers::GetStdHandle),
    },
    Shim {
        name: "GetStringTypeA",
        func: Handler::Sync(wrappers::GetStringTypeA),
    },
    Shim {
        name: "GetStringTypeW",
        func: Handler::Sync(wrappers::GetStringTypeW),
    },
    Shim {
        name: "GetSystemDirectoryA",
        func: Handler::Sync(wrappers::GetSystemDirectoryA),
    },
    Shim {
        name: "GetSystemTime",
        func: Handler::Sync(wrappers::GetSystemTime),
    },
    Shim {
        name: "GetSystemTimeAsFileTime",
        func: Handler::Sync(wrappers::GetSystemTimeAsFileTime),
    },
    Shim {
        name: "GetThreadPriority",
        func: Handler::Sync(wrappers::GetThreadPriority),
    },
    Shim {
        name: "GetTickCount",
        func: Handler::Sync(wrappers::GetTickCount),
    },
    Shim {
        name: "GetTimeZoneInformation",
        func: Handler::Sync(wrappers::GetTimeZoneInformation),
    },
    Shim {
        name: "GetVersion",
        func: Handler::Sync(wrappers::GetVersion),
    },
    Shim {
        name: "GetVersionExA",
        func: Handler::Sync(wrappers::GetVersionExA),
    },
    Shim {
        name: "GetWindowsDirectoryA",
        func: Handler::Sync(wrappers::GetWindowsDirectoryA),
    },
    Shim {
        name: "GlobalAlloc",
        func: Handler::Sync(wrappers::GlobalAlloc),
    },
    Shim {
        name: "GlobalFlags",
        func: Handler::Sync(wrappers::GlobalFlags),
    },
    Shim {
        name: "GlobalFree",
        func: Handler::Sync(wrappers::GlobalFree),
    },
    Shim {
        name: "GlobalReAlloc",
        func: Handler::Sync(wrappers::GlobalReAlloc),
    },
    Shim {
        name: "HeapAlloc",
        func: Handler::Sync(wrappers::HeapAlloc),
    },
    Shim {
        name: "HeapCreate",
        func: Handler::Sync(wrappers::HeapCreate),
    },
    Shim {
        name: "HeapDestroy",
        func: Handler::Sync(wrappers::HeapDestroy),
    },
    Shim {
        name: "HeapFree",
        func: Handler::Sync(wrappers::HeapFree),
    },
    Shim {
        name: "HeapReAlloc",
        func: Handler::Sync(wrappers::HeapReAlloc),
    },
    Shim {
        name: "HeapSetInformation",
        func: Handler::Sync(wrappers::HeapSetInformation),
    },
    Shim {
        name: "HeapSize",
        func: Handler::Sync(wrappers::HeapSize),
    },
    Shim {
        name: "HeapValidate",
        func: Handler::Sync(wrappers::HeapValidate),
    },
    Shim {
        name: "InitOnceBeginInitialize",
        func: Handler::Sync(wrappers::InitOnceBeginInitialize),
    },
    Shim {
        name: "InitOnceComplete",
        func: Handler::Sync(wrappers::InitOnceComplete),
    },
    Shim {
        name: "InitializeCriticalSection",
        func: Handler::Sync(wrappers::InitializeCriticalSection),
    },
    Shim {
        name: "InitializeCriticalSectionAndSpinCount",
        func: Handler::Sync(wrappers::InitializeCriticalSectionAndSpinCount),
    },
    Shim {
        name: "InitializeCriticalSectionEx",
        func: Handler::Sync(wrappers::InitializeCriticalSectionEx),
    },
    Shim {
        name: "InitializeSListHead",
        func: Handler::Sync(wrappers::InitializeSListHead),
    },
    Shim {
        name: "InterlockedDecrement",
        func: Handler::Sync(wrappers::InterlockedDecrement),
    },
    Shim {
        name: "InterlockedIncrement",
        func: Handler::Sync(wrappers::InterlockedIncrement),
    },
    Shim {
        name: "IsBadCodePtr",
        func: Handler::Sync(wrappers::IsBadCodePtr),
    },
    Shim {
        name: "IsBadReadPtr",
        func: Handler::Sync(wrappers::IsBadReadPtr),
    },
    Shim {
        name: "IsBadWritePtr",
        func: Handler::Sync(wrappers::IsBadWritePtr),
    },
    Shim {
        name: "IsDBCSLeadByte",
        func: Handler::Sync(wrappers::IsDBCSLeadByte),
    },
    Shim {
        name: "IsDBCSLeadByteEx",
        func: Handler::Sync(wrappers::IsDBCSLeadByteEx),
    },
    Shim {
        name: "IsDebuggerPresent",
        func: Handler::Sync(wrappers::IsDebuggerPresent),
    },
    Shim {
        name: "IsProcessorFeaturePresent",
        func: Handler::Sync(wrappers::IsProcessorFeaturePresent),
    },
    Shim {
        name: "IsValidCodePage",
        func: Handler::Sync(wrappers::IsValidCodePage),
    },
    Shim {
        name: "LCMapStringA",
        func: Handler::Sync(wrappers::LCMapStringA),
    },
    Shim {
        name: "LCMapStringW",
        func: Handler::Sync(wrappers::LCMapStringW),
    },
    Shim {
        name: "LeaveCriticalSection",
        func: Handler::Sync(wrappers::LeaveCriticalSection),
    },
    Shim {
        name: "LoadLibraryA",
        func: Handler::Sync(wrappers::LoadLibraryA),
    },
    Shim {
        name: "LoadLibraryExW",
        func: Handler::Sync(wrappers::LoadLibraryExW),
    },
    Shim {
        name: "LoadResource",
        func: Handler::Sync(wrappers::LoadResource),
    },
    Shim {
        name: "LocalAlloc",
        func: Handler::Sync(wrappers::LocalAlloc),
    },
    Shim {
        name: "LocalFree",
        func: Handler::Sync(wrappers::LocalFree),
    },
    Shim {
        name: "LockResource",
        func: Handler::Sync(wrappers::LockResource),
    },
    Shim {
        name: "MoveFileA",
        func: Handler::Sync(wrappers::MoveFileA),
    },
    Shim {
        name: "MulDiv",
        func: Handler::Sync(wrappers::MulDiv),
    },
    Shim {
        name: "MultiByteToWideChar",
        func: Handler::Sync(wrappers::MultiByteToWideChar),
    },
    Shim {
        name: "NtCurrentTeb",
        func: Handler::Sync(wrappers::NtCurrentTeb),
    },
    Shim {
        name: "OutputDebugStringA",
        func: Handler::Sync(wrappers::OutputDebugStringA),
    },
    Shim {
        name: "PulseEvent",
        func: Handler::Sync(wrappers::PulseEvent),
    },
    Shim {
        name: "QueryPerformanceCounter",
        func: Handler::Sync(wrappers::QueryPerformanceCounter),
    },
    Shim {
        name: "QueryPerformanceFrequency",
        func: Handler::Sync(wrappers::QueryPerformanceFrequency),
    },
    Shim {
        name: "RaiseException",
        func: Handler::Sync(wrappers::RaiseException),
    },
    Shim {
        name: "ReadFile",
        func: Handler::Sync(wrappers::ReadFile),
    },
    Shim {
        name: "ReleaseSRWLockExclusive",
        func: Handler::Sync(wrappers::ReleaseSRWLockExclusive),
    },
    Shim {
        name: "ReleaseSRWLockShared",
        func: Handler::Sync(wrappers::ReleaseSRWLockShared),
    },
    Shim {
        name: "RemoveDirectoryA",
        func: Handler::Sync(wrappers::RemoveDirectoryA),
    },
    Shim {
        name: "ResetEvent",
        func: Handler::Sync(wrappers::ResetEvent),
    },
    Shim {
        name: "ResumeThread",
        func: Handler::Sync(wrappers::ResumeThread),
    },
    Shim {
        name: "RtlUnwind",
        func: Handler::Sync(wrappers::RtlUnwind),
    },
    Shim {
        name: "SetConsoleCtrlHandler",
        func: Handler::Sync(wrappers::SetConsoleCtrlHandler),
    },
    Shim {
        name: "SetCurrentDirectoryA",
        func: Handler::Sync(wrappers::SetCurrentDirectoryA),
    },
    Shim {
        name: "SetEndOfFile",
        func: Handler::Sync(wrappers::SetEndOfFile),
    },
    Shim {
        name: "SetEnvironmentVariableA",
        func: Handler::Sync(wrappers::SetEnvironmentVariableA),
    },
    Shim {
        name: "SetEvent",
        func: Handler::Sync(wrappers::SetEvent),
    },
    Shim {
        name: "SetFileAttributesA",
        func: Handler::Sync(wrappers::SetFileAttributesA),
    },
    Shim {
        name: "SetFilePointer",
        func: Handler::Sync(wrappers::SetFilePointer),
    },
    Shim {
        name: "SetFileTime",
        func: Handler::Sync(wrappers::SetFileTime),
    },
    Shim {
        name: "SetHandleCount",
        func: Handler::Sync(wrappers::SetHandleCount),
    },
    Shim {
        name: "SetLastError",
        func: Handler::Sync(wrappers::SetLastError),
    },
    Shim {
        name: "SetPriorityClass",
        func: Handler::Sync(wrappers::SetPriorityClass),
    },
    Shim {
        name: "SetStdHandle",
        func: Handler::Sync(wrappers::SetStdHandle),
    },
    Shim {
        name: "SetThreadDescription",
        func: Handler::Sync(wrappers::SetThreadDescription),
    },
    Shim {
        name: "SetThreadPriority",
        func: Handler::Sync(wrappers::SetThreadPriority),
    },
    Shim {
        name: "SetThreadStackGuarantee",
        func: Handler::Sync(wrappers::SetThreadStackGuarantee),
    },
    Shim {
        name: "SetUnhandledExceptionFilter",
        func: Handler::Sync(wrappers::SetUnhandledExceptionFilter),
    },
    Shim {
        name: "SizeofResource",
        func: Handler::Sync(wrappers::SizeofResource),
    },
    Shim {
        name: "Sleep",
        func: Handler::Async(wrappers::Sleep),
    },
    Shim {
        name: "SystemTimeToFileTime",
        func: Handler::Sync(wrappers::SystemTimeToFileTime),
    },
    Shim {
        name: "TerminateProcess",
        func: Handler::Sync(wrappers::TerminateProcess),
    },
    Shim {
        name: "TlsAlloc",
        func: Handler::Sync(wrappers::TlsAlloc),
    },
    Shim {
        name: "TlsFree",
        func: Handler::Sync(wrappers::TlsFree),
    },
    Shim {
        name: "TlsGetValue",
        func: Handler::Sync(wrappers::TlsGetValue),
    },
    Shim {
        name: "TlsSetValue",
        func: Handler::Sync(wrappers::TlsSetValue),
    },
    Shim {
        name: "TryAcquireSRWLockExclusive",
        func: Handler::Sync(wrappers::TryAcquireSRWLockExclusive),
    },
    Shim {
        name: "UnhandledExceptionFilter",
        func: Handler::Sync(wrappers::UnhandledExceptionFilter),
    },
    Shim {
        name: "VirtualAlloc",
        func: Handler::Sync(wrappers::VirtualAlloc),
    },
    Shim {
        name: "VirtualFree",
        func: Handler::Sync(wrappers::VirtualFree),
    },
    Shim {
        name: "VirtualProtect",
        func: Handler::Sync(wrappers::VirtualProtect),
    },
    Shim {
        name: "VirtualQuery",
        func: Handler::Sync(wrappers::VirtualQuery),
    },
    Shim {
        name: "WaitForMultipleObjects",
        func: Handler::Async(wrappers::WaitForMultipleObjects),
    },
    Shim {
        name: "WaitForSingleObject",
        func: Handler::Async(wrappers::WaitForSingleObject),
    },
    Shim {
        name: "WideCharToMultiByte",
        func: Handler::Sync(wrappers::WideCharToMultiByte),
    },
    Shim {
        name: "WriteConsoleA",
        func: Handler::Sync(wrappers::WriteConsoleA),
    },
    Shim {
        name: "WriteConsoleW",
        func: Handler::Sync(wrappers::WriteConsoleW),
    },
    Shim {
        name: "WriteFile",
        func: Handler::Sync(wrappers::WriteFile),
    },
    Shim {
        name: "WritePrivateProfileStringA",
        func: Handler::Sync(wrappers::WritePrivateProfileStringA),
    },
    Shim {
        name: "WriteProfileStringW",
        func: Handler::Sync(wrappers::WriteProfileStringW),
    },
    Shim {
        name: "_lclose",
        func: Handler::Sync(wrappers::_lclose),
    },
    Shim {
        name: "_llseek",
        func: Handler::Sync(wrappers::_llseek),
    },
    Shim {
        name: "_lopen",
        func: Handler::Sync(wrappers::_lopen),
    },
    Shim {
        name: "_lread",
        func: Handler::Sync(wrappers::_lread),
    },
    Shim {
        name: "lstrcmpiA",
        func: Handler::Sync(wrappers::lstrcmpiA),
    },
    Shim {
        name: "lstrcpyA",
        func: Handler::Sync(wrappers::lstrcpyA),
    },
    Shim {
        name: "lstrcpyW",
        func: Handler::Sync(wrappers::lstrcpyW),
    },
    Shim {
        name: "lstrlenA",
        func: Handler::Sync(wrappers::lstrlenA),
    },
    Shim {
        name: "lstrlenW",
        func: Handler::Sync(wrappers::lstrlenW),
    },
    Shim {
        name: "retrowin32_main",
        func: Handler::Async(wrappers::retrowin32_main),
    },
    Shim {
        name: "retrowin32_thread_main",
        func: Handler::Async(wrappers::retrowin32_thread_main),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "kernel32.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/kernel32.dll"),
};
