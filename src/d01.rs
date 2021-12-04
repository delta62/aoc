use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[usize]) -> usize {
    let first = input.get(0).unwrap_or(&0);
    let seed = (first, 0);

    let (_, ret) = input
        .iter()
        .fold(seed, |(last, sum), next| {
            let newsum = if next > last { sum + 1 } else { sum };
            (next, newsum)
        });

    ret
}

#[aoc(day1, part2)]
fn solve_part2(input: &[usize]) -> usize {
    if input.len() < 4 {
        return 0;
    }

    let window = (input[0], input[1], input[2]);
    let seed = (window, 0);

    input
        .iter()
        .skip(3)
        .fold(seed, |(last, sum), &next| {
            let (l0, l1, l2) = last;
            let old_sum = l0 + l1 + l2;
            let new_sum = l1 + l2 + next;
            let next_window = (l1, l2, next);

            if new_sum > old_sum {
                (next_window, sum + 1)
            } else {
                (next_window, sum)
            }
        })
        .1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_input() {
        let input = "0\n1\n2\n";
        let res = parse(input);
        assert_eq!(res, vec![ 0, 1, 2 ]);
    }

    #[test]
    fn solves_empty_list() {
        let input = vec![ ];
        let res = solve_part1(&input);
        assert_eq!(res, 0);
    }

    #[test]
    fn solves_singleton_list() {
        let input = vec![ 42 ];
        let res = solve_part1(&input);
        assert_eq!(res, 0);
    }

    #[test]
    fn solves_decreasing_list() {
        let input = vec![ 3, 2, 1 ];
        let res = solve_part1(&input);
        assert_eq!(res, 0);
    }

    #[test]
    fn solves_increasing_list() {
        let input = vec![ 1, 2, 3 ];
        let res = solve_part1(&input);
        assert_eq!(res, 2);
    }

    #[test]
    fn solves_example_1() {
        let input = vec! [
            199, 200, 208, 210, 200,
            207, 240, 269, 260, 263,
        ];

        let res = solve_part1(&input);
        assert_eq!(res, 7);
    }

    #[test]
    fn solves_empty_part2() {
        let input = vec![ ];
        let res = solve_part2(&input);
        assert_eq!(res, 0);
    }

    #[test]
    fn solves_short_part2() {
        let input = vec![ 1, 2, 3 ];
        let res = solve_part2(&input);
        assert_eq!(res, 0);
    }

    #[test]
    fn solves_single_part2() {
        let input = vec![ 1, 2, 3, 2 ];
        let res = solve_part2(&input);
        assert_eq!(res, 1);
    }

    #[test]
    fn solves_example_2() {
        let input = vec! [
            199, 200, 208, 210, 200,
            207, 240, 269, 260, 263,
        ];

        let res = solve_part2(&input);
        assert_eq!(res, 5);
    }
}
