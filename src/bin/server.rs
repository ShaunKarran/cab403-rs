#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate threadpool;

use std::fs::File;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

use clap::App;
use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};

use threadpool::ThreadPool;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
            WriteLogger::new(LogLevelFilter::Debug, Config::default(), File::create("hangman-server.log").unwrap()),
        ]
    ).unwrap();

    let yaml = load_yaml!("server_cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let port = value_t!(matches, "port", u16).unwrap();
    let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);

    info!("Listening on socket {}", socket);
    let listener = TcpListener::bind(socket).unwrap();
    let mut pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            debug!("{:?}", stream);
        });
    }
}
