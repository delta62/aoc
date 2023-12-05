use crate::error::{PuzzleError, Result};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct PuzzleAnswer {
    pub parse_duration: Duration,
    pub solve_duration: Duration,
    pub result: Result<String>,
}

pub trait UniversalSolution {
    fn year(&self) -> u16;

    fn day(&self) -> u8;

    fn part(&self) -> u8;

    fn solve(&self, input: &[u8]) -> PuzzleAnswer;
}

inventory::collect!(&'static (dyn UniversalSolution + Sync));

impl<T> UniversalSolution for T
where
    T: PuzzleSolution,
{
    fn year(&self) -> u16 {
        self.year()
    }

    fn day(&self) -> u8 {
        self.day()
    }

    fn part(&self) -> u8 {
        self.part()
    }

    fn solve(&self, input: &[u8]) -> PuzzleAnswer {
        let start_time = Instant::now();

        let input = <T as PuzzleSolution>::Input::parse(input).unwrap();
        let parse_duration = Instant::now().duration_since(start_time);

        let result = self.solve(input).to_string();
        let solve_duration = Instant::now().duration_since(start_time) - parse_duration;

        PuzzleAnswer {
            parse_duration,
            solve_duration,
            result,
        }
    }
}

pub trait PuzzleSolution {
    type Input<'a>: PuzzleInput<'a>;
    type Output: SolutionOutput;

    fn year(&self) -> u16;

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
        let mut lines: Vec<_> = input.split(|x| *x == b'\n').collect();

        if let Some(&[]) = lines.last() {
            lines.pop(); // Empty line at end of file
        }

        lines
            .into_iter()
            .map(|line| T::parse(line))
            .try_collect()
            .map_err(|_| PuzzleError::Fail)
    }
}

pub trait SolutionOutput {
    fn to_string(&self) -> Result<String>;
}

impl SolutionOutput for String {
    fn to_string(&self) -> Result<String> {
        Ok(ToString::to_string(self))
    }
}

impl SolutionOutput for u32 {
    fn to_string(&self) -> Result<String> {
        Ok(ToString::to_string(self))
    }
}

impl SolutionOutput for usize {
    fn to_string(&self) -> Result<String> {
        Ok(ToString::to_string(self))
    }
}
