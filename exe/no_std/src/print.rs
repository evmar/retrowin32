use windows_sys::Win32::{
    Foundation::HANDLE,
    Storage::FileSystem::WriteFile,
    System::Console::{GetStdHandle, STD_OUTPUT_HANDLE},
};

static mut STDOUT: HANDLE = 0;

pub fn print(buf: &[u8]) {
    unsafe {
        if STDOUT == 0 {
            STDOUT = GetStdHandle(STD_OUTPUT_HANDLE);
        }
        WriteFile(
            STDOUT,
            buf.as_ptr(),
            buf.len() as u32,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
    }
}
