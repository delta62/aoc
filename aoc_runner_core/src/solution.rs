use crate::error::{PuzzleError, Result};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct PuzzleAnswer {
    /// The amount of time it took to parse from the raw input string to a data type
    /// required by the solver
    pub parse_duration: Duration,

    /// The amount of time it took to run the solver on the pre-parsed input
    pub solve_duration: Duration,

    /// The result of running the solver
    pub result: Result<String>,
}

/// A generalized Advent of Code solver. You will generally want to
/// implement [`PuzzleSolution`] for your types, and the runtime will
/// automatically convert between those types and this one. The advantage
/// of `PuzzleSolution` over this more generalized trait are the dynamic
/// input and output types - the runtime will be able to automatically parse
/// inputs and handle errors gracefully.
pub trait Solution {
    /// The year this solution is for
    fn year(&self) -> u16;

    /// The day of advent (1-25, implicitly in December) that this solution is for
    fn day(&self) -> u8;

    /// The part of the day (1 or 2) that this solution is for
    fn part(&self) -> u8;

    /// Attempt to solve for the given input, returning the result along with
    /// timing metrics related to each phase of the solution
    fn solve(&self, input: &[u8]) -> PuzzleAnswer;
}

inventory::collect!(&'static (dyn Solution + Sync));

impl<T> Solution for T
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

        let input = <T as PuzzleSolution>::Input::parse(input);
        match input {
            Ok(input) => {
                let parse_duration = Instant::now().duration_since(start_time);
                let result = self.solve(input).to_string();
                let solve_duration = Instant::now().duration_since(start_time) - parse_duration;

                PuzzleAnswer {
                    parse_duration,
                    solve_duration,
                    result,
                }
            }
            Err(err) => PuzzleAnswer {
                parse_duration: Default::default(),
                solve_duration: Default::default(),
                result: Err(err),
            },
        }
    }
}

pub trait PuzzleSolution {
    /// The type of input required to run this solution
    type Input<'a>: PuzzleInput<'a>;

    /// The result of running this solution
    type Output: SolutionOutput;

    /// The year this solution is for
    fn year(&self) -> u16;

    /// The day of advent (1-25, implicitly in December) that this solution is for
    fn day(&self) -> u8;

    /// The part of the day (1 or 2) that this solution is for
    fn part(&self) -> u8;

    /// Attempt to solve for the given input
    fn solve<'i>(&self, input: Self::Input<'i>) -> Self::Output;
}

/// A type which can be provided to an Advent of Code solver function
pub trait PuzzleInput<'a>: Sized {
    /// Attempt to parse an instance of this type from raw puzzle input
    fn parse(input: &'a [u8]) -> Result<Self>;
}

impl<'a> PuzzleInput<'a> for &'a str {
    fn parse(input: &'a [u8]) -> Result<Self> {
        std::str::from_utf8(input)
            .map_err(|_| PuzzleError::ParseError("Input is not valid UTF-8".to_owned()))
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

        lines.into_iter().map(|line| T::parse(line)).try_collect()
    }
}

/// A value which can be returned from an Advent of Code solver function
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
