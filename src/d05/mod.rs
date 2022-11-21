mod geo;

use crate::parse_helpers::parse_lines_to_vec;
use geo::Line;
use std::collections::HashMap;

#[aoc_generator(day5)]
fn parse(input: &str) -> Vec<Line> {
    parse_lines_to_vec(input).unwrap()
}

#[aoc(day5, part1)]
fn part1(input: &[Line]) -> usize {
    let mut board: HashMap<_, u8> = HashMap::new();

    input
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .flat_map(|line| line.points())
        .for_each(|point| {
            *board.entry(point).or_default() += 1;
        });

    board.values().filter(|&&n| n >= 2).count()
}

#[aoc(day5, part2)]
fn part2(input: &[Line]) -> usize {
    let mut board: HashMap<_, u8> = HashMap::new();

    for line in input {
        for point in line.points() {
            *board.entry(point).or_default() += 1;
        }
    }

    board.values().filter(|&&n| n >= 2).count()
}

aoc_tests! { day: 5, part1: 5, part2: 12 }
