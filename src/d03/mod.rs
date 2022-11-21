mod array;
mod ptr;

use array::ArrayTree;
use ptr::Tree;

pub struct DiagReport {
    numbers: Vec<u32>,
    width: usize,
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> DiagReport {
    let width = input.lines().next().unwrap().len();

    let numbers = input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    DiagReport { numbers, width }
}

#[aoc(day3, part1)]
pub fn part1(input: &DiagReport) -> u32 {
    let width = input.width;
    let numbers = &input.numbers;
    let mut bits = vec![0; width];

    for number in numbers {
        for (bit, item) in bits.iter_mut().enumerate().take(width) {
            let set = number & (1 << bit) != 0;
            if set {
                *item += 1
            }
        }
    }

    let bit_threshold = numbers.len() / 2;
    let mut gamma_rate = 0;
    for (i, bit) in bits.iter().enumerate() {
        let should_set = *bit > bit_threshold;
        if !should_set {
            continue;
        }

        let mask = 1 << i;
        gamma_rate |= mask;
    }

    let epsilon_mask = (1 << width) - 1;
    let epsilon_rate = !gamma_rate & epsilon_mask;

    gamma_rate * epsilon_rate
}

#[aoc(day3, part2, Pointer)]
fn part2_tree(input: &DiagReport) -> u32 {
    let mut tree = Tree::default();
    input
        .numbers
        .iter()
        .for_each(|&n| tree.add_num(n, input.width - 1));

    let o2_rating = tree.common_bits(input.width);
    let co2_rating = tree.uncommon_bits(input.width);

    o2_rating * co2_rating
}

#[aoc(day3, part2, Array)]
fn part2(input: &DiagReport) -> u32 {
    let mut tree = ArrayTree::new(input.width);
    input.numbers.iter().for_each(|&n| tree.add_num(n));

    let o2_rating = tree.common_bits();
    let co2_rating = tree.uncommon_bits();

    o2_rating * co2_rating
}

aoc_tests! { day: 3, part1: 198, part2: 230 }
