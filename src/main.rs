mod cmd;
mod connections;
mod store;
use connections::connection::handle_connection;
use std::env;
use std::{net::TcpListener, thread};
fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        args.push("8000".into());
    }
    let address: String = String::from("localhost:") + args[1].as_str();
    let stream = match TcpListener::bind(address) {
        Ok(stream) => stream,
        Err(e) => {
            println!("error starting server: {}", e);
            return;
        }
    };

    println!("starting server on port {}!", args[1]);
    for connection in stream.incoming() {
        let mut stream = connection.unwrap();
        thread::spawn(move || handle_connection(&mut stream));
    }
}
