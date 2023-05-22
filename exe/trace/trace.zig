const std = @import("std");

const windows = std.os.windows;
const winapi = @import("./winapi.zig");
const HANDLE = windows.HANDLE;

// Declaring this struct affects the default log level(!)
pub const std_options = struct {
    pub const log_level = .info;
};

const TracePointsMap = std.AutoHashMap(u32, u8);

var stdout: @TypeOf(std.io.getStdOut().writer()) = undefined;

fn logWindowsErr(call: []const u8) void {
    // Logging GetLastError as an enum adds 100kb(!) to the binary size.
    const code = @enumToInt(windows.kernel32.GetLastError());
    std.log.err("{s}: {}", .{ call, code });
}

const ProcInfo = struct {
    proc: HANDLE,
    thread: HANDLE,
};

fn startProcess(cmd: []const u8) !ProcInfo {
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
    return ProcInfo{ .proc = proc.hProcess, .thread = proc.hThread };
}

fn installBreakPoint(proc: HANDLE, addr: u32) !u8 {
    var prev: u8 = undefined;
    if (winapi.ReadProcessMemory(proc, addr, @ptrCast([*]u8, &prev), 1, null) == 0) {
        logWindowsErr("ReadProcessMemory");
        return error.WindowsFailure;
    }

    const int3: u8 = 0xcc;
    if (winapi.WriteProcessMemory(proc, addr, @ptrCast([*]const u8, &int3), 1, null) == 0) {
        logWindowsErr("WriteProcessMemory");
        return error.WindowsFailure;
    }

    if (winapi.FlushInstructionCache(proc, addr, 1) == 0) {
        logWindowsErr("FlushInstructionCache");
        return error.WindowsFailure;
    }

    return prev;
}

