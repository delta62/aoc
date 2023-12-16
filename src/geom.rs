use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct Point<T>
where
    T: Clone
        + Copy
        + Debug
        + Default
        + Eq
        + PartialEq
        + Hash
        + Add<T, Output = T>
        + Sub<T, Output = T>,
{
    pub x: T,
    pub y: T,
}

impl Point<usize> {
    pub fn towards(&self, direction: Direction) -> Option<Point<usize>> {
        use Direction::*;

        match direction {
            Up => {
                if self.y == 0 {
                    None
                } else {
                    Some(Self {
                        x: self.x,
                        y: self.y - 1,
                    })
                }
            }
            Down => Some(Self {
                x: self.x,
                y: self.y + 1,
            }),
            Left => {
                if self.x == 0 {
                    None
                } else {
                    Some(Self {
                        x: self.x - 1,
                        y: self.y,
                    })
                }
            }
            Right => Some(Self {
                x: self.x + 1,
                y: self.y,
            }),
        }
    }
}

impl Point<i64> {
    pub fn towards(&self, direction: Direction) -> Point<i64> {
        use Direction::*;

        match direction {
            Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl<T> Add<Point<T>> for Point<T>
where
    T: Clone
        + Copy
        + Debug
        + Default
        + Eq
        + PartialEq
        + Hash
        + Add<T, Output = T>
        + Sub<T, Output = T>,
{
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    pub fn invert(&self) -> Self {
        use Direction::*;

        match self {
            Down => Up,
            Left => Right,
            Right => Left,
            Up => Down,
        }
    }
}
