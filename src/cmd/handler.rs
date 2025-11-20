use crate::cmd::parser::{get_command, Command};
use crate::store::keystore::{add, get, pop};

pub fn handle_command(input: &String) -> String {
    let cmd = get_command(input);

    match cmd {
        Some(Command::Set(key, value)) => {
            add(key, value);
            "cache updated!".into()
        }
        Some(Command::Get(key)) => get(key),
        Some(Command::Pop(key)) => {
            pop(key);
            "cache updated!".into()
        }
        Some(Command::Help()) => ("USAGE: 'get', 'set', 'pop' [OPTIONS]").into(),
        None => ("USAGE: 'get', 'set', 'pop' [OPTIONS]").into(),
    }
}
