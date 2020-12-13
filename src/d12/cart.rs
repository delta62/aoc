use std::ops::Deref;

pub enum Direction {
    N,
    W,
    S,
    E
}

#[derive(Default)]
pub struct Orientation {
    degrees: isize,
}

impl Orientation {
    pub fn add(&mut self, degrees: isize) {
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
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn translate(&mut self, dir: &Direction, distance: isize) {
        match dir {
            Direction::N => self.y += distance,
            Direction::S => self.y -= distance,
            Direction::E => self.x += distance,
            Direction::W => self.x -= distance,
        }
    }

    pub fn rotate(&mut self, degrees: isize) {
        let degrees = if degrees < 0 { degrees + 360 } else { degrees };
        let rotations = degrees / 90;

        for _ in 0..rotations {
            let x = self.x;
            let y = self.y;

            self.x = y;
            self.y = -x;
        }
    }

    pub fn manhattan(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}
