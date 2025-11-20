use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::cmd::handler::handle_command;

pub fn handle_connection(stream: &mut TcpStream) {
    loop {
        let mut buf = [0u8; 512];
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            break;
        }
        let command = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
        let response = handle_command(&command) + "\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
}
