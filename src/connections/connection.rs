use crate::cmd::handler::handle_command;
use std::io::{BufRead, BufReader};
use std::{io::Write, net::TcpStream};

pub fn handle_connection(stream: &mut TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    loop {
        let mut command = String::new();
        let bytes = reader.read_line(&mut command).unwrap();
        if bytes == 0 {
            break;
        }
        let response = handle_command(&command.trim().to_string()) + "\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
}
