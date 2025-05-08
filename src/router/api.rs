use crate::api;
use std::net::TcpStream;

fn handler(func: fn(stream: TcpStream), accept_method: &str, method: &str, stream: TcpStream) {
    if accept_method != method {
        api::not_allowed(stream);
    } else {
        func(stream);
    }
}

pub fn api_router(path: &str, method: &str, stream: TcpStream) {
    if path == "/api/example" {
        handler(api::example, "GET", method, stream);
    } else {
        api::not_found(stream);
    }
}
