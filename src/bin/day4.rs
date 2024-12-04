const PUZZLE_INPUT: &'static str = include_str!("input/day4.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

fn search_horizontal(row_idx: usize, col_idx: usize, mat: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let forward_slice: String = mat[row_idx][col_idx..(col_idx + 4).min(mat[row_idx].len())]
        .iter()
        .collect();
    let backward_slice: String = mat[row_idx][(col_idx.saturating_sub(3))..=col_idx]
        .iter()
        .rev()
        .collect();

    if forward_slice == "xmas" {
        sum += 1
    };
    if backward_slice == "xmas" {
        sum += 1
    };
    sum
}

fn search_verticle(row_idx: usize, col_idx: usize, mat: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let forward_slice: String = mat[row_idx..(row_idx + 4).min(mat.len())]
        .iter()
        .map(|x| x[col_idx])
        .collect();

    let backward_slice: String = mat[row_idx.saturating_sub(3)..=row_idx]
        .iter()
        .map(|x| x[col_idx])
        .rev()
        .collect();

    if forward_slice == "xmas" {
        sum += 1
    };
    if backward_slice == "xmas" {
        sum += 1
    };
    sum
}

fn search_diagnal(row_idx: usize, col_idx: usize, mat: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;

    let top_left: String = mat[row_idx..]
        .iter()
        .take(4)
        .enumerate()
        .filter_map(|(i, row)| row.get(col_idx + i))
        .collect();

    if top_left == "xmas" {
        sum += 1
    }

    let bot_right: String = mat[..=row_idx]
        .iter()
        .rev()
        .take(4)
        .enumerate()
        .filter_map(|(i, row)| {
            let dif = col_idx.checked_sub(i)?;
            let val = row.get(dif);
            val
        })
        .collect();

    if bot_right == "xmas" {
        sum += 1
    }

    let bot_left: String = mat[..=row_idx]
        .iter()
        .rev()
        .take(4)
        .enumerate()
        .filter_map(|(i, row)| row.get(col_idx + i))
        .collect();

    if bot_left == "xmas" {
        sum += 1
    }

    // top right
    let top_right: String = mat[row_idx..]
        .iter()
        .take(4)
        .enumerate()
        .filter_map(|(i, row)| {
            let dif = col_idx.checked_sub(i)?;
            row.get(dif)
        })
        .collect();

    if top_right == "xmas" {
        sum += 1
    }

    sum
}

fn search_xmas(row_idx: usize, col_idx: usize, mat: &Vec<Vec<char>>) -> i32 {
    search_horizontal(row_idx, col_idx, mat)
        + search_verticle(row_idx, col_idx, mat)
        + search_diagnal(row_idx, col_idx, mat)
}

fn part1(input: &str) -> i32 {
    let mat = input
        .trim()
        .to_lowercase()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (row_idx, row) in mat.iter().enumerate() {
        for (col_idx, &el) in row.iter().enumerate() {
            if el == 'x' {
                sum += search_xmas(row_idx, col_idx, &mat);
            }
        }
    }

    sum
}

fn check_x(row_idx: usize, col_idx: usize, mat: &Vec<Vec<char>>) -> bool {
    let top_left: String = mat[row_idx..]
        .iter()
        .take(3)
        .enumerate()
        .filter_map(|(i, row)| row.get(col_idx + i))
        .collect();

    if top_left != "mas" && top_left != "sam" {
        return false;
    }

    let col_idx = col_idx + 2;

    let top_right: String = mat[row_idx..]
        .iter()
        .take(3)
        .enumerate()
        .filter_map(|(i, row)| {
            let dif = col_idx.checked_sub(i)?;
            row.get(dif)
        })
        .collect();

    if top_right != "mas" && top_right != "sam" {
        return false;
    }

    true
}

fn part2(input: &str) -> i32 {
    let mat = input
        .trim()
        .to_lowercase()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut sum = 0;
    for (row_idx, row) in mat.iter().enumerate() {
        for (col_idx, &el) in row.iter().enumerate() {
            if (el == 'm' || el == 's') && check_x(row_idx, col_idx, &mat) {
                sum += 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 18);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), 9);
    }
}
