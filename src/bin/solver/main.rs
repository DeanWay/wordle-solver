use wordle_solver::game::{GameCondition, WordleGame};
use wordle_solver::solver::WordleSolver;
use wordle_solver::solver_strategy::narrowing_random::NarrowingRandomWordleSolver;

fn main() {
    let dictionary = wordle_solver::io::read_dictionary_from_file("assets/dictionary.json")
        .expect("unable to read dictionary file");
    let mut win_total = 0;
    let games = 10000;
    let mut num_guesses = Vec::<usize>::with_capacity(games);
    for _ in 0..games {
        let strategy = NarrowingRandomWordleSolver::new(dictionary.clone());
        let game = WordleGame::new_with_random_secret_word(&dictionary);
        let mut solver = WordleSolver::new(game, Box::new(strategy));
        let result = solver.run_game();
        if result.result == GameCondition::Win {
            win_total += 1;
        }
        num_guesses.push(result.num_guesses)
    }
    let win_percentage = (win_total as f64 / games as f64) * 100_f64;
    let sum_guesses: usize = num_guesses.iter().sum();
    let avg_guesses: f64 = sum_guesses as f64 / games as f64;
    println!("win_total: {win_total} win percentage: {win_percentage}");
    println!("average_guesses: {avg_guesses}");
}
