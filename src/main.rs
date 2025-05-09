use rs_routine::{go, init};
use std::{io, net::TcpListener};

mod api;
mod libs;
mod pages;
mod router;

const PORT: &str = ":6969";

// fn main() {
//     let listener = TcpListener::bind(format!("0.0.0.0{PORT}")).unwrap();
//     let pool = libs::ThreadPool::new(69);
//
//     println!("Listening on port {PORT}");
//
//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
//
//         pool.execute(move || {
//             router::router(stream);
//         })
//         .expect("send client request to the thread pool");
//     }
// }

fn main() -> io::Result<()> {
    init();

    let listener = TcpListener::bind(format!("0.0.0.0{PORT}")).unwrap();

    println!("Listening on port {PORT}");

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                // Move the stream into the spawned closure
                go(move || router::router(stream));
            }
            Err(err) => eprintln!("Failed to accept connection: {}", err),
        }
    }

    Ok(())
}
