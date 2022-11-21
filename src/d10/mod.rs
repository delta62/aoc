mod line;

use crate::parse_helpers::parse_lines_to_vec;
use line::Line;

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Line> {
    parse_lines_to_vec(input).unwrap()
}

#[aoc(day10, part1)]
fn part1(lines: &[Line]) -> usize {
    lines
        .iter()
        .cloned()
        .fold(0, |acc, mut line| acc + line.score().unwrap_or_default())
}

#[aoc(day10, part2)]
fn part2(lines: &[Line]) -> usize {
    let mut scores: Vec<_> = lines
        .iter()
        .cloned()
        .filter(|line| line.clone().score().is_none())
        .map(|mut line| line.autocomplete_score())
        .collect();

    let middle_index = scores.len() / 2;
    scores.sort();
    *scores.get(middle_index).unwrap()
}

aoc_tests! { day: 10, part1: 26397, part2: 288957 }
