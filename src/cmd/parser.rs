use std::env;
pub enum Command {
    Set(String, String),
    Get(String),
    Pop(String),
    Help(),
}
pub fn get_command() -> Option<Command> {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "set" => {
            if args.len() < 4 {
                print!("Command usage: set 'key' 'value'");
                return None;
            }
            Some(Command::Set(args[2].clone(), args[3].clone()))
        }
        "get" => {
            if args.len() < 3 {
                print!("Command usage: get 'key'");
                return None;
            }
            Some(Command::Get(args[2].clone()))
        }
        "pop" => {
            if args.len() < 3 {
                print!("Command usage: pop 'key'");
                return None;
            }
            Some(Command::Pop(args[2].clone()))
        }
        _ => Some(Command::Help()),
    }
}
