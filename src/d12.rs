use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::Deref;
use std::str::FromStr;

enum Direction { N, W, S, E }

enum Instruction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Forward(isize),
    Rotate(isize),
}

#[derive(Default)]
struct Orientation {
    degrees: isize,
}

impl Orientation {
    fn add(&mut self, degrees: isize) {
        self.degrees = if degrees < 0 { degrees + 360 } else { degrees } % 360;
    }
}

impl Deref for Orientation {
    type Target = isize;

    fn deref(&self) -> &Self::Target {
        &self.degrees
    }
}

#[derive(Default)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn translate(&mut self, dir: &Direction, distance: isize) {
        match dir {
            Direction::N => self.y += distance,
            Direction::S => self.y -= distance,
            Direction::E => self.x += distance,
            Direction::W => self.x -= distance,
        }
    }

    fn rotate(&mut self, degrees: isize) {
        let degrees = if degrees < 0 { degrees + 360 } else { degrees };
        let rotations = degrees / 90;

        for _ in 0..rotations {
            let x = self.x;
            let y = self.y;

            self.x = y;
            self.y = -x;
        }
    }

    fn manhattan(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
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

impl From<Ship> for Point {
    fn from(ship: Ship) -> Self {
        ship.loc
    }
}

#[derive(Default)]
struct Ship {
    loc: Point,
    orientation: Orientation,
}

impl Ship {
    fn step(&mut self, instr: &Instruction) {
        match instr {
            Instruction::North(x)   => self.loc.y += x,
            Instruction::South(x)   => self.loc.y -= x,
            Instruction::East(x)    => self.loc.x += x,
            Instruction::West(x)    => self.loc.x -= x,
            Instruction::Rotate(x)  => self.rotate(*x),
            Instruction::Forward(x) => self.forward(*x),
        }
    }

    fn rotate(&mut self, degrees: isize) {
        self.orientation.add(degrees);
    }

    fn forward(&mut self, distance: isize) {
        match *self.orientation {
            0   => self.loc.x += distance,
            90  => self.loc.y -= distance,
            180 => self.loc.x -= distance,
            270 => self.loc.y += distance,
            x   => panic!("Can't move at angle {}", x),
        }
    }

    fn translate_times(&mut self, vec: &Point, times: isize) {
        for _ in 0..times {
            self.loc.y += vec.y;
            self.loc.x += vec.x;
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
