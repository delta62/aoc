use super::coords::{Coordinate, Coords};
use std::str::FromStr;

const MAX_HEIGHT: u8 = 9;

#[derive(Clone, Debug)]
struct SearchSquare {
    checked: bool,
    value: u8,
}

impl SearchSquare {
    fn new(value: u8) -> Self {
        let checked = false;
        Self { checked, value }
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    squares: Vec<SearchSquare>,
    width: i32,
    height: i32,
}

impl Grid {
    fn get(&self, coord: Coordinate) -> Option<&SearchSquare> {
        let row = coord.row();
        let col = coord.col();

        if row < 0 || row >= self.height || col < 0 || col >= self.width {
            None
        } else {
            let i = row * self.width + col;
            Some(&self.squares[i as usize])
        }
    }

    fn get_mut(&mut self, coord: Coordinate) -> Option<&mut SearchSquare> {
        let row = coord.row();
        let col = coord.col();

        if row < 0 || row >= self.height || col < 0 || col >= self.width {
            None
        } else {
            let i = row * self.width + col;
            Some(&mut self.squares[i as usize])
        }
    }

    fn coords(&self) -> impl Iterator<Item = Coordinate> {
        Coords::new(self.width, self.height)
    }

    pub fn low_points(&self) -> impl Iterator<Item = u8> + '_ {
        self.coords()
            .filter(|coord| {
                let above = self.get(coord.up()).map_or(MAX_HEIGHT, |x| x.value);
                let below = self.get(coord.down()).map_or(MAX_HEIGHT, |x| x.value);
                let left = self.get(coord.left()).map_or(MAX_HEIGHT, |x| x.value);
                let right = self.get(coord.right()).map_or(MAX_HEIGHT, |x| x.value);
                let here = self.get(*coord).map_or(MAX_HEIGHT, |x| x.value);

                here < above && here < below && here < left && here < right
            })
            .map(|coord| {
                let idx = coord.row() * self.width + coord.col();
                self.squares[idx as usize].value + 1
            })
    }

    pub fn basins(mut self) -> Vec<usize> {
        let mut sizes = vec![];

        for coord in self.coords() {
            sizes.push(self.mark(coord, 0));
        }

        sizes
    }

    fn mark(&mut self, coord: Coordinate, size: usize) -> usize {
        if self.checked(coord) {
            return size;
        }

        let square = self.get_mut(coord);
        if let Some(square) = square {
            square.checked = true;
            if square.value == MAX_HEIGHT {
                return size;
            }
        } else {
            return size;
        }

        let mut size = size + 1;
        size = self.mark(coord.up(), size);
        size = self.mark(coord.down(), size);
        size = self.mark(coord.left(), size);
        size = self.mark(coord.right(), size);

        size
    }

    fn checked(&self, coord: Coordinate) -> bool {
        self.get(coord).map_or(true, |x| x.checked)
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut squares = vec![];

        s.chars().enumerate().for_each(|(i, c)| {
            if c == '\n' {
                height += 1;
                if width == 0 {
                    width = i as i32;
                }
                return;
            }

            let value = c.to_digit(10).unwrap() as u8;
            let square = SearchSquare::new(value);
            squares.push(square);
        });

        squares.shrink_to_fit();

        Ok(Self {
            squares,
            width,
            height,
        })
    }
}
