use crate::geom::Point;
use aoc_runner::{aoc, PuzzleInput};
use std::collections::HashSet;

#[aoc(year = 2023, day = 11, part = 1)]
fn part1(mut input: Universe) -> usize {
    input.expand(2);
    input.distances().into_iter().sum::<usize>() / 2
}
#[aoc(year = 2023, day = 11, part = 2)]
fn part2(mut input: Universe) -> usize {
    input.expand(1_000_000);
    input.distances().into_iter().sum::<usize>() / 2
}

pub struct Universe {
    galaxy_locations: HashSet<Point<usize>>,
}

impl Universe {
    fn expand(&mut self, empty_size: usize) {
        let max_line = self.galaxy_locations.iter().max_by_key(|p| p.y).unwrap().y;
        let non_empty_lines: HashSet<_> = self.galaxy_locations.iter().map(|p| p.y).collect();
        let empty_lines = (0..max_line)
            .collect::<HashSet<_>>()
            .difference(&non_empty_lines)
            .copied()
            .collect::<Vec<_>>();

        let max_col = self.galaxy_locations.iter().max_by_key(|p| p.x).unwrap().x;
        let non_empty_columns: HashSet<_> = self.galaxy_locations.iter().map(|p| p.x).collect();
        let empty_columns = (0..max_col)
            .collect::<HashSet<_>>()
            .difference(&non_empty_columns)
            .copied()
            .collect::<Vec<_>>();

        let galaxy_locations = self
            .galaxy_locations
            .iter()
            .map(|&point| offset_for(point, &empty_lines, &empty_columns, empty_size))
            .collect();

        self.galaxy_locations = galaxy_locations;
    }

    fn distances(&self) -> Vec<usize> {
        self.galaxy_locations
            .iter()
            .copied()
            .flat_map(|location| self.distances_from_point(location))
            .collect()
    }

    fn distances_from_point(&self, point: Point<usize>) -> impl Iterator<Item = usize> + '_ {
        self.galaxy_locations.iter().filter_map(move |&galaxy_loc| {
            if galaxy_loc == point {
                None
            } else {
                let dx = galaxy_loc.x.abs_diff(point.x);
                let dy = galaxy_loc.y.abs_diff(point.y);
                Some(dx + dy)
            }
        })
    }
}

impl<'a> PuzzleInput<'a> for Universe {
    fn parse(input: &'a str) -> aoc_runner::Result<Self> {
        let galaxy_locations = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(Point { x, y })
                    } else {
                        None
                    }
                })
            })
            .collect();

        Ok(Self { galaxy_locations })
    }
}

fn offset_for(
    point: Point<usize>,
    empty_lines: &[usize],
    empty_columns: &[usize],
    empty_size: usize,
) -> Point<usize> {
    let cols_to_add = empty_columns.iter().filter(|&&c| c < point.x).count();
    let rows_to_add = empty_lines.iter().filter(|&&r| r < point.y).count();
    let x = point.x + cols_to_add * (empty_size - 1);
    let y = point.y + rows_to_add * (empty_size - 1);

    Point { x, y }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = example_str!("2023/d11e1.txt");
        let input = Universe::parse(&input).unwrap();
        let result = part1(input);
        assert_eq!(result, 374);
    }

    #[test]
    fn example_2() {
        let input = example_str!("2023/d11e1.txt");
        let mut input = Universe::parse(&input).unwrap();
        input.expand(10);

        let result = input.distances().into_iter().sum::<usize>() / 2;
        assert_eq!(result, 1030);
    }

    #[test]
    fn example_3() {
        let input = example_str!("2023/d11e1.txt");
        let mut input = Universe::parse(&input).unwrap();
        input.expand(100);

        let result = input.distances().into_iter().sum::<usize>() / 2;
        assert_eq!(result, 8410);
    }
}
