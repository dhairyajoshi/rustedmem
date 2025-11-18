use crate::cmd::parser::{get_command, Command};

pub fn handle_command() {
    let cmd = get_command();

    match cmd {
        Some(Command::Set(key, value)) => {
            println!("setting {}={}", key, value)
        }
        Some(Command::Get(key)) => {
            println!("getting value for key {}", key)
        }
        Some(Command::Pop(key)) => {
            println!("popping key {}", key)
        }
        Some(Command::Help()) => {
            println!("USAGE: 'get', 'set', 'pop' [OPTIONS]")
        }
        None => {
            println!("USAGE: 'get', 'set', 'pop' [OPTIONS]")
        }
    }
}
