use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

mod cart;
mod ship;

use ship::Ship;
use cart::{Direction, Point};

pub enum Instruction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Forward(isize),
    Rotate(isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (c, x) = input.split_at(1);
        let c = c.chars().next().unwrap();
        let x = x.parse::<isize>().unwrap();

        match c {
            'N' => Ok(Instruction::North(x)),
            'S' => Ok(Instruction::South(x)),
            'E' => Ok(Instruction::East(x)),
            'W' => Ok(Instruction::West(x)),
            'F' => Ok(Instruction::Forward(x)),
            'L' => Ok(Instruction::Rotate(-x)),
            'R' => Ok(Instruction::Rotate(x)),
            _   => Err(()),
        }
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|x| Instruction::from_str(x).unwrap())
        .collect()
}

#[aoc(day12, part1)]
fn solve_part1(input: &[Instruction]) -> usize {
    let mut ship = Ship::default();

    input.iter().for_each(|instr| ship.step(instr));

    Point::from(ship).manhattan()
}

#[aoc(day12, part2)]
fn solve_part2(input: &[Instruction]) -> usize {
    let mut ship = Ship::default();
    let mut waypoint = Point { x: 10, y: 1 };

    for instr in input {
        match instr {
            Instruction::Rotate(x)  => waypoint.rotate(*x),
            Instruction::Forward(x) => ship.translate_times(&waypoint, *x),
            Instruction::North(x)   => waypoint.translate(&Direction::N, *x),
            Instruction::South(x)   => waypoint.translate(&Direction::S, *x),
            Instruction::East(x)    => waypoint.translate(&Direction::E, *x),
            Instruction::West(x)    => waypoint.translate(&Direction::W, *x),
        }
    }

    Point::from(ship).manhattan()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r"F10
N3
F7
R90
F11";

        let input = parse(input);
        let result = solve_part1(&input);
        assert_eq!(result, 25);
    }

    #[test]
    fn ex2() {
        let input = r"F10
N3
F7
R90
F11";

        let input = parse(input);
        let result = solve_part2(&input);
        assert_eq!(result, 286);
    }
}
