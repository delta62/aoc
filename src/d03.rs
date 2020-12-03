use std::str::FromStr;

struct Slope {
    width: usize,
    height: usize,
    data: Vec<Vec<bool>>,
}

impl Slope {
    fn tree_at(&self, row: usize, col: usize) -> bool {
        self.data[row][col]
    }
}

impl FromStr for Slope {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<bool>> = s
            .split('\n')
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        let width = data[0].len();
        let height = data.len();

        Ok(Self { width, height, data })
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Slope {
    Slope::from_str(input).unwrap()
}

#[aoc(day3, part1)]
fn solve_part1(slope: &Slope) -> usize {
    solve_slope(slope, 3, 1)
}

#[aoc(day3, part2)]
fn solve_part2(slope: &Slope) -> usize {
    [ (1, 1), (3, 1), (5, 1), (7, 1), (1, 2) ]
        .iter()
        .map(|(dx, dy)| solve_slope(slope, *dx, *dy))
        .product()
}

fn solve_slope(slope: &Slope, dx: usize, dy: usize) -> usize {
    let mut tree_sum = 0;
    let mut col = 0;
    let mut row = 0;

    while row < slope.height {
        if slope.tree_at(row, col) {
            tree_sum += 1;
        }
        row += dy;
        col = (col + dx) % slope.width;
    }

    tree_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        let slope = parse(input);
        let result = solve_part1(&slope);
        assert_eq!(result, 7);
    }

    #[test]
    fn ex2() {
        let input = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        let slope = parse(input);
        let result = solve_part2(&slope);
        assert_eq!(result, 336);
    }
}
