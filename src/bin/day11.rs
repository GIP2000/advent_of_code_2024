use std::collections::HashMap;

const PUZZLE_INPUT: &'static str = include_str!("input/day11.txt");

fn main() {
    println!("PART1: {}", puzzle(PUZZLE_INPUT, 25));
    println!("PART2: {}", puzzle(PUZZLE_INPUT, 75));
}

fn count_digits(num: usize) -> usize {
    if num == 0 {
        1
    } else {
        (num as f64).log10().floor() as usize + 1
    }
}

fn rec(val: usize, dur: usize, size: usize, dp: &mut HashMap<(usize, usize), usize>) -> usize {
    if dur == 0 {
        return size;
    }

    if let Some(count) = dp.get(&(val, dur)) {
        return *count;
    }

    if val == 0 {
        let res = rec(1, dur - 1, size, dp);
        dp.insert((val, dur), res);
        return res;
    }
    let digit_len = count_digits(val);

    if digit_len % 2 == 0 {
        let val_str = format!("{}", val);
        let (first, second) = val_str.split_at(digit_len / 2);
        let first = first.parse().unwrap();
        let second = second.parse().unwrap();
        let res = rec(first, dur - 1, size, dp) + rec(second, dur - 1, size, dp);
        dp.insert((val, dur), res);
        return res;
    }
    let res = rec(val * 2024, dur - 1, size, dp);

    dp.insert((val, dur), res);
    return res;
}

fn puzzle(input: &str, size: usize) -> usize {
    let mut dp: HashMap<(usize, usize), usize> = HashMap::new();
    input
        .trim()
        .split_whitespace()
        .map(|x| rec(x.parse().unwrap(), size, 1, &mut dp))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_both_parts() {
        assert_eq!(puzzle(TEST_INPUT, 25), 55312);
    }
}
