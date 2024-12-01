#![feature(iter_next_chunk)]
use advent_of_code_2024::map_count::Counter;

const PUZZLE_INPUT: &'static str = include_str!("input/day1.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

fn part1(input: &str) -> i32 {
    let (mut a, mut b): (Vec<_>, Vec<_>) = input
        .trim()
        .lines()
        .map(|line| {
            let [a, b] = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .take(2)
                .next_chunk()
                .unwrap();
            (a, b)
        })
        .unzip();

    a.sort();
    b.sort();

    a.iter().zip(b).map(|(a, b)| (a - b).abs()).sum()
}
fn part2(input: &str) -> i32 {
    let (a, b): (Vec<_>, Counter<_>) = input
        .trim()
        .lines()
        .map(|line| {
            let [a, b] = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .take(2)
                .next_chunk()
                .unwrap();
            (a, b)
        })
        .unzip();

    a.iter()
        .map(|val| b.get(val))
        .zip(a.iter())
        .map(|(a, &b)| a as i32 * b)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 11);
    }

    #[test]
    fn part2_test() {
        part2(TEST_INPUT);
    }
}
