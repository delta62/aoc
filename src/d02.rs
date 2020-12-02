use regex::Regex;

struct PasswordEntry {
    min: usize,
    max: usize,
    ch: char,
    password: String,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<PasswordEntry> {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-zA-Z]): (.*)$").unwrap();

    input
        .split('\n')
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let min = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let max = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let ch = captures.get(3).unwrap().as_str().parse::<char>().unwrap();
            let password = captures.get(4).unwrap().as_str().to_string();

            PasswordEntry { min, max, ch, password }
        })
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
            let min = chars.nth(entry.min - 1);
            let max = chars.nth(entry.max - entry.min - 1);

            match (min, max) {
                (Some(x), Some(y)) if x == y && x == entry.ch => false,
                (Some(c), _) if c == entry.ch => true,
                (_, Some(c)) if c == entry.ch => true,
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
