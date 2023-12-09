use crate::input::Paragraphs;
use aoc_runner::{aoc, PuzzleError, PuzzleInput};
use std::collections::HashMap;

#[aoc(year = 2023, day = 8, part = 1)]
fn part1(input: Puzzle) -> usize {
    let Puzzle { network, moves } = input;
    network.traverse(moves.into_iter()).count()
}

#[derive(Debug)]
pub struct Network {
    nodes: HashMap<usize, Node>,
    start: usize,
    target: usize,
}

impl Network {
    fn traverse<'a>(
        &'a self,
        moves: impl Iterator<Item = Direction> + 'a,
    ) -> impl Iterator<Item = usize> + 'a {
        let mut current = self.nodes[&self.start];
        let mut current_id = self.start;

        moves
            .map(move |dir| {
                let next = match dir {
                    Direction::Left => current.0,
                    Direction::Right => current.1,
                };
                let tmp = current_id;
                current = self.nodes[&next];
                current_id = next;
                tmp
            })
            .take_while(|current| *current != self.target)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Node(usize, usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = PuzzleError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            x => Err(PuzzleError::ParseError(format!("unexpected direction {x}"))),
        }
    }
}

pub struct Moves(std::vec::IntoIter<Direction>);

impl Moves {
    fn into_iter(self) -> impl Iterator<Item = Direction> {
        self.0.cycle()
    }
}

impl<'a> PuzzleInput<'a> for Moves {
    fn parse(input: &'a str) -> aoc_runner::Result<Self> {
        Ok(Self(
            input
                .chars()
                .map(TryFrom::try_from)
                .try_collect::<Vec<_>>()?
                .into_iter(),
        ))
    }
}

pub struct Puzzle {
    moves: Moves,
    network: Network,
}

impl<'a> PuzzleInput<'a> for Puzzle {
    fn parse(input: &'a str) -> aoc_runner::Result<Self> {
        let input = Paragraphs::parse(input)?;
        let mut input = input.iter();

        let moves = Moves::parse(input.next().unwrap())?;
        let network = Network::parse(input.next().unwrap())?;

        Ok(Self { moves, network })
    }
}

impl<'a> PuzzleInput<'a> for Network {
    fn parse(input: &'a str) -> aoc_runner::Result<Self> {
        let mut ids = HashMap::new();
        let mut nodes = HashMap::new();

        input.lines().for_each(|line| {
            let (name, def) = line.split_once(" = ").unwrap();
            let (left, rght) = def
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .unwrap();

            let id = ids.len();
            let this_id = *ids.entry(name).or_insert(id);
            let id = ids.len();
            let left_id = *ids.entry(left).or_insert(id);
            let id = ids.len();
            let rght_id = *ids.entry(rght).or_insert(id);

            let node = Node(left_id, rght_id);
            nodes.insert(this_id, node);
        });

        let start = ids["AAA"];
        let target = ids["ZZZ"];

        Ok(Self {
            nodes,
            start,
            target,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = example_str!("2023/d8e1.txt");
        let input = Puzzle::parse(&input).unwrap();
        let result = part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let input = example_str!("2023/d8e2.txt");
        let input = Puzzle::parse(&input).unwrap();
        let result = part1(input);
        assert_eq!(result, 6);
    }

    // #[test]
    // fn example_2() {
    //     let input = example_str!("2023/d8e1.txt");
    //     let result = part2(&input);
    //     assert_eq!(result, 5905);
    // }
}
