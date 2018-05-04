mod ps;

use alloc::string::String;
use console;

pub fn run_command(mut command: String) -> String {
    if !command.is_empty() {
        match command.as_str() {
            ":" => {}
            "ps" => ps::run(),
            _ => {
                console::write_console("\r\n");
                console::write_console("command not found");
            }
        }
        command.clear()
    }

    command
}
