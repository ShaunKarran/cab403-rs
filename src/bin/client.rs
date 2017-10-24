extern crate bincode;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate serde;
extern crate simplelog;

extern crate cab403_rs;

use std::fs::File;
use std::io::stdin;
use std::net::{SocketAddrV4, TcpStream};
use std::process;

use clap::App;
use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};

use cab403_rs::communication::{Command, Message, encode_and_write, read_and_decode};

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

    loop {
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();

        match user_input.as_ref() {
            "q\n" => {
                debug!("Sending Shutdown command.");
                encode_and_write(Message::Command(Command::Shutdown), &mut stream);
            },
            _ => {
                debug!("Sending placeholder message.");
                encode_and_write(Message::Graphic(String::from("do_nothing")), &mut stream);
            }
        }

        let message: Message = read_and_decode(&mut stream);
        match message {
            Message::Graphic(contents) => {
                println!("{}", contents);
            },
            Message::Command(command) => {
                debug!("Got command, {:?}", command)
            }
        }
    }
}
