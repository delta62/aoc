use aoc_runner::{PuzzleInput, Result};

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
        let s = <&str as PuzzleInput>::parse(input)?;
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
            .map_err(|_| {
                ::aoc_runner::PuzzleError::ParseError(
                    "Unable to parse whitespace-separated integers".to_string(),
                )
            })
    };

    ($expr:expr => $type:ty; $sep:literal) => {
        $expr
            .split($sep)
            .map(|x| x.parse::<$type>())
            .try_collect()
            .map_err(|_| {
                ::aoc_runner::PuzzleError::ParseError(
                    "Unable to parse separated integers".to_string(),
                )
            })
    };
}
