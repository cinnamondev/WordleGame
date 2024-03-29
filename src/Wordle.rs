//mod dictionary;
use std::{fmt,fmt::Formatter,slice};
use std::fmt::Display;

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
            _ => format!(""),
        }
    }
}


#[derive(Debug)]
pub enum GameResult {
    Win(u8, Vec<PositionInfo>),
    Lose(Vec<PositionInfo>),
    Guess(Vec<PositionInfo>)
}

impl GameResult {
    pub fn parse_guess(guess: &Vec<PositionInfo>) -> String {
        guess
            .iter()
            .map(|i| i.to_string())
            .collect()
    }
}
impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameResult::Win(guesses,pos) => {
                let blocks = GameResult::parse_guess(pos);
                write!(f,"You win! Won in {} guesses. Guess: {}",guesses,blocks)
            },
            GameResult::Lose(pos) => {
                let blocks = GameResult::parse_guess(pos);
                write!(f,"You lose! :(")
            },
            GameResult::Guess(pos) => {
                let blocks = GameResult::parse_guess(pos);
                write!(f,"{}", blocks)
            }
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
        Self {
            keyword: keyword.to_string(),
            guesses: 0,
            limit
        }
    }

    pub fn word_len(&self) -> usize {
        self.keyword.len()
    }

    fn process_blocks(&self, guess: &str) -> Vec<PositionInfo> {
        guess.chars().enumerate()// map each character to the PositionInfo enum
            .map(|(i,c)| {
                let kwc: Vec<_> = self.keyword.chars().collect();
                match c {
                    _ if c == kwc[i] => PositionInfo::Correct(c), // correct character, placement
                    _ if kwc.contains(&c) => PositionInfo::Placement(c), // correct character, incorrect placement
                    _ => PositionInfo::Incorrect(c), // incorrect character
                }
            }).collect()
    }
    fn process_guess(&mut self, guess: &str) -> GameResult {
        self.guesses += 1;
        let pos = self.process_blocks(guess);
        match guess {
            _ if guess == self.keyword => {
                GameResult::Win(self.guesses, pos)
            }
            _ => {
                if self.guesses >= self.limit {
                    GameResult::Lose(pos) // Lost the game (if this wasnt right you don't get any more guesses...)
                } else {    // Process the guess
                    GameResult::Guess(
                        pos
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