use std::io;
use crate::Game::GameResult;

mod Game;
fn main() -> io::Result<()> {
    let mut game = Game::Game::new("foo", 6);
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
            Err(e) => {dbg!(e);break;},
        }
    }
    Ok(())
}