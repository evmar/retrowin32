//! Debugger functions like "read a nul-terminated string from process memory".

const std = @import("std");
const windows = std.os.windows;
const HANDLE = windows.HANDLE;
const winapi = @import("./winapi.zig");

/// Patch a byte in a debugee's memory.
pub fn patchProcess(proc: HANDLE, addr: u32, patch: u8) !void {
    if (winapi.WriteProcessMemory(proc, addr, @ptrCast([*]const u8, &patch), 1, null) == 0) {
        winapi.logWindowsErr("WriteProcessMemory");
        return error.WindowsFailure;
    }

    if (winapi.FlushInstructionCache(proc, addr, 1) == 0) {
        winapi.logWindowsErr("FlushInstructionCache");
        return error.WindowsFailure;
    }
}

/// Install a breakpoint in a debugee's memory, returning the code that was patched over.
pub fn installBreakPoint(proc: HANDLE, addr: u32) !u8 {
    var prev: u8 = undefined;
    if (winapi.ReadProcessMemory(proc, addr, @ptrCast([*]u8, &prev), 1, null) == 0) {
        winapi.logWindowsErr("ReadProcessMemory");
        return error.WindowsFailure;
    }

    const int3: u8 = 0xcc;
    try patchProcess(proc, addr, int3);

    return prev;
}

/// Read a nul-terminated string from a debugee's memory.
pub fn readString(proc: HANDLE, addr: u32, unicode: bool, buf: []u8) ![]u8 {
    var len: u32 = 0;
    var ch: u16 = undefined;
    const size: u32 = if (unicode) 2 else 1;
    while (len < buf.len) {
        if (winapi.ReadProcessMemory(proc, addr + (len * size), @ptrCast([*]u8, &ch), size, null) == 0) {
            winapi.logWindowsErr("ReadProcessMemory");
            return error.WindowsFailure;
        }
        if (ch == 0) {
            return buf[0..len];
        }
        if (ch > 0xFF) {
            return error.UnicodeString;
        }
        buf[len] = @intCast(u8, ch);
        len += 1;
    }
    return buf;
}

/// Spawn a subprocess ready for debugging.
pub fn startProcess(cmd: []const u8) !winapi.PROCESS_INFORMATION {
    var cmdbuf: [128:0]u8 = undefined;
    const cmdz = try std.fmt.bufPrintZ(&cmdbuf, "{s}", .{cmd});
    var startup = std.mem.zeroes(winapi.STARTUPINFOA);
    startup.cb = @sizeOf(winapi.STARTUPINFOA);
    var proc = std.mem.zeroes(winapi.PROCESS_INFORMATION);
    if (winapi.CreateProcessA(
        null,
        cmdz.ptr,
        null,
        null,
        @boolToInt(false),
        winapi.PROCESS_CREATION_FLAGS.DEBUG_ONLY_THIS_PROCESS,
        null,
        null,
        &startup,
        &proc,
    ) == 0) {
        winapi.logWindowsErr("CreateProcessA");
        return error.WindowsFailure;
    }
    return proc;
}
