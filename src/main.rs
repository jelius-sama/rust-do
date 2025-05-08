use std::net::TcpListener;

mod api;
mod libs;
mod pages;
mod router;

fn main() {
    let port = ":6969";
    let listener = TcpListener::bind(format!("0.0.0.0{port}")).unwrap();
    let pool = libs::ThreadPool::new(69);

    println!("Listening on port {port}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            router::router(stream);
        })
        .expect("Failed to send client request to the thread pool");
    }
}
