use std::io;
mod Wordle;
mod ui;

use crate::Wordle::{Game, GameResult, GameError};

fn main() -> io::Result<()> {
    let mut game = Game::new("foo", 6);
    loop {
        let mut buffer = String::new();
        let mut stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer)?;
        match game.guess(buffer.trim()) {
            Ok(g) => {
                match g {
                    GameResult::Win(_) => {println!("{}",g);break},
                    GameResult::Lose => {println!("{}",g);break;},
                    _ => println!("{}",g)
                }},
            Err(e) => {
                match e {
                    GameError::BadLen(_) => println!("Too short/long guess! A wordle word is 5 letters."),
                    _ => {dbg!(e);break;}
                }
            },
        }
    }
    Ok(())
}