use std::{
    collections::{HashMap, HashSet},
    usize,
};

const PUZZLE_INPUT: &'static str = include_str!("input/day10.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

// This input was not large enough to need dp.
fn get_paths(
    pos: (usize, usize),
    height: u8,
    mut trail_count: HashSet<(usize, usize)>,
    map: &Vec<Vec<u8>>,
    dp: &mut HashMap<(usize, usize), HashSet<(usize, usize)>>,
) -> HashSet<(usize, usize)> {
    if height == 9 {
        trail_count.insert(pos);
        return trail_count;
    }

    if let Some(result) = dp.get(&pos) {
        return result.clone();
    }

    let (x, y) = pos;

    let mut count = HashSet::new();
    if x + 1 < map.len() && map[y][x + 1] == height + 1 {
        count.extend(get_paths(
            (x + 1, y),
            height + 1,
            trail_count.clone(),
            map,
            dp,
        ));
    }

    if x > 0 && map[y][x - 1] == height + 1 {
        count.extend(get_paths(
            (x - 1, y),
            height + 1,
            trail_count.clone(),
            map,
            dp,
        ));
    }

    if y + 1 < map.len() && map[y + 1][x] == height + 1 {
        count.extend(get_paths(
            (x, y + 1),
            height + 1,
            trail_count.clone(),
            map,
            dp,
        ));
    }

    if y > 0 && map[y - 1][x] == height + 1 {
        count.extend(get_paths(
            (x, y - 1),
            height + 1,
            trail_count.clone(),
            map,
            dp,
        ));
    }

    count
}

fn part1(input: &str) -> usize {
    let map: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .bytes()
                .map(|b| if b != b'.' { b - b'0' } else { 10 })
                .collect()
        })
        .collect();

    let mut dp: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

    let mut count = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                count += get_paths((x, y), 0, HashSet::new(), &map, &mut dp).len();
            }
        }
    }
    count
}

// this input was not hard enough to actually need DP what the fuck. I just assumed it would need
// it.
fn get_raiting(
    pos: (usize, usize),
    height: u8,
    trail_count: usize,
    map: &Vec<Vec<u8>>,
    dp: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if height == 9 {
        return trail_count + 1;
    }

    if let Some(result) = dp.get(&pos) {
        println!("I skipped {result} stuff");
        return *result;
    }

    let (x, y) = pos;

    let mut count = 0;
    if x + 1 < map.len() && map[y][x + 1] == height + 1 {
        count += get_raiting((x + 1, y), height + 1, trail_count, map, dp);
    }

    if x > 0 && map[y][x - 1] == height + 1 {
        count += get_raiting((x - 1, y), height + 1, trail_count, map, dp);
    }

    if y + 1 < map.len() && map[y + 1][x] == height + 1 {
        count += get_raiting((x, y + 1), height + 1, trail_count, map, dp);
    }

    if y > 0 && map[y - 1][x] == height + 1 {
        count += get_raiting((x, y - 1), height + 1, trail_count, map, dp);
    }

    dp.insert(pos, count);

    count
}

fn part2(input: &str) -> usize {
    let map: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .bytes()
                .map(|b| if b != b'.' { b - b'0' } else { 10 })
                .collect()
        })
        .collect();

    let mut dp: HashMap<(usize, usize), usize> = HashMap::new();
    let mut count = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                count += get_raiting((x, y), 0, 0, &map, &mut dp);
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 36);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), 81);
    }
}
