use advent_of_code_2024::sliding_window::SlidingWindowIteratorTrait;

const PUZZLE_INPUT: &'static str = include_str!("input/day2.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

fn part1(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .sliding_window()
                .map(|[a, b]| a - b)
                .enumerate()
                .fold((true, true), |mut acc, (i, v)| {
                    if i == 0 {
                        if v < 0 {
                            acc.1 = false;
                        }
                    } else {
                        if (acc.1 && v < 0) || (!acc.1 && v > 0) {
                            return (false, acc.1);
                        }
                    }
                    if v.abs() > 3 || v.abs() < 1 {
                        (false, acc.1)
                    } else {
                        acc
                    }
                })
                .0
        })
        .filter(|x| *x)
        .count() as i32
}
fn part2(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|x| {
            let count = x.len() as isize;
            let mut outer_is_safe = true;

            for skip in -1..count {
                let mut is_safe = true;
                let mut is_pos = true;

                let iter = x
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i as isize != skip)
                    .sliding_window()
                    .map(|[(_, a), (_, b)]| a - b);

                let mut first = true;
                for v in iter {
                    if first {
                        if v < 0 {
                            is_pos = false;
                        }
                        first = false;
                    } else if (is_pos && v < 0) || (!is_pos && v > 0) {
                        is_safe = false;
                        break;
                    }

                    if v.abs() > 3 || v.abs() < 1 {
                        is_safe = false;
                        break;
                    }
                }

                outer_is_safe = is_safe;

                if is_safe {
                    break;
                };
            }
            outer_is_safe
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
