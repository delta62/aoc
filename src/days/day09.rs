use aoc_runner::{aoc, PuzzleInput};

#[aoc(year = 2023, day = 9, part = 1)]
fn part1(input: Vec<History>) -> i32 {
    input.into_iter().map(|hist| hist.predict_next()).sum()
}

#[aoc(year = 2023, day = 9, part = 2)]
fn part2(input: Vec<History>) -> i32 {
    input.into_iter().map(|hist| hist.predict_prev()).sum()
}

pub struct History {
    nums: Vec<i32>,
}

impl History {
    fn predict_next(self) -> i32 {
        let mut sequences = vec![self.nums];

        loop {
            let last_seq = sequences.last().unwrap();
            let mut all_zero = true;
            let (first, rest) = last_seq.split_first().unwrap();
            let new = rest
                .iter()
                .copied()
                .scan(*first, |last, x| {
                    let diff = x - *last;
                    if diff != 0 {
                        all_zero = false;
                    }

                    *last = x;
                    Some(diff)
                })
                .collect();

            if all_zero {
                break;
            } else {
                sequences.push(new);
            }
        }

        sequences.iter().rev().fold(0, |last_last, row| {
            let this_row_last = *row.last().unwrap();
            this_row_last + last_last
        })
    }

    fn predict_prev(self) -> i32 {
        let mut sequences = vec![self.nums];

        loop {
            let last_seq = sequences.last().unwrap();
            let mut all_zero = true;
            let (first, rest) = last_seq.split_first().unwrap();
            let new: Vec<_> = rest
                .iter()
                .copied()
                .scan(*first, |last, x| {
                    let diff = x - *last;
                    if diff != 0 {
                        all_zero = false;
                    }
                    *last = x;
                    Some(diff)
                })
                .collect();

            if all_zero {
                break;
            } else {
                sequences.push(new);
            }
        }

        sequences.iter().rev().fold(0, |last_first, row| {
            let this_row_first = *row.first().unwrap();
            this_row_first - last_first
        })
    }
}

impl<'a> PuzzleInput<'a> for History {
    fn parse(input: &'a str) -> aoc_runner::Result<Self> {
        let nums = numbers!(input => i32)?;
        Ok(Self { nums })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = example_str!("2023/d9e1.txt");
        let input = <Vec<History>>::parse(&input).unwrap();
        let result = part1(input);
        assert_eq!(result, 114);
    }

    #[test]
    fn example_2() {
        let input = example_str!("2023/d9e1.txt");
        let input = <Vec<History>>::parse(&input).unwrap();
        let result = part2(input);
        assert_eq!(result, 2);
    }
}
