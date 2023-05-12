const std = @import("std");

const windows = std.os.windows;
const winapi = @import("./winapi.zig");

// Declaring this struct affects the default log level(!)
pub const std_options = struct {
    pub const log_level = .info;
};

fn logWindowsErr(call: []const u8) void {
    // Logging GetLastError as an enum adds 100kb(!) to the binary size.
    const code = @enumToInt(windows.kernel32.GetLastError());
    std.log.err("{s}: {}", .{ call, code });
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var trace_points = std.AutoHashMap(u32, void).init(allocator);

    // parse command line
    const cmdlinep = windows.kernel32.GetCommandLineA();
    const cmdline = cmdlinep[0..std.mem.len(cmdlinep)];
    var args = std.mem.split(u8, cmdline, " ");
    const exename = args.first();
    while (args.next()) |arg| {
        const addr = std.fmt.parseInt(u32, arg, 16) catch |err| {
            std.log.err("parsing '{s}'': {}", .{ arg, err });
            return;
        };
        try trace_points.put(addr, {});
    }

    // invoke subprocess
    var exebuf: [128:0]u8 = undefined;
    const exe = try std.fmt.bufPrintZ(&exebuf, "{s}", .{exename});
    var startup = std.mem.zeroes(winapi.STARTUPINFOA);
    startup.cb = @sizeOf(winapi.STARTUPINFOA);
    var proc = std.mem.zeroes(winapi.PROCESS_INFORMATION);
    if (winapi.CreateProcessA(null, exe.ptr, null, null, @boolToInt(false), winapi.PROCESS_CREATION_FLAGS.DEBUG_ONLY_THIS_PROCESS, null, null, &startup, &proc) == 0) {
        logWindowsErr("CreateProcessA");
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
                std.log.info("debug {}\n", .{ev.u.DebugString});
                var buf: [1 << 10]u8 = undefined;
                const len = @min(buf.len, ev.u.DebugString.nDebugStringLength);
                var n: u32 = 0;
                if (winapi.ReadProcessMemory(proc.hProcess, ev.u.DebugString.lpDebugStringData, &buf, len, &n) == 0) {
                    logWindowsErr("ReadProcessMemory");
                }
                const msg = std.mem.trimRight(u8, buf[0..n], "\r\n\x00");
                std.log.info("{s}\n", .{msg});
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
                std.log.info("event {}\n", .{ev.dwDebugEventCode});
            },
        }

        if (winapi.ContinueDebugEvent(ev.dwProcessId, ev.dwThreadId, winapi.ContinueDebugEventStatus.DBG_CONTINUE) == 0) {
            logWindowsErr("ContinueDebugEvent");
        }
    }
}
