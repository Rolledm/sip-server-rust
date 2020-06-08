use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;


// add rocket.io
pub fn on_http_message_received(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let content = fs::read_to_string("./res/home.html").unwrap();
        let response = format!("HTTP/1.1 200OK\r\n\r\n{}", content);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n";
        let content = "<p>404</p>";
        let response = format!("{}{}", status_line, content);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}