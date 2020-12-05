use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::str::FromStr;

struct Seat {
    row: usize,
    col: usize,
}

impl FromStr for Seat {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut min_row = 0;
        let mut max_row = 128;
        let mut min_col = 0;
        let mut max_col = 8;

        for (i, c) in input.char_indices() {
            if i < 7 {
                let range = (max_row - min_row) / 2;
                match c {
                    'F' => max_row -= range,
                    'B' => min_row += range,
                    _ => return Err(()),
                }
            } else {
                let range = (max_col - min_col) / 2;
                match c {
                    'R' => min_col += range,
                    'L' => max_col -= range,
                    _ => return Err(()),
                }
            }
        }

        Ok(Seat {
            row: min_row,
            col: min_col
        })
    }
}

impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.col
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Vec<Seat> {
    input
        .lines()
        .map(|line| Seat::from_str(line).unwrap())
        .collect()
}

#[aoc(day5, part1)]
fn solve_part1(input: &[Seat]) -> usize {
    input
        .iter()
        .map(|seat| seat.id())
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &[Seat]) -> usize {
    let mut smallest = usize::MAX;
    let mut biggest = 0;
    let mut seen = HashSet::new();

    for seat in input {
        let id = seat.id();
        smallest = min(id, smallest);
        biggest = max(id, biggest);
        seen.insert(id);
    }

    for x in (smallest + 1)..(biggest - 1) {
        if !seen.contains(&x) {
            return x;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = parse("FBFBBFFRLR");
        let result = solve_part1(&input);
        assert_eq!(result, 357);
    }

    #[test]
    fn ex2() {
        let input = parse("BFFFBBFRRR");
        let result = solve_part1(&input);
        assert_eq!(result, 567);
    }

    #[test]
    fn ex3() {
        let input = parse("FFFBBBFRRR");
        let result = solve_part1(&input);
        assert_eq!(result, 119);
    }

    #[test]
    fn ex4() {
        let input = parse("BBFFBBFRLL");
        let result = solve_part1(&input);
        assert_eq!(result, 820);
    }
}
