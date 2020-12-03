use std::collections::HashSet;

const TARGET: u32 = 2020;

#[aoc_generator(day1)]
fn part1(input: &str) -> Vec<u32> {
    input
        .split('\n')
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[u32]) -> Option<u32> {
    let mut seen = HashSet::new();

    for x in input {
        if seen.contains(&(TARGET - x)) {
            return Some(x * (TARGET - x))
        }
        seen.insert(x);
    }

    None
}

#[aoc(day1, part2)]
fn solve_part2(input: &[u32]) -> Option<u32> {
    let mut seen = HashSet::new();

    for x in input {
        for y in input {
            if seen.contains(&(TARGET - x - y)) {
                return Some(x * y * (TARGET - x - y));
            }
            seen.insert(y);
        }
        seen.clear();
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![ 1721, 979, 366, 299, 675, 1456 ];
        let result = solve_part1(input.as_slice());
        assert_eq!(result, Some(514579));
    }

    // #[test]
    // fn ex2() {
    //     let input = vec![ 1721, 979, 366, 299, 675, 1456 ];
    //     let result = solve_part2(input.as_slice());
    //     assert_eq!(result, Some(241861950));
    // }
}
