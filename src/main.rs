use std::io::prelude::*;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        // Modern netowrk hardware has a "Direct Memory Access" (DMA) chip
        // That can fill up some memory space, then notify that it is done?
        // Where does that fit into this?
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        let contents = format!(
r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta http-equiv="X-UA-Compatible" content="IE=edge">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Document</title>
</head>
<body style="color: red;">
Hello World! Number: {} 🔥
</body>
</html>"#,
            rand::random::<u8>()
        );

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

// What does it mean to "take ownership" here? Is not borrowed, but 'owned'.
// That has to do with memory layout.
// Is the "TcpStream" dropped from memory after this function is called?
// Is this function blocking? If so, is there a maximum amount of time that would take?

// This is mutable ownership?

// fn handle_connection(mut stream: TcpStream) {

// }
