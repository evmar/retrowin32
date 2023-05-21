const std = @import("std");

const windows = std.os.windows;
const winapi = @import("./winapi.zig");
const HANDLE = windows.HANDLE;

// Declaring this struct affects the default log level(!)
pub const std_options = struct {
    pub const log_level = .info;
};

fn logWindowsErr(call: []const u8) void {
    // Logging GetLastError as an enum adds 100kb(!) to the binary size.
    const code = @enumToInt(windows.kernel32.GetLastError());
    std.log.err("{s}: {}", .{ call, code });
}

fn startProcess(cmd: []const u8) !HANDLE {
    std.log.info("starting {s}", .{cmd});
    var cmdbuf: [128:0]u8 = undefined;
    const cmdz = try std.fmt.bufPrintZ(&cmdbuf, "{s}", .{cmd});
    var startup = std.mem.zeroes(winapi.STARTUPINFOA);
    startup.cb = @sizeOf(winapi.STARTUPINFOA);
    var proc = std.mem.zeroes(winapi.PROCESS_INFORMATION);
    if (winapi.CreateProcessA(null, cmdz.ptr, null, null, @boolToInt(false), winapi.PROCESS_CREATION_FLAGS.DEBUG_ONLY_THIS_PROCESS, null, null, &startup, &proc) == 0) {
        logWindowsErr("CreateProcessA");
        return error.WindowsFailure;
    }
    return proc.hProcess;
}

fn installBreakPoint(proc: HANDLE, addr: u32) !u8 {
    std.log.info("installing breakpoint at {x}", .{addr});

    var byte: u8 = undefined;
    var buf: *[1]u8 = &byte;
    if (winapi.ReadProcessMemory(proc, addr, buf, 1, null) == 0) {
        logWindowsErr("ReadProcessMemory");
        return error.WindowsFailure;
    }
    const prev = byte;

    buf[0] = 0xcc;
    if (winapi.WriteProcessMemory(proc, addr, buf, 1, null) == 0) {
        logWindowsErr("WriteProcessMemory");
        return error.WindowsFailure;
    }
    return prev;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var trace_points = std.AutoHashMap(u32, u8).init(allocator);

    // parse command line
    // note: running "trace bass" from my Windows machine causes this to return the string
    // "trace  bass" (note double space).  This appears to be dependent on Windows version!
    const cmdlinep = windows.kernel32.GetCommandLineA();
    const cmdline = cmdlinep[0..std.mem.len(cmdlinep)];
    var args = std.mem.tokenize(u8, cmdline, " ");
    // first arg is the exe itself
    _ = args.next();

    const cmd = args.next() orelse {
        std.log.err("specify command to run", .{});
        return;
    };

    while (args.next()) |arg| {
        std.log.info("parsing {s}", .{arg});
        const addr = std.fmt.parseInt(u32, arg, 16) catch |err| {
            std.log.err("parsing '{s}': {}", .{ arg, err });
            return;
        };
        try trace_points.put(addr, 0);
    }

    const proc = try startProcess(cmd);

    var iter = trace_points.iterator();
    while (iter.next()) |entry| {
        const addr = entry.key_ptr.*;
        entry.value_ptr.* = try installBreakPoint(proc, addr);
    }

    while (true) {
        var ev = std.mem.zeroes(winapi.DEBUG_EVENT);
        ev.dwProcessId = 1;
        if (winapi.WaitForDebugEvent(&ev, windows.INFINITE) == 0) {
            logWindowsErr("WaitForDebugEvent");
        }

        switch (ev.dwDebugEventCode) {
            .CREATE_PROCESS_DEBUG_EVENT => {},
            .OUTPUT_DEBUG_STRING_EVENT => {
                var buf: [1 << 10]u8 = undefined;
                const len = @min(buf.len, ev.u.DebugString.nDebugStringLength);
                var n: u32 = 0;
                if (winapi.ReadProcessMemory(proc, ev.u.DebugString.lpDebugStringData, &buf, len, &n) == 0) {
                    logWindowsErr("ReadProcessMemory");
                }
                const msg = std.mem.trimRight(u8, buf[0..n], "\r\n\x00");
                std.log.info("{s}", .{msg});
            },
            .LOAD_DLL_DEBUG_EVENT => {
                // Ignore.
                // Note: getting the name of the DLL from this event is a trainwreck.
                // There's an lpImageName attribute but people online say it's not usable,
                // and instead suggest a mess of calls to get the file name from the handle:
                //   https://learn.microsoft.com/en-us/windows/win32/memory/obtaining-a-file-name-from-a-file-handle
            },
            .UNLOAD_DLL_DEBUG_EVENT => {},
            .CREATE_THREAD_DEBUG_EVENT => {},
            .EXIT_THREAD_DEBUG_EVENT => {},
            .EXIT_PROCESS_DEBUG_EVENT => {
                break;
            },

            else => {
                std.log.info("event {}", .{ev.dwDebugEventCode});
            },
        }

        if (winapi.ContinueDebugEvent(ev.dwProcessId, ev.dwThreadId, winapi.ContinueDebugEventStatus.DBG_CONTINUE) == 0) {
            logWindowsErr("ContinueDebugEvent");
        }
    }
}
