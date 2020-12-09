use aoc_runner_derive::{aoc, aoc_generator};
use crate::d01::two_sum;
use std::cmp::{Ordering, max, min};

const BUF_SIZE: usize = 25;

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect()
}

#[aoc(day9, part1)]
fn solve_part1(input: &[isize]) -> isize {
    (0..)
        .find_map(|start_idx| {
            let end_idx = start_idx + BUF_SIZE;
            let range = &input[start_idx..end_idx];
            let target = input[end_idx];

            match two_sum(range, target) {
                None => Some(target),
                _ => None,
            }
        })
        .unwrap()
}

#[aoc(day9, part2)]
fn solve_part2(input: &[isize]) -> isize {
    let target = solve_part1(input);

    (0..)
        .find_map(|start_idx| n_sum(&input[start_idx..], target))
        .map(|(x, y)| x + y)
        .unwrap()
}

fn n_sum(input: &[isize], target: isize) -> Option<(isize, isize)> {
    let mut smallest = isize::MAX;
    let mut largest = isize::MIN;
    let mut sum = 0;

    for x in input {
        smallest = min(*x, smallest);
        largest = max(*x, largest);
        sum += x;

        match sum.cmp(&target) {
            Ordering::Equal => return Some((smallest, largest)),
            Ordering::Greater => return None,
            _ => { },
        }
    }

    None
}
