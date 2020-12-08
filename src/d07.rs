use aoc_runner_derive::aoc;
use lazy_static::lazy_static;
use petgraph::{graph::NodeIndex, visit::Bfs, Graph};
use regex::Regex;
use std::collections::HashMap;

fn parse(input: &str) -> (Graph<&str, usize>, HashMap<&str, NodeIndex>) {
    let mut graph = Graph::new();
    let mut nodes = HashMap::new();

    input
        .lines()
        .for_each(|line| {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"(\d+) ([^,.]+) bags?").unwrap();
            }

            let mut parts = line.split(" bags contain ");
            let parent = parts.next().unwrap();
            let parts = parts.next().unwrap().split(", ");

            let parent_idx = if !nodes.contains_key(parent) {
                let idx = graph.add_node(parent);
                nodes.insert(parent, idx);
                idx
            } else {
                *nodes.get(parent).unwrap()
            };

            for child in parts {
                if let Some(captures) = RE.captures(child) {
                    let count = captures[1].parse::<usize>().unwrap();
                    let color = captures.get(2).unwrap().as_str();

                    let child_idx = if !nodes.contains_key(color) {
                        let idx = graph.add_node(color);
                        nodes.insert(color, idx);
                        idx
                    } else {
                        *nodes.get(color).unwrap()
                    };

                    graph.add_edge(parent_idx, child_idx, count);
                }
            }
        });

    (graph, nodes)
}

#[aoc(day7, part1)]
fn solve_part1(input: &str) -> usize {
    let (mut graph, nodes) = parse(input);
    let goal = nodes.get("shiny gold").unwrap();
    graph.reverse();

    let mut search = Bfs::new(&graph, *goal);

    let mut sum = 0;
    while let Some(n) = search.next(&graph) {
        sum += 1;
    }

    sum - 1
}

#[aoc(day7, part2)]
fn solve_part2(input: &str) -> usize {
    let (mut graph, nodes) = parse(input);
    let goal = nodes.get("shiny gold").unwrap();

    find_cost(&graph, &goal) - 1
}

fn find_cost(graph: &Graph<&str, usize>, from: &NodeIndex) -> usize {
    let mut sum = 1;

    for to in graph.neighbors(*from) {
        let edge = graph.find_edge(*from, to).unwrap();
        let edge_cost = graph.edge_weight(edge).unwrap();
        sum += edge_cost * find_cost(graph, &to);
    }

    sum
}

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

        let res = solve_part1(&input);
        assert_eq!(res, 4);
    }

    #[test]
    fn ex2() {
        let input = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let res = solve_part2(input);
        assert_eq!(res, 126);
    }
}
