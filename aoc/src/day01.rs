use crate::input::Lines;
use lazy_static::lazy_static;
use regex::Regex;
use runner::aoc;
use std::convert::Infallible;

#[aoc(year = 2023, day = 1, part = 1)]
pub fn part1(input: &Lines) -> u32 {
    input.iter().map(line_sum).sum()
}

#[aoc(year = 2023, day = 1, part = 2)]
pub fn part2(input: &Lines) -> Result<u32, Infallible> {
    Ok(input.iter().map(spelling_sum).sum())
}

fn line_sum(line: &str) -> u32 {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next().unwrap();
    let last = digits.last().unwrap_or(first);

    first * 10 + last
}

fn spelling_sum(line: &str) -> u32 {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
        static ref BACKWARDS_RE: Regex =
            Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d").unwrap();
    }

    let first = parse_digit(RE.find(line).unwrap().as_str());

    let backward: String = line.chars().rev().collect();
    let last = parse_backward_digit(BACKWARDS_RE.find(&backward).unwrap().as_str());

    first * 10 + last
}

fn parse_digit(capture: &str) -> u32 {
    match capture {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        s => s.parse().unwrap(),
    }
}

fn parse_backward_digit(capture: &str) -> u32 {
    match capture {
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        s => s.parse().unwrap(),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::runner::{run_part1, run_part2};

//     #[test]
//     fn example_1_works() {
//         let input = "1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet";
//         let result = run_part1::<Day01>(&input).unwrap();
//         assert_eq!(result, 142);
//     }

//     #[test]
//     fn example_2_works() {
//         let input = "two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen";
//         let result = run_part2::<Day01>(&input).unwrap();
//         assert_eq!(result, 281);
//     }

//     #[test]
//     fn part2_overlap() {
//         let input = "2oneight";
//         let result = run_part2::<Day01>(&input).unwrap();
//         assert_eq!(result, 28);
//     }
// }
