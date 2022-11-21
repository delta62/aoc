mod coords;
mod grid;

use grid::Grid;

#[aoc_generator(day9)]
fn parse(input: &str) -> Grid {
    input.parse().unwrap()
}

#[aoc(day9, part1)]
fn part1(grid: &Grid) -> usize {
    grid.low_points().map(|n| n as usize).sum()
}

#[aoc(day9, part2)]
fn part2(grid: &Grid) -> usize {
    let mut basins = grid.clone().basins();
    basins.sort_by(|a, b| b.cmp(a));
    basins.iter().take(3).product()
}

aoc_tests! { day: 9, part1: 15, part2: 1134 }
