use crate::input::Paragraphs;
use aoc_runner::{aoc, parse, parse_opt, PuzzleInput};
use std::ops::Range;

#[aoc(year = 2023, day = 5, part = 1)]
fn part1(input: Almanac) -> usize {
    input.soil_numbers().min().unwrap()
}

#[aoc(year = 2023, day = 5, part = 2)]
fn part2(mut input: Almanac) -> usize {
    input
        .smallest_soil_numbers_from_ranges()
        .into_iter()
        .min()
        .unwrap()
}

#[derive(Default)]
pub struct Almanac {
    seeds: Vec<usize>,
    stages: Vec<Stage>,
}

impl Almanac {
    fn soil_numbers(&self) -> impl Iterator<Item = usize> + '_ {
        self.seeds
            .iter()
            .copied()
            .map(|seed| self.find_soil_value(seed))
    }

    fn smallest_soil_numbers_from_ranges(&mut self) -> Vec<usize> {
        let seed_ranges: Vec<_> = self
            .seeds
            .as_slice()
            .chunks(2)
            .map(|chunk| {
                let start = chunk[0];
                let end = start + chunk[1];
                Range { start, end }
            })
            .collect();

        seed_ranges
            .into_iter()
            .map(|range| self.smallest_soil_for_range(range))
            .collect()
    }

    fn smallest_soil_for_range(&mut self, range: Range<usize>) -> usize {
        todo!()
    }

    fn find_soil_value(&self, seed: usize) -> usize {
        self.stages
            .iter()
            .fold(seed, |seed, stage| stage.almanac_map(seed))
    }
}

pub struct Stage {
    ranges: Vec<MappingRange>,
}

impl Stage {
    fn almanac_map(&self, input: usize) -> usize {
        self.ranges
            .iter()
            .find_map(|range| range.almanac_map(input))
            .unwrap_or(input)
    }
}

pub struct MappingRange {
    dest_start: usize,
    src_start: usize,
    len: usize,
}

impl MappingRange {
    fn almanac_map(&self, input: usize) -> Option<usize> {
        if input < self.src_start || input >= self.src_start + self.len {
            None
        } else {
            let offset = input - self.src_start;
            let dest = self.dest_start + offset;
            Some(dest)
        }
    }
}

impl<'a> PuzzleInput<'a> for Almanac {
    fn parse(input: &'a str) -> aoc_runner::Result<Self> {
        let mut blocks = Paragraphs::parse(input)?.iter();

        let seeds_block = parse_opt!(blocks.next(), "Input was empty")?;
        let seeds = parse_opt!(seeds_block.strip_prefix("seeds: "), "malformed seeds line")?;
        let seeds = parse!(seeds
            .split_whitespace()
            .map(|x| x.parse::<usize>())
            .try_collect())?;

        let almanac = Almanac {
            seeds,
            ..Default::default()
        };

        let almanac = blocks.try_fold(almanac, |mut acc, block| {
            acc.stages.push(Stage::parse(block)?);
            Ok(acc)
        })?;

        Ok(almanac)
    }
}

impl<'a> PuzzleInput<'a> for Stage {
    fn parse(input: &'a str) -> aoc_runner::Result<Self> {
        let ranges = input.lines().skip(1).map(MappingRange::parse).try_collect();
        let ranges = parse!(ranges)?;

        Ok(Self { ranges })
    }
}

impl<'a> PuzzleInput<'a> for MappingRange {
    fn parse(input: &'a str) -> aoc_runner::Result<Self> {
        let mut line = input.split_whitespace().map(|x| parse!(x.parse::<usize>()));
        let dest_start = parse!(parse_opt!(
            line.next(),
            "destination start missing in input"
        )?)?;
        let src_start = parse_opt!(line.next(), "source start missing in input")??;
        let len = parse_opt!(line.next(), "length missing in input")??;

        Ok(Self {
            dest_start,
            src_start,
            len,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = example_str!("2023/d5e1.txt");
        let input = Almanac::parse(&input).unwrap();
        let result = part1(input);
        assert_eq!(result, 35);
    }

    // #[test]
    // fn example_2() {
    //     let input = example_str!("2023/d5e1.txt");
    //     let input = Almanac::parse(&input).unwrap();
    //     let result = part2(input);
    //     assert_eq!(result, 46);
    // }
}
