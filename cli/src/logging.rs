//! Hook up the log crate to print to stdout.

static LOGGER: Logger = Logger;
struct Logger;

impl log::Log for Logger {
    fn log(&self, record: &log::Record) {
        let level = match record.level {
            log::Level::Error => "ERROR",
            log::Level::Warn => "WARN",
            log::Level::Info => "INFO",
        };
        eprintln!("{level} {}:{} {}", record.file, record.line, record.args);
    }
}

pub fn init() {
    log::set_logger(&LOGGER);
    // log::set_max_level(log::LevelFilter::Debug);
}
