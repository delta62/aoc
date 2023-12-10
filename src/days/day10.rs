use crate::geom::{Direction, Point};
use aoc_runner::{aoc, parse_opt, PuzzleError, PuzzleInput};
use std::{collections::HashMap, iter};

#[aoc(year = 2023, day = 10, part = 1)]
fn part1(input: Grid) -> usize {
    input.circuit_length() / 2
}

pub struct Grid {
    start: Point,
    pipes: HashMap<Point, Pipe>,
}

impl Grid {
    fn circuit_length(&self) -> usize {
        let start_direction = self.start_direction();
        let init = (self.start, start_direction);

        iter::successors(Some(init), |&(point, dir)| self.next_point(point, dir)).count()
    }

    fn start_direction(&self) -> Direction {
        use Direction::*;

        if self.pipes[&self.start.towards(Right)].connects_to(Left) {
            Right
        } else if self.pipes[&self.start.towards(Down)].connects_to(Up) {
            Down
        } else if self.pipes[&self.start.towards(Left)].connects_to(Right) {
            Left
        } else if self.pipes[&self.start.towards(Up)].connects_to(Down) {
            Up
        } else {
            unreachable!("Start doesn't connect to anything")
        }
    }

    fn next_point(&self, point: Point, direction: Direction) -> Option<(Point, Direction)> {
        let next_point = point.towards(direction);

        if next_point == self.start {
            return None;
        }

        let next_direction = self.pipes[&next_point].direction_from(direction.invert());

        Some((next_point, next_direction))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Pipe {
    NS,
    WE,
    NE,
    NW,
    SW,
    SE,
}

impl Pipe {
    fn connects_to(&self, dir: Direction) -> bool {
        use Direction::*;
        use Pipe::*;

        match (self, dir) {
            (NS, Up) | (NS, Down) => true,
            (NS, _) => false,
            (WE, Left) | (WE, Right) => true,
            (WE, _) => false,
            (NE, Right) | (NE, Up) => true,
            (NE, _) => false,
            (NW, Left) | (NW, Up) => true,
            (NW, _) => false,
            (SW, Left) | (SW, Down) => true,
            (SW, _) => false,
            (SE, Right) | (SE, Down) => true,
            (SE, _) => false,
        }
    }

    fn direction_from(&self, from: Direction) -> Direction {
        use Direction::*;
        use Pipe::*;

        match (self, from) {
            (NS, Up) => Down,
            (NS, Down) => Up,
            (WE, Left) => Right,
            (WE, Right) => Left,
            (NE, Up) => Right,
            (NE, Right) => Up,
            (NW, Up) => Left,
            (NW, Left) => Up,
            (SW, Down) => Left,
            (SW, Left) => Down,
            (SE, Down) => Right,
            (SE, Right) => Down,
            _ => unreachable!("Unexpected direction {from:?} for {self:?}"),
        }
    }
}

impl<'a> PuzzleInput<'a> for Grid {
    fn parse(input: &'a str) -> aoc_runner::Result<Self> {
        let mut pipes = HashMap::new();
        let mut start = None;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let point = Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                };

                match c {
                    'S' => start = Some(point),
                    '.' => continue,
                    _ => {
                        pipes.insert(point, TryFrom::try_from(c)?);
                    }
                }
            }
        }

        let start = parse_opt!(start, "input didn't have a start point")?;

        Ok(Self { pipes, start })
    }
}

impl TryFrom<char> for Pipe {
    type Error = PuzzleError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::NS,
            '-' => Self::WE,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            c => Err(PuzzleError::ParseError(format!(
                "Unknown pipe character {c}"
            )))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = example_str!("2023/d10e1.txt");
        let input = Grid::parse(&input).unwrap();
        let result = part1(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let input = example_str!("2023/d10e2.txt");
        let input = Grid::parse(&input).unwrap();
        let result = part1(input);
        assert_eq!(result, 8);
    }
}
