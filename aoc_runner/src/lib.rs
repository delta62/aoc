#![feature(iterator_try_collect)]

mod downloader;
mod reporter;
mod runner;

pub use aoc_runner_core::{
    inventory, parse_error, PuzzleError, PuzzleInput, PuzzleSolution, Result, SolutionOutput,
    UniversalSolution,
};
pub use aoc_runner_macros::aoc;
pub use reporter::{DefaultReporter, Reporter};
pub use runner::Runner;
