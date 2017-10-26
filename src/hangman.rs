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

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn test() {
    }
}
