use std::{io::prelude::*, net::TcpStream};

pub fn not_found(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n404 - API Not Found!\n";

    stream.write_all(response.as_bytes()).unwrap();
}

pub fn not_allowed(mut stream: TcpStream) {
    let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n405 - Method Not Allowed!\n";

    stream.write_all(response.as_bytes()).unwrap();
}
