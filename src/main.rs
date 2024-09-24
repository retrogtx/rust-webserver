use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

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
    
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html>Hello World</html>";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
