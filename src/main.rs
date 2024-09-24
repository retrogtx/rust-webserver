use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let filename = "index.html"; 
        let contents = fs::read_to_string(filename).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n";
        let response = fs::read_to_string("404.html").unwrap();
        let full_response = format!("{}{}", status_line, response); 
        stream.write_all(full_response.as_bytes()).unwrap();
        stream.flush().unwrap();
    };
}