use crate::input::Paragraphs;
use aoc_runner::{aoc, parse, parse_opt, PuzzleInput, Result};
use itertools::Itertools;
use std::{cmp, ops::Range};

#[aoc(year = 2023, day = 5, part = 1)]
fn part1(input: Almanac) -> i64 {
    input.min_location_value()
}

#[aoc(year = 2023, day = 5, part = 2)]
fn part2(mut input: Almanac) -> i64 {
    input.seeds_to_ranges();
    input.min_location_value()
}

#[derive(Debug, Default)]
pub struct Almanac {
    seeds: Vec<Range<i64>>,
    stages: Vec<Stage>,
}

impl Almanac {
    fn seeds_to_ranges(&mut self) {
        self.seeds = self
            .seeds
            .chunks(2)
            .map(|chunk| {
                let start = chunk[0].start;
                let end = start + chunk[1].start;

                Range { start, end }
            })
            .collect();
    }

    fn min_location_value(mut self) -> i64 {
        self.stages.reverse();
        let (output_stage, stages) = self.stages.split_first().unwrap();

        output_stage
            .mappings
            .iter()
            .filter_map(|range| find_input_for_output_range(&stages, range))
            .find_map(|range| {
                self.seeds.iter().find_map(|seed_range| {
                    if seed_range.start < range.end && seed_range.start >= range.start {
                        Some(seed_range.start - range.start)
                    } else {
                        None
                    }
                })
            })
            .unwrap()
    }
}

fn find_input_for_output_range(stages: &[Stage], output_range: &Mapping) -> Option<Range<i64>> {
    let (this_stage, rest_stages) = stages.split_first()?;
    let mut mappings = this_stage.find_mappings_to_fulfill(output_range);

    if rest_stages.is_empty() {
        mappings.next().map(|range| range.src)
    } else {
        mappings.find_map(|range| find_input_for_output_range(rest_stages, &range))
    }
}

#[derive(Clone, Debug)]
pub struct Stage {
    /// The mappings that are defined for this stage. Always stored in order
    /// of their outputs, and outputs are guaranteed to be defined from 0..i64::MAX.
    mappings: Vec<Mapping>,
}

impl Stage {
    fn new(mut sparse_ranges: Vec<Mapping>) -> Self {
        sparse_ranges.sort_by(|a, b| a.dest.start.cmp(&b.dest.start));

        let mut last_range_end = 0;
        let mut mappings = Vec::new();

        for range in sparse_ranges {
            if range.dest.start > last_range_end {
                let range = Range {
                    start: last_range_end,
                    end: range.dest.start,
                };

                mappings.push(Mapping {
                    src: range.clone(),
                    dest: range,
                });
            }

            last_range_end = range.dest.end;
            mappings.push(range);
        }

        if last_range_end < i64::MAX {
            let range = Range {
                start: last_range_end,
                end: i64::MAX,
            };

            mappings.push(Mapping {
                src: range.clone(),
                dest: range,
            });
        }

        Self { mappings }
    }

    fn find_mappings_to_fulfill<'a>(
        &'a self,
        range: &'a Mapping,
    ) -> impl Iterator<Item = Mapping> + '_ {
        self.mappings
            .iter()
            .filter_map(|r| r.range_to_satisfy_input(range))
    }

    fn almanac_map(&self, input: i64) -> i64 {
        self.mappings
            .iter()
            .find_map(|range| range.almanac_map(input))
            .unwrap_or(input)
    }
}

#[derive(Clone, Debug)]
pub struct Mapping {
    src: Range<i64>,
    dest: Range<i64>,
}

impl Mapping {
    fn almanac_map(&self, input: i64) -> Option<i64> {
        if self.src.contains(&input) {
            let offset = input - self.src.start;
            let dest = self.dest.start + offset;
            Some(dest)
        } else {
            None
        }
    }

    fn range_to_satisfy_input(&self, downstream: &Self) -> Option<Self> {
        if downstream.src.end < self.dest.start || downstream.src.start >= self.dest.end {
            return None;
        }

        let offset = downstream.src.start - self.dest.start;

        let src = Range {
            start: cmp::max(self.src.start, self.src.start.saturating_add(offset)),
            end: cmp::min(self.src.end, self.src.end.saturating_add(offset)),
        };

        let dest = Range {
            start: cmp::max(self.dest.start, self.dest.start.saturating_add(offset)),
            end: cmp::min(self.dest.end, self.dest.end.saturating_add(offset)),
        };

        Some(Self { src, dest })
    }
}

impl<'a> PuzzleInput<'a> for Almanac {
    fn parse(input: &'a str) -> Result<Self> {
        let mut blocks = Paragraphs::parse(input)?.iter();

        let seeds_block = parse_opt!(blocks.next(), "Input was empty")?;
        let seeds = parse_opt!(seeds_block.strip_prefix("seeds: "), "malformed seeds line")?;
        let seeds = parse!(seeds
            .split_whitespace()
            .map(|x| x.parse::<i64>().map(|x| Range { start: x, end: x }))
            .try_collect())?;

        let almanac = Almanac {
            seeds,
            ..Default::default()
        };

        let mut almanac = blocks.try_fold(almanac, |mut acc, block| {
            acc.stages.push(Stage::parse(block)?);
            Ok(acc)
        })?;

        Ok(almanac)
    }
}

impl<'a> PuzzleInput<'a> for Stage {
    fn parse(input: &'a str) -> Result<Self> {
        let mappings = input.lines().skip(1).map(Mapping::parse).try_collect();
        let mappings = parse!(mappings)?;

        Ok(Self::new(mappings))
    }
}

impl<'a> PuzzleInput<'a> for Mapping {
    fn parse(input: &'a str) -> Result<Self> {
        let mut line = input.split_whitespace().map(|x| parse!(x.parse::<i64>()));
        let dest_start = parse!(parse_opt!(
            line.next(),
            "destination start missing in input"
        )?)?;
        let src_start = parse_opt!(line.next(), "source start missing in input")??;
        let len = parse_opt!(line.next(), "length missing in input")??;

        let src = Range {
            start: src_start,
            end: src_start + len,
        };
        let dest = Range {
            start: dest_start,
            end: dest_start + len,
        };

        Ok(Self { src, dest })
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

    #[test]
    fn example_2() {
        let input = example_str!("2023/d5e1.txt");
        let input = Almanac::parse(&input).unwrap();
        let result = part2(input);
        assert_eq!(result, 46);
    }
}
