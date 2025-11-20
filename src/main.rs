mod cmd;
mod connections;
mod store;
use connections::connection::handle_connection;
use std::{net::TcpListener, sync::Arc, thread};
use store::keystore::KEY_STORE;
fn main() {
    let stream = TcpListener::bind("localhost:8001").unwrap();
    println!("starting server on port 8001!");
    for connection in stream.incoming() {
        let mut stream = connection.unwrap();
        let store = Arc::clone(&KEY_STORE);
        thread::spawn(move || handle_connection(&mut stream, store));
    }
}
