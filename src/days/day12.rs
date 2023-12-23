use aoc_runner::{aoc, parse_opt, PuzzleInput, Result};
use itertools::Itertools;
use std::fmt::Debug;

#[aoc(year = 2023, day = 12, part = 1)]
fn part1(input: Vec<Springs>) -> usize {
    input.into_iter().map(|spring| spring.arrangements()).sum()
}

#[derive(Debug)]
pub struct Springs {
    springs: Vec<Spring>,
    group_sizes: Vec<usize>,
}

#[derive(Default)]
struct StackEntry {
    run_length: usize,
    groups_index: usize,
    spring_index: usize,
}

impl Springs {
    fn arrangements(&self) -> usize {
        let mut stack = vec![StackEntry::default()];
        let mut arrangements = 0;

        while let Some(entry) = stack.pop() {
            let next_spring = self.springs.get(entry.spring_index);
            let next_group_size = self.group_sizes.get(entry.groups_index);

            match (next_spring, next_group_size, entry.run_length) {
                (None, None, 0) => arrangements += 1,
                (None, None, _) => {}
                (None, Some(&group_size), run_length) if run_length == group_size => {
                    stack.push(StackEntry {
                        spring_index: entry.spring_index,
                        run_length: 0,
                        groups_index: entry.groups_index + 1,
                    });
                }
                (None, Some(_), _) => {}
                (Some(Spring::Operational) | Some(Spring::Unknown), None, 0) => {
                    stack.push(StackEntry {
                        spring_index: entry.spring_index + 1,
                        run_length: 0,
                        groups_index: entry.groups_index,
                    });
                }
                (Some(Spring::Operational) | Some(Spring::Unknown), None, _) => {}
                (Some(Spring::Damaged), None, _) => {}
                (Some(Spring::Damaged), Some(&group_size), run_length)
                    if run_length == group_size => {}
                (Some(Spring::Damaged), Some(_), _) => {
                    stack.push(StackEntry {
                        spring_index: entry.spring_index + 1,
                        run_length: entry.run_length + 1,
                        groups_index: entry.groups_index,
                    });
                }
                (Some(Spring::Operational), Some(&group_size), run_length) => {
                    if run_length == group_size {
                        stack.push(StackEntry {
                            run_length: 0,
                            groups_index: entry.groups_index + 1,
                            spring_index: entry.spring_index + 1,
                        });
                    }

                    if run_length == 0 {
                        stack.push(StackEntry {
                            run_length: 0,
                            groups_index: entry.groups_index,
                            spring_index: entry.spring_index + 1,
                        });
                    }
                }
                (Some(Spring::Unknown), Some(&group_size), run_length)
                    if group_size == run_length =>
                {
                    stack.push(StackEntry {
                        run_length: 0,
                        groups_index: entry.groups_index + 1,
                        spring_index: entry.spring_index + 1,
                    });
                }
                (Some(Spring::Unknown), Some(&group_size), run_length)
                    if group_size < run_length => {}
                (Some(Spring::Unknown), Some(_), run_length) => {
                    if run_length == 0 {
                        stack.push(StackEntry {
                            run_length: 0,
                            groups_index: entry.groups_index,
                            spring_index: entry.spring_index + 1,
                        });
                    }

                    stack.push(StackEntry {
                        run_length: run_length + 1,
                        groups_index: entry.groups_index,
                        spring_index: entry.spring_index + 1,
                    });
                }
            }
        }

        arrangements
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Spring {
    Damaged,
    Operational,
    Unknown,
}

impl Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Damaged => write!(f, "#"),
            Self::Operational => write!(f, "."),
            Self::Unknown => write!(f, "?"),
        }
    }
}

impl<'a> PuzzleInput<'a> for Springs {
    fn parse(input: &'a str) -> Result<Self> {
        let (springs, group_sizes) = parse_opt!(input.split_once(' '), "No space in input line")?;
        let group_sizes = numbers!(group_sizes => usize; ',')?;
        let springs = springs
            .chars()
            .map(|c| match c {
                '#' => Spring::Damaged,
                '.' => Spring::Operational,
                '?' => Spring::Unknown,
                _ => unreachable!(),
            })
            .collect();

        Ok(Self {
            springs,
            group_sizes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = example_str!("2023/d12e1.txt");
        let input = <Vec<Springs>>::parse(&input).unwrap();
        let result = part1(input);
        assert_eq!(result, 21);
    }

    #[test]
    fn matches_single_broken_spring() {
        let input = "# 1";
        let input = <Vec<Springs>>::parse(input).unwrap();
        let result = input.first().unwrap().arrangements();
        assert_eq!(result, 1);
    }

    #[test]
    fn matches_with_broken_operational_spring() {
        let input = "#. 1";
        let input = <Vec<Springs>>::parse(input).unwrap();
        let result = input.first().unwrap().arrangements();
        assert_eq!(result, 1);
    }

    #[test]
    fn matches_with_unknown_spring() {
        let input = "? 1";
        let input = <Vec<Springs>>::parse(input).unwrap();
        let result = input.first().unwrap().arrangements();
        assert_eq!(result, 1);
    }

    #[test]
    fn matches_with_double_unknown_spring() {
        let input = "?? 1";
        let input = <Vec<Springs>>::parse(input).unwrap();
        let result = input.first().unwrap().arrangements();
        assert_eq!(result, 2);
    }

    #[test]
    fn matches_with_triple_unknown_spring() {
        let input = "??? 1,1";
        let input = <Vec<Springs>>::parse(input).unwrap();
        let result = input.first().unwrap().arrangements();
        assert_eq!(result, 1);
    }

    #[test]
    fn example1_line1() {
        let input = "???.### 1,1,3";
        let input = <Vec<Springs>>::parse(input).unwrap();
        let result = input.first().unwrap().arrangements();
        assert_eq!(result, 1);
    }

    #[test]
    fn example1_line2() {
        let input = ".??..??...?##. 1,1,3";
        let input = <Vec<Springs>>::parse(input).unwrap();
        let result = input.first().unwrap().arrangements();
        assert_eq!(result, 4);
    }
}
