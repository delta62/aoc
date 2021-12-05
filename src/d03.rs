use aoc_runner_derive::{aoc, aoc_generator};

type Bits = Vec<bool>;

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Bits> {
    input
        .lines()
        .map(|line| {
            let mut bits = Vec::with_capacity(line.len());

            for val in line.chars() {
                match val {
                    '1' => bits.push(true),
                    '0' => bits.push(false),
                    _   => panic!("Invalid input"),
                }
            }

            bits
        })
        .collect()
}

#[aoc(day3, part1)]
fn solve_part1(input: &[Bits]) -> usize {
    let len = input.len();
    if len == 0 {
        return 0;
    }

    let bit_count = input[0].len();
    let mut sums = vec![ 0; bit_count ];

    for bits in input {
        for (i, &bit) in bits.iter().enumerate() {
            if bit {
                sums[i] += 1;
            }
        }
    }

    let mut gamma = 0;
    for bit in sums {
        gamma <<= 1;

        if bit > len / 2 {
            gamma |= 1;
        }
    }

    let mask = (1 << bit_count) - 1;
    let epsilon = !gamma & mask;

    gamma * epsilon
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_input() {
        let input = "11111\n00000\n10101";
        let res = parse(&input);
        assert_eq!(
            res,
            vec![
                [ true,  true,  true,  true,  true  ],
                [ false, false, false, false, false ],
                [ true,  false, true,  false, true  ],
            ]
        );
    }

    #[test]
    fn solves_example_1() {
        let input = parse(r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010");
        let res = solve_part1(&input);
        assert_eq!(res, 198);
    }

    #[test]
    fn gives_0_for_empty_part1() {
        let res = solve_part1(&vec![ ]);
        assert_eq!(res, 0);
    }
}
