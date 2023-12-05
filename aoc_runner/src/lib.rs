#![feature(iterator_try_collect)]

mod downloader;
mod error;
mod reporter;
mod runner;
mod solution;

pub use error::{PuzzleError, Result};
pub use inventory;
pub use reporter::{DefaultReporter, Reporter};
pub use runner::Runner;
pub use solution::{PuzzleInput, PuzzleSolution, SolutionOutput, UniversalSolution};
