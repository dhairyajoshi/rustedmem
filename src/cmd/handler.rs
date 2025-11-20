use crate::cmd::parser::{get_command, Command};
use crate::store::keystore::{add, get, pop};

pub fn handle_command() {
    let cmd = get_command();

    match cmd {
        Some(Command::Set(key, value)) => {
            add(key, value);
        }
        Some(Command::Get(key)) => {
            let val = get(key);
            println!("{}", val);
        }
        Some(Command::Pop(key)) => {
            pop(key);
        }
        Some(Command::Help()) => {
            println!("USAGE: 'get', 'set', 'pop' [OPTIONS]")
        }
        None => {
            println!("USAGE: 'get', 'set', 'pop' [OPTIONS]")
        }
    }
}
