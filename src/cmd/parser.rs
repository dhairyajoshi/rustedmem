use std::io;
pub enum Command {
    Set(String, String),
    Get(String),
    Pop(String),
    Help(),
}
pub fn get_command() -> Option<Command> {
    println!("Enter command: ");
    let mut command: String = String::new();
    io::stdin().read_line(&mut command).expect("enter command");
    let args: Vec<String> = command.trim().split(" ").map(String::from).collect();

    match args[0].as_str() {
        "set" => {
            if args.len() < 3 {
                print!("Command usage: set 'key' 'value'");
                return None;
            }
            Some(Command::Set(args[1].clone(), args[2].clone()))
        }
        "get" => {
            if args.len() < 2 {
                print!("Command usage: get 'key'");
                return None;
            }
            Some(Command::Get(args[1].clone()))
        }
        "pop" => {
            if args.len() < 2 {
                print!("Command usage: pop 'key'");
                return None;
            }
            Some(Command::Pop(args[1].clone()))
        }
        _ => Some(Command::Help()),
    }
}
