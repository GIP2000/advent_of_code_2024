use advent_of_code_2024::explode::ExplodeIter;

const PUZZLE_INPUT: &'static str = include_str!("input/day7.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

fn part1(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let (val, lst) = line.split_once(":").unwrap();

            let goal = val.trim().parse().unwrap();
            lst.trim()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                // This is prob a more resonable way to do it I just figure that there will be more
                // explosions in advent of code
                //
                // .fold(vec![], |mut acc, val| {
                //     if acc.len() <= 0 {
                //         acc.push(val);
                //         return acc;
                //     }
                //     acc.iter()
                //         .flat_map(|prev| [prev + val, prev * val].into_iter())
                //         .collect()
                // })
                // .into_iter()
                .explode(|prev_val, val| [prev_val + val, prev_val * val])
                .any(|f| f == goal)
                .then(|| goal)
        })
        .sum()
}

fn concat_digits(a: u64, b: u64) -> u64 {
    let digit_count = if b == 0 {
        1
    } else {
        (b as f64).log10().floor() as u32 + 1
    };

    a * 10u64.pow(digit_count) + b
}

fn part2(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let (val, lst) = line.split_once(":").unwrap();

            let goal = val.trim().parse().unwrap();

            lst.trim()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .explode(|prev_val, val| {
                    [
                        prev_val + val,
                        prev_val * val,
                        concat_digits(*prev_val, *val),
                    ]
                })
                .any(|f| f == goal)
                .then(|| goal)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 3749);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), 11387);
    }
}
