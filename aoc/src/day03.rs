use aoc_macros::aoc;
use aoc_runner::PuzzleInput;
use regex::Regex;

#[aoc(year = 2023, day = 3, part = 1)]
fn part1(input: Grid) -> usize {
    input.adjacenct_to_sym().sum()
}

#[derive(Debug)]
struct Number {
    value: usize,
    x: usize,
    y: usize,
}

impl Number {
    fn overlaps(&self, x: usize, y: usize) -> bool {
        let len: usize = (self.value as f64).log10().floor() as usize;
        x >= self.x.saturating_sub(1)
            && x <= self.x + len + 1
            && y >= self.y.saturating_sub(1)
            && y <= self.y + 1
    }
}

#[derive(Debug)]
struct Symbol {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Grid {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Grid {
    fn adjacenct_to_sym<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.numbers
            .iter()
            .filter(|n| self.symbols.iter().any(|s| n.overlaps(s.x, s.y)))
            .map(|n| n.value)
    }
}

impl<'a> PuzzleInput<'a> for Grid {
    fn parse(input: &'a [u8]) -> aoc_runner::Result<Self> {
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
                    symbols.push(Symbol { x, y });
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
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let input = Grid::parse(input.as_bytes()).unwrap();
        let result = part1(input);
        assert_eq!(result, 4361);
    }
}
