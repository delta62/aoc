use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

struct Node {
    val: String,
    parents: HashSet<usize>,
}

impl Node {
    fn new(val: String) -> Self {
        Self {
            val,
            parents: HashSet::new(),
        }
    }
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    fn contains(&self, val: &str) -> bool {
        self.nodes.iter().any(|n| n.val.as_str() == val)
    }

    fn insert(&mut self, val: String) {
        self.nodes.push(Node::new(val));
    }

    fn get_points_to(&self, val: &str) -> Vec<&str> {
        match self.nodes.iter().find(|n| n.val.as_str() == val) {
            Some(node) => {
                node.parents
                    .iter()
                    .map(|idx| self.nodes[*idx].val.as_str())
                    .collect()
            }
            None => Vec::new(),
        }
    }

    fn set_parent(&mut self, parent: &str, child: &str) {
        let child_idx = self.nodes
            .iter()
            .position(|n| n.val.as_str() == child)
            .unwrap();

        let parent_idx = self.nodes
            .iter()
            .position(|n| n.val.as_str() == parent)
            .unwrap();

        self.nodes[child_idx].parents.insert(parent_idx);
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Graph {
    let mut graph = Graph::new();

    input
        .lines()
        .for_each(|line| {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"(\d+) ([^,.]+) bags?").unwrap();
            }

            let mut parts = line.split(" bags contain ");
            let parent = parts.next().unwrap().to_string();
            let parts = parts.next().unwrap().split(", ");

            if !graph.contains(&parent) {
                graph.insert(parent.clone());
            }

            for child in parts {
                if let Some(captures) = RE.captures(child) {
                    let _count = captures[1].parse::<usize>().unwrap();
                    let color = captures[2].to_string();

                    if !graph.contains(&color) {
                        graph.insert(color.clone());
                    }

                    graph.set_parent(&parent, &color);
                }
            }
        });

    graph
}

#[aoc(day7, part1)]
fn solve_part1(input: &Graph) -> usize {
    let mut points_to = HashSet::new();
    let mut todo = Vec::new();

    todo.push("shiny gold");

    while !todo.is_empty() {
        let goal = todo.pop().unwrap();
        let parents = input.get_points_to(goal);
        for parent in parents {
            todo.push(parent);
            // println!("{} points to {}", parent, goal);
            points_to.insert(parent);
        }
    }

    points_to.len()
}

// #[aoc(day7, part2)]
// fn solve_part2(input: &Graph) -> usize {
//     42
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let input = parse(input);
        let res = solve_part1(&input);
        assert_eq!(res, 4);
    }

    #[test]
    fn ex2() {
    }
}
