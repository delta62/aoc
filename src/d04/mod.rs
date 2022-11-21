mod state;

use state::GameState;

#[aoc_generator(day4)]
fn parse(input: &str) -> GameState {
    input.parse().unwrap()
}

#[aoc(day4, part1)]
fn part1(input: &GameState) -> u32 {
    let state = input.clone();
    state.wins().next().unwrap()
}

#[aoc(day4, part2)]
fn part2(input: &GameState) -> u32 {
    let state = input.clone();
    state.wins().last().unwrap()
}

aoc_tests! { day: 4, part1: 4512, part2: 1924 }
