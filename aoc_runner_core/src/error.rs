use std::fmt::Display;

#[derive(Debug)]
pub enum PuzzleError {
    Fail,
    ParseError(&'static str),
    SolutionError(String),
}

pub fn parse_error(reason: &'static str) -> PuzzleError {
    PuzzleError::ParseError(reason)
}

impl Display for PuzzleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was an error")
    }
}

impl std::error::Error for PuzzleError {}

pub type Result<T> = std::result::Result<T, PuzzleError>;
