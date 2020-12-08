use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Op {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split_whitespace();
        let op = parts.next().unwrap();
        let addr = parts.next().unwrap().parse::<isize>().unwrap();

        match op {
            "nop" => Ok(Op::Nop(addr)),
            "acc" => Ok(Op::Acc(addr)),
            "jmp" => Ok(Op::Jmp(addr)),
            _     => Err(()),
        }
    }
}

struct Program {
    acc: isize,
    done: HashSet<usize>,
    pc: usize,
}

impl Program {
    fn new() -> Self {
        Self {
            pc: 0,
            acc: 0,
            done: HashSet::new(),
        }
    }

    fn run(&mut self, ops: &[Op]) -> Result<isize, isize> {
        loop {
            if self.done.contains(&self.pc) {
                return Err(self.acc);
            }

            if self.pc == ops.len() {
                return Ok(self.acc);
            }

            self.done.insert(self.pc);

            match &ops[self.pc] {
                Op::Nop(_) => {
                    self.pc += 1;
                }
                Op::Acc(x) => {
                    self.acc += x;
                    self.pc += 1;
                }
                Op::Jmp(x) => {
                    self.pc = ((self.pc as isize) + (*x as isize)) as usize;
                }
            }
        }
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
        self.done.clear();
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| Op::from_str(line).unwrap())
        .collect()
}

#[aoc(day8, part1)]
fn solve_part1(input: &[Op]) -> isize {
    Program::new().run(input).unwrap_err()
}

#[aoc(day8, part2)]
fn solve_part2(input: &[Op]) -> isize {
    let mut prog = Program::new();

    for (i, op) in input.iter().enumerate() {
        let mut test_input = Vec::from(input);
        match op {
            Op::Jmp(x) => {
                test_input[i] = Op::Nop(*x);
            }
            Op::Nop(x) => {
                test_input[i] = Op::Jmp(*x);
            }
            _ => continue,
        }

        if let Ok(acc) = prog.run(&test_input) {
            return acc;
        } else {
            prog.reset();
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let input = parse(input);
        let result = solve_part1(&input);
        assert_eq!(result, 5);
    }

    #[test]
    fn ex2() {
        let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let input = parse(input);
        let result = solve_part2(&input);
        assert_eq!(result, 8);
    }
}
