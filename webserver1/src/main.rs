use std::io;
use std::net::TcpListener;

use webserver1;

extern crate chrono;

fn main() -> io::Result<()> {
    println!("Starting server...");

    let listener = TcpListener::bind("127.0.0.1:8001")?;

    println!("Server started!");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match webserver1::handle_client(stream) {
                Err(e) => eprintln!("Error handling client: {}", e),
                _ => (),
            },
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
