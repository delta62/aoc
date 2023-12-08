use aoc_runner::{aoc, parse, parse_opt, PuzzleError, PuzzleInput, Result};
use std::{
    cmp::{Ordering, PartialEq},
    collections::{HashMap, HashSet},
};

#[aoc(year = 2023, day = 7, part = 1)]
fn part1<'a>(mut input: Vec<Hand>) -> usize {
    input.sort();

    input
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1))
        .sum()
}

#[aoc(year = 2023, day = 7, part = 2)]
fn part2<'a>(mut input: Vec<Hand>) -> usize {
    42
}

#[derive(Debug, Eq, Ord)]
pub struct Hand {
    bid: usize,
    cards: Vec<Card>,
    score: HandScore,
}

impl Hand {
    fn new(bid: usize, cards: Vec<Card>) -> Self {
        let score = Self::score_hand(&cards);
        Self { score, bid, cards }
    }

    fn score_hand(cards: &[Card]) -> HandScore {
        let groups = cards.iter().fold(HashMap::new(), |mut acc, card| {
            let entry: &mut usize = acc.entry(card).or_default();
            *entry += 1;
            acc
        });

        let mut group_sizes: Vec<_> = groups.values().collect();

        group_sizes.sort();
        group_sizes.reverse();

        match group_sizes[0] {
            5 => HandScore::FiveOfAKind,
            4 => HandScore::FourOfAKind,
            3 => match group_sizes[1] {
                2 => HandScore::FullHouse,
                _ => HandScore::ThreeOfAKind,
            },
            2 => match group_sizes[1] {
                2 => HandScore::TwoPair,
                _ => HandScore::OnePair,
            },
            1 => HandScore::HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.iter().collect::<HashSet<_>>() == other.cards.iter().collect::<HashSet<_>>()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.score
                .cmp(&other.score)
                .then(self.cards.cmp(&other.cards)),
        )
    }
}

impl<'a> PuzzleInput<'a> for Hand {
    fn parse(input: &'a str) -> Result<Self> {
        let (cards, bid) = parse_opt!(input.split_once(' '), "expected space-separated input")?;
        let bid = parse!(bid.parse::<usize>())?;
        let cards = cards.chars().map(Card::try_from).try_collect::<Vec<_>>()?;

        Ok(Self::new(bid, cards))
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum HandScore {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, Ord, Eq, Hash, PartialEq, PartialOrd)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = PuzzleError;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        use Card::*;

        Ok(match value {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            c => Err(PuzzleError::ParseError(format!(
                "Unexpected character: {c}"
            )))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = example_str!("2023/d7e1.txt");
        let input = <Vec<Hand>>::parse(&input).unwrap();
        let result = part1(input);
        assert_eq!(result, 6440);
    }

    #[test]
    fn example_2() {
        let input = example_str!("2023/d7e1.txt");
        let input = <Vec<Hand>>::parse(&input).unwrap();
        let result = part2(input);
        assert_eq!(result, 5905);
    }
}
