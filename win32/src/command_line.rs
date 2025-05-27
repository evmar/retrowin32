#[derive(Default)]
pub struct CommandLine {
    pub string: String,
}

impl CommandLine {
    pub fn new(cmdline: String) -> Self {
        CommandLine { string: cmdline }
    }

    pub fn exe_name(&self) -> String {
        // TODO: we only need to parse the exe name, not the arguments
        split_cmdline(&self.string).swap_remove(0)
    }
}

// TODO: follow the logic for CommandLineToArgvW
// https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw
fn split_cmdline(cmdline: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut arg = String::new();
    let mut in_quote = false;
    for c in cmdline.chars() {
        match c {
            ' ' if !in_quote => {
                if !arg.is_empty() {
                    args.push(arg);
                    arg = String::new();
                }
            }
            '"' => {
                in_quote = !in_quote;
            }
            _ => {
                arg.push(c);
            }
        }
    }
    if !arg.is_empty() {
        args.push(arg);
    }
    args
}
