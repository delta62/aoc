use aoc_runner::{aoc, parse, parse_opt, PuzzleError, PuzzleInput, Result};
use itertools::Itertools;
use std::str::FromStr;

#[aoc(year = 2023, day = 2, part = 1)]
fn part1(input: Vec<Game>) -> usize {
    let config = Sample {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .into_iter()
        .filter(|game| game.is_configuration_possible(&config))
        .map(|game| game.id)
        .sum()
}

#[aoc(year = 2023, day = 2, part = 2)]
fn part2(input: Vec<Game>) -> usize {
    input.into_iter().map(|game| game.min_power()).sum()
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    samples: Vec<Sample>,
}

impl<'a> PuzzleInput<'a> for Game {
    fn parse(input: &'a str) -> Result<Self> {
        let (game_id, samples) = parse_opt!(
            input.split_once(':'),
            "No ':' separating game ID from samples"
        )?;
        let game_id = parse_opt!(
            game_id.strip_prefix("Game "),
            "No 'Game ' at beginning of input line"
        )?;
        let id = parse!(game_id.parse::<usize>(), "Game ID was not an integer")?;

        let samples = samples.split(';').map(|s| s.trim().parse()).try_collect()?;

        Ok(Self { id, samples })
    }
}

impl Game {
    fn min_power(&self) -> usize {
        self.samples
            .iter()
            .fold(MaybeSample::default(), |mut acc, sample| {
                acc.red = acc
                    .red
                    .map(|val| usize::max(val, sample.red))
                    .or(Some(sample.red));

                acc.green = acc
                    .green
                    .map(|val| usize::max(val, sample.green))
                    .or(Some(sample.green));

                acc.blue = acc
                    .blue
                    .map(|val| usize::max(val, sample.blue))
                    .or(Some(sample.blue));

                acc
            })
            .power()
    }

    fn is_configuration_possible(&self, config: &Sample) -> bool {
        self.samples.iter().all(|sample| sample.subset_of(config))
    }
}

#[derive(Debug, Default)]
struct MaybeSample {
    red: Option<usize>,
    green: Option<usize>,
    blue: Option<usize>,
}

impl MaybeSample {
    fn power(&self) -> usize {
        self.red.unwrap_or_default()
            * self.green.unwrap_or_default()
            * self.blue.unwrap_or_default()
    }
}

#[derive(Debug, Default)]
struct Sample {
    red: usize,
    blue: usize,
    green: usize,
}

impl FromStr for Sample {
    type Err = PuzzleError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        s.split(',')
            .map(|s| s.trim())
            .try_fold(Sample::default(), |mut acc, s| {
                let (quantity, color) = s.split_once(' ').ok_or(PuzzleError::ParseError(
                    "no space separating quantity from color".to_string(),
                ))?;
                let quantity = quantity.parse::<usize>().map_err(|_| {
                    PuzzleError::ParseError("quantity was not an integer".to_string())
                })?;

                match color {
                    "red" => acc.red += quantity,
                    "green" => acc.green += quantity,
                    "blue" => acc.blue += quantity,
                    _ => unreachable!(),
                }

                Ok(acc)
            })
    }
}

impl Sample {
    fn subset_of(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = example_str!("2023/d2e1.txt");
        let input = <Vec<Game>>::parse(&input).unwrap();
        let solution = part1(input);
        assert_eq!(solution, 8);
    }

    #[test]
    fn example_2() {
        let input = example_str!("2023/d2e1.txt");
        let input = <Vec<Game>>::parse(&input).unwrap();
        let solution = part2(input);
        assert_eq!(solution, 2286);
    }
}
