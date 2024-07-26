//! A system for enabling tracing of different subsystems of winapi.
//! Each winapi file has a magic TRACE_CONTEXT constant string like
//! "kernel32/file".  The user can specify tracing based on prefix
//! matching, and a "-" suppresses, so e.g.
//!   --win32-trace=kernel32/,-kernel32/file
//! Pass '*' to enable all.

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug)]
struct Rule {
    key: String,
    enabled: bool,
}

struct State {
    rules: Vec<Rule>,
    enabled: HashMap<*const u8, bool>,
}

impl State {
    fn new(scheme: &str) -> Self {
        let mut rules = Vec::new();
        for mut part in scheme.split(',') {
            if part.len() == 0 {
                continue;
            }
            if part == "*" {
                part = ""
            }
            let enabled = if part.starts_with('-') {
                part = &part[1..];
                false
            } else {
                true
            };
            rules.push(Rule {
                key: part.into(),
                enabled,
            });
        }
        State {
            rules,
            enabled: HashMap::new(),
        }
    }

    fn lookup(&mut self, context: &'static str) -> bool {
        // Confusing: for a static 'foo', foo.as_ptr() has different values
        // when referenced from different mods (e.g. from ddraw's various mods),
        // but only in Debug builds.
        // This code still works in any case.
        if let Some(&enabled) = self.enabled.get(&context.as_ptr()) {
            return enabled;
        }
        let mut enabled = false;
        for rule in &self.rules {
            if context.starts_with(&rule.key) {
                enabled = rule.enabled;
                // Don't break, so last match wins.
            }
        }
        self.enabled.insert(context.as_ptr(), enabled);
        return enabled;
    }
}

static mut STATE: UnsafeCell<Option<State>> = UnsafeCell::new(None);

pub fn set_scheme(scheme: &str) {
    unsafe { *STATE.get_mut() = Some(State::new(scheme)) };
}

#[inline(never)]
pub fn enabled(context: &'static str) -> bool {
    unsafe {
        match STATE.get_mut() {
            None => return false,
            Some(state) => state.lookup(context),
        }
    }
}

#[inline(never)]
pub fn trace_begin(context: &str, func: &str, args: &[(&str, &dyn std::fmt::Debug)]) -> String {
    let mut msg = format!("{}/{}(", context, func);
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            msg.push_str(", ");
        }
        write!(&mut msg, "{}:{:x?}", arg.0, arg.1).unwrap();
    }
    msg.push_str(")");
    msg
}

#[inline(never)]
pub fn trace_return(context: &str, file: &'static str, line: u32, ret: &dyn std::fmt::Debug) {
    log::logger().log(
        &log::Record::builder()
            .level(log::Level::Info)
            .file(Some(file))
            .line(Some(line))
            .args(format_args!("{context} -> {ret:x?}"))
            .build(),
    );
}
