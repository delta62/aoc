use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split_whitespace();
        let op = parts.next().unwrap();
        let addr = parts.next().unwrap().parse::<isize>().unwrap();

        match op {
            "nop" => Ok(Instruction::Nop(addr)),
            "acc" => Ok(Instruction::Acc(addr)),
            "jmp" => Ok(Instruction::Jmp(addr)),
            _ => Err(()),
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

    fn run_no_repeat(&mut self, ops: &[Instruction]) -> isize {
        loop {
            if self.done.contains(&self.pc) {
                return self.acc
            }

            self.done.insert(self.pc);
            let op = &ops[self.pc];

            match op {
                Instruction::Nop(_) => {
                    self.pc += 1;
                }
                Instruction::Acc(x) => {
                    self.acc += x;
                    self.pc += 1;
                }
                Instruction::Jmp(x) => {
                    self.pc = ((self.pc as isize) + (*x as isize)) as usize;
                }
            }
        }
    }

    fn run(&mut self, ops: &[Instruction]) -> Result<isize, ()> {
        loop {
            if self.done.contains(&self.pc) {
                return Err(());
            }

            if self.pc == ops.len() {
                break;
            }

            self.done.insert(self.pc);
            let op = &ops[self.pc];

            match op {
                Instruction::Nop(_) => {
                    self.pc += 1;
                }
                Instruction::Acc(x) => {
                    self.acc += x;
                    self.pc += 1;
                }
                Instruction::Jmp(x) => {
                    self.pc = ((self.pc as isize) + (*x as isize)) as usize;
                }
            }
        }

        Ok(self.acc)
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
        self.done.clear();
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect()
}

#[aoc(day8, part1)]
fn solve_part1(input: &[Instruction]) -> isize {
    Program::new().run_no_repeat(input)
}

#[aoc(day8, part2)]
fn solve_part2(input: &[Instruction]) -> isize {
    let mut prog = Program::new();

    for (i, op) in input.iter().enumerate() {
        if let Instruction::Jmp(x) = op {
            let mut test_input = Vec::new();
            for x in input {
                test_input.push(*x);
            }

            test_input[i] = Instruction::Nop(*x);

            if let Ok(acc) = prog.run(&test_input) {
                return acc;
            } else {
                prog.reset();
            }
        }
    }

    for (i, op) in input.iter().enumerate() {
        if let Instruction::Nop(x) = op {
            let mut test_input = Vec::new();
            for x in input {
                test_input.push(*x);
            }

            test_input[i] = Instruction::Jmp(*x);

            if let Ok(acc) = prog.run(&test_input) {
                return acc;
            } else {
                prog.reset();
            }
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
