extern crate threadpool;

use std::net::TcpListener;
use threadpool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();
    let mut pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            println!("{:?}", stream);
        });
    }
}
