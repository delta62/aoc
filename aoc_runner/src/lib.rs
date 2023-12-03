#![feature(iterator_try_collect)]

mod error;
mod runner;
mod solution;

pub use error::{PuzzleError, Result};
pub use runner::Runner;
pub use solution::{UniversalSolution, PuzzleSolution, PuzzleInput, SolutionOutput};
