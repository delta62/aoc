use aoc_runner_derive::aoc;
use lazy_static::lazy_static;
use regex::Regex;

struct PasswordEntry<'a> {
    min: usize,
    max: usize,
    ch: char,
    password: &'a str,
}

impl<'a> PasswordEntry<'a> {
    fn new(line: &'a str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-zA-Z]): (.*)$").unwrap();
        }

        let captures = RE.captures(line).unwrap();
        let min = captures[1].parse::<usize>().unwrap();
        let max = captures[2].parse::<usize>().unwrap();
        let ch = captures[3].parse::<char>().unwrap();
        let password = captures.get(4).unwrap().as_str();

        Self { min, max, ch, password }
    }
}

#[aoc(day2, part1)]
fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| PasswordEntry::new(line))
        .filter(|entry| {
            let matches = entry.password.chars().fold(0, |acc, c| {
                if c == entry.ch { acc + 1 } else { acc }
            });
            matches >= entry.min && matches <= entry.max
        })
        .count()
}

#[aoc(day2, part2)]
fn solve_part2(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| PasswordEntry::new(line))
        .filter(|entry| {
            let mut chars = entry.password.chars();
            let fst = chars.nth(entry.min - 1).unwrap();
            let snd = chars.nth(entry.max - entry.min - 1).unwrap();

            match (fst, snd) {
                (x, y) if x == y => false,
                (c, _) if c == entry.ch => true,
                (_, c) if c == entry.ch => true,
                _ => false,
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let result = solve_part1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn ex2() {
        let input = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let result = solve_part2(&input);
        assert_eq!(result, 1);
    }
}
