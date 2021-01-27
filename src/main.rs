use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// Technically the network is considered a device, and so are files
// But we don't know the size of whatever is coming from the network.
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 16384];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

// How do I really observe what is going on at runtime?
// I want to see the raw request - the whole thing.
// Does that mena reading to a string?
// What's the use of a buffer? Are there OS calls to the network stack?
// The networking is all happening locally.
