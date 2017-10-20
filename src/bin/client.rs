extern crate bincode;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate simplelog;

use std::fs::File;
use std::io::{Read, Write};
use std::io::stdin;
use std::net::{SocketAddrV4, TcpStream};
use std::process;

use clap::App;
use bincode::{deserialize, serialize, Infinite};
use serde::{Serialize};
use serde::de::DeserializeOwned;
use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestMessage {
    display: String,
    other_value: u32,
}

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Warn, Config::default()).unwrap(),
            WriteLogger::new(LogLevelFilter::Debug, Config::default(), File::create("hangman-client.log").unwrap()),
        ]
    ).unwrap();

    let yaml = load_yaml!("client_cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let hostname = value_t!(matches, "hostname", String).unwrap();
    let port = value_t!(matches, "port", u16).unwrap();
    let socket = SocketAddrV4::new(hostname.parse().unwrap(), port);

    info!("Connecting to socket {}", socket);
    let mut stream = match TcpStream::connect(socket) {
        Ok(stream) => stream,
        Err(error) => {
            error!("Failed to connect to server: {}", error);
            process::exit(1);
        }
    };

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();

    encode_and_write(user_input, &mut stream);

    let message: TestMessage = read_and_decode(&mut stream);

    println!("{:?}", message);
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
