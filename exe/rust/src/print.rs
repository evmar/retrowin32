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

pub struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        super::print::print(s.as_bytes());
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use ::core::fmt::Write;
            let mut stdout = $crate::print::Stdout;
            stdout.write_fmt(format_args!($($arg)*)).ok();
        }
    };
}

#[macro_export]
macro_rules! println {
    () => { $crate::println!("") };
    ($($arg:tt)*) => {
        {
            use ::core::fmt::Write;
            let mut stdout = $crate::print::Stdout;
            stdout.write_fmt(format_args!($($arg)*)).ok();
            stdout.write_char('\n').ok();
        }
    };
}
