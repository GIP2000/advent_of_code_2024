use advent_of_code_2024::{combo::ComboIterTrait, optional_iter::OptionIter};
use std::collections::HashSet;

const PUZZLE_INPUT: &'static str = include_str!("input/day8.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

fn part1(input: &str) -> i32 {
    let square_size = input.lines().count();

    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .bytes()
                .enumerate()
                .filter_map(move |(x, byte)| {
                    if byte == b'.' {
                        return None;
                    }
                    return Some(((x, y), byte));
                })
        })
        .combo()
        .flat_map(|(((ax, ay), byte_a), ((bx, by), byte_b))| {
            if byte_b != byte_a {
                return OptionIter::None;
            }

            let dx = ax.abs_diff(bx);
            let dy = ay.abs_diff(by);

            let plus_clousre_x = |n: usize| {
                if n + dx < square_size {
                    Some(n + dx)
                } else {
                    None
                }
            };
            let minus_closure_x = |n: usize| n.checked_sub(dx);

            let xs = if ax > bx {
                (plus_clousre_x(ax), minus_closure_x(bx))
            } else {
                (minus_closure_x(ax), plus_clousre_x(bx))
            };

            let first = if xs.0.is_some() && ay.checked_sub(dy).is_some() {
                Some((xs.0.unwrap(), ay - dy))
            } else {
                None
            };

            let second = if xs.1.is_some() && ((by + dy) < square_size) {
                Some((xs.1.unwrap(), by + dy))
            } else {
                None
            };

            OptionIter::Some([first, second].into_iter().filter_map(|x| x))
        })
        .collect::<HashSet<_>>()
        .len() as i32
}
fn part2(input: &str) -> i32 {
    let square_size = input.lines().count() as isize;

    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .bytes()
                .enumerate()
                .filter_map(move |(x, byte)| {
                    if byte == b'.' {
                        return None;
                    }
                    return Some(((x, y), byte));
                })
        })
        .combo()
        .fold(
            HashSet::new(),
            |mut set, (((ax, ay), byte_a), ((bx, by), byte_b))| {
                if byte_b != byte_a {
                    return set;
                }

                let dx = bx as isize - ax as isize;
                let dy = by as isize - ay as isize;

                let mut first_x = ax as isize;
                let mut first_y = ay as isize;

                while first_x >= 0 && first_x < square_size && first_y >= 0 && first_y < square_size
                {
                    set.insert((first_x, first_y));
                    first_x += dx;
                    first_y += dy;
                }
                let dx = -dx;
                let dy = -dy;

                let mut first_x = bx as isize;
                let mut first_y = by as isize;

                while first_x >= 0 && first_x < square_size && first_y >= 0 && first_y < square_size
                {
                    set.insert((first_x, first_y));
                    first_x += dx;
                    first_y += dy;
                }

                return set;
            },
        )
        .len() as i32
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "...........
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 14);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), 34);
    }
}
