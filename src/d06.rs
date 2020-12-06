use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

struct Group {
    size: usize,
    responses: HashMap<char, usize>,
}

impl Group {
    fn new() -> Self {
        Self {
            size: 0,
            responses: HashMap::new(),
        }
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Group> {
    let mut ret = Vec::new();
    let mut group = Group::new();

    for line in input.lines() {
        if line.is_empty() {
            ret.push(group);
            group = Group::new();
        } else {
            group.size += 1;
            for c in line.chars() {
                let entry = group.responses.entry(c).or_insert(0);
                *entry += 1;
            }
        }
    }

    if !group.responses.is_empty() {
        ret.push(group);
    }

    ret
}

#[aoc(day6, part1)]
fn solve_part1(input: &[Group]) -> usize {
    input
        .iter()
        .map(|group| group.responses.len())
        .sum()
}

#[aoc(day6, part2)]
fn solve_part2(input: &[Group]) -> usize {
    input
        .iter()
        .map(|group| {
            group.responses
                .values()
                .filter(|x| **x == group.size)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r"abc

a
b
c

ab
ac

a
a
a
a

b";
        let input = parse(input);
        let res = solve_part1(&input);
        assert_eq!(res, 11);
    }

    #[test]
    fn ex2() {
        let input = r"abc

a
b
c

ab
ac

a
a
a
a

b
";
        let input = parse(input);
        let res = solve_part2(&input);
        assert_eq!(res, 6);
    }
}
