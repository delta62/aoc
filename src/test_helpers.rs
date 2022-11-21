#[cfg(test)]
pub fn example(day: usize) -> String {
    let path = format!("examples/2021/day{}.txt", day);
    std::fs::read_to_string(path).unwrap()
}

macro_rules! aoc_tests {
    (day: $day:literal, part1: $part1:expr$(, part2: $part2:expr)?) => {
        #[cfg(test)]
        mod test {
            use super::*;
            use crate::test_helpers::example;

            #[test]
            fn solves_part1_example() {
                let input = parse(&example($day));
                let result = part1(&input);
                assert_eq!($part1, result);
            }

            $(
                #[test]
                fn solves_part2_example() {
                    let input = parse(&example($day));
                    let result = part2(&input);
                    assert_eq!($part2, result);
                }
            )?
        }
    };
}
