use aoc_runner_derive::aoc;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Nw,
    N,
    Ne,
    W,
    E,
    Sw,
    S,
    Se,
}

enum SimulationResult {
    InProgress,
    Settled,
}

#[derive(Copy, Clone)]
enum State {
    Floor,
    EmptySeat,
    FullSeat,
}

struct Simulation {
    columns: usize,
    rows: usize,
    state: Vec<State>,
}

impl Simulation {
    fn step(&mut self, move_threshold: usize, distance: Option<usize>) -> SimulationResult {
        let mut changed = false;

        self.state = self.state
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                match (cell, self.scan_count(i, distance)) {
                    (State::EmptySeat, 0) => {
                        changed = true;
                        State::FullSeat
                    }
                    (State::FullSeat, x) if x >= move_threshold => {
                        changed = true;
                        State::EmptySeat
                    }
                    (&state, _) => state,
                }
            })
            .collect();

        match changed {
            true => SimulationResult::InProgress,
            false => SimulationResult::Settled,
        }
    }

    fn occupied_seats(&self) -> usize {
        self.state
            .iter()
            .filter(|x| matches!(x, State::FullSeat))
            .count()
    }

    fn scan_count(&self, from: usize, distance: Option<usize>) -> usize {
        [
            Direction::Nw, Direction::N, Direction::Ne,
            Direction::W,                Direction::E,
            Direction::Sw, Direction::S, Direction::Se,
        ]
            .iter()
            .filter(|&dir| self.raycast(from, dir, distance))
            .count()
    }

    fn raycast(&self, from: usize, dir: &Direction, distance: Option<usize>) -> bool {
        let from = from as isize;
        let distance = distance.unwrap_or(999) as isize;
        let rows = self.rows as isize;
        let cols = self.columns as isize;

        let from_row = from / self.columns as isize;
        let from_col = from % self.columns as isize;

        for i in 0..distance {
            let i = i + 1;
            let (c, r) = match dir {
                Direction::Nw => (from_col - i, from_row - i),
                Direction::N  => (from_col,     from_row - i),
                Direction::Ne => (from_col + i, from_row - i),
                Direction::W  => (from_col - i, from_row    ),
                Direction::E  => (from_col + i, from_row    ),
                Direction::Sw => (from_col - i, from_row + i),
                Direction::S  => (from_col,     from_row + i),
                Direction::Se => (from_col + i, from_row + i),
            };

            if r < 0 || c < 0 || r >= rows || c >= cols {
                return false;
            }

            let idx = r * cols + c;

            match self.state[idx as usize] {
                State::EmptySeat => return false,
                State::FullSeat => return true,
                _ => { }
            }
        }

        false
    }
}

impl std::fmt::Display for Simulation {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, x) in self.state.iter().enumerate() {
            if i % self.columns == 0 && i != 0 {
                writeln!(f)?;
            }

            let c = match x {
                State::FullSeat  => '#',
                State::EmptySeat => 'L',
                State::Floor     => '.',
            };

            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

impl FromStr for Simulation {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut columns = 0;
        let mut state = Vec::new();

        input
            .chars()
            .enumerate()
            .for_each(|(i, c)| {
                match c {
                    '#' => state.push(State::FullSeat),
                    'L' => state.push(State::EmptySeat),
                    '.' => state.push(State::Floor),
                    '\n' if columns == 0 => columns = i,
                    '\n' => { }
                    _ => panic!("Unrecognized input {}", c),
                }
            });

        let rows = state.len() / columns;

        Ok(Self { columns, rows, state })
    }
}

#[aoc(day11, part1)]
fn solve_part1(input: &str) -> usize {
    let mut sim = Simulation::from_str(input).unwrap();

    while let SimulationResult::InProgress = sim.step(4, Some(1)) { }

    sim.occupied_seats()
}

#[aoc(day11, part2)]
fn solve_part2(input: &str) -> usize {
    let mut sim = Simulation::from_str(input).unwrap();

    while let SimulationResult::InProgress = sim.step(5, None) { }

    sim.occupied_seats()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let result = solve_part1(input);
        assert_eq!(result, 37);
    }

    #[test]
    fn ex2() {
        let input = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let result = solve_part2(input);
        assert_eq!(result, 26);
    }
}
