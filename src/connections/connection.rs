use std::{
    collections::HashMap,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

pub fn handle_connection(stream: &mut TcpStream, store: Arc<Mutex<HashMap<String, String>>>) {
    stream.write_all(b"connected!\n").unwrap();
}
