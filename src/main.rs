use std::net::TcpListener;

mod api;
mod libs;
mod pages;
mod router;

const PORT: &str = ":6969";

fn main() {
    let listener = TcpListener::bind(format!("0.0.0.0{PORT}")).unwrap();
    let pool = libs::ThreadPool::new(69);

    println!("Listening on port {PORT}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            router::router(stream);
        })
        .expect("Failed to send client request to the thread pool");
    }
}
