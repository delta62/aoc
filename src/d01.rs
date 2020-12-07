use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

const TARGET: isize = 2020;

#[aoc_generator(day1)]
fn part1(input: &str) -> Vec<isize> {
    input
        .split('\n')
        .map(|x| x.parse::<isize>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[isize]) -> isize {
    two_sum(input, TARGET)
        .map(|(x, y)| x * y)
        .unwrap()
}

#[aoc(day1, part2)]
fn solve_part2(input: &[isize]) -> isize {
    input
        .iter()
        .find_map(|z| {
            two_sum(input, TARGET - z).map(|(x, y)| x * y * z)
        })
        .unwrap()
}

fn two_sum(input: &[isize], target: isize) -> Option<(isize, isize)> {
    let mut seen = HashSet::new();
    input
        .iter()
        .find_map(|x| {
            let x = *x;
            let wanted = target - x;
            let ret = seen.get(&wanted).map(|_| (x, wanted));
            seen.insert(x);
            ret
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_ex1() {
        let input = vec![ 1721, 979, 366, 299, 675, 1456 ];
        let result = solve_part1(input.as_slice());
        assert_eq!(result, 514579);
    }

    #[test]
    fn day1_ex2() {
        let input = vec![ 1721, 979, 366, 299, 675, 1456 ];
        let result = solve_part2(input.as_slice());
        assert_eq!(result, 241861950);
    }
}
