use aoc_runner_derive::{aoc, aoc_generator};

const TARGET: u32 = 2020;

#[aoc_generator(day1)]
fn part1(input: &str) -> Vec<u32> {
    input
        .split('\n')
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[u32]) -> u32 {

    // Sort the list in descending order
    let mut sorted = vec![0; input.len()];
    sorted[..].copy_from_slice(input);
    sorted.sort_unstable_by_key(|x| u32::MAX - x);

    for big in sorted.iter() {
        // Walk in from the end of the list
        for small in sorted.iter().rev() {
            if big + small == TARGET {
                return big * small;
            }

            if big + small > TARGET {
                break;
            }
        }
    }

    panic!("Sum not found!");
}

#[aoc(day1, part2)]
fn solve_part2(input: &[u32]) -> u32 {

    // Sort the list in descending order
    let mut sorted = vec![0; input.len()];
    sorted[..].copy_from_slice(input);
    sorted.sort_unstable_by_key(|x| u32::MAX - x);

    for big in sorted.iter() {
        for middle in sorted[1..].iter() {
            for small in sorted.iter().rev() {
                if big + middle + small == TARGET {
                    return big * middle * small;
                }

                if big + middle + small > TARGET {
                    break;
                }
            }
        }
    }

    panic!("Sum not found!");
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
