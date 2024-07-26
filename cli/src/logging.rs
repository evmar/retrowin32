//! Hook up the log crate to print to stdout.

static LOGGER: Logger = Logger;
struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn flush(&self) {}

    fn log(&self, record: &log::Record) {
        let level = match record.level() {
            log::Level::Error => "ERROR",
            log::Level::Warn => "WARN",
            log::Level::Info => "INFO",
            log::Level::Trace => "TRACE",
            log::Level::Debug => "DEBUG",
        };
        eprintln!(
            "{level} {}:{} {}",
            record.file().unwrap_or(""),
            record.line().unwrap_or(0),
            record.args()
        );
    }
}

pub fn init(max_level: log::LevelFilter) {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(max_level);
}
