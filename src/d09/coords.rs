#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    row: i32,
    col: i32,
}

impl Coordinate {
    pub fn row(&self) -> i32 {
        self.row
    }

    pub fn col(&self) -> i32 {
        self.col
    }

    pub fn up(&self) -> Coordinate {
        Self {
            row: self.row - 1,
            col: self.col,
        }
    }

    pub fn down(&self) -> Coordinate {
        Self {
            row: self.row + 1,
            col: self.col,
        }
    }

    pub fn left(&self) -> Coordinate {
        Self {
            row: self.row,
            col: self.col - 1,
        }
    }

    pub fn right(&self) -> Coordinate {
        Self {
            row: self.row,
            col: self.col + 1,
        }
    }
}

pub struct Coords {
    i: i32,
    width: i32,
    height: i32,
}

impl Coords {
    pub fn new(width: i32, height: i32) -> Self {
        let i = 0;
        Self { i, width, height }
    }
}

impl Iterator for Coords {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.width * self.height {
            return None;
        }

        let row = self.i / self.width;
        let col = self.i % self.width;

        self.i += 1;

        Some(Coordinate { row, col })
    }
}
