use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

enum Instruction {
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

struct Ship {
    east: isize,
    north: isize,
    orientation: isize,
}

impl Ship {
    fn new() -> Self {
        Self {
            east: 0,
            north: 0,
            orientation: 0,
        }
    }

    fn perform(&mut self, instr: &Instruction) {
        match instr {
            Instruction::North(x)   => self.north += x,
            Instruction::South(x)   => self.north -= x,
            Instruction::East(x)    => self.east += x,
            Instruction::West(x)    => self.east -= x,
            Instruction::Rotate(x)  => self.rotate(*x),
            Instruction::Forward(x) => self.forward(*x),
        }
    }

    fn rotate(&mut self, degrees: isize) {
        let mut degrees = degrees;

        if degrees < 0 {
            degrees += 360;
        }

        self.orientation = (self.orientation + degrees) % 360;
    }

    fn forward(&mut self, distance: isize) {
        match self.orientation {
            0   => self.east  += distance,
            90  => self.north -= distance,
            180 => self.east  -= distance,
            270 => self.north += distance,
            _   => panic!("Can't move at angle {}", self.orientation),
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
fn solve_part1(input: &[Instruction]) -> isize {
    let mut ship = Ship::new();

    for instr in input {
        ship.perform(instr);
    }

    ship.east.abs() + ship.north.abs()
}

#[aoc(day12, part2)]
fn solve_part2(input: &[Instruction]) -> usize {
    42
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
    }
}
