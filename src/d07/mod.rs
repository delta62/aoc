mod dist;

use crate::parse_helpers::parse_values_separated_by;
use dist::{fuel_cost_of_distances, sum_of_distances, Position};

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Position> {
    parse_values_separated_by(',', input).unwrap()
}

#[aoc(day7, part1)]
fn part1(input: &[Position]) -> usize {
    let max = *input.iter().max().unwrap();

    (0..=max).map(|x| sum_of_distances(x, input)).min().unwrap()
}

#[aoc(day7, part2)]
fn part2(input: &[Position]) -> usize {
    let max = *input.iter().max().unwrap();

    (0..=max)
        .map(|x| fuel_cost_of_distances(x, input))
        .min()
        .unwrap()
}

aoc_tests! { day: 7, part1: 37, part2: 168 }
