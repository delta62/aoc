#![feature(iterator_try_collect)]

mod error;
mod solution;

pub use error::{parse_error, PuzzleError, Result};
pub use inventory;
pub use solution::{PuzzleAnswer, PuzzleInput, PuzzleSolution, SolutionOutput, UniversalSolution};
