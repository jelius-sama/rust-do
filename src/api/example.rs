use std::{io::prelude::*, net::TcpStream};

pub fn example(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!\n";

    stream.write_all(response.as_bytes()).unwrap();
}
