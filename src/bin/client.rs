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
use std::thread;

use clap::App;
use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};

use cab403_rs::communication::{Command, Message, Selection, encode_and_write, read_and_decode};

fn main() {
    // Initialise logger.
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Warn, Config::default()).unwrap(),
            WriteLogger::new(LogLevelFilter::Debug, Config::default(), File::create("hangman-client.log").unwrap()),
        ]
    ).unwrap();

    // Parse command line arguments.
    let yaml = load_yaml!("client_cli.yml");
    let args = App::from_yaml(yaml).get_matches();

    // Run the program and exit with the appropriate code.
    match run(args) {
        Ok(0) => process::exit(0),
        Ok(_) => process::exit(1),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    }
}

fn run(args: clap::ArgMatches) -> Result<usize, usize> {
    // Create socket.
    let hostname = value_t!(args, "hostname", String).unwrap();
    let port = value_t!(args, "port", u16).unwrap();
    let socket = SocketAddrV4::new(hostname.parse().unwrap(), port);

    // Connect to server.
    info!("Connecting to socket {}", socket);
    let stream = match TcpStream::connect(socket) {
        Ok(stream) => stream,
        Err(error) => {
            error!("Failed to connect to server: {}", error);
            process::exit(1);
        }
    };

    // "Split" the stream into reader and writer.
    // There is not actually any difference between them as they are just a clone of the same stream.
    let mut reader = stream;
    let mut writer = reader.try_clone().expect("Failed to clone stream");

    // Spawn the threads to handle the simultaneous reading and writing.
    let reader_thread = thread::spawn(move || read(&mut reader));
    let writer_thread = thread::spawn(move || write(&mut writer));

    // Wait for the threads to complete.
    reader_thread.join().unwrap();
    writer_thread.join().unwrap();

    Ok(0)
}

fn read(stream: &mut TcpStream) {
    loop {
        // Wait for messages from the server.
        let message: Message = read_and_decode(stream);
        match message {
            Message::Graphic(contents) => {
                println!("{}", contents);
            },
            Message::Command(Command::Shutdown) => {
                debug!("Recieved Shutdown command from server");
                return;
            },
            _ => {}
        }
    }

}

fn write(stream: &mut TcpStream) {
    loop {
        // Wait for user input.
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        user_input.pop(); // Remove the new line from the end of the user input.
        match user_input.as_ref() {
            "3" => {
                debug!("Sending Quit command.");
                encode_and_write(Message::MenuSelection(Selection::Quit), stream);
            },
            _ => {
                debug!("Sending placeholder message.");
                encode_and_write(Message::Graphic(String::from("do_nothing")), stream);
            }
        }
    }
}
