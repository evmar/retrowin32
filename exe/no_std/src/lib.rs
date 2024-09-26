#![no_std]

pub mod fmt;
pub mod fpu;
pub mod print;

#[cfg(not(test))]
#[panic_handler]
unsafe fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    print::print(b"panicked");
    windows_sys::Win32::System::Threading::ExitProcess(1);
}
