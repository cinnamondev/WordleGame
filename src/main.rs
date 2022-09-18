use std::io;
mod Wordle;
mod ui;

use crate::Wordle::{Game, GameResult, GameError};

fn main() -> io::Result<()> {
    let mut game = Game::new("helloworld", 6);
    loop {
        let mut buffer = String::new();
        let mut stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer)?;
        match game.guess(buffer.trim()) {
            Ok(g) => {
                match g {
                    GameResult::Win(guesses, positions) => {
                        println!("{} You won in {} guesses,",GameResult::parse_guess(&positions), guesses);
                        break;
                    },
                    GameResult::Guess(positions) => {
                        println!("{} +1 guess.", GameResult::parse_guess(&positions));
                    },
                    GameResult::Lose(positions) => {
                        println!("{} You lost!", GameResult::parse_guess(&positions));
                        break;
                    },
                }},
            Err(e) => {
                match e {
                    GameError::BadLen(_) => println!("Too short/long guess! Word length is {} letters.", game.word_len()),
                    _ => {dbg!(e);break;}
                }
            },
        }
    }
    Ok(())
}