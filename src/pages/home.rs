use std::{fs, io::prelude::*, net::TcpStream};

pub fn home(mut stream: TcpStream) {
    let (status_line, filename) = ("HTTP/1.1 200 OK", "assets/index.html");
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
