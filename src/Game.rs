//mod dictionary;
use std::{fmt,fmt::Formatter,slice};

#[derive(Debug,Copy, Clone)]
pub enum PositionInfo {
    Correct(char),
    Incorrect(char),
    Placement(char),
    Unknown
}

impl ToString for PositionInfo {
    fn to_string(&self) -> String {
        match self {
            PositionInfo::Correct(c) => format!("\x1b[0;30;42m {} \x1b[0m ",c),
            PositionInfo::Placement(c) => format!("\x1b[0;30;43m {} \x1b[0m ",c),
            PositionInfo::Incorrect(c) => format!("\x1b[0;30;41m {} \x1b[0m ",c),
            Unknown => "idklol".to_string()
        }
    }
}

#[derive(Debug)]
pub enum GameResult {
    Win(u8),
    Lose,
    Guess(Vec<PositionInfo>)
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameResult::Win(guesses) => write!(f,"You win! Won in {} guesses.",guesses),
            GameResult::Lose => write!(f,"You lose! :("),
            GameResult::Guess(pos) => write!(f,"{}",
                                             pos
                                                 .iter()
                                                 .map(|i| i.to_string())
                                                 .collect::<String>())
        }
    }
}

#[derive(Debug)]
pub enum GameError {
    BadLen(String),
    TooManyGuesses
}

pub struct Game {
    keyword: String,
    pub guesses: u8,
    limit: u8,
}

impl Game {
    pub fn new(keyword: &str, limit: u8) -> Self {
        let kw = keyword.to_string();
        Self {
            keyword: kw, // turn into a vector of chars.
            guesses: 0,
            limit
        }
    }

    pub fn word_len(&self) -> usize {
        self.keyword.len()
    }
    fn process_guess(&mut self, guess: &str) -> GameResult {
        self.guesses += 1;
        match guess {
            _ if guess == self.keyword => {
                GameResult::Win(self.guesses)
            }
            _ => {
                if self.guesses >= self.limit {
                    GameResult::Lose // Lost the game (if this wasnt right you don't get any more guesses...)
                } else {    // Process the guess
                    GameResult::Guess(
                        guess.chars().enumerate()
                            .map(|(i, c)| {
                                let kwc: Vec<_> = self.keyword.chars().collect();
                                match c {
                                    _ if c == kwc[i] => PositionInfo::Correct(c),
                                    _ if kwc.contains(&c) => PositionInfo::Placement(c),
                                    _ => PositionInfo::Incorrect(c),
                                }
                            }).collect()
                    )
                }
            }
        }
    }
    // i suppose we could just accept one type but if it works for anything we can make string why not
    pub fn guess<T: AsRef<str>>(&mut self, guess: T) -> Result<GameResult, GameError> {
        //dbg!(self.guesses);
        let string = guess.as_ref();
        if string.chars().count() != self.keyword.len() { // input validation
            Err(GameError::BadLen(string.to_string()))
        } else {
            match self.guesses {
                _ if self.guesses >= self.limit => Err(GameError::TooManyGuesses),
                _ => Ok(self.process_guess(string))
            }
        }

    }
}