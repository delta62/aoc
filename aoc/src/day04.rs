use aoc_macros::aoc;
use aoc_runner::PuzzleInput;
use std::collections::HashSet;

#[aoc(year = 2023, day = 4, part = 1)]
fn part1<'a>(input: Vec<Ticket>) -> usize {
    input.into_iter().map(|ticket| ticket.score()).sum()
}

pub struct Ticket {
    mine: HashSet<usize>,
    winning: HashSet<usize>,
}

impl Ticket {
    fn score(&self) -> usize {
        let winning_count = self.mine.intersection(&self.winning).count();

        if winning_count == 0 {
            0
        } else {
            let exp = winning_count as u32 - 1;
            2usize.pow(exp)
        }
    }
}

impl<'a> PuzzleInput<'a> for Ticket {
    fn parse(input: &'a [u8]) -> aoc_runner::Result<Self> {
        let input = <&str as PuzzleInput>::parse(input)?;
        let (winning, mine) = input.split_once('|').unwrap();
        let (_card_num, winning) = winning.split_once(':').unwrap();

        let winning = winning
            .split_whitespace()
            .map(|s| usize::from_str_radix(s, 10).unwrap())
            .collect();
        let mine = mine
            .split_whitespace()
            .map(|s| usize::from_str_radix(s, 10).unwrap())
            .collect();

        Ok(Ticket { mine, winning })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = example_str!("2023/d4e1.txt");
        let input = input
            .lines()
            .map(|line| Ticket::parse(line.as_bytes()).unwrap())
            .collect();
        let result = part1(input);
        assert_eq!(result, 13);
    }
}
