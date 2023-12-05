use aoc_runner::{PuzzleError, PuzzleInput, Result};

pub struct Lines<'a>(&'a str);

impl<'a> Lines<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a str> {
        self.0.lines()
    }
}

impl<'a> PuzzleInput<'a> for Lines<'a> {
    fn parse(input: &'a [u8]) -> Result<Self> {
        let s = std::str::from_utf8(input).map_err(|_| PuzzleError::Fail)?;
        Ok(Lines::new(s))
    }
}

/// Parse several numbers separated by whitespace or the given separator
macro_rules! numbers {
    ($expr:expr => $type:ty) => {
        $expr
            .split_whitespace()
            .map(|x| x.parse::<$type>())
            .try_collect()
            .map_err(|_| ::aoc_runner::parse_error("Unable to parse whitespace-separated integers"))
    };

    ($expr:expr => $type:ty; $sep:literal) => {
        $expr
            .split($sep)
            .map(|x| x.parse::<$type>())
            .try_collect()
            .map_err(|_| ::aoc_runner::parse_error("Unable to parse separated integers"))
    };
}
