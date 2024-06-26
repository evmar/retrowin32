//! Hooks up the log crate (log::info!() etc.) to forward to a JS-based host.
//! This doesn't map directly to console.log etc. because we want to (eventually)
//! surface these logs in the UI.

use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const JSLOGGER_TS: &'static str = r#"
export interface JsLogger {
  log(level: number, msg: string): void;
}"#;

#[wasm_bindgen]
extern "C" {
    pub type JsLogger;

    #[wasm_bindgen(method)]
    fn log(this: &JsLogger, level: u8, msg: String);
}

struct HostLogger {
    host: JsLogger,
}
impl log::Log for HostLogger {
    fn log(&self, record: &log::Record) {
        self.host.log(
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

// There are no threads in wasm, but the log crate requires these traits.
unsafe impl Sync for HostLogger {}
unsafe impl Send for HostLogger {}

fn panic_hook(info: &std::panic::PanicInfo) {
    log::error!("{}", info);
}

pub fn init(host: JsLogger) {
    let logger: &'static mut HostLogger = Box::leak(Box::new(HostLogger { host }));
    log::set_logger(logger).unwrap();
    std::panic::set_hook(Box::new(panic_hook));
}
