use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day10, part1)]
fn solve_part1(input: &[usize]) -> usize {
    let mut data = Vec::from(input);
    data.sort_unstable();

    let mut ones = 0;
    let mut threes = 0;

    data.iter().fold(0, |acc, &x| {
        let diff = x - acc;

        if diff == 1 {
            ones += 1;
        } else if diff == 3 {
            threes += 1;
        }

        x
    });

    ones * (threes + 1)
}

/*
 * case 1:
 * [ 0, 3, 6, 9 ]
 * Nothing can be removed
 *
 * case 2:
 * [ 0, 3, 4, 6, 9 ]
 * 4 can be removed
 *
 * case 3:
 * [ 0, 3, 4, 5, 6, 9 ]
 * 4 can be removed
 * 5 can be removed
 * 4 and 5 can be removed
 *
 * case 4:
 * [ 0, 3, 4, 5, 6, 7, 10 ]
 * 4 can be removed
 * 5 can be removed
 * 6 can be removed
 * 4 and 5 can be removed
 * 5 and 6 can be removed
 * 4 and 6 can be removed
 *
 * case 5:
 * [ 0, 3, 4, 5, 6, 7, 8, 11 ]
 * 4 can be removed
 * 5 can be removed
 * 6 can be removed
 * 7 can be removed
 * 4 and 5 can be removed
 * 4 and 6 can be removed
 * 4 and 7 can be removed
 * 5 and 6 can be removed
 * 5 and 7 can be removed
 *
 * 3win
 * [ 0 3 4  ] -
 * [ 3 4 5  ] 4
 * [ 4 5 6  ] 5
 * [ 5 6 7  ] 6
 * [ 6 7 8  ] 7
 * [ 7 8 11 ] -
 *
 * 4win
 * [ 0 3 4 5  ] -
 * [ 3 4 5 6  ] 4, 5
 * [ 4 5 6 7  ] 5, 6
 * [ 5 6 7 8  ] 6, 7
 * [ 6 7 8 11 ] -
 */

#[aoc(day10, part2)]
fn solve_part2(input: &[usize]) -> usize {
    let mut data = Vec::from(input);
    data.sort_unstable();
    data.insert(0, 0);
    data.push(data[data.len() - 1] + 3);

    let mut singles = 0;

    let end_idx = data.len() - 3;
    for i in 1..end_idx {
        let first = data[i - 1];
        let middle = data[i];
        let last = data[i + 1];

        if last <= first + 3 {
            println!("single {}", middle);
            singles += 1;
        }
    }

    let mut doubles = 0;
    let end_idx = data.len() - 4;
    for i in 1..end_idx {
        let first = data[i - 1];
        let mid1 = data[i];
        let mid2 = data[i + 1];
        let last = data[i + 2];

        if last <= first + 3 {
            println!("double {}, {}", mid1, mid2);
            doubles += 1;
        }
    }

    2usize.pow(singles - doubles) + 2usize.pow(doubles)
    // 2usize.pow(singles + doubles)
}

#[cfg(test)]
mod tests {
    use super::*;

//     #[test]
//     fn ex1() {
//         let input = r"16
// 10
// 15
// 5
// 1
// 11
// 7
// 19
// 6
// 12
// 4";
//         let input = parse(input);
//         let result = solve_part1(&input);
//         assert_eq!(result, 35);
//     }

    #[test]
    fn ex2() {
        let input = r"16
10
15
5
1
11
7
19
6
12
4";
        let input = parse(input);
        let result = solve_part2(&input);
        assert_eq!(result, 8);
    }
}
