use aoc_runner::{aoc, parse_opt, PuzzleInput, Result};
use std::collections::HashSet;

#[aoc(year = 2023, day = 4, part = 1)]
fn part1(input: Vec<Ticket>) -> usize {
    input.into_iter().map(|ticket| ticket.score()).sum()
}

#[aoc(year = 2023, day = 4, part = 2)]
fn part2(input: TicketStack) -> usize {
    input.ticket_count()
}

pub struct Ticket {
    mine: HashSet<usize>,
    winning: HashSet<usize>,
}

impl Ticket {
    fn score(&self) -> usize {
        let winning_count = self.winning_count();

        if winning_count == 0 {
            0
        } else {
            let exp = winning_count as u32 - 1;
            2usize.pow(exp)
        }
    }

    fn winning_count(&self) -> usize {
        self.mine.intersection(&self.winning).count()
    }
}

impl<'a> PuzzleInput<'a> for Ticket {
    fn parse(input: &'a str) -> Result<Self> {
        let (winning, mine) = parse_opt!(input.split_once('|'), "input didn't contain a |")?;
        let (_card_num, winning) =
            parse_opt!(winning.split_once(':'), "input line didn't contain a ':'")?;

        let winning = numbers!(winning => usize)?;
        let mine = numbers!(mine => usize)?;

        Ok(Ticket { mine, winning })
    }
}

pub struct TicketStack {
    tickets: Vec<Ticket>,
}

impl TicketStack {
    fn ticket_count(&self) -> usize {
        let mut ticket_counts = vec![1; self.tickets.len()];

        for (i, ticket) in self.tickets.iter().enumerate() {
            let match_count = ticket.winning_count();
            let increment_count = ticket_counts[i];
            let start = i + 1;
            let end = start + match_count;

            for x in &mut ticket_counts[start..end] {
                *x += increment_count;
            }
        }

        ticket_counts.into_iter().sum()
    }
}

impl<'a> PuzzleInput<'a> for TicketStack {
    fn parse(input: &'a str) -> Result<Self> {
        let tickets = <Vec<Ticket>>::parse(input)?;

        Ok(Self { tickets })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = example_str!("2023/d4e1.txt");
        let input = <Vec<Ticket>>::parse(&input).unwrap();
        let result = part1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn example_2() {
        let input = example_str!("2023/d4e1.txt");
        let input = TicketStack::parse(&input).unwrap();
        let result = part2(input);
        assert_eq!(result, 30);
    }
}
