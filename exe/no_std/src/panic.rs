//! Support for panics under no_std.

#[cfg(not(test))]
#[panic_handler]
unsafe fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    use crate::println;
    println!("{}", info);
    windows_sys::Win32::System::Threading::ExitProcess(1);
}
