extern crate bincode;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate serde;
extern crate simplelog;
extern crate threadpool;

extern crate cab403_rs;

use std::fs::File;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};

use clap::App;
use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};

use threadpool::ThreadPool;

use cab403_rs::communication::{Command, Message, encode_and_write, read_and_decode};

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

    let pool = ThreadPool::new(10);
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        pool.execute(move || handle_client(&mut stream));
    }
}

fn handle_client(stream: &mut TcpStream) {
    loop {
        let message = read_and_decode(stream);
        match message {
            Message::Command(Command::Shutdown) => {
                info!("Shutting down.");
            },
            _ => {
                let send_message = Message::Graphic(String::from("Welcome!"));
                encode_and_write(send_message, stream);
            }
        }
    }
}
