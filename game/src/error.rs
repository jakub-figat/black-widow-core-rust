use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct GameError(pub String);

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid action: {}", self.0)
    }
}

impl Error for GameError {}

pub type GameResult<T> = Result<T, GameError>;
