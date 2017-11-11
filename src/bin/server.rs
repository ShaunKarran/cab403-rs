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
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};

use clap::App;
use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};
use threadpool::ThreadPool;

use cab403_rs::communication::{Command, Message, ServerMessage, Selection, State, encode_and_write, read_and_decode};
use cab403_rs::graphics;

fn main() {
    // Initialise logger.
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
            WriteLogger::new(LogLevelFilter::Debug, Config::default(), File::create("hangman-server.log").unwrap()),
        ]
    ).unwrap();

    // Parse command line arguments.
    let yaml = load_yaml!("server_cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Bind the server's socket.
    let port = value_t!(matches, "port", u16).unwrap();
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    info!("Listening on socket {}", socket);
    let listener = TcpListener::bind(socket).unwrap();

    // Create a pool of threads to handle clients as they connect.
    let pool = ThreadPool::new(10);
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        info!("Accepted connection from {:?}", stream);

        pool.execute(move || handle_client(&mut stream));
    }
}

fn handle_client(stream: &mut TcpStream) {
    debug!("Sending welcome message to {:?}", stream);
    encode_and_write(Message::Graphic(String::from(graphics::WELCOME_MESSAGE)), stream);
    debug!("Sending main menu to {:?}", stream);
    encode_and_write(Message::Graphic(String::from(graphics::MAIN_MENU)), stream);
    // encode_and_write(ServerMessage { state: State::MainMenu, graphic: welcome_message }, stream);

    'main: loop {
        let message = read_and_decode(stream);
        match message {
            Message::MenuSelection(Selection::PlayHangman) => { play_hangman() },
            Message::MenuSelection(Selection::ShowLeaderboard) => { show_leaderboard() },
            Message::MenuSelection(Selection::Quit) => {
                debug!("Sending Shutdown command to {:?}", stream);
                encode_and_write(Message::Command(Command::Shutdown), stream);
                break 'main;
            },
            _ => { continue 'main },
        };
    }

    info!("Closing connection from {:?}", stream);
}

fn play_hangman() {}

fn show_leaderboard() {}
