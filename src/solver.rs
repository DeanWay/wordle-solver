use crate::{
    game::{GameCondition, WordleGame},
    solver_strategy::SolverStrategy,
};

pub struct WordleSolver {
    game: WordleGame,
    strategy: Box<dyn SolverStrategy>,
}

#[derive(Debug)]
pub struct WordleGameScore {
    pub num_guesses: usize,
    pub result: GameCondition,
}

impl WordleSolver {
    pub fn new(game: WordleGame, strategy: Box<dyn SolverStrategy>) -> Self {
        WordleSolver { game, strategy }
    }

    pub fn run_game(&mut self) -> WordleGameScore {
        loop {
            let game_state = self.game.game_state();
            if game_state.condition != GameCondition::Playing {
                return WordleGameScore {
                    num_guesses: game_state.guesses.len(),
                    result: game_state.condition,
                };
            }
            let guess = self.strategy.next_guess(&game_state);
            if self.game.make_guess(&guess).is_err() {
                continue;
            }
        }
    }
}
