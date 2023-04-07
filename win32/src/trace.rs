use std::collections::HashMap;

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
        let mut enabled = true;
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

static mut STATE: Option<State> = None;

pub fn set_scheme(scheme: &str) {
    unsafe { STATE = Some(State::new(scheme)) };
}

pub fn enabled(context: &'static str) -> bool {
    let state = unsafe {
        match &mut STATE {
            None => return false,
            Some(state) => state,
        }
    };
    state.lookup(context)
}
