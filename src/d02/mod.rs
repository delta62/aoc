mod command;
mod position;

use crate::parse_helpers::parse_lines_to_vec;
use command::MoveCommand;
use position::Position;

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<MoveCommand> {
    parse_lines_to_vec(input).unwrap()
}

#[aoc(day2, part1)]
fn part1(input: &[MoveCommand]) -> i32 {
    input
        .iter()
        .fold(Position::default(), |state, cmd| state.update(cmd))
        .as_solution()
}

#[aoc(day2, part2)]
fn part2(input: &[MoveCommand]) -> i32 {
    input
        .iter()
        .fold(Position::default(), |state, cmd| state.update_aim(cmd))
        .as_solution()
}

aoc_tests! { day: 2, part1: 150, part2: 900 }
