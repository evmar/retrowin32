const std = @import("std");
const windows = std.os.windows;

// This declares a function named `retrowin32_callback1` exists in `retrowin32.dll`,
// which is true in the emulator enivronment because the emulator exposes this function
// just for this program.  See win32/lib/ for related support files.
pub extern "retrowin32_test" fn retrowin32_test_callback1(
    func: *const fn (u32) callconv(.Stdcall) u32,
    data: u32,
) callconv(windows.WINAPI) u32;

fn callback0(data: u32) callconv(windows.WINAPI) u32 {
    std.debug.print("callback0 invoked: {x}\n", .{data});
    return 0x4567;
}

pub fn main() !void {
    const ret = retrowin32_test_callback1(callback0, 0x1234);
    std.debug.print("retrowin32_callback1 returned: {x}\n", .{ret});
}
