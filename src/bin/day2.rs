const PUZZLE_INPUT: &'static str = include_str!("input/day2.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

fn part1(input: &str) -> i32 {
    0
}
fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 0);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
