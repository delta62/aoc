mod school;

use school::{Fish, School};

use crate::parse_helpers::parse_values_separated_by;

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Fish> {
    parse_values_separated_by(',', input).unwrap()
}

#[aoc(day6, part1)]
fn part1(input: &[Fish]) -> usize {
    let mut fish = School::from_iter(input.iter().copied());
    (0..80).for_each(|_| fish.simulate());
    fish.count()
}

#[aoc(day6, part2)]
fn part2(input: &[Fish]) -> usize {
    let mut fish = School::from_iter(input.iter().copied());
    (0..256).for_each(|_| fish.simulate());
    fish.count()
}

aoc_tests! { day: 6, part1: 5934, part2: 26984457539 }
