use aoc_runner::{aoc, PuzzleInput, Result};
use regex::Regex;

#[aoc(year = 2023, day = 3, part = 1)]
fn part1(input: Grid) -> usize {
    input.adjacenct_to_sym().sum()
}

#[aoc(year = 2023, day = 3, part = 2)]
fn part2(input: Grid) -> usize {
    input
        .gears()
        .filter_map(|gear| input.tension(gear.x, gear.y))
        .sum()
}

#[derive(Debug)]
struct Number {
    value: usize,
    x: usize,
    y: usize,
}

impl Number {
    fn overlaps(&self, x: usize, y: usize) -> bool {
        x >= self.x.saturating_sub(1)
            && x <= self.x + self.len() + 1
            && y >= self.y.saturating_sub(1)
            && y <= self.y + 1
    }

    fn len(&self) -> usize {
        (self.value as f64).log10().floor() as usize
    }
}

#[derive(Clone, Copy, Debug)]
struct Symbol {
    value: char,
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Grid {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Grid {
    fn gears(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.symbols.iter().filter(|s| s.value == '*').copied()
    }

    fn tension(&self, x: usize, y: usize) -> Option<usize> {
        let adjacent: Vec<_> = self.numbers.iter().filter(|n| n.overlaps(x, y)).collect();

        if adjacent.len() == 2 {
            Some(adjacent.into_iter().map(|n| n.value).product())
        } else {
            None
        }
    }

    fn adjacenct_to_sym(&self) -> impl Iterator<Item = usize> + '_ {
        self.numbers
            .iter()
            .filter(|n| self.symbols.iter().any(|s| n.overlaps(s.x, s.y)))
            .map(|n| n.value)
    }
}

impl<'a> PuzzleInput<'a> for Grid {
    fn parse(input: &'a [u8]) -> Result<Self> {
        let s = <&str as PuzzleInput>::parse(input)?;

        let mut numbers = vec![];
        let mut symbols = vec![];

        let regex = Regex::new(r"(?<num>\d+)|(?<sym>[^.])").unwrap();

        for (y, row) in s.lines().enumerate() {
            let captures = regex.captures_iter(row);

            for cap in captures {
                if let Some(num) = &cap.name("num") {
                    let value = num.as_str().parse::<usize>().unwrap();
                    let x = num.start();
                    numbers.push(Number { x, y, value });
                }

                if let Some(sym) = &cap.name("sym") {
                    let x = sym.start();
                    let value = sym.as_str().chars().next().unwrap();
                    symbols.push(Symbol { value, x, y });
                }
            }
        }

        Ok(Self { numbers, symbols })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = example_str!("2023/d3e1.txt");
        let input = Grid::parse(input.as_bytes()).unwrap();
        let result = part1(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn example2() {
        let input = example_str!("2023/d3e1.txt");
        let input = Grid::parse(input.as_bytes()).unwrap();
        let result = part2(input);
        assert_eq!(result, 467835);
    }
}
