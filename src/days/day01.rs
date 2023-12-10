use aoc_runner::{aoc, PuzzleError, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    static ref BACKWARDS_RE: Regex =
        Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d").unwrap();
}

#[aoc(year = 2023, day = 1, part = 1)]
fn part1<'a>(input: &'a str) -> u32 {
    input
        .lines()
        .map(line_sum)
        .try_fold(0, |acc, n| n.map(|n| acc + n))
        .unwrap()
}

#[aoc(year = 2023, day = 1, part = 2)]
fn part2<'a>(input: &'a str) -> u32 {
    input
        .lines()
        .map(spelling_sum)
        .try_fold(0, |acc, n| n.map(|n| acc + n))
        .unwrap()
}

fn line_sum(line: &str) -> Result<u32> {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next().unwrap();
    let last = digits.last().unwrap_or(first);

    Ok(first * 10 + last)
}

fn spelling_sum(line: &str) -> Result<u32> {
    let capture = RE
        .find(line)
        .ok_or(PuzzleError::ParseError(
            "Unable to find first number".to_owned(),
        ))?
        .as_str();
    let first = parse_digit(capture)?;

    let backward: String = line.chars().rev().collect();
    let capture: String = BACKWARDS_RE
        .find(&backward)
        .ok_or(PuzzleError::ParseError(
            "Unable to find last number".to_owned(),
        ))?
        .as_str()
        .chars()
        .rev()
        .collect();
    let last = parse_digit(&capture)?;

    Ok(first * 10 + last)
}

fn parse_digit(capture: &str) -> Result<u32> {
    match capture {
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        s => s
            .parse()
            .map_err(|_| PuzzleError::ParseError(format!("Unable to parse digit from {s}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = example_str!("2023/d1e1.txt");
        let result = part1(&input);
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn example_2() {
        let input = example_str!("2023/d1e2.txt");
        let result = part2(&input);
        assert_eq!(result.unwrap(), 281);
    }

    #[test]
    fn part2_overlap() {
        let input = "2oneight";
        let result = part2(&input);
        assert_eq!(result.unwrap(), 28);
    }
}
