#![feature(iterator_try_collect)]

mod error;
mod solution;

pub use error::{PuzzleError, Result};
pub use inventory;
pub use solution::{PuzzleAnswer, PuzzleInput, PuzzleSolution, Solution, SolutionOutput};
