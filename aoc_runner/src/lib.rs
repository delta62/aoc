#![feature(iterator_try_collect)]

#[macro_use]
mod error;
mod downloader;
mod reporter;
mod runner;
mod solution;

pub use error::{parse_error, PuzzleError, Result};
pub use inventory;
pub use reporter::{DefaultReporter, Reporter};
pub use runner::Runner;
pub use solution::{PuzzleInput, PuzzleSolution, SolutionOutput, UniversalSolution};
