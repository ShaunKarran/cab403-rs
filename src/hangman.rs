use std::cmp::min;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Game {
    word: String,
    word_character_set: HashSet<char>,
    number_allowed_guesses: usize,
    guesses: Vec<char>,
    guesses_character_set: HashSet<char>,
}

impl Game {
    pub fn new(word_set: HashSet<String>) -> Game {
        let word = Game::select_word(word_set);
        let word_character_set = HashSet::from_iter(word.chars());
        let number_allowed_guesses = Game::calculate_number_allowed_guesses(&word);

        Game {
            word: word,
            word_character_set: word_character_set,
            number_allowed_guesses: number_allowed_guesses,
            guesses: Vec::new(),
            guesses_character_set: HashSet::new(),
        }
    }

    fn select_word(word_set: HashSet<String>) -> String {
        String::from("placeholder")
    }

    fn calculate_number_allowed_guesses(word: &String) -> usize {
        // Number of guesses = min{length of word 1 + length of word 2 + 10, 26}
        min(word.len() + 10, 26)
    }

    pub fn update_guess(mut self, guess: char) {
        self.guesses.push(guess);
        self.guesses_character_set.insert(guess);
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn test() {
    }
}
