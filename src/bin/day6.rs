use std::collections::HashSet;

const PUZZLE_INPUT: &'static str = include_str!("input/day6.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

#[derive(Clone, Eq, PartialEq)]
enum GridElement {
    Guard,
    Item,
    Empty,
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_90(&mut self) {
        use Direction::*;
        *self = match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        };
    }

    pub fn move_in_dir(&self, pos: &mut (usize, usize)) -> Option<()> {
        use Direction::*;
        match self {
            Up => pos.1 = pos.1.checked_sub(1)?,
            Down => pos.1 += 1,
            Left => pos.0 = pos.0.checked_sub(1)?,
            Right => pos.0 += 1,
        };
        Some(())
    }
}

fn part1(input: &str) -> i32 {
    let mut guard_pos = (0, 0);

    let map: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, el)| match el {
                    b'^' => {
                        guard_pos = (x, y);
                        GridElement::Guard
                    }
                    b'#' => GridElement::Item,
                    b'.' => GridElement::Empty,
                    _ => unreachable!("value should not be in input {}", el),
                })
                .collect()
        })
        .collect();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut direction = Direction::Up;

    while let Some(Some(_)) = map.get(guard_pos.1).map(|r| r.get(guard_pos.0)) {
        let mut new_guard_pos = guard_pos.clone();
        visited.insert(guard_pos);

        if let None = direction.move_in_dir(&mut new_guard_pos) {
            break;
        };

        if let Some(Some(GridElement::Item)) =
            map.get(new_guard_pos.1).map(|r| r.get(new_guard_pos.0))
        {
            direction.rotate_90();
        } else {
            guard_pos = new_guard_pos;
        }
    }

    visited.len() as i32
}

fn simulate(map: Vec<Vec<GridElement>>, mut guard_pos: (usize, usize)) -> bool {
    let mut visited: HashSet<((usize, usize), Direction)> = HashSet::new();
    let mut direction = Direction::Up;
    let mut off_flag = false;

    while let Some(Some(_)) = map.get(guard_pos.1).map(|r| r.get(guard_pos.0)) {
        let mut new_guard_pos = guard_pos.clone();

        if !off_flag && visited.contains(&(guard_pos, direction.clone())) {
            return true;
        } else {
            off_flag = false;
        }

        visited.insert((guard_pos, direction.clone()));

        if let None = direction.move_in_dir(&mut new_guard_pos) {
            return false;
        };

        if let Some(Some(GridElement::Item)) =
            map.get(new_guard_pos.1).map(|r| r.get(new_guard_pos.0))
        {
            direction.rotate_90();
            off_flag = true;
        } else {
            guard_pos = new_guard_pos;
        }
    }
    return false;
}

fn part2(input: &str) -> i32 {
    let mut guard_pos = (0, 0);

    let map: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, el)| match el {
                    b'^' => {
                        guard_pos = (x, y);
                        GridElement::Guard
                    }
                    b'#' => GridElement::Item,
                    b'.' => GridElement::Empty,
                    _ => unreachable!(""),
                })
                .collect()
        })
        .collect();

    // brute force OP
    (0..map.len())
        .flat_map(|y| (0..map[y].len()).map(move |x| (x, y)))
        .filter(|&(x, y)| {
            let mut new_map = map.clone();
            match new_map[y][x] {
                GridElement::Empty => {
                    new_map[y][x] = GridElement::Item;
                }
                _ => {
                    return false;
                }
            }
            simulate(new_map, guard_pos)
        })
        .count() as i32
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 41);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), 6);
    }
}
