use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum GameError {
    InvalidAction(String),
    InvalidPayload(String)
}


impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::InvalidAction(message) => write!(f, "Invalid action: {}", message),
            GameError::InvalidPayload(message) => write!(f, "Invalid payload: {}", message)
        }
    }
}

impl Error for GameError {}

pub type GameResult<T> = Result<T, GameError>;
