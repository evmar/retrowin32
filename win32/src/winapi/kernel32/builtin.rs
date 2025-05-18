#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate::winapi::kernel32::{self, *};
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
    pub unsafe fn AcquireSRWLockExclusive(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/sync/srw_lock") {
                trace::Record::new(
                    kernel32::AcquireSRWLockExclusive_pos,
                    "kernel32/sync/srw_lock",
                    "AcquireSRWLockExclusive",
                    &[("SRWLock", &SRWLock)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::AcquireSRWLockExclusive(sys, SRWLock);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn AcquireSRWLockShared(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/sync/srw_lock") {
                trace::Record::new(
                    kernel32::AcquireSRWLockShared_pos,
                    "kernel32/sync/srw_lock",
                    "AcquireSRWLockShared",
                    &[("SRWLock", &SRWLock)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::AcquireSRWLockShared(sys, SRWLock);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn AddVectoredExceptionHandler(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let first = <u32>::from_stack(mem, stack_args + 0u32);
            let handler = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::AddVectoredExceptionHandler_pos,
                    "kernel32/misc",
                    "AddVectoredExceptionHandler",
                    &[("first", &first), ("handler", &handler)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::AddVectoredExceptionHandler(sys, first, handler);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn Beep(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let dwFreq = <u32>::from_stack(mem, stack_args + 0u32);
            let dwDuration = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::Beep_pos,
                    "kernel32/misc",
                    "Beep",
                    &[("dwFreq", &dwFreq), ("dwDuration", &dwDuration)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::Beep(sys, dwFreq, dwDuration);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CloseHandle(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hObject = <HFILE>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::CloseHandle_pos,
                    "kernel32/misc",
                    "CloseHandle",
                    &[("hObject", &hObject)],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::CloseHandle(&mut *(sys.machine() as *mut crate::Machine), hObject);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CompareStringA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let Locale = <u32>::from_stack(mem, stack_args + 0u32);
            let dwCmpFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpString1 = <u32>::from_stack(mem, stack_args + 8u32);
            let cchCount1 = <i32>::from_stack(mem, stack_args + 12u32);
            let lpString2 = <u32>::from_stack(mem, stack_args + 16u32);
            let cchCount2 = <i32>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::CompareStringA_pos,
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
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::CompareStringA(
                sys, Locale, dwCmpFlags, lpString1, cchCount1, lpString2, cchCount2,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CompareStringW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let Locale = <u32>::from_stack(mem, stack_args + 0u32);
            let dwCmpFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpString1 = <u32>::from_stack(mem, stack_args + 8u32);
            let cchCount1 = <i32>::from_stack(mem, stack_args + 12u32);
            let lpString2 = <u32>::from_stack(mem, stack_args + 16u32);
            let cchCount2 = <i32>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::CompareStringW_pos,
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
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::CompareStringW(
                sys, Locale, dwCmpFlags, lpString1, cchCount1, lpString2, cchCount2,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateDirectoryA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/fs") {
                trace::Record::new(
                    kernel32::CreateDirectoryA_pos,
                    "kernel32/file/fs",
                    "CreateDirectoryA",
                    &[
                        ("lpPathName", &lpPathName),
                        ("lpSecurityAttributes", &lpSecurityAttributes),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::CreateDirectoryA(
                &mut *(sys.machine() as *mut crate::Machine),
                lpPathName,
                lpSecurityAttributes,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateDirectoryW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpPathName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpSecurityAttributes =
                <Option<&mut SECURITY_ATTRIBUTES>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/fs") {
                trace::Record::new(
                    kernel32::CreateDirectoryW_pos,
                    "kernel32/file/fs",
                    "CreateDirectoryW",
                    &[
                        ("lpPathName", &lpPathName),
                        ("lpSecurityAttributes", &lpSecurityAttributes),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::CreateDirectoryW(sys, lpPathName, lpSecurityAttributes);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateEventA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpEventAttributes = <u32>::from_stack(mem, stack_args + 0u32);
            let bManualReset = <bool>::from_stack(mem, stack_args + 4u32);
            let bInitialState = <bool>::from_stack(mem, stack_args + 8u32);
            let lpName = <Option<&str>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/sync/event") {
                trace::Record::new(
                    kernel32::CreateEventA_pos,
                    "kernel32/sync/event",
                    "CreateEventA",
                    &[
                        ("lpEventAttributes", &lpEventAttributes),
                        ("bManualReset", &bManualReset),
                        ("bInitialState", &bInitialState),
                        ("lpName", &lpName),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::CreateEventA(
                &mut *(sys.machine() as *mut crate::Machine),
                lpEventAttributes,
                bManualReset,
                bInitialState,
                lpName,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateFileA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let dwDesiredAccess = <Result<GENERIC, u32>>::from_stack(mem, stack_args + 4u32);
            let dwShareMode = <u32>::from_stack(mem, stack_args + 8u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, stack_args + 12u32);
            let dwCreationDisposition =
                <Result<CreationDisposition, u32>>::from_stack(mem, stack_args + 16u32);
            let dwFlagsAndAttributes =
                <Result<FlagsAndAttributes, u32>>::from_stack(mem, stack_args + 20u32);
            let hTemplateFile = <HFILE>::from_stack(mem, stack_args + 24u32);
            let __trace_record = if trace::enabled("kernel32/file/file") {
                trace::Record::new(
                    kernel32::CreateFileA_pos,
                    "kernel32/file/file",
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
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::CreateFileA(
                &mut *(sys.machine() as *mut crate::Machine),
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateFileMappingA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpFileMappingAttributes =
                <Option<&mut SECURITY_ATTRIBUTES>>::from_stack(mem, stack_args + 4u32);
            let flProtect = <u32>::from_stack(mem, stack_args + 8u32);
            let dwMaximumSizeHigh = <u32>::from_stack(mem, stack_args + 12u32);
            let dwMaximumSizeLow = <u32>::from_stack(mem, stack_args + 16u32);
            let lpName = <Option<&str>>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("kernel32/file/mapping") {
                trace::Record::new(
                    kernel32::CreateFileMappingA_pos,
                    "kernel32/file/mapping",
                    "CreateFileMappingA",
                    &[
                        ("hFile", &hFile),
                        ("lpFileMappingAttributes", &lpFileMappingAttributes),
                        ("flProtect", &flProtect),
                        ("dwMaximumSizeHigh", &dwMaximumSizeHigh),
                        ("dwMaximumSizeLow", &dwMaximumSizeLow),
                        ("lpName", &lpName),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::CreateFileMappingA(
                sys,
                hFile,
                lpFileMappingAttributes,
                flProtect,
                dwMaximumSizeHigh,
                dwMaximumSizeLow,
                lpName,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateFileW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let dwDesiredAccess = <Result<GENERIC, u32>>::from_stack(mem, stack_args + 4u32);
            let dwShareMode = <u32>::from_stack(mem, stack_args + 8u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, stack_args + 12u32);
            let dwCreationDisposition =
                <Result<CreationDisposition, u32>>::from_stack(mem, stack_args + 16u32);
            let dwFlagsAndAttributes =
                <Result<FlagsAndAttributes, u32>>::from_stack(mem, stack_args + 20u32);
            let hTemplateFile = <HFILE>::from_stack(mem, stack_args + 24u32);
            let __trace_record = if trace::enabled("kernel32/file/file") {
                trace::Record::new(
                    kernel32::CreateFileW_pos,
                    "kernel32/file/file",
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
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::CreateFileW(
                &mut *(sys.machine() as *mut crate::Machine),
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateMutexA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpMutexAttributes =
                <Option<&mut SECURITY_ATTRIBUTES>>::from_stack(mem, stack_args + 0u32);
            let bInitialOwner = <bool>::from_stack(mem, stack_args + 4u32);
            let lpName = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/sync/mutex") {
                trace::Record::new(
                    kernel32::CreateMutexA_pos,
                    "kernel32/sync/mutex",
                    "CreateMutexA",
                    &[
                        ("lpMutexAttributes", &lpMutexAttributes),
                        ("bInitialOwner", &bInitialOwner),
                        ("lpName", &lpName),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::CreateMutexA(sys, lpMutexAttributes, bInitialOwner, lpName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreatePipe(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hReadPipe = <Option<&mut HFILE>>::from_stack(mem, stack_args + 0u32);
            let hWritePipe = <Option<&mut HFILE>>::from_stack(mem, stack_args + 4u32);
            let lpPipeAttributes =
                <Option<&mut SECURITY_ATTRIBUTES>>::from_stack(mem, stack_args + 8u32);
            let nSize = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/pipe") {
                trace::Record::new(
                    kernel32::CreatePipe_pos,
                    "kernel32/pipe",
                    "CreatePipe",
                    &[
                        ("hReadPipe", &hReadPipe),
                        ("hWritePipe", &hWritePipe),
                        ("lpPipeAttributes", &lpPipeAttributes),
                        ("nSize", &nSize),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::CreatePipe(sys, hReadPipe, hWritePipe, lpPipeAttributes, nSize);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateProcessA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
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
            let __trace_record = if trace::enabled("kernel32/process") {
                trace::Record::new(
                    kernel32::CreateProcessA_pos,
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
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::CreateProcessA(
                sys,
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
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateProcessW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpApplicationName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpCommandLine = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpProcessAttributes =
                <Option<&mut SECURITY_ATTRIBUTES>>::from_stack(mem, stack_args + 8u32);
            let lpThreadAttributes =
                <Option<&mut SECURITY_ATTRIBUTES>>::from_stack(mem, stack_args + 12u32);
            let bInheritHandles = <bool>::from_stack(mem, stack_args + 16u32);
            let dwCreationFlags = <u32>::from_stack(mem, stack_args + 20u32);
            let lpEnvironment = <Option<&mut u32>>::from_stack(mem, stack_args + 24u32);
            let lpCurrentDirectory = <Option<&Str16>>::from_stack(mem, stack_args + 28u32);
            let lpStartupInfo = <Option<&mut STARTUPINFOW>>::from_stack(mem, stack_args + 32u32);
            let lpProcessInformation =
                <Option<&mut PROCESS_INFORMATION>>::from_stack(mem, stack_args + 36u32);
            let __trace_record = if trace::enabled("kernel32/process") {
                trace::Record::new(
                    kernel32::CreateProcessW_pos,
                    "kernel32/process",
                    "CreateProcessW",
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
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::CreateProcessW(
                sys,
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
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateThread(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn> + '_>> {
        unsafe {
            let mem = sys.mem().detach();
            let lpThreadAttributes = <u32>::from_stack(mem, stack_args + 0u32);
            let dwStackSize = <u32>::from_stack(mem, stack_args + 4u32);
            let lpStartAddress = <u32>::from_stack(mem, stack_args + 8u32);
            let lpParameter = <u32>::from_stack(mem, stack_args + 12u32);
            let dwCreationFlags = <u32>::from_stack(mem, stack_args + 16u32);
            let lpThreadId = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("kernel32/thread") {
                trace::Record::new(
                    kernel32::CreateThread_pos,
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
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = kernel32::CreateThread(
                    &mut *(sys.machine() as *mut crate::Machine),
                    lpThreadAttributes,
                    dwStackSize,
                    lpStartAddress,
                    lpParameter,
                    dwCreationFlags,
                    lpThreadId,
                )
                .await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn DebugBreak(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(kernel32::DebugBreak_pos, "kernel32/misc", "DebugBreak", &[])
                    .enter()
            } else {
                None
            };
            let result = kernel32::DebugBreak(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DecodePointer(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let ptr = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::DecodePointer_pos,
                    "kernel32/misc",
                    "DecodePointer",
                    &[("ptr", &ptr)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::DecodePointer(sys, ptr);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DeleteCriticalSection(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/sync/critical_section") {
                trace::Record::new(
                    kernel32::DeleteCriticalSection_pos,
                    "kernel32/sync/critical_section",
                    "DeleteCriticalSection",
                    &[("lpCriticalSection", &lpCriticalSection)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::DeleteCriticalSection(sys, lpCriticalSection);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DeleteFileA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/fs") {
                trace::Record::new(
                    kernel32::DeleteFileA_pos,
                    "kernel32/file/fs",
                    "DeleteFileA",
                    &[("lpFileName", &lpFileName)],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::DeleteFileA(&mut *(sys.machine() as *mut crate::Machine), lpFileName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DeleteFileW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/fs") {
                trace::Record::new(
                    kernel32::DeleteFileW_pos,
                    "kernel32/file/fs",
                    "DeleteFileW",
                    &[("lpFileName", &lpFileName)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::DeleteFileW(sys, lpFileName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DisableThreadLibraryCalls(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hLibModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/dll") {
                trace::Record::new(
                    kernel32::DisableThreadLibraryCalls_pos,
                    "kernel32/dll",
                    "DisableThreadLibraryCalls",
                    &[("hLibModule", &hLibModule)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::DisableThreadLibraryCalls(sys, hLibModule);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DuplicateHandle(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hSourceProcessHandle = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let hSourceHandle = <HANDLE<()>>::from_stack(mem, stack_args + 4u32);
            let hTargetProcessHandle = <HANDLE<()>>::from_stack(mem, stack_args + 8u32);
            let lpTargetHandle = <Option<&mut HANDLE<()>>>::from_stack(mem, stack_args + 12u32);
            let dwDesiredAccess = <u32>::from_stack(mem, stack_args + 16u32);
            let bInheritHandle = <bool>::from_stack(mem, stack_args + 20u32);
            let dwOptions = <u32>::from_stack(mem, stack_args + 24u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::DuplicateHandle_pos,
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
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::DuplicateHandle(
                sys,
                hSourceProcessHandle,
                hSourceHandle,
                hTargetProcessHandle,
                lpTargetHandle,
                dwDesiredAccess,
                bInheritHandle,
                dwOptions,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn EncodePointer(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let ptr = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::EncodePointer_pos,
                    "kernel32/misc",
                    "EncodePointer",
                    &[("ptr", &ptr)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::EncodePointer(sys, ptr);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn EnterCriticalSection(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/sync/critical_section") {
                trace::Record::new(
                    kernel32::EnterCriticalSection_pos,
                    "kernel32/sync/critical_section",
                    "EnterCriticalSection",
                    &[("lpCriticalSection", &lpCriticalSection)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::EnterCriticalSection(sys, lpCriticalSection);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn EnumSystemLocalesA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpLocaleEnumProc = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::EnumSystemLocalesA_pos,
                    "kernel32/nls",
                    "EnumSystemLocalesA",
                    &[
                        ("lpLocaleEnumProc", &lpLocaleEnumProc),
                        ("dwFlags", &dwFlags),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::EnumSystemLocalesA(sys, lpLocaleEnumProc, dwFlags);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ExitProcess(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let uExitCode = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::ExitProcess_pos,
                    "kernel32/misc",
                    "ExitProcess",
                    &[("uExitCode", &uExitCode)],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::ExitProcess(&mut *(sys.machine() as *mut crate::Machine), uExitCode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ExitThread(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let dwExitCode = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/thread") {
                trace::Record::new(
                    kernel32::ExitThread_pos,
                    "kernel32/thread",
                    "ExitThread",
                    &[("dwExitCode", &dwExitCode)],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::ExitThread(&mut *(sys.machine() as *mut crate::Machine), dwExitCode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FileTimeToDosDateTime(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 0u32);
            let lpFatDate = <Option<&mut u16>>::from_stack(mem, stack_args + 4u32);
            let lpFatTime = <Option<&mut u16>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::FileTimeToDosDateTime_pos,
                    "kernel32/time",
                    "FileTimeToDosDateTime",
                    &[
                        ("lpFileTime", &lpFileTime),
                        ("lpFatDate", &lpFatDate),
                        ("lpFatTime", &lpFatTime),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::FileTimeToDosDateTime(sys, lpFileTime, lpFatDate, lpFatTime);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FileTimeToLocalFileTime(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 0u32);
            let lpLocalFileTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::FileTimeToLocalFileTime_pos,
                    "kernel32/time",
                    "FileTimeToLocalFileTime",
                    &[
                        ("lpFileTime", &lpFileTime),
                        ("lpLocalFileTime", &lpLocalFileTime),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::FileTimeToLocalFileTime(sys, lpFileTime, lpLocalFileTime);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FileTimeToSystemTime(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 0u32);
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::FileTimeToSystemTime_pos,
                    "kernel32/time",
                    "FileTimeToSystemTime",
                    &[("lpFileTime", &lpFileTime), ("lpSystemTime", &lpSystemTime)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::FileTimeToSystemTime(
                &mut *(sys.machine() as *mut crate::Machine),
                lpFileTime,
                lpSystemTime,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FindClose(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFindFile = <HFIND>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/find") {
                trace::Record::new(
                    kernel32::FindClose_pos,
                    "kernel32/file/find",
                    "FindClose",
                    &[("hFindFile", &hFindFile)],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::FindClose(&mut *(sys.machine() as *mut crate::Machine), hFindFile);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FindFirstFileA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpFindFileData =
                <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/find") {
                trace::Record::new(
                    kernel32::FindFirstFileA_pos,
                    "kernel32/file/find",
                    "FindFirstFileA",
                    &[
                        ("lpFileName", &lpFileName),
                        ("lpFindFileData", &lpFindFileData),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::FindFirstFileA(
                &mut *(sys.machine() as *mut crate::Machine),
                lpFileName,
                lpFindFileData,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FindFirstFileW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpFindFileData =
                <Option<&mut WIN32_FIND_DATAW>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/find") {
                trace::Record::new(
                    kernel32::FindFirstFileW_pos,
                    "kernel32/file/find",
                    "FindFirstFileW",
                    &[
                        ("lpFileName", &lpFileName),
                        ("lpFindFileData", &lpFindFileData),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::FindFirstFileW(sys, lpFileName, lpFindFileData);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FindNextFileA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFindFile = <HFIND>::from_stack(mem, stack_args + 0u32);
            let lpFindFileData =
                <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/find") {
                trace::Record::new(
                    kernel32::FindNextFileA_pos,
                    "kernel32/file/find",
                    "FindNextFileA",
                    &[
                        ("hFindFile", &hFindFile),
                        ("lpFindFileData", &lpFindFileData),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::FindNextFileA(
                &mut *(sys.machine() as *mut crate::Machine),
                hFindFile,
                lpFindFileData,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FindNextFileW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFindFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpFindFileData =
                <Option<&mut WIN32_FIND_DATAW>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/find") {
                trace::Record::new(
                    kernel32::FindNextFileW_pos,
                    "kernel32/file/find",
                    "FindNextFileW",
                    &[
                        ("hFindFile", &hFindFile),
                        ("lpFindFileData", &lpFindFileData),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::FindNextFileW(sys, hFindFile, lpFindFileData);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FindResourceA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let lpName = <ResourceKey<&str>>::from_stack(mem, stack_args + 4u32);
            let lpType = <ResourceKey<&str>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/resource") {
                trace::Record::new(
                    kernel32::FindResourceA_pos,
                    "kernel32/resource",
                    "FindResourceA",
                    &[
                        ("hModule", &hModule),
                        ("lpName", &lpName),
                        ("lpType", &lpType),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::FindResourceA(
                &mut *(sys.machine() as *mut crate::Machine),
                hModule,
                lpName,
                lpType,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FindResourceW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let lpName = <ResourceKey<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpType = <ResourceKey<&Str16>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/resource") {
                trace::Record::new(
                    kernel32::FindResourceW_pos,
                    "kernel32/resource",
                    "FindResourceW",
                    &[
                        ("hModule", &hModule),
                        ("lpName", &lpName),
                        ("lpType", &lpType),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::FindResourceW(
                &mut *(sys.machine() as *mut crate::Machine),
                hModule,
                lpName,
                lpType,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FlushFileBuffers(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/file") {
                trace::Record::new(
                    kernel32::FlushFileBuffers_pos,
                    "kernel32/file/file",
                    "FlushFileBuffers",
                    &[("hFile", &hFile)],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::FlushFileBuffers(&mut *(sys.machine() as *mut crate::Machine), hFile);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FormatMessageA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let dwFlags = <u32>::from_stack(mem, stack_args + 0u32);
            let lpSource = <u32>::from_stack(mem, stack_args + 4u32);
            let dwMessageId = <u32>::from_stack(mem, stack_args + 8u32);
            let dwLanguageId = <u32>::from_stack(mem, stack_args + 12u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 16u32);
            let nSize = <u32>::from_stack(mem, stack_args + 20u32);
            let args = <u32>::from_stack(mem, stack_args + 24u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::FormatMessageA_pos,
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
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::FormatMessageA(
                &mut *(sys.machine() as *mut crate::Machine),
                dwFlags,
                lpSource,
                dwMessageId,
                dwLanguageId,
                lpBuffer,
                nSize,
                args,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FormatMessageW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let dwFlags = <Result<FormatMessageFlags, u32>>::from_stack(mem, stack_args + 0u32);
            let lpSource = <u32>::from_stack(mem, stack_args + 4u32);
            let dwMessageId = <u32>::from_stack(mem, stack_args + 8u32);
            let dwLanguageId = <u32>::from_stack(mem, stack_args + 12u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 16u32);
            let nSize = <u32>::from_stack(mem, stack_args + 20u32);
            let args = <u32>::from_stack(mem, stack_args + 24u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::FormatMessageW_pos,
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
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::FormatMessageW(
                &mut *(sys.machine() as *mut crate::Machine),
                dwFlags,
                lpSource,
                dwMessageId,
                dwLanguageId,
                lpBuffer,
                nSize,
                args,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FreeEnvironmentStringsA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let penv = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/env") {
                trace::Record::new(
                    kernel32::FreeEnvironmentStringsA_pos,
                    "kernel32/env",
                    "FreeEnvironmentStringsA",
                    &[("penv", &penv)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::FreeEnvironmentStringsA(
                &mut *(sys.machine() as *mut crate::Machine),
                penv,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FreeEnvironmentStringsW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let penv = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/env") {
                trace::Record::new(
                    kernel32::FreeEnvironmentStringsW_pos,
                    "kernel32/env",
                    "FreeEnvironmentStringsW",
                    &[("penv", &penv)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::FreeEnvironmentStringsW(
                &mut *(sys.machine() as *mut crate::Machine),
                penv,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FreeLibrary(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hLibModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/dll") {
                trace::Record::new(
                    kernel32::FreeLibrary_pos,
                    "kernel32/dll",
                    "FreeLibrary",
                    &[("hLibModule", &hLibModule)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::FreeLibrary(sys, hLibModule);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FreeResource(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hResData = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/resource") {
                trace::Record::new(
                    kernel32::FreeResource_pos,
                    "kernel32/resource",
                    "FreeResource",
                    &[("hResData", &hResData)],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::FreeResource(&mut *(sys.machine() as *mut crate::Machine), hResData);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetACP(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(kernel32::GetACP_pos, "kernel32/nls", "GetACP", &[]).enter()
            } else {
                None
            };
            let result = kernel32::GetACP(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetCPInfo(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let _CodePage = <u32>::from_stack(mem, stack_args + 0u32);
            let _lpCPInfo = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::GetCPInfo_pos,
                    "kernel32/nls",
                    "GetCPInfo",
                    &[("CodePage", &_CodePage), ("lpCPInfo", &_lpCPInfo)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetCPInfo(sys, _CodePage, _lpCPInfo);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetCommandLineA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/command_line") {
                trace::Record::new(
                    kernel32::GetCommandLineA_pos,
                    "kernel32/command_line",
                    "GetCommandLineA",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetCommandLineA(&mut *(sys.machine() as *mut crate::Machine));
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetCommandLineW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/command_line") {
                trace::Record::new(
                    kernel32::GetCommandLineW_pos,
                    "kernel32/command_line",
                    "GetCommandLineW",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetCommandLineW(&mut *(sys.machine() as *mut crate::Machine));
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetConsoleMode(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hConsoleHandle = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpMode = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/console") {
                trace::Record::new(
                    kernel32::GetConsoleMode_pos,
                    "kernel32/console",
                    "GetConsoleMode",
                    &[("hConsoleHandle", &hConsoleHandle), ("lpMode", &lpMode)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetConsoleMode(sys, hConsoleHandle, lpMode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetConsoleOutputCP(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::GetConsoleOutputCP_pos,
                    "kernel32/nls",
                    "GetConsoleOutputCP",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetConsoleOutputCP(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetConsoleScreenBufferInfo(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let _hConsoleOutput = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let lpConsoleScreenBufferInfo =
                <Option<&mut CONSOLE_SCREEN_BUFFER_INFO>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/console") {
                trace::Record::new(
                    kernel32::GetConsoleScreenBufferInfo_pos,
                    "kernel32/console",
                    "GetConsoleScreenBufferInfo",
                    &[
                        ("hConsoleOutput", &_hConsoleOutput),
                        ("lpConsoleScreenBufferInfo", &lpConsoleScreenBufferInfo),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetConsoleScreenBufferInfo(
                sys,
                _hConsoleOutput,
                lpConsoleScreenBufferInfo,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetCurrentDirectoryA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let nBufferLength = <u32>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/fs") {
                trace::Record::new(
                    kernel32::GetCurrentDirectoryA_pos,
                    "kernel32/file/fs",
                    "GetCurrentDirectoryA",
                    &[("nBufferLength", &nBufferLength), ("lpBuffer", &lpBuffer)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetCurrentDirectoryA(
                &mut *(sys.machine() as *mut crate::Machine),
                nBufferLength,
                lpBuffer,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetCurrentDirectoryW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let nBufferLength = <u32>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/fs") {
                trace::Record::new(
                    kernel32::GetCurrentDirectoryW_pos,
                    "kernel32/file/fs",
                    "GetCurrentDirectoryW",
                    &[("nBufferLength", &nBufferLength), ("lpBuffer", &lpBuffer)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetCurrentDirectoryW(
                &mut *(sys.machine() as *mut crate::Machine),
                nBufferLength,
                lpBuffer,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetCurrentProcess(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/process") {
                trace::Record::new(
                    kernel32::GetCurrentProcess_pos,
                    "kernel32/process",
                    "GetCurrentProcess",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetCurrentProcess(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetCurrentProcessId(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::GetCurrentProcessId_pos,
                    "kernel32/misc",
                    "GetCurrentProcessId",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetCurrentProcessId(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetCurrentThread(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/thread") {
                trace::Record::new(
                    kernel32::GetCurrentThread_pos,
                    "kernel32/thread",
                    "GetCurrentThread",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetCurrentThread(&mut *(sys.machine() as *mut crate::Machine));
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetCurrentThreadId(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/thread") {
                trace::Record::new(
                    kernel32::GetCurrentThreadId_pos,
                    "kernel32/thread",
                    "GetCurrentThreadId",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetCurrentThreadId(&mut *(sys.machine() as *mut crate::Machine));
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetDiskFreeSpaceA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpRootPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpSectorsPerCluster = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let lpBytesPerSector = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let lpNumberOfFreeClusters = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpTotalNumberOfClusters = <Option<&mut u32>>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("kernel32/file/misc") {
                trace::Record::new(
                    kernel32::GetDiskFreeSpaceA_pos,
                    "kernel32/file/misc",
                    "GetDiskFreeSpaceA",
                    &[
                        ("lpRootPathName", &lpRootPathName),
                        ("lpSectorsPerCluster", &lpSectorsPerCluster),
                        ("lpBytesPerSector", &lpBytesPerSector),
                        ("lpNumberOfFreeClusters", &lpNumberOfFreeClusters),
                        ("lpTotalNumberOfClusters", &lpTotalNumberOfClusters),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetDiskFreeSpaceA(
                &mut *(sys.machine() as *mut crate::Machine),
                lpRootPathName,
                lpSectorsPerCluster,
                lpBytesPerSector,
                lpNumberOfFreeClusters,
                lpTotalNumberOfClusters,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetDriveTypeA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpRootPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/misc") {
                trace::Record::new(
                    kernel32::GetDriveTypeA_pos,
                    "kernel32/file/misc",
                    "GetDriveTypeA",
                    &[("lpRootPathName", &lpRootPathName)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetDriveTypeA(sys, lpRootPathName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetDriveTypeW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpRootPathName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/misc") {
                trace::Record::new(
                    kernel32::GetDriveTypeW_pos,
                    "kernel32/file/misc",
                    "GetDriveTypeW",
                    &[("lpRootPathName", &lpRootPathName)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetDriveTypeW(sys, lpRootPathName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetEnvironmentStrings(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/env") {
                trace::Record::new(
                    kernel32::GetEnvironmentStrings_pos,
                    "kernel32/env",
                    "GetEnvironmentStrings",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::GetEnvironmentStrings(&mut *(sys.machine() as *mut crate::Machine));
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetEnvironmentStringsW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/env") {
                trace::Record::new(
                    kernel32::GetEnvironmentStringsW_pos,
                    "kernel32/env",
                    "GetEnvironmentStringsW",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::GetEnvironmentStringsW(&mut *(sys.machine() as *mut crate::Machine));
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetEnvironmentVariableA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let name = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let buf = <ArrayOut<u8>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/env") {
                trace::Record::new(
                    kernel32::GetEnvironmentVariableA_pos,
                    "kernel32/env",
                    "GetEnvironmentVariableA",
                    &[("name", &name), ("buf", &buf)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetEnvironmentVariableA(sys, name, buf);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetEnvironmentVariableW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let name = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let buf = <ArrayOut<u16>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/env") {
                trace::Record::new(
                    kernel32::GetEnvironmentVariableW_pos,
                    "kernel32/env",
                    "GetEnvironmentVariableW",
                    &[("name", &name), ("buf", &buf)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetEnvironmentVariableW(sys, name, buf);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetExitCodeProcess(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hProcess = <HPROCESS>::from_stack(mem, stack_args + 0u32);
            let lpExitCode = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/process") {
                trace::Record::new(
                    kernel32::GetExitCodeProcess_pos,
                    "kernel32/process",
                    "GetExitCodeProcess",
                    &[("hProcess", &hProcess), ("lpExitCode", &lpExitCode)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetExitCodeProcess(sys, hProcess, lpExitCode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetFileAttributesA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/metadata") {
                trace::Record::new(
                    kernel32::GetFileAttributesA_pos,
                    "kernel32/file/metadata",
                    "GetFileAttributesA",
                    &[("lpFileName", &lpFileName)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetFileAttributesA(
                &mut *(sys.machine() as *mut crate::Machine),
                lpFileName,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetFileAttributesW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/metadata") {
                trace::Record::new(
                    kernel32::GetFileAttributesW_pos,
                    "kernel32/file/metadata",
                    "GetFileAttributesW",
                    &[("lpFileName", &lpFileName)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetFileAttributesW(sys, lpFileName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetFileInformationByHandle(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpFileInformation =
                <Option<&mut BY_HANDLE_FILE_INFORMATION>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/metadata") {
                trace::Record::new(
                    kernel32::GetFileInformationByHandle_pos,
                    "kernel32/file/metadata",
                    "GetFileInformationByHandle",
                    &[("hFile", &hFile), ("lpFileInformation", &lpFileInformation)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetFileInformationByHandle(
                &mut *(sys.machine() as *mut crate::Machine),
                hFile,
                lpFileInformation,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetFileSize(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpFileSizeHigh = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/metadata") {
                trace::Record::new(
                    kernel32::GetFileSize_pos,
                    "kernel32/file/metadata",
                    "GetFileSize",
                    &[("hFile", &hFile), ("lpFileSizeHigh", &lpFileSizeHigh)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetFileSize(
                &mut *(sys.machine() as *mut crate::Machine),
                hFile,
                lpFileSizeHigh,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetFileTime(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpCreationTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 4u32);
            let lpLastAccessTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 8u32);
            let lpLastWriteTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/file/metadata") {
                trace::Record::new(
                    kernel32::GetFileTime_pos,
                    "kernel32/file/metadata",
                    "GetFileTime",
                    &[
                        ("hFile", &hFile),
                        ("lpCreationTime", &lpCreationTime),
                        ("lpLastAccessTime", &lpLastAccessTime),
                        ("lpLastWriteTime", &lpLastWriteTime),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetFileTime(
                &mut *(sys.machine() as *mut crate::Machine),
                hFile,
                lpCreationTime,
                lpLastAccessTime,
                lpLastWriteTime,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetFileType(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/metadata") {
                trace::Record::new(
                    kernel32::GetFileType_pos,
                    "kernel32/file/metadata",
                    "GetFileType",
                    &[("hFile", &hFile)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetFileType(&mut *(sys.machine() as *mut crate::Machine), hFile);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetFullPathNameA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let nBufferLength = <u32>::from_stack(mem, stack_args + 4u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
            let lpFilePart = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/file/path") {
                trace::Record::new(
                    kernel32::GetFullPathNameA_pos,
                    "kernel32/file/path",
                    "GetFullPathNameA",
                    &[
                        ("lpFileName", &lpFileName),
                        ("nBufferLength", &nBufferLength),
                        ("lpBuffer", &lpBuffer),
                        ("lpFilePart", &lpFilePart),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetFullPathNameA(
                &mut *(sys.machine() as *mut crate::Machine),
                lpFileName,
                nBufferLength,
                lpBuffer,
                lpFilePart,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetFullPathNameW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let nBufferLength = <u32>::from_stack(mem, stack_args + 4u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
            let lpFilePart = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/file/path") {
                trace::Record::new(
                    kernel32::GetFullPathNameW_pos,
                    "kernel32/file/path",
                    "GetFullPathNameW",
                    &[
                        ("lpFileName", &lpFileName),
                        ("nBufferLength", &nBufferLength),
                        ("lpBuffer", &lpBuffer),
                        ("lpFilePart", &lpFilePart),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetFullPathNameW(
                &mut *(sys.machine() as *mut crate::Machine),
                lpFileName,
                nBufferLength,
                lpBuffer,
                lpFilePart,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetLastError(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::GetLastError_pos,
                    "kernel32/misc",
                    "GetLastError",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetLastError(&mut *(sys.machine() as *mut crate::Machine));
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetLocalTime(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::GetLocalTime_pos,
                    "kernel32/time",
                    "GetLocalTime",
                    &[("lpSystemTime", &lpSystemTime)],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::GetLocalTime(&mut *(sys.machine() as *mut crate::Machine), lpSystemTime);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetLocaleInfoA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let Locale = <u32>::from_stack(mem, stack_args + 0u32);
            let LCType = <u32>::from_stack(mem, stack_args + 4u32);
            let lpLCData = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let cchData = <i32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::GetLocaleInfoA_pos,
                    "kernel32/nls",
                    "GetLocaleInfoA",
                    &[
                        ("Locale", &Locale),
                        ("LCType", &LCType),
                        ("lpLCData", &lpLCData),
                        ("cchData", &cchData),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetLocaleInfoA(sys, Locale, LCType, lpLCData, cchData);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetLocaleInfoW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let Locale = <u32>::from_stack(mem, stack_args + 0u32);
            let LCType = <u32>::from_stack(mem, stack_args + 4u32);
            let lpLCData = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let cchData = <i32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::GetLocaleInfoW_pos,
                    "kernel32/nls",
                    "GetLocaleInfoW",
                    &[
                        ("Locale", &Locale),
                        ("LCType", &LCType),
                        ("lpLCData", &lpLCData),
                        ("cchData", &cchData),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetLocaleInfoW(sys, Locale, LCType, lpLCData, cchData);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetLogicalDrives(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/file/misc") {
                trace::Record::new(
                    kernel32::GetLogicalDrives_pos,
                    "kernel32/file/misc",
                    "GetLogicalDrives",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetLogicalDrives(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetModuleFileNameA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let filename = <ArrayOut<u8>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/dll") {
                trace::Record::new(
                    kernel32::GetModuleFileNameA_pos,
                    "kernel32/dll",
                    "GetModuleFileNameA",
                    &[("hModule", &hModule), ("filename", &filename)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetModuleFileNameA(
                &mut *(sys.machine() as *mut crate::Machine),
                hModule,
                filename,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetModuleFileNameW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let lpFilename = <u32>::from_stack(mem, stack_args + 4u32);
            let nSize = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/dll") {
                trace::Record::new(
                    kernel32::GetModuleFileNameW_pos,
                    "kernel32/dll",
                    "GetModuleFileNameW",
                    &[
                        ("hModule", &hModule),
                        ("lpFilename", &lpFilename),
                        ("nSize", &nSize),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetModuleFileNameW(
                &mut *(sys.machine() as *mut crate::Machine),
                hModule,
                lpFilename,
                nSize,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetModuleHandleA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpModuleName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/dll") {
                trace::Record::new(
                    kernel32::GetModuleHandleA_pos,
                    "kernel32/dll",
                    "GetModuleHandleA",
                    &[("lpModuleName", &lpModuleName)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetModuleHandleA(
                &mut *(sys.machine() as *mut crate::Machine),
                lpModuleName,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetModuleHandleExW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let dwFlags = <u32>::from_stack(mem, stack_args + 0u32);
            let lpModuleName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let hModule = <Option<&mut HMODULE>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/dll") {
                trace::Record::new(
                    kernel32::GetModuleHandleExW_pos,
                    "kernel32/dll",
                    "GetModuleHandleExW",
                    &[
                        ("dwFlags", &dwFlags),
                        ("lpModuleName", &lpModuleName),
                        ("hModule", &hModule),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetModuleHandleExW(
                &mut *(sys.machine() as *mut crate::Machine),
                dwFlags,
                lpModuleName,
                hModule,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetModuleHandleW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpModuleName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/dll") {
                trace::Record::new(
                    kernel32::GetModuleHandleW_pos,
                    "kernel32/dll",
                    "GetModuleHandleW",
                    &[("lpModuleName", &lpModuleName)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetModuleHandleW(
                &mut *(sys.machine() as *mut crate::Machine),
                lpModuleName,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetNumberOfConsoleInputEvents(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hConsoleInput = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let lpNumberOfEvents = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/console") {
                trace::Record::new(
                    kernel32::GetNumberOfConsoleInputEvents_pos,
                    "kernel32/console",
                    "GetNumberOfConsoleInputEvents",
                    &[
                        ("hConsoleInput", &hConsoleInput),
                        ("lpNumberOfEvents", &lpNumberOfEvents),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::GetNumberOfConsoleInputEvents(sys, hConsoleInput, lpNumberOfEvents);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetOEMCP(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(kernel32::GetOEMCP_pos, "kernel32/nls", "GetOEMCP", &[]).enter()
            } else {
                None
            };
            let result = kernel32::GetOEMCP(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetPrivateProfileIntA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpAppName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let nDefault = <u32>::from_stack(mem, stack_args + 8u32);
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/ini") {
                trace::Record::new(
                    kernel32::GetPrivateProfileIntA_pos,
                    "kernel32/ini",
                    "GetPrivateProfileIntA",
                    &[
                        ("lpAppName", &lpAppName),
                        ("lpKeyName", &lpKeyName),
                        ("nDefault", &nDefault),
                        ("lpFileName", &lpFileName),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::GetPrivateProfileIntA(sys, lpAppName, lpKeyName, nDefault, lpFileName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetPrivateProfileIntW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let nDefault = <u32>::from_stack(mem, stack_args + 8u32);
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/ini") {
                trace::Record::new(
                    kernel32::GetPrivateProfileIntW_pos,
                    "kernel32/ini",
                    "GetPrivateProfileIntW",
                    &[
                        ("lpAppName", &lpAppName),
                        ("lpKeyName", &lpKeyName),
                        ("nDefault", &nDefault),
                        ("lpFileName", &lpFileName),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::GetPrivateProfileIntW(sys, lpAppName, lpKeyName, nDefault, lpFileName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetPrivateProfileStringA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpAppName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let lpDefault = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let lpReturnedString = <Option<&str>>::from_stack(mem, stack_args + 12u32);
            let nSize = <u32>::from_stack(mem, stack_args + 16u32);
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("kernel32/ini") {
                trace::Record::new(
                    kernel32::GetPrivateProfileStringA_pos,
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
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetPrivateProfileStringA(
                sys,
                lpAppName,
                lpKeyName,
                lpDefault,
                lpReturnedString,
                nSize,
                lpFileName,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetPrivateProfileStringW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpDefault = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let lpReturnedString = <ArrayOut<u16>>::from_stack(mem, stack_args + 12u32);
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("kernel32/ini") {
                trace::Record::new(
                    kernel32::GetPrivateProfileStringW_pos,
                    "kernel32/ini",
                    "GetPrivateProfileStringW",
                    &[
                        ("lpAppName", &lpAppName),
                        ("lpKeyName", &lpKeyName),
                        ("lpDefault", &lpDefault),
                        ("lpReturnedString", &lpReturnedString),
                        ("lpFileName", &lpFileName),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetPrivateProfileStringW(
                sys,
                lpAppName,
                lpKeyName,
                lpDefault,
                lpReturnedString,
                lpFileName,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetProcAddress(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let lpProcName = <GetProcAddressArg>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/dll") {
                trace::Record::new(
                    kernel32::GetProcAddress_pos,
                    "kernel32/dll",
                    "GetProcAddress",
                    &[("hModule", &hModule), ("lpProcName", &lpProcName)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetProcAddress(
                &mut *(sys.machine() as *mut crate::Machine),
                hModule,
                lpProcName,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetProcessHeap(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::GetProcessHeap_pos,
                    "kernel32/memory",
                    "GetProcessHeap",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetProcessHeap(&mut *(sys.machine() as *mut crate::Machine));
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetProfileIntW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let nDefault = <i32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/ini") {
                trace::Record::new(
                    kernel32::GetProfileIntW_pos,
                    "kernel32/ini",
                    "GetProfileIntW",
                    &[
                        ("lpAppName", &lpAppName),
                        ("lpKeyName", &lpKeyName),
                        ("nDefault", &nDefault),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetProfileIntW(sys, lpAppName, lpKeyName, nDefault);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetProfileStringW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpDefault = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let lpReturnedString = <ArrayOut<u16>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/ini") {
                trace::Record::new(
                    kernel32::GetProfileStringW_pos,
                    "kernel32/ini",
                    "GetProfileStringW",
                    &[
                        ("lpAppName", &lpAppName),
                        ("lpKeyName", &lpKeyName),
                        ("lpDefault", &lpDefault),
                        ("lpReturnedString", &lpReturnedString),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::GetProfileStringW(sys, lpAppName, lpKeyName, lpDefault, lpReturnedString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetStartupInfoA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/dll") {
                trace::Record::new(
                    kernel32::GetStartupInfoA_pos,
                    "kernel32/dll",
                    "GetStartupInfoA",
                    &[("lpStartupInfo", &lpStartupInfo)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetStartupInfoA(sys, lpStartupInfo);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetStartupInfoW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/dll") {
                trace::Record::new(
                    kernel32::GetStartupInfoW_pos,
                    "kernel32/dll",
                    "GetStartupInfoW",
                    &[("lpStartupInfo", &lpStartupInfo)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetStartupInfoW(
                &mut *(sys.machine() as *mut crate::Machine),
                lpStartupInfo,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetStdHandle(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let nStdHandle = <Result<STD, i32>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/std") {
                trace::Record::new(
                    kernel32::GetStdHandle_pos,
                    "kernel32/file/std",
                    "GetStdHandle",
                    &[("nStdHandle", &nStdHandle)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetStdHandle(sys, nStdHandle);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetStringTypeA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let Locale = <LCID>::from_stack(mem, stack_args + 0u32);
            let dwInfoType = <u32>::from_stack(mem, stack_args + 4u32);
            let lpSrcStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cchSrc = <i32>::from_stack(mem, stack_args + 12u32);
            let lpCharType = <Option<&mut u32>>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::GetStringTypeA_pos,
                    "kernel32/nls",
                    "GetStringTypeA",
                    &[
                        ("Locale", &Locale),
                        ("dwInfoType", &dwInfoType),
                        ("lpSrcStr", &lpSrcStr),
                        ("cchSrc", &cchSrc),
                        ("lpCharType", &lpCharType),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::GetStringTypeA(sys, Locale, dwInfoType, lpSrcStr, cchSrc, lpCharType);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetStringTypeW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let dwInfoType = <u32>::from_stack(mem, stack_args + 0u32);
            let lpSrcStr = <u32>::from_stack(mem, stack_args + 4u32);
            let cchSrc = <i32>::from_stack(mem, stack_args + 8u32);
            let lpCharType = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::GetStringTypeW_pos,
                    "kernel32/nls",
                    "GetStringTypeW",
                    &[
                        ("dwInfoType", &dwInfoType),
                        ("lpSrcStr", &lpSrcStr),
                        ("cchSrc", &cchSrc),
                        ("lpCharType", &lpCharType),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetStringTypeW(sys, dwInfoType, lpSrcStr, cchSrc, lpCharType);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetSystemDirectoryA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpBuffer = <u32>::from_stack(mem, stack_args + 0u32);
            let uSize = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::GetSystemDirectoryA_pos,
                    "kernel32/misc",
                    "GetSystemDirectoryA",
                    &[("lpBuffer", &lpBuffer), ("uSize", &uSize)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetSystemDirectoryA(
                &mut *(sys.machine() as *mut crate::Machine),
                lpBuffer,
                uSize,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetSystemTime(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::GetSystemTime_pos,
                    "kernel32/time",
                    "GetSystemTime",
                    &[("lpSystemTime", &lpSystemTime)],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::GetSystemTime(&mut *(sys.machine() as *mut crate::Machine), lpSystemTime);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetSystemTimeAsFileTime(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpSystemTimeAsFileTime =
                <Option<&mut FILETIME>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::GetSystemTimeAsFileTime_pos,
                    "kernel32/time",
                    "GetSystemTimeAsFileTime",
                    &[("lpSystemTimeAsFileTime", &lpSystemTimeAsFileTime)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetSystemTimeAsFileTime(
                &mut *(sys.machine() as *mut crate::Machine),
                lpSystemTimeAsFileTime,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetThreadLocale(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::GetThreadLocale_pos,
                    "kernel32/nls",
                    "GetThreadLocale",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetThreadLocale(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetThreadPriority(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/thread") {
                trace::Record::new(
                    kernel32::GetThreadPriority_pos,
                    "kernel32/thread",
                    "GetThreadPriority",
                    &[("hThread", &hThread)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetThreadPriority(sys, hThread);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetTickCount(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::GetTickCount_pos,
                    "kernel32/time",
                    "GetTickCount",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetTickCount(&mut *(sys.machine() as *mut crate::Machine));
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetTimeZoneInformation(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpTimeZoneInformation =
                <Option<&mut TIME_ZONE_INFORMATION>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::GetTimeZoneInformation_pos,
                    "kernel32/time",
                    "GetTimeZoneInformation",
                    &[("lpTimeZoneInformation", &lpTimeZoneInformation)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetTimeZoneInformation(sys, lpTimeZoneInformation);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetUserDefaultLCID(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::GetUserDefaultLCID_pos,
                    "kernel32/nls",
                    "GetUserDefaultLCID",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetUserDefaultLCID(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetUserDefaultUILanguage(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::GetUserDefaultUILanguage_pos,
                    "kernel32/misc",
                    "GetUserDefaultUILanguage",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetUserDefaultUILanguage(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetVersion(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(kernel32::GetVersion_pos, "kernel32/misc", "GetVersion", &[])
                    .enter()
            } else {
                None
            };
            let result = kernel32::GetVersion(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetVersionExA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpVersionInformation =
                <Option<&mut OSVERSIONINFO>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::GetVersionExA_pos,
                    "kernel32/misc",
                    "GetVersionExA",
                    &[("lpVersionInformation", &lpVersionInformation)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetVersionExA(sys, lpVersionInformation);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetWindowsDirectoryA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpBuffer = <u32>::from_stack(mem, stack_args + 0u32);
            let uSize = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::GetWindowsDirectoryA_pos,
                    "kernel32/misc",
                    "GetWindowsDirectoryA",
                    &[("lpBuffer", &lpBuffer), ("uSize", &uSize)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GetWindowsDirectoryA(
                &mut *(sys.machine() as *mut crate::Machine),
                lpBuffer,
                uSize,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GlobalAddAtomA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::GlobalAddAtomA_pos,
                    "kernel32/misc",
                    "GlobalAddAtomA",
                    &[("lpString", &lpString)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GlobalAddAtomA(sys, lpString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GlobalAlloc(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let uFlags = <GMEM>::from_stack(mem, stack_args + 0u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::GlobalAlloc_pos,
                    "kernel32/memory",
                    "GlobalAlloc",
                    &[("uFlags", &uFlags), ("dwBytes", &dwBytes)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GlobalAlloc(
                &mut *(sys.machine() as *mut crate::Machine),
                uFlags,
                dwBytes,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GlobalFlags(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hMem = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::GlobalFlags_pos,
                    "kernel32/memory",
                    "GlobalFlags",
                    &[("hMem", &hMem)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GlobalFlags(sys, hMem);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GlobalFree(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hMem = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::GlobalFree_pos,
                    "kernel32/memory",
                    "GlobalFree",
                    &[("hMem", &hMem)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GlobalFree(&mut *(sys.machine() as *mut crate::Machine), hMem);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GlobalHandle(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let pMem = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::GlobalHandle_pos,
                    "kernel32/memory",
                    "GlobalHandle",
                    &[("pMem", &pMem)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GlobalHandle(sys, pMem);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GlobalLock(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hMem = <HGLOBAL>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::GlobalLock_pos,
                    "kernel32/memory",
                    "GlobalLock",
                    &[("hMem", &hMem)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GlobalLock(sys, hMem);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GlobalReAlloc(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hMem = <u32>::from_stack(mem, stack_args + 0u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 4u32);
            let uFlags = <GMEM>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::GlobalReAlloc_pos,
                    "kernel32/memory",
                    "GlobalReAlloc",
                    &[("hMem", &hMem), ("dwBytes", &dwBytes), ("uFlags", &uFlags)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GlobalReAlloc(
                &mut *(sys.machine() as *mut crate::Machine),
                hMem,
                dwBytes,
                uFlags,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GlobalUnlock(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hMem = <HGLOBAL>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::GlobalUnlock_pos,
                    "kernel32/memory",
                    "GlobalUnlock",
                    &[("hMem", &hMem)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::GlobalUnlock(sys, hMem);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn HeapAlloc(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <Result<HeapAllocFlags, u32>>::from_stack(mem, stack_args + 4u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::HeapAlloc_pos,
                    "kernel32/memory",
                    "HeapAlloc",
                    &[
                        ("hHeap", &hHeap),
                        ("dwFlags", &dwFlags),
                        ("dwBytes", &dwBytes),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::HeapAlloc(
                &mut *(sys.machine() as *mut crate::Machine),
                hHeap,
                dwFlags,
                dwBytes,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn HeapCompact(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::HeapCompact_pos,
                    "kernel32/memory",
                    "HeapCompact",
                    &[("hHeap", &hHeap), ("dwFlags", &dwFlags)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::HeapCompact(sys, hHeap, dwFlags);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn HeapCreate(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let flOptions = <Result<HeapCreateFlags, u32>>::from_stack(mem, stack_args + 0u32);
            let dwInitialSize = <u32>::from_stack(mem, stack_args + 4u32);
            let dwMaximumSize = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::HeapCreate_pos,
                    "kernel32/memory",
                    "HeapCreate",
                    &[
                        ("flOptions", &flOptions),
                        ("dwInitialSize", &dwInitialSize),
                        ("dwMaximumSize", &dwMaximumSize),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::HeapCreate(
                &mut *(sys.machine() as *mut crate::Machine),
                flOptions,
                dwInitialSize,
                dwMaximumSize,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn HeapDestroy(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::HeapDestroy_pos,
                    "kernel32/memory",
                    "HeapDestroy",
                    &[("hHeap", &hHeap)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::HeapDestroy(sys, hHeap);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn HeapFree(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::HeapFree_pos,
                    "kernel32/memory",
                    "HeapFree",
                    &[("hHeap", &hHeap), ("dwFlags", &dwFlags), ("lpMem", &lpMem)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::HeapFree(
                &mut *(sys.machine() as *mut crate::Machine),
                hHeap,
                dwFlags,
                lpMem,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn HeapReAlloc(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::HeapReAlloc_pos,
                    "kernel32/memory",
                    "HeapReAlloc",
                    &[
                        ("hHeap", &hHeap),
                        ("dwFlags", &dwFlags),
                        ("lpMem", &lpMem),
                        ("dwBytes", &dwBytes),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::HeapReAlloc(
                &mut *(sys.machine() as *mut crate::Machine),
                hHeap,
                dwFlags,
                lpMem,
                dwBytes,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn HeapSetInformation(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let HeapHandle = <u32>::from_stack(mem, stack_args + 0u32);
            let HeapInformationClass = <u32>::from_stack(mem, stack_args + 4u32);
            let HeapInformation = <u32>::from_stack(mem, stack_args + 8u32);
            let HeapInformationLength = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::HeapSetInformation_pos,
                    "kernel32/memory",
                    "HeapSetInformation",
                    &[
                        ("HeapHandle", &HeapHandle),
                        ("HeapInformationClass", &HeapInformationClass),
                        ("HeapInformation", &HeapInformation),
                        ("HeapInformationLength", &HeapInformationLength),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::HeapSetInformation(
                sys,
                HeapHandle,
                HeapInformationClass,
                HeapInformation,
                HeapInformationLength,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn HeapSize(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::HeapSize_pos,
                    "kernel32/memory",
                    "HeapSize",
                    &[("hHeap", &hHeap), ("dwFlags", &dwFlags), ("lpMem", &lpMem)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::HeapSize(
                &mut *(sys.machine() as *mut crate::Machine),
                hHeap,
                dwFlags,
                lpMem,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn HeapValidate(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMem = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::HeapValidate_pos,
                    "kernel32/memory",
                    "HeapValidate",
                    &[("hHeap", &hHeap), ("dwFlags", &dwFlags), ("lpMem", &lpMem)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::HeapValidate(sys, hHeap, dwFlags, lpMem);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn HeapWalk(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hHeap = <u32>::from_stack(mem, stack_args + 0u32);
            let lpEntry = <Option<&mut PROCESS_HEAP_ENTRY>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::HeapWalk_pos,
                    "kernel32/memory",
                    "HeapWalk",
                    &[("hHeap", &hHeap), ("lpEntry", &lpEntry)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::HeapWalk(sys, hHeap, lpEntry);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn InitOnceBeginInitialize(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let fPending = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let lpContext = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/sync/once") {
                trace::Record::new(
                    kernel32::InitOnceBeginInitialize_pos,
                    "kernel32/sync/once",
                    "InitOnceBeginInitialize",
                    &[
                        ("lpInitOnce", &lpInitOnce),
                        ("dwFlags", &dwFlags),
                        ("fPending", &fPending),
                        ("lpContext", &lpContext),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::InitOnceBeginInitialize(sys, lpInitOnce, dwFlags, fPending, lpContext);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn InitOnceComplete(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpContext = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/sync/once") {
                trace::Record::new(
                    kernel32::InitOnceComplete_pos,
                    "kernel32/sync/once",
                    "InitOnceComplete",
                    &[
                        ("lpInitOnce", &lpInitOnce),
                        ("dwFlags", &dwFlags),
                        ("lpContext", &lpContext),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::InitOnceComplete(sys, lpInitOnce, dwFlags, lpContext);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn InitializeCriticalSection(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/sync/critical_section") {
                trace::Record::new(
                    kernel32::InitializeCriticalSection_pos,
                    "kernel32/sync/critical_section",
                    "InitializeCriticalSection",
                    &[("lpCriticalSection", &lpCriticalSection)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::InitializeCriticalSection(sys, lpCriticalSection);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn InitializeCriticalSectionAndSpinCount(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSpinCount = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/sync/critical_section") {
                trace::Record::new(
                    kernel32::InitializeCriticalSectionAndSpinCount_pos,
                    "kernel32/sync/critical_section",
                    "InitializeCriticalSectionAndSpinCount",
                    &[
                        ("lpCriticalSection", &lpCriticalSection),
                        ("dwSpinCount", &dwSpinCount),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::InitializeCriticalSectionAndSpinCount(
                sys,
                lpCriticalSection,
                dwSpinCount,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn InitializeCriticalSectionEx(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSpinCount = <u32>::from_stack(mem, stack_args + 4u32);
            let flags = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/sync/critical_section") {
                trace::Record::new(
                    kernel32::InitializeCriticalSectionEx_pos,
                    "kernel32/sync/critical_section",
                    "InitializeCriticalSectionEx",
                    &[
                        ("lpCriticalSection", &lpCriticalSection),
                        ("dwSpinCount", &dwSpinCount),
                        ("flags", &flags),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::InitializeCriticalSectionEx(sys, lpCriticalSection, dwSpinCount, flags);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn InitializeSListHead(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let ListHead = <Option<&mut SLIST_HEADER>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::InitializeSListHead_pos,
                    "kernel32/misc",
                    "InitializeSListHead",
                    &[("ListHead", &ListHead)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::InitializeSListHead(sys, ListHead);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn InterlockedDecrement(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let addend = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/sync/interlocked") {
                trace::Record::new(
                    kernel32::InterlockedDecrement_pos,
                    "kernel32/sync/interlocked",
                    "InterlockedDecrement",
                    &[("addend", &addend)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::InterlockedDecrement(sys, addend);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn InterlockedIncrement(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let addend = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/sync/interlocked") {
                trace::Record::new(
                    kernel32::InterlockedIncrement_pos,
                    "kernel32/sync/interlocked",
                    "InterlockedIncrement",
                    &[("addend", &addend)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::InterlockedIncrement(sys, addend);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IsBadCodePtr(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpfn = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::IsBadCodePtr_pos,
                    "kernel32/memory",
                    "IsBadCodePtr",
                    &[("lpfn", &lpfn)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::IsBadCodePtr(sys, lpfn);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IsBadReadPtr(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lp = <u32>::from_stack(mem, stack_args + 0u32);
            let ucb = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::IsBadReadPtr_pos,
                    "kernel32/memory",
                    "IsBadReadPtr",
                    &[("lp", &lp), ("ucb", &ucb)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::IsBadReadPtr(sys, lp, ucb);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IsBadWritePtr(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lp = <u32>::from_stack(mem, stack_args + 0u32);
            let ucb = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::IsBadWritePtr_pos,
                    "kernel32/memory",
                    "IsBadWritePtr",
                    &[("lp", &lp), ("ucb", &ucb)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::IsBadWritePtr(sys, lp, ucb);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IsDBCSLeadByte(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let _TestChar = <u8>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::IsDBCSLeadByte_pos,
                    "kernel32/nls",
                    "IsDBCSLeadByte",
                    &[("TestChar", &_TestChar)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::IsDBCSLeadByte(sys, _TestChar);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IsDBCSLeadByteEx(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let _TestChar = <u8>::from_stack(mem, stack_args + 0u32);
            let _CodePage = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::IsDBCSLeadByteEx_pos,
                    "kernel32/nls",
                    "IsDBCSLeadByteEx",
                    &[("TestChar", &_TestChar), ("CodePage", &_CodePage)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::IsDBCSLeadByteEx(sys, _TestChar, _CodePage);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IsDebuggerPresent(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::IsDebuggerPresent_pos,
                    "kernel32/misc",
                    "IsDebuggerPresent",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::IsDebuggerPresent(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IsProcessorFeaturePresent(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let feature = <Result<ProcessorFeature, u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::IsProcessorFeaturePresent_pos,
                    "kernel32/misc",
                    "IsProcessorFeaturePresent",
                    &[("feature", &feature)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::IsProcessorFeaturePresent(sys, feature);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IsValidCodePage(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let CodePage = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::IsValidCodePage_pos,
                    "kernel32/nls",
                    "IsValidCodePage",
                    &[("CodePage", &CodePage)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::IsValidCodePage(sys, CodePage);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IsValidLocale(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let Locale = <u32>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::IsValidLocale_pos,
                    "kernel32/nls",
                    "IsValidLocale",
                    &[("Locale", &Locale), ("dwFlags", &dwFlags)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::IsValidLocale(sys, Locale, dwFlags);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LCMapStringA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let locale = <LCID>::from_stack(mem, stack_args + 0u32);
            let dwMapFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpSrcStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cchSrc = <i32>::from_stack(mem, stack_args + 12u32);
            let lpDestStr = <ArrayOut<u8>>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::LCMapStringA_pos,
                    "kernel32/nls",
                    "LCMapStringA",
                    &[
                        ("locale", &locale),
                        ("dwMapFlags", &dwMapFlags),
                        ("lpSrcStr", &lpSrcStr),
                        ("cchSrc", &cchSrc),
                        ("lpDestStr", &lpDestStr),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::LCMapStringA(sys, locale, dwMapFlags, lpSrcStr, cchSrc, lpDestStr);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LCMapStringW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let locale = <LCID>::from_stack(mem, stack_args + 0u32);
            let dwMapFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let lpSrcStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cchSrc = <i32>::from_stack(mem, stack_args + 12u32);
            let lpDestStr = <ArrayOut<u16>>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::LCMapStringW_pos,
                    "kernel32/nls",
                    "LCMapStringW",
                    &[
                        ("locale", &locale),
                        ("dwMapFlags", &dwMapFlags),
                        ("lpSrcStr", &lpSrcStr),
                        ("cchSrc", &cchSrc),
                        ("lpDestStr", &lpDestStr),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::LCMapStringW(sys, locale, dwMapFlags, lpSrcStr, cchSrc, lpDestStr);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LeaveCriticalSection(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/sync/critical_section") {
                trace::Record::new(
                    kernel32::LeaveCriticalSection_pos,
                    "kernel32/sync/critical_section",
                    "LeaveCriticalSection",
                    &[("lpCriticalSection", &lpCriticalSection)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::LeaveCriticalSection(sys, lpCriticalSection);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LoadLibraryA(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn> + '_>> {
        unsafe {
            let mem = sys.mem().detach();
            let filename = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/dll") {
                trace::Record::new(
                    kernel32::LoadLibraryA_pos,
                    "kernel32/dll",
                    "LoadLibraryA",
                    &[("filename", &filename)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result =
                    kernel32::LoadLibraryA(&mut *(sys.machine() as *mut crate::Machine), filename)
                        .await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn LoadLibraryExW(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn> + '_>> {
        unsafe {
            let mem = sys.mem().detach();
            let lpLibFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let hFile = <HFILE>::from_stack(mem, stack_args + 4u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/dll") {
                trace::Record::new(
                    kernel32::LoadLibraryExW_pos,
                    "kernel32/dll",
                    "LoadLibraryExW",
                    &[
                        ("lpLibFileName", &lpLibFileName),
                        ("hFile", &hFile),
                        ("dwFlags", &dwFlags),
                    ],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = kernel32::LoadLibraryExW(
                    &mut *(sys.machine() as *mut crate::Machine),
                    lpLibFileName,
                    hFile,
                    dwFlags,
                )
                .await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn LoadLibraryW(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn> + '_>> {
        unsafe {
            let mem = sys.mem().detach();
            let filename = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/dll") {
                trace::Record::new(
                    kernel32::LoadLibraryW_pos,
                    "kernel32/dll",
                    "LoadLibraryW",
                    &[("filename", &filename)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result =
                    kernel32::LoadLibraryW(&mut *(sys.machine() as *mut crate::Machine), filename)
                        .await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn LoadResource(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let hResInfo = <HRSRC>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/resource") {
                trace::Record::new(
                    kernel32::LoadResource_pos,
                    "kernel32/resource",
                    "LoadResource",
                    &[("hModule", &hModule), ("hResInfo", &hResInfo)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::LoadResource(
                &mut *(sys.machine() as *mut crate::Machine),
                hModule,
                hResInfo,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LocalAlloc(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let uFlags = <GMEM>::from_stack(mem, stack_args + 0u32);
            let dwBytes = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::LocalAlloc_pos,
                    "kernel32/memory",
                    "LocalAlloc",
                    &[("uFlags", &uFlags), ("dwBytes", &dwBytes)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::LocalAlloc(
                &mut *(sys.machine() as *mut crate::Machine),
                uFlags,
                dwBytes,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LocalFileTimeToFileTime(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpLocalFileTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 0u32);
            let lpFileTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::LocalFileTimeToFileTime_pos,
                    "kernel32/time",
                    "LocalFileTimeToFileTime",
                    &[
                        ("lpLocalFileTime", &lpLocalFileTime),
                        ("lpFileTime", &lpFileTime),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::LocalFileTimeToFileTime(sys, lpLocalFileTime, lpFileTime);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LocalFree(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hMem = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::LocalFree_pos,
                    "kernel32/memory",
                    "LocalFree",
                    &[("hMem", &hMem)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::LocalFree(&mut *(sys.machine() as *mut crate::Machine), hMem);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LockFile(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let dwFileOffsetLow = <u32>::from_stack(mem, stack_args + 4u32);
            let dwFileOffsetHigh = <u32>::from_stack(mem, stack_args + 8u32);
            let nNumberOfBytesToLockLow = <u32>::from_stack(mem, stack_args + 12u32);
            let nNumberOfBytesToLockHigh = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("kernel32/file/file") {
                trace::Record::new(
                    kernel32::LockFile_pos,
                    "kernel32/file/file",
                    "LockFile",
                    &[
                        ("hFile", &hFile),
                        ("dwFileOffsetLow", &dwFileOffsetLow),
                        ("dwFileOffsetHigh", &dwFileOffsetHigh),
                        ("nNumberOfBytesToLockLow", &nNumberOfBytesToLockLow),
                        ("nNumberOfBytesToLockHigh", &nNumberOfBytesToLockHigh),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::LockFile(
                sys,
                hFile,
                dwFileOffsetLow,
                dwFileOffsetHigh,
                nNumberOfBytesToLockLow,
                nNumberOfBytesToLockHigh,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LockResource(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hResData = <HRSRC>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/resource") {
                trace::Record::new(
                    kernel32::LockResource_pos,
                    "kernel32/resource",
                    "LockResource",
                    &[("hResData", &hResData)],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::LockResource(&mut *(sys.machine() as *mut crate::Machine), hResData);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn MoveFileA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpExistingFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpNewFileName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/fs") {
                trace::Record::new(
                    kernel32::MoveFileA_pos,
                    "kernel32/file/fs",
                    "MoveFileA",
                    &[
                        ("lpExistingFileName", &lpExistingFileName),
                        ("lpNewFileName", &lpNewFileName),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::MoveFileA(sys, lpExistingFileName, lpNewFileName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn MoveFileW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpExistingFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpNewFileName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/fs") {
                trace::Record::new(
                    kernel32::MoveFileW_pos,
                    "kernel32/file/fs",
                    "MoveFileW",
                    &[
                        ("lpExistingFileName", &lpExistingFileName),
                        ("lpNewFileName", &lpNewFileName),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::MoveFileW(sys, lpExistingFileName, lpNewFileName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn MulDiv(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let nNumber = <i32>::from_stack(mem, stack_args + 0u32);
            let nNumerator = <i32>::from_stack(mem, stack_args + 4u32);
            let nDenominator = <i32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::MulDiv_pos,
                    "kernel32/misc",
                    "MulDiv",
                    &[
                        ("nNumber", &nNumber),
                        ("nNumerator", &nNumerator),
                        ("nDenominator", &nDenominator),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::MulDiv(sys, nNumber, nNumerator, nDenominator);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn MultiByteToWideChar(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let CodePage = <Result<CP, u32>>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <Result<MB, u32>>::from_stack(mem, stack_args + 4u32);
            let lpMultiByteStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cbMultiByte = <i32>::from_stack(mem, stack_args + 12u32);
            let lpWideCharStr = <Option<ArrayOut<u16>>>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::MultiByteToWideChar_pos,
                    "kernel32/nls",
                    "MultiByteToWideChar",
                    &[
                        ("CodePage", &CodePage),
                        ("dwFlags", &dwFlags),
                        ("lpMultiByteStr", &lpMultiByteStr),
                        ("cbMultiByte", &cbMultiByte),
                        ("lpWideCharStr", &lpWideCharStr),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::MultiByteToWideChar(
                &mut *(sys.machine() as *mut crate::Machine),
                CodePage,
                dwFlags,
                lpMultiByteStr,
                cbMultiByte,
                lpWideCharStr,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn OpenFile(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpReOpenBuff = <Option<&mut OFSTRUCT>>::from_stack(mem, stack_args + 4u32);
            let uStyle = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/file/file") {
                trace::Record::new(
                    kernel32::OpenFile_pos,
                    "kernel32/file/file",
                    "OpenFile",
                    &[
                        ("lpFileName", &lpFileName),
                        ("lpReOpenBuff", &lpReOpenBuff),
                        ("uStyle", &uStyle),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::OpenFile(
                &mut *(sys.machine() as *mut crate::Machine),
                lpFileName,
                lpReOpenBuff,
                uStyle,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn OpenMutexA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let dwDesiredAccess = <u32>::from_stack(mem, stack_args + 0u32);
            let bInheritHandle = <bool>::from_stack(mem, stack_args + 4u32);
            let lpName = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/sync/mutex") {
                trace::Record::new(
                    kernel32::OpenMutexA_pos,
                    "kernel32/sync/mutex",
                    "OpenMutexA",
                    &[
                        ("dwDesiredAccess", &dwDesiredAccess),
                        ("bInheritHandle", &bInheritHandle),
                        ("lpName", &lpName),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::OpenMutexA(sys, dwDesiredAccess, bInheritHandle, lpName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn OutputDebugStringA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let msg = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::OutputDebugStringA_pos,
                    "kernel32/misc",
                    "OutputDebugStringA",
                    &[("msg", &msg)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::OutputDebugStringA(sys, msg);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn PeekConsoleInputA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hConsoleInput = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 4u32);
            let nLength = <u32>::from_stack(mem, stack_args + 8u32);
            let lpNumberOfEventsRead = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/console") {
                trace::Record::new(
                    kernel32::PeekConsoleInputA_pos,
                    "kernel32/console",
                    "PeekConsoleInputA",
                    &[
                        ("hConsoleInput", &hConsoleInput),
                        ("lpBuffer", &lpBuffer),
                        ("nLength", &nLength),
                        ("lpNumberOfEventsRead", &lpNumberOfEventsRead),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::PeekConsoleInputA(
                sys,
                hConsoleInput,
                lpBuffer,
                nLength,
                lpNumberOfEventsRead,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn PeekNamedPipe(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hNamedPipe = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let nBufferSize = <u32>::from_stack(mem, stack_args + 8u32);
            let lpBytesRead = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpTotalBytesAvail = <Option<&mut u32>>::from_stack(mem, stack_args + 16u32);
            let lpBytesLeftThisMessage = <Option<&mut u32>>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("kernel32/pipe") {
                trace::Record::new(
                    kernel32::PeekNamedPipe_pos,
                    "kernel32/pipe",
                    "PeekNamedPipe",
                    &[
                        ("hNamedPipe", &hNamedPipe),
                        ("lpBuffer", &lpBuffer),
                        ("nBufferSize", &nBufferSize),
                        ("lpBytesRead", &lpBytesRead),
                        ("lpTotalBytesAvail", &lpTotalBytesAvail),
                        ("lpBytesLeftThisMessage", &lpBytesLeftThisMessage),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::PeekNamedPipe(
                sys,
                hNamedPipe,
                lpBuffer,
                nBufferSize,
                lpBytesRead,
                lpTotalBytesAvail,
                lpBytesLeftThisMessage,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn PulseEvent(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hEvent = <HEVENT>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/sync/event") {
                trace::Record::new(
                    kernel32::PulseEvent_pos,
                    "kernel32/sync/event",
                    "PulseEvent",
                    &[("hEvent", &hEvent)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::PulseEvent(sys, hEvent);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn QueryPerformanceCounter(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpPerformanceCount =
                <Option<&mut LARGE_INTEGER>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::QueryPerformanceCounter_pos,
                    "kernel32/time",
                    "QueryPerformanceCounter",
                    &[("lpPerformanceCount", &lpPerformanceCount)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::QueryPerformanceCounter(
                &mut *(sys.machine() as *mut crate::Machine),
                lpPerformanceCount,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn QueryPerformanceFrequency(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFrequency = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::QueryPerformanceFrequency_pos,
                    "kernel32/time",
                    "QueryPerformanceFrequency",
                    &[("lpFrequency", &lpFrequency)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::QueryPerformanceFrequency(
                &mut *(sys.machine() as *mut crate::Machine),
                lpFrequency,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RaiseException(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let dwExceptionCode = <u32>::from_stack(mem, stack_args + 0u32);
            let dwExceptionFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let nNumberOfArguments = <u32>::from_stack(mem, stack_args + 8u32);
            let lpArguments = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::RaiseException_pos,
                    "kernel32/misc",
                    "RaiseException",
                    &[
                        ("dwExceptionCode", &dwExceptionCode),
                        ("dwExceptionFlags", &dwExceptionFlags),
                        ("nNumberOfArguments", &nNumberOfArguments),
                        ("lpArguments", &lpArguments),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::RaiseException(
                sys,
                dwExceptionCode,
                dwExceptionFlags,
                nNumberOfArguments,
                lpArguments,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ReadConsoleA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hConsoleInput = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <Option<&mut u8>>::from_stack(mem, stack_args + 4u32);
            let nNumberOfCharsToRead = <u32>::from_stack(mem, stack_args + 8u32);
            let lpNumberOfCharsRead = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let pInputControl =
                <Option<&mut CONSOLE_READCONSOLE_CONTROL>>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("kernel32/console") {
                trace::Record::new(
                    kernel32::ReadConsoleA_pos,
                    "kernel32/console",
                    "ReadConsoleA",
                    &[
                        ("hConsoleInput", &hConsoleInput),
                        ("lpBuffer", &lpBuffer),
                        ("nNumberOfCharsToRead", &nNumberOfCharsToRead),
                        ("lpNumberOfCharsRead", &lpNumberOfCharsRead),
                        ("pInputControl", &pInputControl),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::ReadConsoleA(
                sys,
                hConsoleInput,
                lpBuffer,
                nNumberOfCharsToRead,
                lpNumberOfCharsRead,
                pInputControl,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ReadConsoleInputA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hConsoleInput = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 4u32);
            let nLength = <u32>::from_stack(mem, stack_args + 8u32);
            let lpNumberOfEventsRead = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/console") {
                trace::Record::new(
                    kernel32::ReadConsoleInputA_pos,
                    "kernel32/console",
                    "ReadConsoleInputA",
                    &[
                        ("hConsoleInput", &hConsoleInput),
                        ("lpBuffer", &lpBuffer),
                        ("nLength", &nLength),
                        ("lpNumberOfEventsRead", &lpNumberOfEventsRead),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::ReadConsoleInputA(
                sys,
                hConsoleInput,
                lpBuffer,
                nLength,
                lpNumberOfEventsRead,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ReadFile(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <ArrayOut<u8>>::from_stack(mem, stack_args + 4u32);
            let lpNumberOfBytesRead = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpOverlapped = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("kernel32/file/file") {
                trace::Record::new(
                    kernel32::ReadFile_pos,
                    "kernel32/file/file",
                    "ReadFile",
                    &[
                        ("hFile", &hFile),
                        ("lpBuffer", &lpBuffer),
                        ("lpNumberOfBytesRead", &lpNumberOfBytesRead),
                        ("lpOverlapped", &lpOverlapped),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::ReadFile(
                &mut *(sys.machine() as *mut crate::Machine),
                hFile,
                lpBuffer,
                lpNumberOfBytesRead,
                lpOverlapped,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ReleaseSRWLockExclusive(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/sync/srw_lock") {
                trace::Record::new(
                    kernel32::ReleaseSRWLockExclusive_pos,
                    "kernel32/sync/srw_lock",
                    "ReleaseSRWLockExclusive",
                    &[("SRWLock", &SRWLock)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::ReleaseSRWLockExclusive(sys, SRWLock);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ReleaseSRWLockShared(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/sync/srw_lock") {
                trace::Record::new(
                    kernel32::ReleaseSRWLockShared_pos,
                    "kernel32/sync/srw_lock",
                    "ReleaseSRWLockShared",
                    &[("SRWLock", &SRWLock)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::ReleaseSRWLockShared(sys, SRWLock);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RemoveDirectoryA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/fs") {
                trace::Record::new(
                    kernel32::RemoveDirectoryA_pos,
                    "kernel32/file/fs",
                    "RemoveDirectoryA",
                    &[("lpPathName", &lpPathName)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::RemoveDirectoryA(
                &mut *(sys.machine() as *mut crate::Machine),
                lpPathName,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RemoveDirectoryW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpPathName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/fs") {
                trace::Record::new(
                    kernel32::RemoveDirectoryW_pos,
                    "kernel32/file/fs",
                    "RemoveDirectoryW",
                    &[("lpPathName", &lpPathName)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::RemoveDirectoryW(sys, lpPathName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ResetEvent(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hEvent = <HEVENT>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/sync/event") {
                trace::Record::new(
                    kernel32::ResetEvent_pos,
                    "kernel32/sync/event",
                    "ResetEvent",
                    &[("hEvent", &hEvent)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::ResetEvent(sys, hEvent);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ResumeThread(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/thread") {
                trace::Record::new(
                    kernel32::ResumeThread_pos,
                    "kernel32/thread",
                    "ResumeThread",
                    &[("hThread", &hThread)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::ResumeThread(sys, hThread);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RtlUnwind(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let TargetFrame = <u32>::from_stack(mem, stack_args + 0u32);
            let TargetIp = <u32>::from_stack(mem, stack_args + 4u32);
            let ExceptionRecord = <u32>::from_stack(mem, stack_args + 8u32);
            let ReturnValue = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::RtlUnwind_pos,
                    "kernel32/misc",
                    "RtlUnwind",
                    &[
                        ("TargetFrame", &TargetFrame),
                        ("TargetIp", &TargetIp),
                        ("ExceptionRecord", &ExceptionRecord),
                        ("ReturnValue", &ReturnValue),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::RtlUnwind(sys, TargetFrame, TargetIp, ExceptionRecord, ReturnValue);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetConsoleCtrlHandler(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let _handlerRoutine = <DWORD>::from_stack(mem, stack_args + 0u32);
            let _add = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/console") {
                trace::Record::new(
                    kernel32::SetConsoleCtrlHandler_pos,
                    "kernel32/console",
                    "SetConsoleCtrlHandler",
                    &[("handlerRoutine", &_handlerRoutine), ("add", &_add)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetConsoleCtrlHandler(sys, _handlerRoutine, _add);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetConsoleMode(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hConsoleHandle = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let dwMode = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/console") {
                trace::Record::new(
                    kernel32::SetConsoleMode_pos,
                    "kernel32/console",
                    "SetConsoleMode",
                    &[("hConsoleHandle", &hConsoleHandle), ("dwMode", &dwMode)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetConsoleMode(sys, hConsoleHandle, dwMode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetCurrentDirectoryA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/fs") {
                trace::Record::new(
                    kernel32::SetCurrentDirectoryA_pos,
                    "kernel32/file/fs",
                    "SetCurrentDirectoryA",
                    &[("lpPathName", &lpPathName)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetCurrentDirectoryA(sys, lpPathName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetCurrentDirectoryW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpPathName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/fs") {
                trace::Record::new(
                    kernel32::SetCurrentDirectoryW_pos,
                    "kernel32/file/fs",
                    "SetCurrentDirectoryW",
                    &[("lpPathName", &lpPathName)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetCurrentDirectoryW(sys, lpPathName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetEndOfFile(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/file") {
                trace::Record::new(
                    kernel32::SetEndOfFile_pos,
                    "kernel32/file/file",
                    "SetEndOfFile",
                    &[("hFile", &hFile)],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::SetEndOfFile(&mut *(sys.machine() as *mut crate::Machine), hFile);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetEnvironmentVariableA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let name = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let value = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/env") {
                trace::Record::new(
                    kernel32::SetEnvironmentVariableA_pos,
                    "kernel32/env",
                    "SetEnvironmentVariableA",
                    &[("name", &name), ("value", &value)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetEnvironmentVariableA(sys, name, value);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetEnvironmentVariableW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpValue = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/env") {
                trace::Record::new(
                    kernel32::SetEnvironmentVariableW_pos,
                    "kernel32/env",
                    "SetEnvironmentVariableW",
                    &[("lpName", &lpName), ("lpValue", &lpValue)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetEnvironmentVariableW(sys, lpName, lpValue);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetErrorMode(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let uMode = <SEM>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::SetErrorMode_pos,
                    "kernel32/misc",
                    "SetErrorMode",
                    &[("uMode", &uMode)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetErrorMode(sys, uMode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetEvent(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hEvent = <HEVENT>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/sync/event") {
                trace::Record::new(
                    kernel32::SetEvent_pos,
                    "kernel32/sync/event",
                    "SetEvent",
                    &[("hEvent", &hEvent)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetEvent(&mut *(sys.machine() as *mut crate::Machine), hEvent);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetFileAttributesA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let dwFileAttributes = <Result<FileAttribute, u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/metadata") {
                trace::Record::new(
                    kernel32::SetFileAttributesA_pos,
                    "kernel32/file/metadata",
                    "SetFileAttributesA",
                    &[
                        ("lpFileName", &lpFileName),
                        ("dwFileAttributes", &dwFileAttributes),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetFileAttributesA(
                &mut *(sys.machine() as *mut crate::Machine),
                lpFileName,
                dwFileAttributes,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetFileAttributesW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let dwFileAttributes = <Result<FileAttribute, u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/metadata") {
                trace::Record::new(
                    kernel32::SetFileAttributesW_pos,
                    "kernel32/file/metadata",
                    "SetFileAttributesW",
                    &[
                        ("lpFileName", &lpFileName),
                        ("dwFileAttributes", &dwFileAttributes),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetFileAttributesW(sys, lpFileName, dwFileAttributes);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetFilePointer(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lDistanceToMove = <i32>::from_stack(mem, stack_args + 4u32);
            let lpDistanceToMoveHigh = <Option<&mut i32>>::from_stack(mem, stack_args + 8u32);
            let dwMoveMethod = <Result<FILE, u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/file/metadata") {
                trace::Record::new(
                    kernel32::SetFilePointer_pos,
                    "kernel32/file/metadata",
                    "SetFilePointer",
                    &[
                        ("hFile", &hFile),
                        ("lDistanceToMove", &lDistanceToMove),
                        ("lpDistanceToMoveHigh", &lpDistanceToMoveHigh),
                        ("dwMoveMethod", &dwMoveMethod),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetFilePointer(
                &mut *(sys.machine() as *mut crate::Machine),
                hFile,
                lDistanceToMove,
                lpDistanceToMoveHigh,
                dwMoveMethod,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetFileTime(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpCreationTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 4u32);
            let lpLastAccessTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 8u32);
            let lpLastWriteTime = <Option<&FILETIME>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/file/metadata") {
                trace::Record::new(
                    kernel32::SetFileTime_pos,
                    "kernel32/file/metadata",
                    "SetFileTime",
                    &[
                        ("hFile", &hFile),
                        ("lpCreationTime", &lpCreationTime),
                        ("lpLastAccessTime", &lpLastAccessTime),
                        ("lpLastWriteTime", &lpLastWriteTime),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetFileTime(
                &mut *(sys.machine() as *mut crate::Machine),
                hFile,
                lpCreationTime,
                lpLastAccessTime,
                lpLastWriteTime,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetHandleCount(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let uNumber = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::SetHandleCount_pos,
                    "kernel32/misc",
                    "SetHandleCount",
                    &[("uNumber", &uNumber)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetHandleCount(sys, uNumber);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetLastError(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let dwErrCode = <Result<ERROR, u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::SetLastError_pos,
                    "kernel32/misc",
                    "SetLastError",
                    &[("dwErrCode", &dwErrCode)],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::SetLastError(&mut *(sys.machine() as *mut crate::Machine), dwErrCode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetLocalTime(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::SetLocalTime_pos,
                    "kernel32/time",
                    "SetLocalTime",
                    &[("lpSystemTime", &lpSystemTime)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetLocalTime(sys, lpSystemTime);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetPriorityClass(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hProcess = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let dwPriorityClass = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::SetPriorityClass_pos,
                    "kernel32/misc",
                    "SetPriorityClass",
                    &[
                        ("hProcess", &hProcess),
                        ("dwPriorityClass", &dwPriorityClass),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetPriorityClass(sys, hProcess, dwPriorityClass);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetStdHandle(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let nStdHandle = <Result<STD, i32>>::from_stack(mem, stack_args + 0u32);
            let hHandle = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/std") {
                trace::Record::new(
                    kernel32::SetStdHandle_pos,
                    "kernel32/file/std",
                    "SetStdHandle",
                    &[("nStdHandle", &nStdHandle), ("hHandle", &hHandle)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetStdHandle(sys, nStdHandle, hHandle);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetThreadDescription(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
            let lpThreadDescription = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/thread") {
                trace::Record::new(
                    kernel32::SetThreadDescription_pos,
                    "kernel32/thread",
                    "SetThreadDescription",
                    &[
                        ("hThread", &hThread),
                        ("lpThreadDescription", &lpThreadDescription),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetThreadDescription(sys, hThread, lpThreadDescription);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetThreadPriority(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
            let nPriority = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/thread") {
                trace::Record::new(
                    kernel32::SetThreadPriority_pos,
                    "kernel32/thread",
                    "SetThreadPriority",
                    &[("hThread", &hThread), ("nPriority", &nPriority)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetThreadPriority(sys, hThread, nPriority);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetThreadStackGuarantee(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let StackSizeInBytes = <Option<&mut u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/thread") {
                trace::Record::new(
                    kernel32::SetThreadStackGuarantee_pos,
                    "kernel32/thread",
                    "SetThreadStackGuarantee",
                    &[("StackSizeInBytes", &StackSizeInBytes)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetThreadStackGuarantee(sys, StackSizeInBytes);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetUnhandledExceptionFilter(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let _lpTopLevelExceptionFilter = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::SetUnhandledExceptionFilter_pos,
                    "kernel32/misc",
                    "SetUnhandledExceptionFilter",
                    &[("lpTopLevelExceptionFilter", &_lpTopLevelExceptionFilter)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SetUnhandledExceptionFilter(sys, _lpTopLevelExceptionFilter);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SizeofResource(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, stack_args + 0u32);
            let hResInfo = <HRSRC>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/resource") {
                trace::Record::new(
                    kernel32::SizeofResource_pos,
                    "kernel32/resource",
                    "SizeofResource",
                    &[("hModule", &hModule), ("hResInfo", &hResInfo)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SizeofResource(
                &mut *(sys.machine() as *mut crate::Machine),
                hModule,
                hResInfo,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn Sleep(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn> + '_>> {
        unsafe {
            let mem = sys.mem().detach();
            let dwMilliseconds = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::Sleep_pos,
                    "kernel32/time",
                    "Sleep",
                    &[("dwMilliseconds", &dwMilliseconds)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = kernel32::Sleep(sys, dwMilliseconds).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn SleepEx(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn> + '_>> {
        unsafe {
            let mem = sys.mem().detach();
            let dwMilliseconds = <u32>::from_stack(mem, stack_args + 0u32);
            let bAlertable = <bool>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::SleepEx_pos,
                    "kernel32/time",
                    "SleepEx",
                    &[
                        ("dwMilliseconds", &dwMilliseconds),
                        ("bAlertable", &bAlertable),
                    ],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = kernel32::SleepEx(sys, dwMilliseconds, bAlertable).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn SystemTimeToFileTime(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpSystemTime = <Option<&SYSTEMTIME>>::from_stack(mem, stack_args + 0u32);
            let lpFileTime = <Option<&mut FILETIME>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/time") {
                trace::Record::new(
                    kernel32::SystemTimeToFileTime_pos,
                    "kernel32/time",
                    "SystemTimeToFileTime",
                    &[("lpSystemTime", &lpSystemTime), ("lpFileTime", &lpFileTime)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::SystemTimeToFileTime(
                &mut *(sys.machine() as *mut crate::Machine),
                lpSystemTime,
                lpFileTime,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn TerminateProcess(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hProcess = <u32>::from_stack(mem, stack_args + 0u32);
            let uExitCode = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::TerminateProcess_pos,
                    "kernel32/misc",
                    "TerminateProcess",
                    &[("hProcess", &hProcess), ("uExitCode", &uExitCode)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::TerminateProcess(sys, hProcess, uExitCode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn TerminateThread(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, stack_args + 0u32);
            let dwExitCode = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/thread") {
                trace::Record::new(
                    kernel32::TerminateThread_pos,
                    "kernel32/thread",
                    "TerminateThread",
                    &[("hThread", &hThread), ("dwExitCode", &dwExitCode)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::TerminateThread(sys, hThread, dwExitCode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn TlsAlloc(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("kernel32/thread") {
                trace::Record::new(kernel32::TlsAlloc_pos, "kernel32/thread", "TlsAlloc", &[])
                    .enter()
            } else {
                None
            };
            let result = kernel32::TlsAlloc(&mut *(sys.machine() as *mut crate::Machine));
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn TlsFree(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/thread") {
                trace::Record::new(
                    kernel32::TlsFree_pos,
                    "kernel32/thread",
                    "TlsFree",
                    &[("dwTlsIndex", &dwTlsIndex)],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::TlsFree(&mut *(sys.machine() as *mut crate::Machine), dwTlsIndex);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn TlsGetValue(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/thread") {
                trace::Record::new(
                    kernel32::TlsGetValue_pos,
                    "kernel32/thread",
                    "TlsGetValue",
                    &[("dwTlsIndex", &dwTlsIndex)],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::TlsGetValue(&mut *(sys.machine() as *mut crate::Machine), dwTlsIndex);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn TlsSetValue(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, stack_args + 0u32);
            let lpTlsValue = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/thread") {
                trace::Record::new(
                    kernel32::TlsSetValue_pos,
                    "kernel32/thread",
                    "TlsSetValue",
                    &[("dwTlsIndex", &dwTlsIndex), ("lpTlsValue", &lpTlsValue)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::TlsSetValue(
                &mut *(sys.machine() as *mut crate::Machine),
                dwTlsIndex,
                lpTlsValue,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn TryAcquireSRWLockExclusive(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/sync/srw_lock") {
                trace::Record::new(
                    kernel32::TryAcquireSRWLockExclusive_pos,
                    "kernel32/sync/srw_lock",
                    "TryAcquireSRWLockExclusive",
                    &[("SRWLock", &SRWLock)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::TryAcquireSRWLockExclusive(sys, SRWLock);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn UnhandledExceptionFilter(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let _exceptionInfo = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/misc") {
                trace::Record::new(
                    kernel32::UnhandledExceptionFilter_pos,
                    "kernel32/misc",
                    "UnhandledExceptionFilter",
                    &[("exceptionInfo", &_exceptionInfo)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::UnhandledExceptionFilter(sys, _exceptionInfo);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn UnlockFile(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let dwFileOffsetLow = <u32>::from_stack(mem, stack_args + 4u32);
            let dwFileOffsetHigh = <u32>::from_stack(mem, stack_args + 8u32);
            let nNumberOfBytesToUnlockLow = <u32>::from_stack(mem, stack_args + 12u32);
            let nNumberOfBytesToUnlockHigh = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("kernel32/file/file") {
                trace::Record::new(
                    kernel32::UnlockFile_pos,
                    "kernel32/file/file",
                    "UnlockFile",
                    &[
                        ("hFile", &hFile),
                        ("dwFileOffsetLow", &dwFileOffsetLow),
                        ("dwFileOffsetHigh", &dwFileOffsetHigh),
                        ("nNumberOfBytesToUnlockLow", &nNumberOfBytesToUnlockLow),
                        ("nNumberOfBytesToUnlockHigh", &nNumberOfBytesToUnlockHigh),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::UnlockFile(
                sys,
                hFile,
                dwFileOffsetLow,
                dwFileOffsetHigh,
                nNumberOfBytesToUnlockLow,
                nNumberOfBytesToUnlockHigh,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn VirtualAlloc(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSize = <u32>::from_stack(mem, stack_args + 4u32);
            let flAllocationType = <Result<MEM, u32>>::from_stack(mem, stack_args + 8u32);
            let flProtec = <Result<PAGE, u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::VirtualAlloc_pos,
                    "kernel32/memory",
                    "VirtualAlloc",
                    &[
                        ("lpAddress", &lpAddress),
                        ("dwSize", &dwSize),
                        ("flAllocationType", &flAllocationType),
                        ("flProtec", &flProtec),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::VirtualAlloc(
                &mut *(sys.machine() as *mut crate::Machine),
                lpAddress,
                dwSize,
                flAllocationType,
                flProtec,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn VirtualFree(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSize = <u32>::from_stack(mem, stack_args + 4u32);
            let dwFreeType = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::VirtualFree_pos,
                    "kernel32/memory",
                    "VirtualFree",
                    &[
                        ("lpAddress", &lpAddress),
                        ("dwSize", &dwSize),
                        ("dwFreeType", &dwFreeType),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::VirtualFree(sys, lpAddress, dwSize, dwFreeType);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn VirtualProtect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
            let dwSize = <u32>::from_stack(mem, stack_args + 4u32);
            let flNewProtect = <u32>::from_stack(mem, stack_args + 8u32);
            let lpflOldProtect = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::VirtualProtect_pos,
                    "kernel32/memory",
                    "VirtualProtect",
                    &[
                        ("lpAddress", &lpAddress),
                        ("dwSize", &dwSize),
                        ("flNewProtect", &flNewProtect),
                        ("lpflOldProtect", &lpflOldProtect),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                kernel32::VirtualProtect(sys, lpAddress, dwSize, flNewProtect, lpflOldProtect);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn VirtualQuery(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpAddress = <u32>::from_stack(mem, stack_args + 0u32);
            let lpBuffer =
                <Option<&mut MEMORY_BASIC_INFORMATION>>::from_stack(mem, stack_args + 4u32);
            let dwLength = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/memory") {
                trace::Record::new(
                    kernel32::VirtualQuery_pos,
                    "kernel32/memory",
                    "VirtualQuery",
                    &[
                        ("lpAddress", &lpAddress),
                        ("lpBuffer", &lpBuffer),
                        ("dwLength", &dwLength),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::VirtualQuery(sys, lpAddress, lpBuffer, dwLength);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn WaitForMultipleObjects(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn> + '_>> {
        unsafe {
            let mem = sys.mem().detach();
            let nCount = <u32>::from_stack(mem, stack_args + 0u32);
            let lpHandles = <u32>::from_stack(mem, stack_args + 4u32);
            let bWaitAll = <bool>::from_stack(mem, stack_args + 8u32);
            let dwMilliseconds = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/sync/wait") {
                trace::Record::new(
                    kernel32::WaitForMultipleObjects_pos,
                    "kernel32/sync/wait",
                    "WaitForMultipleObjects",
                    &[
                        ("nCount", &nCount),
                        ("lpHandles", &lpHandles),
                        ("bWaitAll", &bWaitAll),
                        ("dwMilliseconds", &dwMilliseconds),
                    ],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = kernel32::WaitForMultipleObjects(
                    &mut *(sys.machine() as *mut crate::Machine),
                    nCount,
                    lpHandles,
                    bWaitAll,
                    dwMilliseconds,
                )
                .await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn WaitForSingleObject(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn> + '_>> {
        unsafe {
            let mem = sys.mem().detach();
            let handle = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let dwMilliseconds = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/sync/wait") {
                trace::Record::new(
                    kernel32::WaitForSingleObject_pos,
                    "kernel32/sync/wait",
                    "WaitForSingleObject",
                    &[("handle", &handle), ("dwMilliseconds", &dwMilliseconds)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = kernel32::WaitForSingleObject(
                    &mut *(sys.machine() as *mut crate::Machine),
                    handle,
                    dwMilliseconds,
                )
                .await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn WideCharToMultiByte(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let CodePage = <Result<CP, u32>>::from_stack(mem, stack_args + 0u32);
            let dwFlags = <Result<WC, u32>>::from_stack(mem, stack_args + 4u32);
            let lpWideCharStr = <u32>::from_stack(mem, stack_args + 8u32);
            let cchWideChar = <i32>::from_stack(mem, stack_args + 12u32);
            let lpMultiByteStr = <u32>::from_stack(mem, stack_args + 16u32);
            let cbMultiByte = <i32>::from_stack(mem, stack_args + 20u32);
            let lpDefaultChar = <Option<&mut u32>>::from_stack(mem, stack_args + 24u32);
            let lpUsedDefaultChar = <Option<&mut u32>>::from_stack(mem, stack_args + 28u32);
            let __trace_record = if trace::enabled("kernel32/nls") {
                trace::Record::new(
                    kernel32::WideCharToMultiByte_pos,
                    "kernel32/nls",
                    "WideCharToMultiByte",
                    &[
                        ("CodePage", &CodePage),
                        ("dwFlags", &dwFlags),
                        ("lpWideCharStr", &lpWideCharStr),
                        ("cchWideChar", &cchWideChar),
                        ("lpMultiByteStr", &lpMultiByteStr),
                        ("cbMultiByte", &cbMultiByte),
                        ("lpDefaultChar", &lpDefaultChar),
                        ("lpUsedDefaultChar", &lpUsedDefaultChar),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::WideCharToMultiByte(
                &mut *(sys.machine() as *mut crate::Machine),
                CodePage,
                dwFlags,
                lpWideCharStr,
                cchWideChar,
                lpMultiByteStr,
                cbMultiByte,
                lpDefaultChar,
                lpUsedDefaultChar,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn WriteConsoleA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hConsoleOutput = <HANDLE<()>>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <Array<u8>>::from_stack(mem, stack_args + 4u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpReserved = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("kernel32/console") {
                trace::Record::new(
                    kernel32::WriteConsoleA_pos,
                    "kernel32/console",
                    "WriteConsoleA",
                    &[
                        ("hConsoleOutput", &hConsoleOutput),
                        ("lpBuffer", &lpBuffer),
                        ("lpNumberOfCharsWritten", &lpNumberOfCharsWritten),
                        ("lpReserved", &lpReserved),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::WriteConsoleA(
                sys,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                lpReserved,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn WriteConsoleW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hConsoleOutput = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <Array<u16>>::from_stack(mem, stack_args + 4u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let _lpReserved = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("kernel32/console") {
                trace::Record::new(
                    kernel32::WriteConsoleW_pos,
                    "kernel32/console",
                    "WriteConsoleW",
                    &[
                        ("hConsoleOutput", &hConsoleOutput),
                        ("lpBuffer", &lpBuffer),
                        ("lpNumberOfCharsWritten", &lpNumberOfCharsWritten),
                        ("lpReserved", &_lpReserved),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::WriteConsoleW(
                &mut *(sys.machine() as *mut crate::Machine),
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                _lpReserved,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn WriteFile(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <Array<u8>>::from_stack(mem, stack_args + 4u32);
            let lpNumberOfBytesWritten = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpOverlapped = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("kernel32/file/file") {
                trace::Record::new(
                    kernel32::WriteFile_pos,
                    "kernel32/file/file",
                    "WriteFile",
                    &[
                        ("hFile", &hFile),
                        ("lpBuffer", &lpBuffer),
                        ("lpNumberOfBytesWritten", &lpNumberOfBytesWritten),
                        ("lpOverlapped", &lpOverlapped),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::WriteFile(
                &mut *(sys.machine() as *mut crate::Machine),
                hFile,
                lpBuffer,
                lpNumberOfBytesWritten,
                lpOverlapped,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn WritePrivateProfileStringA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpAppName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let lpFileName = <Option<&str>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("kernel32/ini") {
                trace::Record::new(
                    kernel32::WritePrivateProfileStringA_pos,
                    "kernel32/ini",
                    "WritePrivateProfileStringA",
                    &[
                        ("lpAppName", &lpAppName),
                        ("lpKeyName", &lpKeyName),
                        ("lpString", &lpString),
                        ("lpFileName", &lpFileName),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::WritePrivateProfileStringA(
                sys, lpAppName, lpKeyName, lpString, lpFileName,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn WriteProfileStringW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/ini") {
                trace::Record::new(
                    kernel32::WriteProfileStringW_pos,
                    "kernel32/ini",
                    "WriteProfileStringW",
                    &[
                        ("lpAppName", &lpAppName),
                        ("lpKeyName", &lpKeyName),
                        ("lpString", &lpString),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::WriteProfileStringW(sys, lpAppName, lpKeyName, lpString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _hread(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <ArrayOut<u8>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/file16") {
                trace::Record::new(
                    kernel32::_hread_pos,
                    "kernel32/file/file16",
                    "_hread",
                    &[("hFile", &hFile), ("lpBuffer", &lpBuffer)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::_hread(
                &mut *(sys.machine() as *mut crate::Machine),
                hFile,
                lpBuffer,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _lclose(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/file/file16") {
                trace::Record::new(
                    kernel32::_lclose_pos,
                    "kernel32/file/file16",
                    "_lclose",
                    &[("hFile", &hFile)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::_lclose(&mut *(sys.machine() as *mut crate::Machine), hFile);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _llseek(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lOffset = <i32>::from_stack(mem, stack_args + 4u32);
            let iOrigin = <i32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/file/file16") {
                trace::Record::new(
                    kernel32::_llseek_pos,
                    "kernel32/file/file16",
                    "_llseek",
                    &[
                        ("hFile", &hFile),
                        ("lOffset", &lOffset),
                        ("iOrigin", &iOrigin),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::_llseek(
                &mut *(sys.machine() as *mut crate::Machine),
                hFile,
                lOffset,
                iOrigin,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _lopen(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let iReadWrite = <i32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/file16") {
                trace::Record::new(
                    kernel32::_lopen_pos,
                    "kernel32/file/file16",
                    "_lopen",
                    &[("lpPathName", &lpPathName), ("iReadWrite", &iReadWrite)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::_lopen(
                &mut *(sys.machine() as *mut crate::Machine),
                lpPathName,
                iReadWrite,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn _lread(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hFile = <HFILE>::from_stack(mem, stack_args + 0u32);
            let lpBuffer = <ArrayOut<u8>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/file/file16") {
                trace::Record::new(
                    kernel32::_lread_pos,
                    "kernel32/file/file16",
                    "_lread",
                    &[("hFile", &hFile), ("lpBuffer", &lpBuffer)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::_lread(
                &mut *(sys.machine() as *mut crate::Machine),
                hFile,
                lpBuffer,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn lstrcatA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpString1 = <u32>::from_stack(mem, stack_args + 0u32);
            let lpString2 = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/libc") {
                trace::Record::new(
                    kernel32::lstrcatA_pos,
                    "kernel32/libc",
                    "lstrcatA",
                    &[("lpString1", &lpString1), ("lpString2", &lpString2)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::lstrcatA(sys, lpString1, lpString2);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn lstrcmpA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpString1 = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpString2 = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/libc") {
                trace::Record::new(
                    kernel32::lstrcmpA_pos,
                    "kernel32/libc",
                    "lstrcmpA",
                    &[("lpString1", &lpString1), ("lpString2", &lpString2)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::lstrcmpA(sys, lpString1, lpString2);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn lstrcmpiA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpString1 = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpString2 = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/libc") {
                trace::Record::new(
                    kernel32::lstrcmpiA_pos,
                    "kernel32/libc",
                    "lstrcmpiA",
                    &[("lpString1", &lpString1), ("lpString2", &lpString2)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::lstrcmpiA(sys, lpString1, lpString2);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn lstrcpyA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpString1 = <u32>::from_stack(mem, stack_args + 0u32);
            let lpString2 = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/libc") {
                trace::Record::new(
                    kernel32::lstrcpyA_pos,
                    "kernel32/libc",
                    "lstrcpyA",
                    &[("lpString1", &lpString1), ("lpString2", &lpString2)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::lstrcpyA(
                &mut *(sys.machine() as *mut crate::Machine),
                lpString1,
                lpString2,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn lstrcpyW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpString1 = <u32>::from_stack(mem, stack_args + 0u32);
            let lpString2 = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/libc") {
                trace::Record::new(
                    kernel32::lstrcpyW_pos,
                    "kernel32/libc",
                    "lstrcpyW",
                    &[("lpString1", &lpString1), ("lpString2", &lpString2)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::lstrcpyW(
                &mut *(sys.machine() as *mut crate::Machine),
                lpString1,
                lpString2,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn lstrcpynA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpString1 = <u32>::from_stack(mem, stack_args + 0u32);
            let lpString2 = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let iMaxLength = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("kernel32/libc") {
                trace::Record::new(
                    kernel32::lstrcpynA_pos,
                    "kernel32/libc",
                    "lstrcpynA",
                    &[
                        ("lpString1", &lpString1),
                        ("lpString2", &lpString2),
                        ("iMaxLength", &iMaxLength),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::lstrcpynA(sys, lpString1, lpString2, iMaxLength);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn lstrlenA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/libc") {
                trace::Record::new(
                    kernel32::lstrlenA_pos,
                    "kernel32/libc",
                    "lstrlenA",
                    &[("lpString", &lpString)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::lstrlenA(sys, lpString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn lstrlenW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/libc") {
                trace::Record::new(
                    kernel32::lstrlenW_pos,
                    "kernel32/libc",
                    "lstrlenW",
                    &[("lpString", &lpString)],
                )
                .enter()
            } else {
                None
            };
            let result = kernel32::lstrlenW(sys, lpString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn retrowin32_main(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn> + '_>> {
        unsafe {
            let mem = sys.mem().detach();
            let entry_point = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("kernel32/init") {
                trace::Record::new(
                    kernel32::retrowin32_main_pos,
                    "kernel32/init",
                    "retrowin32_main",
                    &[("entry_point", &entry_point)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = kernel32::retrowin32_main(
                    &mut *(sys.machine() as *mut crate::Machine),
                    entry_point,
                )
                .await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn retrowin32_thread_main(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn> + '_>> {
        unsafe {
            let mem = sys.mem().detach();
            let entry_point = <u32>::from_stack(mem, stack_args + 0u32);
            let param = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("kernel32/init") {
                trace::Record::new(
                    kernel32::retrowin32_thread_main_pos,
                    "kernel32/init",
                    "retrowin32_thread_main",
                    &[("entry_point", &entry_point), ("param", &param)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = kernel32::retrowin32_thread_main(
                    &mut *(sys.machine() as *mut crate::Machine),
                    entry_point,
                    param,
                )
                .await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
}
const SHIMS: [Shim; 245usize] = [
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
        name: "Beep",
        func: Handler::Sync(wrappers::Beep),
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
        name: "CreateDirectoryW",
        func: Handler::Sync(wrappers::CreateDirectoryW),
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
        name: "CreateFileMappingA",
        func: Handler::Sync(wrappers::CreateFileMappingA),
    },
    Shim {
        name: "CreateFileW",
        func: Handler::Sync(wrappers::CreateFileW),
    },
    Shim {
        name: "CreateMutexA",
        func: Handler::Sync(wrappers::CreateMutexA),
    },
    Shim {
        name: "CreatePipe",
        func: Handler::Sync(wrappers::CreatePipe),
    },
    Shim {
        name: "CreateProcessA",
        func: Handler::Sync(wrappers::CreateProcessA),
    },
    Shim {
        name: "CreateProcessW",
        func: Handler::Sync(wrappers::CreateProcessW),
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
        name: "DecodePointer",
        func: Handler::Sync(wrappers::DecodePointer),
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
        name: "DeleteFileW",
        func: Handler::Sync(wrappers::DeleteFileW),
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
        name: "EncodePointer",
        func: Handler::Sync(wrappers::EncodePointer),
    },
    Shim {
        name: "EnterCriticalSection",
        func: Handler::Sync(wrappers::EnterCriticalSection),
    },
    Shim {
        name: "EnumSystemLocalesA",
        func: Handler::Sync(wrappers::EnumSystemLocalesA),
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
        name: "FileTimeToDosDateTime",
        func: Handler::Sync(wrappers::FileTimeToDosDateTime),
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
        name: "FindFirstFileW",
        func: Handler::Sync(wrappers::FindFirstFileW),
    },
    Shim {
        name: "FindNextFileA",
        func: Handler::Sync(wrappers::FindNextFileA),
    },
    Shim {
        name: "FindNextFileW",
        func: Handler::Sync(wrappers::FindNextFileW),
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
        name: "FreeResource",
        func: Handler::Sync(wrappers::FreeResource),
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
        name: "GetConsoleOutputCP",
        func: Handler::Sync(wrappers::GetConsoleOutputCP),
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
        name: "GetCurrentDirectoryW",
        func: Handler::Sync(wrappers::GetCurrentDirectoryW),
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
        name: "GetDriveTypeW",
        func: Handler::Sync(wrappers::GetDriveTypeW),
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
        name: "GetFileAttributesW",
        func: Handler::Sync(wrappers::GetFileAttributesW),
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
        name: "GetLocaleInfoA",
        func: Handler::Sync(wrappers::GetLocaleInfoA),
    },
    Shim {
        name: "GetLocaleInfoW",
        func: Handler::Sync(wrappers::GetLocaleInfoW),
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
        name: "GetNumberOfConsoleInputEvents",
        func: Handler::Sync(wrappers::GetNumberOfConsoleInputEvents),
    },
    Shim {
        name: "GetOEMCP",
        func: Handler::Sync(wrappers::GetOEMCP),
    },
    Shim {
        name: "GetPrivateProfileIntA",
        func: Handler::Sync(wrappers::GetPrivateProfileIntA),
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
        name: "GetThreadLocale",
        func: Handler::Sync(wrappers::GetThreadLocale),
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
        name: "GetUserDefaultLCID",
        func: Handler::Sync(wrappers::GetUserDefaultLCID),
    },
    Shim {
        name: "GetUserDefaultUILanguage",
        func: Handler::Sync(wrappers::GetUserDefaultUILanguage),
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
        name: "GlobalAddAtomA",
        func: Handler::Sync(wrappers::GlobalAddAtomA),
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
        name: "GlobalHandle",
        func: Handler::Sync(wrappers::GlobalHandle),
    },
    Shim {
        name: "GlobalLock",
        func: Handler::Sync(wrappers::GlobalLock),
    },
    Shim {
        name: "GlobalReAlloc",
        func: Handler::Sync(wrappers::GlobalReAlloc),
    },
    Shim {
        name: "GlobalUnlock",
        func: Handler::Sync(wrappers::GlobalUnlock),
    },
    Shim {
        name: "HeapAlloc",
        func: Handler::Sync(wrappers::HeapAlloc),
    },
    Shim {
        name: "HeapCompact",
        func: Handler::Sync(wrappers::HeapCompact),
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
        name: "HeapWalk",
        func: Handler::Sync(wrappers::HeapWalk),
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
        name: "IsValidLocale",
        func: Handler::Sync(wrappers::IsValidLocale),
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
        func: Handler::Async(wrappers::LoadLibraryA),
    },
    Shim {
        name: "LoadLibraryExW",
        func: Handler::Async(wrappers::LoadLibraryExW),
    },
    Shim {
        name: "LoadLibraryW",
        func: Handler::Async(wrappers::LoadLibraryW),
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
        name: "LocalFileTimeToFileTime",
        func: Handler::Sync(wrappers::LocalFileTimeToFileTime),
    },
    Shim {
        name: "LocalFree",
        func: Handler::Sync(wrappers::LocalFree),
    },
    Shim {
        name: "LockFile",
        func: Handler::Sync(wrappers::LockFile),
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
        name: "MoveFileW",
        func: Handler::Sync(wrappers::MoveFileW),
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
        name: "OpenFile",
        func: Handler::Sync(wrappers::OpenFile),
    },
    Shim {
        name: "OpenMutexA",
        func: Handler::Sync(wrappers::OpenMutexA),
    },
    Shim {
        name: "OutputDebugStringA",
        func: Handler::Sync(wrappers::OutputDebugStringA),
    },
    Shim {
        name: "PeekConsoleInputA",
        func: Handler::Sync(wrappers::PeekConsoleInputA),
    },
    Shim {
        name: "PeekNamedPipe",
        func: Handler::Sync(wrappers::PeekNamedPipe),
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
        name: "ReadConsoleA",
        func: Handler::Sync(wrappers::ReadConsoleA),
    },
    Shim {
        name: "ReadConsoleInputA",
        func: Handler::Sync(wrappers::ReadConsoleInputA),
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
        name: "RemoveDirectoryW",
        func: Handler::Sync(wrappers::RemoveDirectoryW),
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
        name: "SetConsoleMode",
        func: Handler::Sync(wrappers::SetConsoleMode),
    },
    Shim {
        name: "SetCurrentDirectoryA",
        func: Handler::Sync(wrappers::SetCurrentDirectoryA),
    },
    Shim {
        name: "SetCurrentDirectoryW",
        func: Handler::Sync(wrappers::SetCurrentDirectoryW),
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
        name: "SetEnvironmentVariableW",
        func: Handler::Sync(wrappers::SetEnvironmentVariableW),
    },
    Shim {
        name: "SetErrorMode",
        func: Handler::Sync(wrappers::SetErrorMode),
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
        name: "SetFileAttributesW",
        func: Handler::Sync(wrappers::SetFileAttributesW),
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
        name: "SetLocalTime",
        func: Handler::Sync(wrappers::SetLocalTime),
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
        name: "SleepEx",
        func: Handler::Async(wrappers::SleepEx),
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
        name: "TerminateThread",
        func: Handler::Sync(wrappers::TerminateThread),
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
        name: "UnlockFile",
        func: Handler::Sync(wrappers::UnlockFile),
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
        name: "_hread",
        func: Handler::Sync(wrappers::_hread),
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
        name: "lstrcatA",
        func: Handler::Sync(wrappers::lstrcatA),
    },
    Shim {
        name: "lstrcmpA",
        func: Handler::Sync(wrappers::lstrcmpA),
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
        name: "lstrcpynA",
        func: Handler::Sync(wrappers::lstrcpynA),
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
