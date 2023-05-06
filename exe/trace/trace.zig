const std = @import("std");

const windows = std.os.windows;
const winapi = @import("./winapi.zig");

fn logWindowsErr(call: []const u8) void {
    // Logging GetLastError as an enum adds 100kb(!) to the binary size.
    const code = @enumToInt(windows.kernel32.GetLastError());
    std.log.err("{s}: {}", .{ call, code });
}

pub fn main() void {
    var startup = std.mem.zeroes(winapi.STARTUPINFOA);
    startup.cb = @sizeOf(winapi.STARTUPINFOA);
    var proc = std.mem.zeroes(winapi.PROCESS_INFORMATION);
    if (winapi.CreateProcessA(null, "bass", null, null, @boolToInt(false), winapi.PROCESS_CREATION_FLAGS.DEBUG_ONLY_THIS_PROCESS, null, null, &startup, &proc) == 0) {
        logWindowsErr("CreateProcessA");
    }

    while (true) {
        var ev = std.mem.zeroes(winapi.DEBUG_EVENT);
        ev.dwProcessId = 1;
        if (winapi.WaitForDebugEvent(&ev, 0xffff_ffff) == 0) {
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
