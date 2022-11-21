use std::{collections::VecDeque, str::FromStr};

#[derive(Clone, Default)]
struct Square {
    val: u8,
    checked: bool,
}

#[derive(Clone)]
pub struct Board {
    won: bool,
    squares: [Square; 25],
}

impl Board {
    pub fn from_lines(lines: &[&'_ str]) -> Self {
        Self::from_values(
            lines
                .iter()
                .flat_map(|line| line.split_whitespace())
                .map(|val| val.parse().unwrap()),
        )
    }

    fn from_values(values: impl Iterator<Item = u8>) -> Self {
        let won = false;
        let mut squares: [Square; 25] = Default::default();
        for (i, val) in values.enumerate() {
            squares[i].val = val;
        }

        Self { squares, won }
    }

    fn update(&mut self, val: u8) -> Option<u32> {
        if self.won {
            return None;
        }

        self.squares
            .iter_mut()
            .enumerate()
            .into_iter()
            .find_map(|(i, square)| {
                (square.val == val).then(|| {
                    square.checked = true;
                    i
                })
            })
            .and_then(|i| {
                let row_win = self.is_row_win(i).then(|| ());
                let col_win = self.is_col_win(i).then(|| ());

                row_win.or(col_win).map(|_| {
                    self.won = true;
                    self.score(val)
                })
            })
    }

    fn score(&self, called: u8) -> u32 {
        let square_sum = self
            .squares
            .iter()
            .map(|square| if square.checked { 0 } else { square.val as u32 })
            .sum::<u32>();

        square_sum * called as u32
    }

    fn is_row_win(&self, index: usize) -> bool {
        let row_num = index / 5;
        (0..5).all(|col| self.squares[row_num * 5 + col].checked)
    }

    fn is_col_win(&self, index: usize) -> bool {
        let col_num = index % 5;
        (0..5).all(|row| self.squares[row * 5 + col_num].checked)
    }
}

#[derive(Clone)]
pub struct GameState {
    calls: VecDeque<u8>,
    boards: Vec<Board>,
}

impl GameState {
    pub fn new(calls: VecDeque<u8>, boards: Vec<Board>) -> Self {
        Self { calls, boards }
    }

    pub fn wins(self) -> Wins {
        Wins::new(self)
    }
}

impl FromStr for GameState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let calls = lines
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        let boards = lines
            .filter(|&line| !line.is_empty())
            .collect::<Vec<_>>()
            .as_slice()
            .chunks(5)
            .map(Board::from_lines)
            .collect();

        Ok(GameState::new(calls, boards))
    }
}

pub struct Wins {
    state: GameState,
    unfinished_boards: usize,
}

impl Wins {
    fn new(state: GameState) -> Self {
        let unfinished_boards = state.boards.iter().filter(|board| !board.won).count();
        Self {
            state,
            unfinished_boards,
        }
    }
}

impl Iterator for Wins {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.unfinished_boards == 0 {
                return None;
            }

            let call = self.state.calls.pop_front()?;
            let mut win_score = None;
            for board in &mut self.state.boards {
                let result = board.update(call);
                win_score = win_score.or(result);
            }

            if let Some(score) = win_score {
                return Some(score);
            }
        }
    }
}
