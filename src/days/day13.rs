use crate::input::Paragraphs;
use aoc_runner::{aoc, PuzzleInput, Result};
use itertools::Itertools;
use std::iter;

#[aoc(year = 2023, day = 13, part = 1)]
fn part1(input: Patterns) -> usize {
    input
        .patterns
        .into_iter()
        .map(|pat| {
            pat.find_horizontal_mirror_col(0)
                .unwrap_or_else(|| pat.find_vertical_mirror_row(0).unwrap() * 100)
        })
        .sum()
}

#[aoc(year = 2023, day = 13, part = 2)]
fn part2(input: Patterns) -> usize {
    input
        .patterns
        .into_iter()
        .map(|pat| {
            pat.find_horizontal_mirror_col(1)
                .unwrap_or_else(|| pat.find_vertical_mirror_row(1).unwrap() * 100)
        })
        .sum()
}

pub struct Patterns {
    patterns: Vec<Pattern>,
}

pub struct Pattern {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl Pattern {
    fn find_horizontal_mirror_col(&self, delta: usize) -> Option<usize> {
        find_mirror_with_delta(&self.cols, delta)
    }

    fn find_vertical_mirror_row(&self, delta: usize) -> Option<usize> {
        find_mirror_with_delta(&self.rows, delta)
    }
}

impl<'a> PuzzleInput<'a> for Patterns {
    fn parse(input: &'a str) -> Result<Self> {
        let patterns = Paragraphs::parse(input)?
            .iter()
            .map(Pattern::parse)
            .try_collect()?;

        Ok(Self { patterns })
    }
}

impl<'a> PuzzleInput<'a> for Pattern {
    fn parse(input: &'a str) -> Result<Self> {
        let rows = input.lines().map(|line| line.chars()).map(hash).collect();
        let cols = transpose(input)
            .iter()
            .map(|col| col.chars())
            .map(hash)
            .collect();

        Ok(Self { cols, rows })
    }
}

fn hash(input: impl DoubleEndedIterator<Item = char>) -> usize {
    input.rev().enumerate().fold(0, |acc, (i, c)| match c {
        '#' => acc + (1 << i),
        '.' => acc,
        _ => unimplemented!(),
    })
}

fn find_mirror_with_delta(values: &[usize], delta: usize) -> Option<usize> {
    fn find_delta(a: impl Iterator<Item = usize>, b: impl Iterator<Item = usize>) -> usize {
        iter::zip(a, b)
            .map(|(a, b)| (a ^ b).count_ones())
            .sum::<u32>() as usize
    }

    (1..values.len()).find(|&i| {
        let (lft, rgt) = values.split_at(i);
        find_delta(lft.iter().copied().rev(), rgt.iter().copied()) == delta
    })
}

fn transpose(input: &str) -> Vec<String> {
    let col_length = input
        .lines()
        .next()
        .map(|line| line.len())
        .unwrap_or_default();
    let accumulator = vec![String::new(); col_length];

    input
        .lines()
        .into_iter()
        .fold(accumulator, |mut acc, line| {
            line.chars().enumerate().for_each(|(i, c)| acc[i].push(c));
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_part1() {
        let input = example_str!("2023/d13e1.txt");
        let input = Patterns::parse(&input).unwrap();
        let result = part1(input);
        assert_eq!(result, 405);
    }

    #[test]
    fn example2_part1() {
        let input = example_str!("2023/d13e1.txt");
        let input = Patterns::parse(&input).unwrap();
        let result = part2(input);
        assert_eq!(result, 400);
    }
}
