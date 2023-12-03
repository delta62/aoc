mod day01;
mod day02;
mod error;
mod input;
mod solution;

use day01::{Day1Part1Solution, Day1Part2Solution};
use day02::{Day2Part1Solution, Day2Part2Solution};
use solution::UniversalSolution;
use std::fs;

pub fn main() {
    let solutions: Vec<Box<dyn UniversalSolution>> = vec![
        Box::new(Day1Part1Solution::default()),
        Box::new(Day1Part2Solution::default()),
        Box::new(Day2Part1Solution::default()),
        Box::new(Day2Part2Solution::default()),
    ];

    for solution in solutions.iter() {
        let day = solution.day();
        let part = solution.part();

        let input_path = format!("aoc/input/2023/day{day:02}.txt");
        let input = fs::read(input_path).unwrap();
        let result = solution.solve(input.as_slice()).unwrap();

        println!("d{day}p{part}: {result}");
    }
}
