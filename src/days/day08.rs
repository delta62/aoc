use crate::input::Paragraphs;
use aoc_runner::{aoc, parse_opt, PuzzleError, PuzzleInput, Result};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc(year = 2023, day = 8, part = 1)]
fn part1(input: Puzzle) -> usize {
    let Puzzle { network, moves } = input;
    let moves = moves.into_iter();

    network.traverse(moves)
}

#[aoc(year = 2023, day = 8, part = 2)]
fn part2(input: Puzzle) -> usize {
    let Puzzle { network, moves } = input;
    let moves = moves.into_iter();

    network.parallel_traverse(moves)
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct NodeId(usize);

#[derive(Debug)]
pub struct Network {
    nodes: HashMap<NodeId, Node>,
    start: Option<NodeId>,
    target: Option<NodeId>,
}

impl Network {
    fn traverse<'a>(&'a self, moves: impl Iterator<Item = Direction> + 'a) -> usize {
        let mut current = self.nodes[&self.start.unwrap()];
        let mut move_count = 0;

        for dir in moves {
            move_count += 1;
            let next = match dir {
                Direction::Left => current.left,
                Direction::Right => current.right,
            };

            if next == self.target.unwrap() {
                return move_count;
            }

            current = self.nodes[&next];
        }

        unreachable!()
    }

    fn parallel_traverse<'a>(&'a self, moves: impl Iterator<Item = Direction> + 'a) -> usize {
        let mut move_count = 0;
        let mut current: Vec<_> = self
            .nodes
            .values()
            .filter_map(|n| if n.is_start { Some(n.id) } else { None })
            .collect();

        for dir in moves {
            move_count += 1;
            let mut all_finished = true;

            for current_id in &mut current {
                let next_id = match dir {
                    Direction::Left => self.nodes[&current_id].left,
                    Direction::Right => self.nodes[&current_id].right,
                };

                if !self.nodes[&next_id].is_end {
                    all_finished = false;
                }

                *current_id = next_id;
            }

            if all_finished {
                return move_count;
            }
        }

        unreachable!()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Node {
    id: NodeId,
    left: NodeId,
    right: NodeId,
    is_start: bool,
    is_end: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = PuzzleError;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
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
    fn parse(input: &'a str) -> Result<Self> {
        Ok(Self(
            input
                .chars()
                .map(Direction::try_from)
                .try_collect::<Direction, Vec<_>, PuzzleError>()?
                .into_iter(),
        ))
    }
}

pub struct Puzzle {
    moves: Moves,
    network: Network,
}

impl<'a> PuzzleInput<'a> for Puzzle {
    fn parse(input: &'a str) -> Result<Self> {
        let input = Paragraphs::parse(input)?;
        let mut input = input.iter();

        let moves = parse_opt!(input.next(), "input didn't include any moves")?;
        let moves = Moves::parse(moves)?;

        let network = parse_opt!(input.next(), "input didn't include any nodes")?;
        let network = Network::parse(network)?;

        Ok(Self { moves, network })
    }
}

impl<'a> PuzzleInput<'a> for Network {
    fn parse(input: &'a str) -> Result<Self> {
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

            let next_id = ids.len();
            let id = NodeId(*ids.entry(name).or_insert(next_id));

            let next_id = ids.len();
            let left = NodeId(*ids.entry(left).or_insert(next_id));

            let next_id = ids.len();
            let right = NodeId(*ids.entry(rght).or_insert(next_id));

            let is_start = name.ends_with('A');
            let is_end = name.ends_with('Z');

            let node = Node {
                id,
                left,
                right,
                is_start,
                is_end,
            };
            nodes.insert(id, node);
        });

        let start = ids.get("AAA").copied().map(NodeId);
        let target = ids.get("ZZZ").copied().map(NodeId);

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

    #[test]
    fn example_3() {
        let input = example_str!("2023/d8e3.txt");
        let input = Puzzle::parse(&input).unwrap();
        let result = part2(input);
        assert_eq!(result, 6);
    }
}
