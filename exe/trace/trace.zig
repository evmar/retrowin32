const std = @import("std");
const windows = std.os.windows;
const HANDLE = windows.HANDLE;
const winapi = @import("./winapi.zig");
const debug = @import("./debug.zig");
const Allocator = std.mem.Allocator;

// Declaring this struct affects the default log level(!)
pub const std_options = struct {
    pub const log_level = .info;
};

var stdout: @TypeOf(std.io.getStdOut().writer()) = undefined;

const TracePoints = std.AutoHashMap(u32, u8);
const State = enum { init, unpacking, run };

const tracee = struct {
    var proc: winapi.PROCESS_INFORMATION = undefined;
    var state: State = State.init;
    var trace_points: TracePoints = undefined;
    const pre_upx_addr = 0x44d98c;
    const post_upx_addr = 0x41f079;
};

fn processDllLoadEvent(load: winapi.LOAD_DLL_DEBUG_INFO) !void {
    // Note: getting the name of the DLL from this event is a trainwreck.
    // There's an lpImageName attribute but people online say it's not usable,
    // and instead suggest a mess of calls to get the file name from the handle:
    //   https://learn.microsoft.com/en-us/windows/win32/memory/obtaining-a-file-name-from-a-file-handle

    if (load.lpImageName == 0) return;

    var addr: u32 = 0;
    if (winapi.ReadProcessMemory(
        tracee.proc.hProcess,
        load.lpImageName,
        @ptrCast([*]u8, &addr),
        4,
        null,
    ) == 0) {
        // Fails for some dlls like kernel32.
        return;
    }
    if (addr == 0) return;

    var buf: [128]u8 = undefined;
    const name = try debug.readString(tracee.proc.hProcess, addr, load.fUnicode != 0, &buf);
    std.log.info("load dll '{s}'", .{name});
}

fn processBreakpoint(addr: u32) !void {
    switch (tracee.state) {
        State.init => {
            // We hit a breakpoint on process start that is not one of our registered breakpoints,
            // so there's no need to uninstall it.
            const instr = try debug.installBreakPoint(tracee.proc.hProcess, tracee.pre_upx_addr);
            try tracee.trace_points.put(tracee.pre_upx_addr, instr);
            tracee.state = State.unpacking;
            return;
        },
        State.unpacking => {
            if (addr != tracee.pre_upx_addr) {
                std.debug.panic("unexpected unpack breakpoint {x}", .{addr});
            }
            var iter = tracee.trace_points.iterator();
            while (iter.next()) |entry| {
                if (entry.value_ptr.* == 0) {
                    entry.value_ptr.* = try debug.installBreakPoint(tracee.proc.hProcess, entry.key_ptr.*);
                }
            }
            tracee.state = State.run;
        },
        State.run => {
            // expected
        },
    }

    const entry = tracee.trace_points.fetchRemove(addr) orelse std.debug.panic("unexpected breakpoint {x}", .{addr});
    try debug.patchProcess(tracee.proc.hProcess, addr, entry.value);

    var context: winapi.CONTEXT = std.mem.zeroes(winapi.CONTEXT);
    context.ContextFlags = winapi.CONTEXT_FULL;
    if (winapi.GetThreadContext(tracee.proc.hThread, &context) == 0) {
        winapi.logWindowsErr("GetThreadContext");
        return error.WindowsFailure;
    }

    try std.fmt.format(
        stdout,
        "@{x}\n  eax:{x} ebx:{x} ecx:{x} edx:{x} esi:{x} edi:{x}\n",
        .{
            addr,
            context.Eax,
            context.Ebx,
            context.Ecx,
            context.Edx,
            context.Esi,
            context.Edi,
        },
    );

    context.Eip -= 1; // retry instruction
    // Dead code here: we used to single-step here to reinstall breakpoint after.
    // context.EFlags |= 0x100; // trap flag for single-step
    if (winapi.SetThreadContext(tracee.proc.hThread, &context) == 0) {
        winapi.logWindowsErr("SetThreadContext");
        return error.WindowsFailure;
    }
}

fn processExceptionEvent(ev: winapi.EXCEPTION_DEBUG_INFO) !void {
    const ex = ev.ExceptionRecord;
    switch (ex.ExceptionCode) {
        .EXCEPTION_BREAKPOINT => {
            try processBreakpoint(ex.ExceptionAddress);
        },
        // .EXCEPTION_SINGLE_STEP => {
        //     // reinstall breakpoint
        //     // const entry = tracee.trace_points.fetchRemove(tracee.last_breakpoint) orelse {
        //     //     std.debug.panic("unexpected single step {x}", .{tracee.last_breakpoint});
        //     // };
        //     // try debug.installBreakPoint(tracee.proc.hProcess, self.addr);
        // },
        else => {
            std.log.err("unhandled exception event {}", .{ev});
        },
    }
}

fn processDebugStringEvent(ev: winapi.OUTPUT_DEBUG_STRING_INFO) !void {
    var buf: [1 << 10]u8 = undefined;
    const len = @min(buf.len, ev.nDebugStringLength);
    var n: u32 = 0;
    if (winapi.ReadProcessMemory(
        tracee.proc.hProcess,
        ev.lpDebugStringData,
        &buf,
        len,
        &n,
    ) == 0) {
        winapi.logWindowsErr("ReadProcessMemory");
        return error.WindowsFailure;
    }
    const msg = std.mem.trimRight(u8, buf[0..n], "\r\n\x00");
    std.log.info("OutputDebugString: {s}", .{msg});
}

fn parseArgs() ![]const u8 {
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
        try tracee.trace_points.put(addr, 0);
    }
    return cmd;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    stdout = std.io.getStdOut().writer();
    tracee.trace_points = TracePoints.init(allocator);

    const cmd = try parseArgs();
    tracee.proc = try debug.startProcess(cmd);

    while (true) {
        var ev = std.mem.zeroes(winapi.DEBUG_EVENT);
        if (winapi.WaitForDebugEvent(&ev, windows.INFINITE) == 0) {
            winapi.logWindowsErr("WaitForDebugEvent");
            return error.Win32Error;
        }

        switch (ev.dwDebugEventCode) {
            .CREATE_PROCESS_DEBUG_EVENT => {},
            .EXIT_PROCESS_DEBUG_EVENT => {
                break;
            },

            .OUTPUT_DEBUG_STRING_EVENT => {
                try processDebugStringEvent(ev.u.DebugString);
            },

            .LOAD_DLL_DEBUG_EVENT => {
                try processDllLoadEvent(ev.u.LoadDll);
            },
            .UNLOAD_DLL_DEBUG_EVENT => {},

            .CREATE_THREAD_DEBUG_EVENT => {},
            .EXIT_THREAD_DEBUG_EVENT => {},

            .EXCEPTION_DEBUG_EVENT => {
                try processExceptionEvent(ev.u.Exception);
            },

            else => {
                std.log.err("unexpected event {}", .{ev.dwDebugEventCode});
            },
        }

        if (winapi.ContinueDebugEvent(
            ev.dwProcessId,
            ev.dwThreadId,
            winapi.ContinueDebugEventStatus.DBG_CONTINUE,
        ) == 0) {
            winapi.logWindowsErr("ContinueDebugEvent");
        }
    }
}
