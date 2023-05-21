const std = @import("std");
const windows = std.os.windows;

const BOOL = windows.BOOL;
const DWORD = windows.DWORD;
const HANDLE = windows.HANDLE;
const WINAPI = windows.WINAPI;

pub const DEBUG_EVENT_CODE = enum(DWORD) {
    EXCEPTION_DEBUG_EVENT = 1,
    CREATE_THREAD_DEBUG_EVENT = 2,
    CREATE_PROCESS_DEBUG_EVENT = 3,
    EXIT_THREAD_DEBUG_EVENT = 4,
    EXIT_PROCESS_DEBUG_EVENT = 5,
    LOAD_DLL_DEBUG_EVENT = 6,
    UNLOAD_DLL_DEBUG_EVENT = 7,
    OUTPUT_DEBUG_STRING_EVENT = 8,
    RIP_EVENT = 9,
};

pub const CREATE_PROCESS_DEBUG_INFO = extern struct {
    hFile: HANDLE,
    hProcess: HANDLE,
    hThread: HANDLE,
    lpBaseOfImage: ?*anyopaque,
    dwDebugInfoFileOffset: u32,
    nDebugInfoSize: u32,
    lpThreadLocalBase: ?*anyopaque,
    lpStartAddress: ?*anyopaque,
    lpImageName: ?*anyopaque,
    fUnicode: u16,
};

pub const LOAD_DLL_DEBUG_INFO = extern struct {
    hFile: HANDLE,
    lpBaseOfDll: u32,
    dwDebugInfoFileOffset: u32,
    nDebugInfoSize: u32,
    lpImageName: u32,
    fUnicode: u16,
};

pub const OUTPUT_DEBUG_STRING_INFO = extern struct {
    lpDebugStringData: u32,
    fUnicode: u16,
    nDebugStringLength: u16,
};

pub const DEBUG_EVENT = extern struct {
    dwDebugEventCode: DEBUG_EVENT_CODE,
    dwProcessId: DWORD,
    dwThreadId: DWORD,
    u: extern union {
        // EXCEPTION_DEBUG_INFO      Exception;
        // CREATE_THREAD_DEBUG_INFO  CreateThread;
        CreateProcessInfo: CREATE_PROCESS_DEBUG_INFO,
        // CREATE_PROCESS_DEBUG_INFO CreateProcessInfo;
        // EXIT_THREAD_DEBUG_INFO    ExitThread;
        // EXIT_PROCESS_DEBUG_INFO   ExitProcess;
        LoadDll: LOAD_DLL_DEBUG_INFO,
        // UNLOAD_DLL_DEBUG_INFO     UnloadDll;
        DebugString: OUTPUT_DEBUG_STRING_INFO,
        // RIP_INFO                  RipInfo;

        // TODO: remove once we define all the other fields
        ensure_large_enough: [100]u32,
    },
};

pub extern "kernel32" fn WaitForDebugEvent(
    lpDebugEvent: *DEBUG_EVENT,
    dwMilliseconds: DWORD,
) callconv(windows.WINAPI) BOOL;

pub const ContinueDebugEventStatus = enum(DWORD) {
    DBG_CONTINUE = 0x00010002,
};

pub extern "kernel32" fn ContinueDebugEvent(
    dwProcessId: DWORD,
    dwThreadId: DWORD,
    dwContinueStatus: ContinueDebugEventStatus,
) callconv(windows.WINAPI) BOOL;

pub const PROCESS_CREATION_FLAGS = enum(u32) {
    DEBUG_ONLY_THIS_PROCESS = 2,
};

pub const STARTUPINFOA = extern struct {
    cb: u32,
    lpReserved: ?[*:0]const u8,
    lpDesktop: ?[*:0]const u8,
    lpTitle: ?[*:0]const u8,
    dwX: u32,
    dwY: u32,
    dwXSize: u32,
    dwYSize: u32,
    dwXCountChars: u32,
    dwYCountChars: u32,
    dwFillAttribute: u32,
    dwFlags: u32,
    wShowWindow: u16,
    cbReserved2: u16,
    lpReserved2: ?*u8,
    hStdInput: ?HANDLE,
    hStdOutput: ?HANDLE,
    hStdError: ?HANDLE,
};

pub const PROCESS_INFORMATION = extern struct {
    hProcess: HANDLE,
    hThread: ?HANDLE,
    dwProcessId: u32,
    dwThreadId: u32,
};

pub extern "kernel32" fn CreateProcessA(
    lpApplicationName: ?[*:0]const u8,
    lpCommandLine: ?[*:0]const u8,
    lpProcessAttributes: ?*anyopaque,
    lpThreadAttributes: ?*anyopaque,
    bInheritHandles: BOOL,
    dwCreationFlags: PROCESS_CREATION_FLAGS,
    lpEnvironment: ?*anyopaque,
    lpCurrentDirectory: ?[*:0]const u8,
    lpStartupInfo: ?*STARTUPINFOA,
    lpProcessInformation: ?*PROCESS_INFORMATION,
) callconv(windows.WINAPI) BOOL;

pub extern "kernel32" fn ReadProcessMemory(
    hProcess: HANDLE,
    lpBaseAddress: u32,
    lpBuffer: [*]u8,
    nSize: u32,
    lpNumberOfBytesRead: ?*u32,
) callconv(windows.WINAPI) BOOL;

pub extern "kernel32" fn WriteProcessMemory(
    hProcess: HANDLE,
    lpBaseAddress: u32,
    lpBuffer: [*]u8,
    nSize: u32,
    lpNumberOfBytesWritten: ?*u32,
) callconv(windows.WINAPI) BOOL;