use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Movement {
    Down(usize),
    Forward(usize),
    Up(usize),
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Movement::*;

        let mut iter = s.splitn(2, ' ');
        let dir = iter.next().unwrap();
        let len = iter.next().unwrap();
        let len = len.parse::<usize>().unwrap();

        match dir {
            "down"    => Ok(Down(len)),
            "forward" => Ok(Forward(len)),
            "up"      => Ok(Up(len)),
            _         => Err(()),
        }
    }
}

struct Position {
    depth: usize,
    distance: usize,
}

impl Default for Position {
    fn default() -> Self {
        Position { depth: 0, distance: 0 }
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Movement> {
    input
        .lines()
        .map(|line| Movement::from_str(line).unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[Movement]) -> usize {
    use Movement::*;

    let Position { depth, distance } = input
        .iter()
        .fold(Position::default(), |Position { depth, distance }, movement| {
            match movement {
                Down(x) => Position { depth: depth + x, distance },
                Forward(x) => Position { depth, distance: distance + x },
                Up(x) => Position { depth: depth - x, distance },
            }
        });

    depth * distance
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_input() {
        let input = "up 42\ndown 12\nforward 3\n";
        let res = parse(&input);
        assert_eq!(
            res,
            vec![ Movement::Up(42), Movement::Down(12), Movement::Forward(3) ],
        );
    }

    #[test]
    fn solves_example_1() {
        let input = vec![
            Movement::Forward(5),
            Movement::Down(5),
            Movement::Forward(8),
            Movement::Up(3),
            Movement::Down(8),
            Movement::Forward(2),
        ];
        let res = solve_part1(&input);
        assert_eq!(res, 150);
    }
}
