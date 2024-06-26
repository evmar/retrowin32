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

impl log::Log for JsLogger {
    fn log(&self, record: &log::Record) {
        self.log(
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
unsafe impl Sync for JsLogger {}
unsafe impl Send for JsLogger {}

fn panic_hook(info: &std::panic::PanicInfo) {
    log::error!("{}", info);
}

pub fn init(host: JsLogger) {
    let logger: &'static mut JsLogger = Box::leak(Box::new(host));
    log::set_logger(logger).unwrap();
    std::panic::set_hook(Box::new(panic_hook));
}
