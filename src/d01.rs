const TARGET: u32 = 2020;

#[aoc_generator(day1)]
fn part1(input: &str) -> Vec<u32> {
    input
        .split('\n')
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn solve_part1(input1: &[u32]) -> u32 {

    // Sort the list in descending order
    let mut input = vec![0; input1.len()];
    input[..].copy_from_slice(input1);
    input.sort_unstable_by_key(|x| u32::MAX - x);

    for big in input.iter() {
        // Walk in from the end of the list
        for small in input.iter().rev() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_ex1() {
        let input = vec![ 1721, 979, 366, 299, 675, 1456 ];
        let result = solve_part1(input.as_slice());
        assert_eq!(result, 514579);
    }
}
