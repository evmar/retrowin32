//! This is a replacement for the Rust 'log' crate, without much of the complexity
//! (e.g. dynamic configuration of which log levels to print, etc.).
//!
//! In particular 'log' likes to inline everything which makes reading disassembly harder.

#[derive(Clone, Copy)]
pub enum Level {
    Error = 5,
    Warn = 4,
    Info = 3,
    // Debug = 2,
    // Trace = 1,
}

pub struct Record<'a> {
    pub level: Level,
    pub file: &'static str,
    pub line: u32,
    pub args: std::fmt::Arguments<'a>,
}

pub trait Log {
    fn log(&self, record: &Record);
}

struct NoLogger;
impl Log for NoLogger {
    fn log(&self, _record: &Record) {
        unreachable!("logger not configured");
    }
}

pub static mut LOGGER: &dyn Log = &NoLogger;

pub fn set_logger(logger: &'static dyn Log) {
    unsafe { LOGGER = logger }
}

pub fn log_record(record: &Record) {
    unsafe {
        LOGGER.log(record);
    }
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)+) => (
        $crate::log_record(&$crate::Record{
            level: $level,
            file: std::file!(),
            line: std::line!(),
            args: format_args!($($arg)+)
        });
    )
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ($crate::log!($crate::Level::Info, $($arg)+));
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => ($crate::log!($crate::Level::Warn, $($arg)+));
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => ($crate::log!($crate::Level::Error, $($arg)+));
}
