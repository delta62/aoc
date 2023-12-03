use std::fmt::Display;

#[derive(Debug)]
pub enum PuzzleError {
    Fail,
}

impl Display for PuzzleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was an error")
    }
}

impl std::error::Error for PuzzleError {}

pub type Result<T> = std::result::Result<T, PuzzleError>;
