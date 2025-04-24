//! Support for panics under no_std.

#[cfg(not(test))]
#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    use crate::println;
    println!("{}", info);
    unsafe {
        windows_sys::Win32::System::Threading::ExitProcess(1);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __CxxFrameHandler3() {}
