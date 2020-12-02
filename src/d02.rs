use regex::Regex;
use std::str::FromStr;

struct PasswordEntry {
    min: usize,
    max: usize,
    ch: char,
    password: String,
}

impl FromStr for PasswordEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-zA-Z]): (.*)").unwrap();
        }

        let captures = RE.captures(s).ok_or(())?;
        let min = captures[1].parse::<usize>().unwrap();
        let max = captures[2].parse::<usize>().unwrap();
        let ch = captures[3].parse::<char>().unwrap();
        let password = captures[4].to_string();

        Ok(Self { min, max, ch, password })
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<PasswordEntry> {
    input
        .split('\n')
        .map(|line| PasswordEntry::from_str(line).unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[PasswordEntry]) -> usize {
    input
        .iter()
        .filter(|entry| {
            let matches = entry.password.as_str().match_indices(entry.ch).count();
            matches >= entry.min && matches <= entry.max
        })
        .count()
}

#[aoc(day2, part2)]
fn solve_part2(input: &[PasswordEntry]) -> usize {
    input
        .iter()
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
        let input = parse(input);
        let result = solve_part1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn ex2() {
        let input = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let input = parse(input);
        let result = solve_part2(&input);
        assert_eq!(result, 1);
    }
}
