// Refrence:
// This is day 1 of 2021

use advent_of_code_2024::sliding_window::SlidingWindowIteratorTrait;
use std::str::FromStr;
const PUZZLE_INPUT: &'static str = include_str!("input/warmup.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

fn final_compare(nums: impl Iterator<Item = u32>) -> u32 {
    nums.sliding_window()
        .fold(0, |acc, [el, prev]| if el > prev { acc + 1 } else { acc })
}

fn prepare_input(input: &str) -> impl Iterator<Item = u32> + use<'_> {
    input.trim().lines().map(u32::from_str).map(Result::unwrap)
}

fn part1(input: &str) -> u32 {
    let nums = prepare_input(input);
    final_compare(nums)
}

fn part2(input: &str) -> u32 {
    let nums = prepare_input(input);
    let sums = nums.sliding_window().map(|[a, b, c]| a + b + c);
    final_compare(sums)
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 7);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), 5);
    }
}
