use crate::parse_helpers::parse_lines_to_vec;

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<i32> {
    parse_lines_to_vec(input).unwrap()
}

#[aoc(day1, part1)]
fn part1(input: &[i32]) -> usize {
    input
        .iter()
        .scan(input[0], |last, &depth| {
            let deeper = depth > *last;
            *last = depth;
            Some(if deeper { 1 } else { 0 })
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &[i32]) -> usize {
    let init: i32 = input.iter().take(3).sum();

    input
        .windows(3)
        .map(|x| x.iter().sum())
        .scan(init, |last, depth| {
            let deeper = depth > *last;
            *last = depth;
            Some(if deeper { 1 } else { 0 })
        })
        .sum()
}

aoc_tests! { day: 1, part1: 7, part2: 5 }
