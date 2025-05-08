use crate::{api, router};
use std::{
    io::{BufReader, prelude::*},
    net::TcpStream,
    process::exit,
};

pub fn router(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");
    let version = parts.next().unwrap_or("");

    if method == "" || path == "" || version == "" {
        println!("Could not parse http header!");
        exit(1);
    }

    if path.starts_with("/api/") {
        router::api_router(path, method, stream);
    } else {
        if method != "GET" {
            api::not_allowed(stream);
        } else {
            router::pages_router(path, stream);
        }
    };
}
