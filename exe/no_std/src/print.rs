use windows_sys::Win32::{
    Storage::FileSystem::WriteFile,
    System::Console::{GetStdHandle, STD_OUTPUT_HANDLE},
};

pub fn print(buf: &[u8]) {
    unsafe {
        let stdout = GetStdHandle(STD_OUTPUT_HANDLE);
        WriteFile(
            stdout,
            buf.as_ptr(),
            buf.len() as u32,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
    }
}
