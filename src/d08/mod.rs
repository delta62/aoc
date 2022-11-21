mod entry;

use crate::parse_helpers::parse_lines_to_vec;
use entry::Entry;

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Entry> {
    parse_lines_to_vec(input).unwrap()
}

#[aoc(day8, part1)]
fn part1(input: &[Entry]) -> usize {
    input
        .iter()
        .flat_map(|entry| entry.output())
        .map(|signal_pattern| signal_pattern.try_identify())
        .filter(|maybe_num| maybe_num.is_some())
        .count()
}

#[aoc(day8, part2)]
fn part2(input: &[Entry]) -> usize {
    input.iter().map(|entry| entry.output_val()).sum()
}

aoc_tests! { day: 8, part1: 26, part2: 61229 }
