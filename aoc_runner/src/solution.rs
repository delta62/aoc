use crate::error::{PuzzleError, Result};
use std::{fmt::Display, str::FromStr};

pub trait UniversalSolution {
    fn day(&self) -> u8;

    fn part(&self) -> u8;

    fn solve(&self, input: &[u8]) -> Result<String>;
}

impl<T> UniversalSolution for T
where
    T: PuzzleSolution,
{
    fn day(&self) -> u8 {
        self.day()
    }

    fn part(&self) -> u8 {
        self.part()
    }

    fn solve(&self, input: &[u8]) -> Result<String> {
        let input = <T as PuzzleSolution>::Input::parse(input)?;
        Ok(self.solve(input).to_string())
    }
}

pub trait PuzzleSolution {
    type Input<'a>: PuzzleInput<'a>;
    type Output: SolutionOutput;

    fn day(&self) -> u8;

    fn part(&self) -> u8;

    fn solve<'i>(&self, input: Self::Input<'i>) -> Self::Output;
}

pub trait PuzzleInput<'a>: Sized {
    fn parse(input: &'a [u8]) -> Result<Self>;
}

impl<'a> PuzzleInput<'a> for &'a str {
    fn parse(input: &'a [u8]) -> Result<Self> {
        std::str::from_utf8(input).map_err(|_| PuzzleError::Fail)
    }
}

impl<'a, T> PuzzleInput<'a> for Vec<T>
where
    T: PuzzleInput<'a>,
{
    fn parse(input: &'a [u8]) -> Result<Self> {
        input.split(|x| *x == b'\n')
        .map(|line| T::parse(line)).try_collect().map_err(|_| PuzzleError::Fail)
    }
}

pub trait SolutionOutput {
    fn to_string(&self) -> String;
}

impl SolutionOutput for String {
    fn to_string(&self) -> String {
        ToString::to_string(self)
    }
}

impl SolutionOutput for u32 {
    fn to_string(&self) -> String {
        ToString::to_string(self)
    }
}

impl SolutionOutput for usize {
    fn to_string(&self) -> String {
        ToString::to_string(self)
    }
}

impl<T, E> SolutionOutput for std::result::Result<T, E>
where
    T: Display,
    E: Display,
{
    fn to_string(&self) -> String {
        match self {
            Ok(solution) => solution.to_string(),
            Err(err) => err.to_string(),
        }
    }
}
