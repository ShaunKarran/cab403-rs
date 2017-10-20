extern crate bincode;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate simplelog;
extern crate threadpool;

use std::fs::File;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};

use bincode::{deserialize, serialize, Infinite};
use clap::App;
use serde::{Serialize};
use serde::de::DeserializeOwned;
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
    let message: TestMessage = read_and_decode(stream);
    debug!("{:?}", message.display);

    let send_message = TestMessage { display: String::from("This is a test message."), other_value: 12 };
    encode_and_write(send_message, stream);
}

fn encode_and_write<T>(data: T, stream: &mut TcpStream) where T: Serialize {
    let encoded_data = serialize(&data, Infinite).unwrap();

    stream.write(&encoded_data).unwrap();
}

fn read_and_decode<T>(stream: &mut TcpStream) -> T where T: DeserializeOwned {
    let mut input_buffer = [0; 256];
    let _ = stream.read(&mut input_buffer).unwrap();

    deserialize(&input_buffer).unwrap()
}
