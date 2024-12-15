use std::collections::HashSet;

const PUZZLE_INPUT: &'static str = include_str!("input/day12.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

fn make_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|x| x.trim().bytes().collect())
        .collect()
}

fn print_grid(_v: &Vec<Vec<bool>>) {
    // if v.first().is_some() {
    //     print!("   ");
    //     for x in 0..v[0].len() {
    //         print!("{}  ", x);
    //     }
    //     println!("");
    // }
    // for (y, row) in v.iter().enumerate() {
    //     print!("{}. ", y);
    //     for &el in row {
    //         print!("{} ", if el { "✅" } else { "💨" });
    //     }
    //     print!("\n");
    // }
}

fn calculate_score(pos: Vec<(usize, usize)>) -> u128 {
    let area = pos.len();

    let (x_max, y_max) = pos.iter().fold((0, 0), |(mut x_max, mut y_max), &(x, y)| {
        if x > x_max {
            x_max = x;
        }
        if y > y_max {
            y_max = y;
        }
        (x_max, y_max)
    });

    let mut grid_area: Vec<Vec<bool>> = vec![vec![false; x_max + 1]; y_max + 1];

    for &(x, y) in pos.iter() {
        grid_area[y][x] = true;
    }

    let mut p = 0u128;

    for (y, row) in grid_area.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|(_, x)| **x) {
            // is there nothing on top of me
            if y == 0 || !grid_area[y - 1][x] {
                p += 1;
            }

            // is there nothing to the left of me
            if x == 0 || !grid_area[y][x - 1] {
                p += 1;
            }

            // is there nothing to the right of me
            if x == row.len() - 1 || !grid_area[y][x + 1] {
                p += 1;
            }

            // is there nothing below me
            if y == grid_area.len() - 1 || !grid_area[y + 1][x] {
                p += 1;
            }
        }
    }
    (area as u128) * p
}

