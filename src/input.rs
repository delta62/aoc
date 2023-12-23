use aoc_runner::{PuzzleInput, Result};

/// An iterator of paragraphs of text. Each paragraph is denoted by
/// an empty line. The empty lines separating the paragraphs are omitted
/// from the iterator's output.
pub struct Paragraphs<'a>(&'a str);

impl<'a> Paragraphs<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a str> {
        self.0.strip_suffix('\n').unwrap_or(self.0).split("\n\n")
    }
}

impl<'a> PuzzleInput<'a> for Paragraphs<'a> {
    fn parse(input: &'a str) -> Result<Self> {
        Ok(Self::new(input))
    }
}

/// Parse several numbers separated by whitespace or the given separator
macro_rules! numbers {
    ($expr:expr => $type:ty) => {{
        use ::itertools::Itertools;

        $expr
            .split_whitespace()
            .map(|x| x.parse::<$type>())
            .try_collect()
            .map_err(|_| {
                ::aoc_runner::PuzzleError::ParseError(
                    "Unable to parse whitespace-separated integers".to_string(),
                )
            })
    }};

    ($expr:expr => $type:ty; $sep:literal) => {{
        use ::itertools::Itertools;

        $expr
            .split($sep)
            .map(|x| x.parse::<$type>())
            .try_collect()
            .map_err(|_| {
                ::aoc_runner::PuzzleError::ParseError(
                    "Unable to parse separated integers".to_string(),
                )
            })
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paragraphs_breaks_on_empty_lines() {
        let p = Paragraphs::new("a\n\nb\n");
        let mut iter = p.iter();

        assert_eq!(Some("a"), iter.next());
        assert_eq!(Some("b"), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn paragraphs_yields_a_single_line() {
        let p = Paragraphs::new("a\n");
        let mut iter = p.iter();

        assert_eq!(Some("a"), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn paragraphs_yields_a_single_line_with_no_newline() {
        let p = Paragraphs::new("a");
        let mut iter = p.iter();

        assert_eq!(Some("a"), iter.next());
        assert_eq!(None, iter.next());
    }
}
