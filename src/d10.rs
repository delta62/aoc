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

#[aoc(day10, part2)]
fn solve_part2(input: &[usize]) -> usize {
    let mut data = Vec::from(input);
    data.sort_unstable();
    data.insert(0, 0);
    data.push(data[data.len() - 1] + 3);

    let mut data = data.iter().map(|&x| (x, 0)).collect::<Vec<_>>();
    data[0].1 = 1;

    for i in 0..data.len() {
        let (x, paths) = data[i];

        let mut j = i + 1;
        loop {
            if j >= data.len() {
                break;
            }

            let (y, _) = data[j];

            if y - x > 3 {
                break;
            }

            data[j].1 += paths;

            j += 1;
        }
    }

    data[data.len() - 1].1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
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
        let result = solve_part1(&input);
        assert_eq!(result, 35);
    }

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
