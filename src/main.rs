mod day01;
mod input;
mod runner;

use crate::runner::{run_part1, run_part2};
use day01::Day01;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("input/day01.txt").unwrap();

    let result = run_part1::<Day01>(&input).unwrap();
    println!("{result}");

    let result = run_part2::<Day01>(&input).unwrap();
    println!("{result}");
}
