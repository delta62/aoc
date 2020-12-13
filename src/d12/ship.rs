use super::Instruction;
use super::cart::{Orientation, Point};

#[derive(Default)]
pub struct Ship {
    loc: Point,
    orientation: Orientation,
}

impl Ship {
    pub fn step(&mut self, instr: &Instruction) {
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

    pub fn translate_times(&mut self, vec: &Point, times: isize) {
        for _ in 0..times {
            self.loc.y += vec.y;
            self.loc.x += vec.x;
        }
    }
}

impl From<Ship> for Point {
    fn from(ship: Ship) -> Self {
        ship.loc
    }
}
