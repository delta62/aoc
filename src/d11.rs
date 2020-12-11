use aoc_runner_derive::aoc;
use std::str::FromStr;

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
    state: Vec<State>,
}

impl Simulation {
    fn step(&mut self) -> SimulationResult {
        let mut changed = false;

        self.state = self.state
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                match (cell, self.adjacent_occupied(i)) {
                    (State::EmptySeat, 0) => {
                        changed = true;
                        State::FullSeat
                    }
                    (State::FullSeat, x) if x >= 4 => {
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

    fn adjacent_occupied(&self, index: usize) -> usize {
        let rows = self.state.len() / self.columns;
        let max_row = rows - 1;
        let max_col = self.columns - 1;

        let row = index / self.columns;
        let col = index % self.columns;

        let nw = if col == 0 || row == 0             { 0 } else { self.cell_value(row - 1, col - 1) };
        let n  = if row == 0                         { 0 } else { self.cell_value(row - 1, col    ) };
        let ne = if col == max_col || row == 0       { 0 } else { self.cell_value(row - 1, col + 1) };
        let w  = if col == 0                         { 0 } else { self.cell_value(row,     col - 1) };
        let e  = if col == max_col                   { 0 } else { self.cell_value(row,     col + 1) };
        let sw = if col == 0 || row == max_row       { 0 } else { self.cell_value(row + 1, col - 1) };
        let s  = if row == max_row                   { 0 } else { self.cell_value(row + 1, col    ) };
        let se = if row == max_row || col == max_col { 0 } else { self.cell_value(row + 1, col + 1) };

        nw + n + ne + w + e + sw + s + se
    }

    fn cell_value(&self, row: usize, col: usize) -> usize {
        let idx = row * self.columns + col;
        match self.state[idx] {
            State::FullSeat => 1,
            _ => 0,
        }
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

        Ok(Self { columns, state })
    }
}

#[aoc(day11, part1)]
fn solve_part1(input: &str) -> usize {
    let mut sim = Simulation::from_str(input).unwrap();

    while let SimulationResult::InProgress = sim.step() { }

    sim.occupied_seats()
}

#[aoc(day11, part2)]
fn solve_part2(input: &str) -> isize {
    42
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
