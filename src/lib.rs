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

pub mod hangman {
    use std::cmp::min;
    use std::collections::HashSet;

    pub struct Game {
        word: String,
        guesses: Vec<u8>,
        number_allowed_guesses: usize,
    }

    impl Game {
        pub fn new(word_set: HashSet<String>) -> Game {
            let word = Game::select_word(word_set);
            let number_allowed_guesses = Game::calculate_number_allowed_guesses(&word);

            Game {
                word: word,
                guesses: Vec::new(),
                number_allowed_guesses: number_allowed_guesses,
            }
        }

        fn select_word(word_set: HashSet<String>) -> String {
            String::from("placeholder")
        }

        fn calculate_number_allowed_guesses(word: &String) -> usize {
            // Number of guesses = min{length of word 1 + length of word 2 + 10, 26}
            min(word.len() + 10, 26)
        }

        pub fn update_guess(mut self, guess: u8) {
            self.guesses.push(guess);
        }
    }
}
