use crate::pages;
use std::net::TcpStream;

pub fn pages_router(path: &str, stream: TcpStream) {
    if path == "/" {
        pages::home(stream);
    } else {
        pages::not_found(stream);
    }
}