fn fill_map(
    plant: u8,
    x: usize,
    y: usize,
    grid: &Vec<Vec<u8>>,
    map: &mut Vec<(usize, usize)>,
    found_set: &mut HashSet<(usize, usize)>,
) {
    if found_set.contains(&(x, y)) {
        return;
    }

    if let Some(&p) = grid.get(y).and_then(|r| r.get(x)) {
        if p == plant {
            map.push((x, y));
            found_set.insert((x, y));
            fill_map(plant, x, y + 1, grid, map, found_set);
            if y > 0 {
                fill_map(plant, x, y - 1, grid, map, found_set);
            }
            fill_map(plant, x + 1, y, grid, map, found_set);
            if x > 0 {
                fill_map(plant, x - 1, y, grid, map, found_set);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn next(self) -> Self {
        use Dir::*;
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }
}

trait AddDir
where
    Self: Sized + Clone + Copy,
{
    fn add_dir(self, rhs: Dir, max: Self) -> Option<Self>;

    fn check_wall(self, rhs: Dir, max: Self) -> Option<(Self, Dir)> {
        use Dir::*;
        let in_path_self = self.add_dir(rhs, max)?;
        Some(match rhs {
            Right => (in_path_self.add_dir(Up, max)?, Up),
            Down => (in_path_self.add_dir(Right, max)?, Right),
            Left => (in_path_self.add_dir(Down, max)?, Down),
            Up => (in_path_self.add_dir(Left, max)?, Left),
        })
    }
}

impl AddDir for (usize, usize) {
    fn add_dir(self, rhs: Dir, max: Self) -> Option<Self> {
        use Dir::*;
        match rhs {
            Down => {
                if self.1 + 1 > max.1 {
                    None
                } else {
                    Some((self.0, self.1 + 1))
                }
            }
            Up => Some((self.0, self.1.checked_sub(1)?)),
            Left => Some((self.0.checked_sub(1)?, self.1)),
            Right => {
                if self.0 + 1 > max.0 {
                    None
                } else {
                    Some((self.0 + 1, self.1))
                }
            }
        }
    }
}

fn make_bool_grid(pos: Vec<(usize, usize)>) -> (Vec<Vec<bool>>, (usize, usize)) {
    let (max_pos, (x_min, y_min)) = pos.iter().fold(
        ((0, 0), (usize::MAX, usize::MAX)),
        |((mut x_max, mut y_max), (mut x_min, mut y_min)), &(x, y)| {
            if x > x_max {
                x_max = x;
            }
            if y > y_max {
                y_max = y;
            }

            if x < x_min {
                x_min = x;
            }
            if y < y_min {
                y_min = y;
            }
            ((x_max, y_max), (x_min, y_min))
        },
    );

    let (x_max, y_max) = max_pos.clone();
    let x_max = x_max - x_min;
    let y_max = y_max - y_min;
    let max_pos = (x_max, y_max);

    let mut grid_area: Vec<Vec<bool>> = vec![vec![false; x_max + 1]; y_max + 1];

    for &(x, y) in pos.iter() {
        grid_area[y - y_min][x - x_min] = true;
    }
    (grid_area, max_pos)
}

fn calculate_discount_rate_outer(grid_area: &Vec<Vec<bool>>, max_pos: (usize, usize)) -> usize {
    print_grid(&grid_area);

    if grid_area.len() == 1 && grid_area[0].len() == 1 {
        return 4;
    }

    let first_pos = grid_area
        .iter()
        .enumerate()
        .filter_map(|(y, x)| Some((x.iter().position(|v| *v)?, y)))
        .next()
        .unwrap();

    let mut pos = first_pos;

    let mut sides = 0;
    let mut dir = Dir::Right;
    let mut first = true;

    let mut set = HashSet::new();

    'outer: loop {
        set.insert((pos, dir));

        for _ in 0..4 {
            if !first && first_pos == pos && dir == Dir::Right {
                break 'outer;
            }
            first = false;

            if let Some((new_pos, new_dir)) = pos.check_wall(dir, max_pos) {
                if grid_area[new_pos.1][new_pos.0] {
                    pos = new_pos;
                    dir = new_dir;
                    sides += 1;
                    continue 'outer;
                }
            }
            if let Some(new_pos) = pos.add_dir(dir, max_pos) {
                if !set.contains(&(new_pos, dir)) && grid_area[new_pos.1][new_pos.0] {
                    pos = new_pos;
                    continue 'outer;
                }
            }
            sides += 1;
            dir = dir.next();
        }

        unreachable!("I didn't get back to the front");
    }

    sides
}

fn make_inverse_areas(
    grid_area: &Vec<Vec<bool>>,
    (x_max, y_max): (usize, usize),
) -> Vec<(Vec<Vec<bool>>, (usize, usize))> {
    let inverse_area: Vec<Vec<_>> = grid_area
        .iter()
        .map(|x| x.iter().cloned().map(|x| !x).collect())
        .collect();

    fn fill_map(
        x: usize,
        y: usize,
        grid: &Vec<Vec<bool>>,
        map: &mut Vec<(usize, usize)>,
        found_set: &mut HashSet<(usize, usize)>,
    ) {
        if found_set.contains(&(x, y)) {
            return;
        }

        if let Some(&p) = grid.get(y).and_then(|r| r.get(x)) {
            if p {
                map.push((x, y));
                found_set.insert((x, y));
                fill_map(x, y + 1, grid, map, found_set);
                if y > 0 {
                    fill_map(x, y - 1, grid, map, found_set);
                }
                fill_map(x + 1, y, grid, map, found_set);
                if x > 0 {
                    fill_map(x - 1, y, grid, map, found_set);
                }
            }
        }
    }
    let mut found_set = HashSet::new();

    let mut vals = vec![];
    for y in 0..=y_max {
        for x in 0..=x_max {
            // if its a pot hole
            if inverse_area[y][x] {
                let mut map = vec![];
                fill_map(x, y, &inverse_area, &mut map, &mut found_set);
                if let None = map
                    .iter()
                    .find(|(x, y)| *x == 0 || *y == 0 || *x == x_max || *y == y_max)
                {
                    vals.push(map);
                }
            }
        }
    }
    vals.into_iter()
        .filter_map(|x| {
            if x.len() > 0 {
                Some(make_bool_grid(x))
            } else {
                None
            }
        })
        .collect()
}

fn part1(input: &str) -> u128 {
    let grid = make_grid(input);

    let mut regions: Vec<(u8, Vec<(usize, usize)>)> = vec![];

    // this is all the paces I added already
    let mut added_places: HashSet<(usize, usize)> = HashSet::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, plant) in row.iter().enumerate() {
            if added_places.contains(&(x, y)) {
                continue;
            }
            let mut vec = vec![];
            fill_map(*plant, x, y, &grid, &mut vec, &mut added_places);
            if vec.len() <= 0 {
                continue;
            }
            regions.push((*plant, vec));
        }
    }

    regions
        .into_iter()
        .map(|(_, fence)| calculate_score(fence))
        .sum()
}
fn part2(input: &str) -> usize {
    let grid = make_grid(input);

    let mut regions: Vec<(u8, Vec<(usize, usize)>)> = vec![];

    // this is all the paces I added already
    let mut added_places: HashSet<(usize, usize)> = HashSet::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, plant) in row.iter().enumerate() {
            if added_places.contains(&(x, y)) {
                continue;
            }
            let mut vec = vec![];
            fill_map(*plant, x, y, &grid, &mut vec, &mut added_places);
            if vec.len() <= 0 {
                continue;
            }
            regions.push((*plant, vec));
        }
    }

    regions
        .into_iter()
        .map(|(_plant, fence)| {
            let area = fence.len();
            let (grid_area, max_pos) = make_bool_grid(fence);
            let holes = make_inverse_areas(&grid_area, max_pos);

            area * (calculate_discount_rate_outer(&grid_area, max_pos)
                + holes
                    .iter()
                    .map(|(grid_area, max_pos)| calculate_discount_rate_outer(grid_area, *max_pos))
                    .sum::<usize>())
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const LONG_TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const FIRST: &str = "AAAA
BBCD
BBCC
EEEC";

    const XO: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    #[test]
    fn part1_short() {
        assert_eq!(part1(FIRST), 140);
    }

    #[test]
    fn part1_inner() {
        assert_eq!(part1(XO), 772);
    }

    #[test]
    fn part1_long() {
        assert_eq!(part1(LONG_TEST_INPUT), 1930);
    }

    #[test]
    fn part2_short() {
        assert_eq!(part2(FIRST), 80);
    }

    #[test]
    fn part2_inner() {
        assert_eq!(part2(XO), 436);
    }

    #[test]
    fn part2_long() {
        assert_eq!(part2(LONG_TEST_INPUT), 1206);
    }
}
