//! Hooks up the log crate (log::info!() etc.) to forward to a JS-based host.
//! This doesn't map directly to console.log etc. because we want to (eventually)
//! surface these logs in the UI.

use crate::emulator;

struct JsLogger;

const JS_LOGGER: JsLogger = JsLogger;

impl log::Log for JsLogger {
    fn log(&self, record: &log::Record) {
        emulator::js_host().log(
            record.level() as u8,
            format!(
                "{}:{} {}",
                record.file().unwrap_or(""),
                record.line().unwrap_or(0),
                record.args()
            ),
        );
    }

    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_logger(&JS_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    std::panic::set_hook(Box::new(move |info| {
        // Don't use log::error!() here, because that includes the current file
        // and line, which just points at the logging code.
        emulator::js_host().log(log::Level::Error as u8, format!("panic: {}", info));
    }));
}
