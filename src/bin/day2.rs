use advent_of_code_2024::sliding_window::SlidingWindowIteratorTrait;
use std::str::FromStr;

const PUZZLE_INPUT: &'static str = include_str!("input/day2.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

fn part1(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .filter_map(|x| {
            let mut iter = x
                .split_whitespace()
                .map(|v| i32::from_str(v).unwrap())
                .sliding_window()
                .map(|[a, b]| a - b)
                .peekable();

            let increasing = *iter.peek().unwrap() > 0;
            let range = if increasing { 1..=3 } else { -3..=-1 };

            iter.all(|v| range.contains(&v)).then(|| Some(()))
        })
        .count() as i32
}
fn part2(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect::<Box<[i32]>>()
        })
        .filter(|x| {
            (-1..x.len() as isize).into_iter().any(|skip| {
                let mut iter = x
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i as isize != skip)
                    .sliding_window()
                    .map(|[(_, a), (_, b)]| a - b)
                    .peekable();

                let increasing = *iter.peek().unwrap() > 0;
                let range = if increasing { 1..=3 } else { -3..=-1 };

                iter.all(|v| range.contains(&v))
            })
        })
        .count() as i32
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}
