use std::str::FromStr;

#[derive(Debug)]
pub enum ParseError {
    IllegalCharacter(char, Vec<char>),
    UnexpectedEof,
}

impl ParseError {
    pub fn illegal_char(got: char, expected: Vec<char>) -> Self {
        Self::IllegalCharacter(got, expected)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParseError {}

pub fn parse_lines_to_vec<'a, E, R>(input: &'a str) -> Result<Vec<R>, E>
where
    R: FromStr<Err = E>,
{
    input.lines().try_fold(vec![], |mut acc, line| {
        acc.push(line.parse()?);
        Ok(acc)
    })
}

pub fn parse_values_separated_by<E, R>(pattern: char, input: &str) -> Result<Vec<R>, E>
where
    R: FromStr<Err = E>,
{
    input.trim().split(pattern).map(|n| n.parse()).try_collect()
}
