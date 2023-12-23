use aoc_runner::{aoc, parse, parse_opt, PuzzleInput};
use itertools::Itertools;

#[aoc(year = 2023, day = 6, part = 1)]
fn part1(input: Races) -> usize {
    input
        .races
        .into_iter()
        .map(|race| race.num_winning_times())
        .product()
}

#[aoc(year = 2023, day = 6, part = 2)]
fn part2(input: Race) -> usize {
    input.num_winning_times()
}

pub struct Races {
    races: Vec<Race>,
}

pub struct Race {
    time_ms: usize,
    distance_mm: usize,
}

impl Race {
    fn num_winning_times(&self) -> usize {
        (0..self.time_ms)
            .map(|ms| self.distance_for_button_ms(ms))
            .filter(|&distance| distance > self.distance_mm)
            .count()
    }

    fn distance_for_button_ms(&self, ms: usize) -> usize {
        let race_time = self.time_ms - ms;
        race_time * ms
    }
}

impl<'a> PuzzleInput<'a> for Races {
    fn parse(input: &'a str) -> aoc_runner::Result<Self> {
        let (times, distances) =
            parse_opt!(input.split_once('\n'), "input must be exactly two lines")?;
        let times = parse_opt!(
            times.strip_prefix("Time:"),
            "first line doesn't contain times"
        )?
        .split_whitespace()
        .map(|time| parse!(time.parse::<usize>()));

        let distances = parse_opt!(
            distances.strip_prefix("Distance:"),
            "second line doesn't contain distances"
        )?
        .split_whitespace()
        .map(|distance| parse!(distance.parse::<usize>()));

        let races: Vec<_> = std::iter::zip(times, distances)
            .map(|(time, distance)| {
                time.and_then(|time| {
                    distance.map(|distance| Race {
                        time_ms: time,
                        distance_mm: distance,
                    })
                })
            })
            .try_collect()?;

        Ok(Self { races })
    }
}

impl<'a> PuzzleInput<'a> for Race {
    fn parse(input: &'a str) -> aoc_runner::Result<Self> {
        let (times, distances) =
            parse_opt!(input.split_once('\n'), "input must be exactly two lines")?;

        let time: String = parse_opt!(
            times.strip_prefix("Time:"),
            "first line doesn't contain times"
        )?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
        let time_ms = parse!(time.parse::<usize>())?;

        let distance: String = parse_opt!(
            distances.strip_prefix("Distance:"),
            "second line doesn't contain distances"
        )?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
        let distance_mm = parse!(distance.parse::<usize>())?;

        Ok(Self {
            time_ms,
            distance_mm,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = example_str!("2023/d6e1.txt");
        let input = Races::parse(&input).unwrap();
        let result = part1(input);
        assert_eq!(result, 288);
    }

    #[test]
    fn example_2() {
        let input = example_str!("2023/d6e1.txt");
        let input = Race::parse(&input).unwrap();
        let result = part2(input);
        assert_eq!(result, 71503);
    }
}
