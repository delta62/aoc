mod day01;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("input/day01.txt").unwrap();

    let result = day01::part1(&input);
    println!("{result}");

    let result = day01::part2(&input);
    println!("{result}");
}
