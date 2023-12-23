use crate::input::Paragraphs;
use aoc_runner::{aoc, PuzzleInput};
use itertools::Itertools;
use std::iter;

#[aoc(year = 2023, day = 13, part = 1)]
fn part1(input: Patterns) -> usize {
    input
        .patterns
        .into_iter()
        .map(|pat| {
            pat.find_horizontal_mirror_col()
                .unwrap_or_else(|| pat.find_vertical_mirror_row().unwrap() * 100)
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
    fn find_horizontal_mirror_col(&self) -> Option<usize> {
        find_mirror(&self.cols)
    }

    fn find_vertical_mirror_row(&self) -> Option<usize> {
        find_mirror(&self.rows)
    }
}

impl<'a> PuzzleInput<'a> for Patterns {
    fn parse(input: &'a str) -> aoc_runner::Result<Self> {
        let patterns = Paragraphs::parse(input)?
            .iter()
            .map(Pattern::parse)
            .try_collect()?;

        Ok(Self { patterns })
    }
}

impl<'a> PuzzleInput<'a> for Pattern {
    fn parse(input: &'a str) -> aoc_runner::Result<Self> {
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

fn find_mirror(values: &[usize]) -> Option<usize> {
    fn all_equal(a: impl Iterator<Item = usize>, b: impl Iterator<Item = usize>) -> bool {
        iter::zip(a, b).all(|(a, b)| a == b)
    }

    (1..values.len()).find(|&i| {
        let (lft, rgt) = values.split_at(i);
        all_equal(lft.iter().copied().rev(), rgt.iter().copied())
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
}