/// Read a nul-terminated string from a process's memory.
fn readString(proc: HANDLE, addr: u32, unicode: bool, buf: []u8) ![]u8 {
    var len: u32 = 0;
    var ch: u16 = undefined;
    const size: u32 = if (unicode) 2 else 1;
    while (len < buf.len) {
        if (winapi.ReadProcessMemory(proc, addr + (len * size), @ptrCast([*]u8, &ch), size, null) == 0) {
            logWindowsErr("ReadProcessMemory");
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

fn processDllLoad(proc: HANDLE, load: *winapi.LOAD_DLL_DEBUG_INFO) !void {
    // Note: getting the name of the DLL from this event is a trainwreck.
    // There's an lpImageName attribute but people online say it's not usable,
    // and instead suggest a mess of calls to get the file name from the handle:
    //   https://learn.microsoft.com/en-us/windows/win32/memory/obtaining-a-file-name-from-a-file-handle

    if (load.lpImageName == 0) return;

    var addr: u32 = 0;
    if (winapi.ReadProcessMemory(proc, load.lpImageName, @ptrCast([*]u8, &addr), 4, null) == 0) {
        // Fails for some dlls like kernel32.
        return;
    }
    if (addr == 0) return;

    var buf: [128]u8 = undefined;
    const name = try readString(proc, addr, load.fUnicode != 0, &buf);
    std.log.info("load dll '{s}'", .{name});
}

fn processBreakpoint(proc: HANDLE, thread: HANDLE, addr: u32, trace_points: TracePointsMap) !void {
    const prev = trace_points.getPtr(addr) orelse return;
    if (winapi.WriteProcessMemory(proc, addr, @ptrCast([*]const u8, prev), 1, null) == 0) {
        logWindowsErr("WriteProcessMemory");
        return error.WindowsFailure;
    }
    prev.* = 0;
    if (winapi.FlushInstructionCache(proc, addr, 1) == 0) {
        logWindowsErr("FlushInstructionCache");
        return error.WindowsFailure;
    }

    var context: winapi.CONTEXT = std.mem.zeroes(winapi.CONTEXT);
    context.ContextFlags = winapi.CONTEXT_FULL;
    if (winapi.GetThreadContext(thread, &context) == 0) {
        logWindowsErr("GetThreadContext");
        return error.WindowsFailure;
    }

    try std.fmt.format(stdout, "{x}: eax:{x} ebx:{x} ecx:{x} edx:{x} ebp:{x} esp:{x} esi:{x} edi:{x}\n", .{
        addr,
        context.Eax,
        context.Ebx,
        context.Ecx,
        context.Edx,
        context.Ebp,
        context.Esp,
        context.Esi,
        context.Edi,
    });

    context.Eip -= 1; // retry instruction
    context.EFlags |= 0x100; // trap flag for single-step
    if (winapi.SetThreadContext(thread, &context) == 0) {
        logWindowsErr("GetThreadContext");
        return error.WindowsFailure;
    }
}

fn parseArgs(trace_points: *TracePointsMap) ![]const u8 {
    // note: running "trace bass" from my Windows machine causes this to return the string
    // "trace  bass" (note double space).  This appears to be dependent on Windows version!
    const cmdlinep = windows.kernel32.GetCommandLineA();
    const cmdline = cmdlinep[0..std.mem.len(cmdlinep)];
    var args = std.mem.tokenize(u8, cmdline, " ");
    // first arg is the exe itself
    _ = args.next();

    const cmd = args.next() orelse {
        std.log.err("specify command to run", .{});
        return error.Args;
    };

    while (args.next()) |arg| {
        const addr = std.fmt.parseInt(u32, arg, 16) catch |err| {
            std.log.err("parsing '{s}': {}", .{ arg, err });
            return error.Args;
        };
        try trace_points.put(addr, 0);
    }
    return cmd;
}

pub fn main() !void {
    stdout = std.io.getStdOut().writer();
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var trace_points = TracePointsMap.init(allocator);
    var installed_trace_points = false;
    var last_breakpoint: u32 = 0;
    const cmd = try parseArgs(&trace_points);
    const proc = try startProcess(cmd);

    while (true) {
        var ev = std.mem.zeroes(winapi.DEBUG_EVENT);
        if (winapi.WaitForDebugEvent(&ev, windows.INFINITE) == 0) {
            logWindowsErr("WaitForDebugEvent");
            return error.Win32Error;
        }

        switch (ev.dwDebugEventCode) {
            .CREATE_PROCESS_DEBUG_EVENT => {},
            .OUTPUT_DEBUG_STRING_EVENT => {
                var buf: [1 << 10]u8 = undefined;
                const len = @min(buf.len, ev.u.DebugString.nDebugStringLength);
                var n: u32 = 0;
                if (winapi.ReadProcessMemory(proc.proc, ev.u.DebugString.lpDebugStringData, &buf, len, &n) == 0) {
                    logWindowsErr("ReadProcessMemory");
                }
                const msg = std.mem.trimRight(u8, buf[0..n], "\r\n\x00");
                std.log.info("{s}", .{msg});
            },
            .LOAD_DLL_DEBUG_EVENT => {
                try processDllLoad(proc.proc, &ev.u.LoadDll);
            },
            .UNLOAD_DLL_DEBUG_EVENT => {},
            .CREATE_THREAD_DEBUG_EVENT => {},
            .EXIT_THREAD_DEBUG_EVENT => {},
            .EXIT_PROCESS_DEBUG_EVENT => {
                break;
            },

            .EXCEPTION_DEBUG_EVENT => {
                const ex = &ev.u.Exception.ExceptionRecord;
                switch (ex.ExceptionCode) {
                    .EXCEPTION_BREAKPOINT => {
                        if (!installed_trace_points) {
                            var iter = trace_points.iterator();
                            while (iter.next()) |entry| {
                                const addr = entry.key_ptr.*;
                                std.log.info("installing breakpoint at {x}", .{addr});
                                entry.value_ptr.* = try installBreakPoint(proc.proc, addr);
                            }
                            installed_trace_points = true;
                        }
                        try processBreakpoint(proc.proc, proc.thread, ex.ExceptionAddress, trace_points);
                        last_breakpoint = ex.ExceptionAddress;
                    },
                    .EXCEPTION_SINGLE_STEP => {
                        // reinstall breakpoint
                        if (last_breakpoint == 0) {
                            std.debug.panic("unexpected single step without last breakpoint", .{});
                        }
                        var prev = trace_points.getPtr(last_breakpoint) orelse {
                            std.debug.panic("unexpected single step {x}", .{last_breakpoint});
                        };
                        if (prev.* != 0) {
                            std.debug.panic("double-install breakpoint? {x}", .{last_breakpoint});
                        }
                        prev.* = try installBreakPoint(proc.proc, last_breakpoint);
                        last_breakpoint = 0;
                    },
                    else => {
                        std.log.info("exception {}", .{ev.u.Exception});
                    },
                }
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
