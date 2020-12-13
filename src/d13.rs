use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day13, part1)]
fn solve_part1(input: &[usize]) -> usize {
    42
}

#[aoc(day12, part2)]
fn solve_part2(input: &[usize]) -> usize {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
    }

    #[test]
    fn ex2() {
    }
}
