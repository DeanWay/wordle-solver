#![allow(dead_code)]
use std::collections::HashSet;

use super::SolverStrategy;
use crate::{constraint, game::WordleGameState};
use rand::seq::IteratorRandom;

pub struct NarrowingRandomWordleSolver {
    dictionary: HashSet<String>,
}

impl NarrowingRandomWordleSolver {
    pub fn new(dictionary: Vec<String>) -> Self {
        Self {
            dictionary: HashSet::from_iter(dictionary),
        }
    }

    fn narrow(&mut self, game_state: &WordleGameState) {
        if !game_state.guesses.is_empty() {
            let d = &mut self.dictionary;
            d.retain(|word| {
                constraint::word_satisfies_contraint(word, game_state.guesses.last().unwrap())
            });
        }
    }
}

impl SolverStrategy for NarrowingRandomWordleSolver {
    fn next_guess(&mut self, game_state: &WordleGameState) -> String {
        self.narrow(game_state);
        self.dictionary
            .iter()
            .choose(&mut rand::thread_rng())
            .expect("impossible to win if we run out of options")
            .to_string()
    }
}
