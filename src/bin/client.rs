extern crate bincode;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate simplelog;

use std::fs::File;
use std::io::Read;
use std::net::{SocketAddrV4, TcpStream};
use std::process;

use clap::App;
use bincode::{deserialize, serialize, Infinite};
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

    let mut encoded_message = [0; 256];
    let _ = stream.read(&mut encoded_message).unwrap();

    let message: TestMessage = deserialize(&encoded_message[..]).unwrap();

    println!("{:?}", message);
}
