use std::collections::HashSet;

use rand::seq::SliceRandom;
pub struct WordleGame {
    dictionary: HashSet<String>,
    guesses: Vec<GuessResult>,
    secret_word: String,
    max_guesses: usize,
}

pub type GuessResult = Vec<(char, LetterResult)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LetterResult {
    CorrectPlacement,
    CorrectLetter,
    IncorrectLetter,
}

#[derive(Debug)]
pub struct WordleGameState<'a> {
    pub guesses: &'a Vec<GuessResult>,
    pub condition: GameCondition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCondition {
    Win,
    Loss,
    Playing,
}

impl WordleGame {
    pub fn new(dictionary: &[String], secret_word: &str) -> Self {
        let dictionary_set = HashSet::from_iter(dictionary.iter().cloned());
        let secret_word = secret_word.to_owned();
        assert!(dictionary_set.contains(&secret_word));

        Self {
            dictionary: dictionary_set,
            guesses: vec![],
            secret_word,
            max_guesses: 6,
        }
    }

    pub fn new_with_random_secret_word(dictionary: &[String]) -> Self {
        let secret_word = dictionary.choose(&mut rand::thread_rng()).unwrap().clone();
        Self::new(dictionary, &secret_word)
    }

    pub fn make_guess(&mut self, guess: &str) -> Result<(), &str> {
        if !self.dictionary.contains(guess) {
            return Err("Invalid word");
        }
        if self.guesses.len() >= self.max_guesses {
            return Err("No more guesses available");
        }
        let guess_result = Self::check_guess(guess, &self.secret_word);
        self.guesses.push(guess_result);
        Ok(())
    }

    pub fn game_state(&self) -> WordleGameState {
        WordleGameState {
            guesses: &self.guesses,
            condition: self.game_condition(),
        }
    }

    pub fn game_condition(&self) -> GameCondition {
        let has_won = self.guesses.iter().any(|guess| {
            guess
                .iter()
                .all(|(_, res)| *res == LetterResult::CorrectPlacement)
        });
        if has_won {
            GameCondition::Win
        } else if self.guesses.len() >= self.max_guesses {
            GameCondition::Loss
        } else {
            GameCondition::Playing
        }
    }

    fn check_guess(guess: &str, secret_word: &str) -> GuessResult {
        let mut guess_result = Vec::with_capacity(guess.len());
        let mut letter_counts = crate::util::unique_element_counts(secret_word.chars());
        let zipped = guess.chars().zip(secret_word.chars());
        for (guess_c, selected_c) in zipped {
            if guess_c == selected_c {
                guess_result.push((guess_c, LetterResult::CorrectPlacement));
                *letter_counts.get_mut(&selected_c).unwrap() -= 1;
            } else {
                guess_result.push((guess_c, LetterResult::IncorrectLetter))
            }
        }

        for (i, guess_c) in guess.chars().enumerate() {
            if let Some(count) = letter_counts.get_mut(&guess_c) {
                if *count > 0 && guess_result[i].1 != LetterResult::CorrectPlacement {
                    guess_result[i] = (guess_c, LetterResult::CorrectLetter);
                    *count -= 1;
                }
            }
        }
        guess_result
    }
}

#[cfg(test)]
mod test_get_guess_result {
    use super::{LetterResult::*, WordleGame};

    #[test]
    fn repeated_correct_letter_picks_first() {
        let guess = "slate";
        let secret = "salad";
        let expected_result = vec![
            ('s', CorrectPlacement),
            ('l', CorrectLetter),
            ('a', CorrectLetter),
            ('t', IncorrectLetter),
            ('e', IncorrectLetter),
        ];
        assert_eq!(WordleGame::check_guess(guess, secret), expected_result)
    }

    #[test]
    fn all_incorrect() {
        let guess = "would";
        let secret = "crate";
        let expected_result = vec![
            ('w', IncorrectLetter),
            ('o', IncorrectLetter),
            ('u', IncorrectLetter),
            ('l', IncorrectLetter),
            ('d', IncorrectLetter),
        ];
        assert_eq!(WordleGame::check_guess(guess, secret), expected_result)
    }

    #[test]
    fn all_correct_placement() {
        let guess = "slate";
        let secret = "slate";
        let expected_result = vec![
            ('s', CorrectPlacement),
            ('l', CorrectPlacement),
            ('a', CorrectPlacement),
            ('t', CorrectPlacement),
            ('e', CorrectPlacement),
        ];
        assert_eq!(WordleGame::check_guess(guess, secret), expected_result)
    }

    #[test]
    fn all_correct_letter() {
        let guess = "tares";
        let secret = "stare";
        let expected_result = vec![
            ('t', CorrectLetter),
            ('a', CorrectLetter),
            ('r', CorrectLetter),
            ('e', CorrectLetter),
            ('s', CorrectLetter),
        ];
        assert_eq!(WordleGame::check_guess(guess, secret), expected_result)
    }

    #[test]
    fn correct_placement_captures_letters() {
        let guess = "lllll";
        let secret = "hello";
        let expected_result = vec![
            ('l', IncorrectLetter),
            ('l', IncorrectLetter),
            ('l', CorrectPlacement),
            ('l', CorrectPlacement),
            ('l', IncorrectLetter),
        ];
        assert_eq!(WordleGame::check_guess(guess, secret), expected_result)
    }

    #[test]
    fn correct_placement_and_correct_letter() {
        let guess = "llzll";
        let secret = "hello";
        let expected_result = vec![
            ('l', CorrectLetter),
            ('l', IncorrectLetter),
            ('z', IncorrectLetter),
            ('l', CorrectPlacement),
            ('l', IncorrectLetter),
        ];
        assert_eq!(WordleGame::check_guess(guess, secret), expected_result)
    }
}
