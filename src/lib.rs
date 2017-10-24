extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod communication {
    use std::io::{Read, Write};
    use std::net::TcpStream;

    use bincode::{deserialize, serialize, Infinite};
    use serde::Serialize;
    use serde::de::DeserializeOwned;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub enum Command {
        Shutdown
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub enum Message {
        Graphic(String),
        Command(Command),
    }

    pub fn encode_and_write<T>(data: T, stream: &mut TcpStream) where T: Serialize {
        let encoded_data = serialize(&data, Infinite).unwrap();

        stream.write(&encoded_data).unwrap();
    }

    pub fn read_and_decode<T>(stream: &mut TcpStream) -> T where T: DeserializeOwned {
        let mut input_buffer = [0; 256];
        let _ = stream.read(&mut input_buffer).unwrap();

        deserialize(&input_buffer).unwrap()
    }
}
