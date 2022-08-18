use std::io::{self, Write};
use wordle_solver::game::{GameCondition, GuessResult, LetterResult, WordleGame};

fn main() {
    let dictionary = wordle_solver::io::read_dictionary_from_file("assets/dictionary.json")
        .expect("unable to read dictionary file");
    let mut game = WordleGame::new_with_random_secret_word(&dictionary);
    loop {
        print!("Make guess: ");
        io::stdout().flush().expect("unable to flush stdout");
        let input = get_line().expect("unable to read line");
        if let Err(msg) = game.make_guess(&input) {
            println!("{}", msg);
            continue;
        }
        let game_state = game.game_state();
        let guess_result = game_state.guesses.last().unwrap();
        println!("{}", guess_colors(guess_result));
        if game_state.condition == GameCondition::Win {
            println!("You Win!");
            break;
        } else if game_state.condition == GameCondition::Loss {
            println!("You Lose!");
            break;
        }
    }
}

fn get_line() -> io::Result<String> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

fn guess_colors(letter_results: &GuessResult) -> String {
    letter_results
        .iter()
        .map(|(_, r)| match r {
            LetterResult::CorrectPlacement => "🟩",
            LetterResult::CorrectLetter => "🟨",
            LetterResult::IncorrectLetter => "⬛️",
        })
        .collect::<Vec<&str>>()
        .join("")
}
