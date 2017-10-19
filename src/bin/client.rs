#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;

use std::fs::File;
use std::net::{SocketAddrV4, TcpStream};

use clap::App;
use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};

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
    let stream = TcpStream::connect(socket).unwrap();
}
