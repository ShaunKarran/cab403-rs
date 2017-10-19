extern crate bincode;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate simplelog;
extern crate threadpool;

use std::fs::File;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};

use bincode::{deserialize, serialize, Infinite};
use clap::App;
use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};

use threadpool::ThreadPool;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestMessage {
    display: String,
    other_value: u32,
}

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
    let message = TestMessage { display: String::from("This is a test message."), other_value: 12 };
    let encoded_message = serialize(&message, Infinite).unwrap();

    stream.write(&encoded_message).unwrap();
}
