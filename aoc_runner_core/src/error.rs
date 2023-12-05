use std::fmt::Display;

#[derive(Debug)]
pub enum PuzzleError {
    ParseError(String),
    SolutionError(String),
}

impl Display for PuzzleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PuzzleError::*;

        match self {
            ParseError(s) => write!(f, "Error while parsing input: {s}"),
            SolutionError(s) => write!(f, "Error while running solution: {s}"),
        }
    }
}

impl std::error::Error for PuzzleError {}

pub type Result<T> = std::result::Result<T, PuzzleError>;

/// Convenience macro for parsing something which may fail, converting to a solution
/// error if anything goes wrong
#[macro_export]
macro_rules! parse {
    ($expr:expr) => {
        $expr.map_err(|err| ::aoc_runner::PuzzleError::ParseError(err.to_string()))
    };

    ($expr:expr, $message:literal) => {
        $expr.map_err(|_| ::aoc_runner::PuzzleError::ParseError($message.to_string()))
    };
}

/// Convenience macro for parsing something which may return None, converting to a solution
/// error if anything goes wrong
#[macro_export]
macro_rules! parse_opt {
    ($expr:expr, $message:literal) => {
        $expr.ok_or_else(|| ::aoc_runner::PuzzleError::ParseError($message.to_string()))
    };
}
