use std::{cmp, cmp::Ordering, num::ParseIntError, str::FromStr};

#[derive(Eq, Hash, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse()?;
        let y = parts.next().unwrap().parse()?;

        Ok(Self { x, y })
    }
}

pub struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        if p1.x <= p2.x {
            Self { p1, p2 }
        } else {
            Self { p1: p2, p2: p1 }
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    pub fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        let dx = match self.p2.x.cmp(&self.p1.x) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };

        let dy = match self.p2.y.cmp(&self.p1.y) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };

        let x_steps = (self.p1.x - self.p2.x).abs();
        let y_steps = (self.p1.y - self.p2.y).abs();
        let steps = cmp::max(x_steps, y_steps);

        let init = Point {
            x: self.p1.x,
            y: self.p1.y,
        };

        (0..=steps).scan(init, move |state, _| {
            let ret = Point { ..(*state) };

            state.x += dx;
            state.y += dy;

            Some(ret)
        })
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split(" -> ");
        let p1 = points.next().unwrap().parse()?;
        let p2 = points.next().unwrap().parse()?;

        Ok(Self::new(p1, p2))
    }
}
