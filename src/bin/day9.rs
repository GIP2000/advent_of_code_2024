#![feature(iter_array_chunks)]
#![feature(array_repeat)]

use advent_of_code_2024::optional_iter::OptionIter;
use std::{
    iter::{once, repeat},
    mem,
};

const PUZZLE_INPUT: &'static str = include_str!("input/day9.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

fn part1(input: &str) -> usize {
    let mut file_sys = input
        .trim()
        .bytes()
        .chain(once(b'0'))
        .map(|x| (x - b'0') as usize)
        .array_chunks()
        .enumerate()
        .flat_map(|(id, [block_count, free_space])| {
            repeat(Some(id))
                .take(block_count)
                .chain(repeat(None).take(free_space))
        })
        .collect::<Vec<_>>();

    let mut start_p = 0;
    let mut end_p = file_sys.len() - 1;

    loop {
        if start_p >= end_p {
            break;
        }

        match file_sys.get(start_p) {
            Some(Some(_)) => {
                start_p += 1;
                continue;
            }
            Some(None) => {}
            None => break,
        };

        let id = match file_sys.get(end_p) {
            Some(Some(id)) => id.clone(),
            Some(None) => {
                end_p -= 1;
                continue;
            }
            None => break,
        };

        file_sys[end_p] = None;
        file_sys[start_p] = Some(id);
    }

    file_sys
        .into_iter()
        .filter_map(|x| x)
        .enumerate()
        .map(|(i, x)| i * x)
        .sum::<usize>() as usize
}

enum FileSpace {
    Full { id: usize, len: usize },
    Empty(usize),
}

fn part2(input: &str) -> usize {
    let mut file_sys = input
        .trim()
        .bytes()
        .chain(once(b'0'))
        .map(|x| (x - b'0') as usize)
        .array_chunks()
        .enumerate()
        .flat_map(|(id, [block_count, free_space])| {
            once(FileSpace::Full {
                id,
                len: block_count,
            })
            .chain(if free_space > 0 {
                OptionIter::Some(once(FileSpace::Empty(free_space)))
            } else {
                OptionIter::None
            })
        })
        .collect::<Vec<_>>();

    let mut start_p = 0;
    let mut end_p = file_sys.len() - 1;

    loop {
        if start_p > end_p {
            end_p = if let Some(x) = end_p.checked_sub(1) {
                x
            } else {
                break;
            };

            start_p = 0;
        }

        let (id, len) = match file_sys.get(end_p) {
            Some(FileSpace::Empty(_)) => {
                end_p = if let Some(x) = end_p.checked_sub(1) {
                    x
                } else {
                    break;
                };
                continue;
            }
            Some(FileSpace::Full { id, len }) => {
                let len = len.clone();
                let id = id.clone();
                (id, len)
            }
            None => {
                // we have tried to fit evertything somehwere and have failed
                break;
            }
        };

        let size = match file_sys.get(start_p) {
            Some(FileSpace::Empty(size)) => {
                let size = size.clone();
                if size >= len {
                    size
                } else {
                    start_p += 1;
                    continue;
                }
            }
            Some(_) => {
                start_p += 1;
                continue;
            }
            None => {
                end_p -= 1;
                start_p = 0;
                continue;
            }
        };

        if len == size {
            // just do a swap
            file_sys[start_p] = mem::replace(&mut file_sys[end_p], FileSpace::Empty(len));
        } else {
            // shuffle
            assert!(len < size);
            file_sys.insert(start_p, FileSpace::Full { id, len });
            match &mut file_sys[start_p + 1] {
                FileSpace::Full { id: _, len: _ } => unreachable!("This should always be empty"),
                FileSpace::Empty(size) => *size -= len,
            }
            end_p += 1;
        }

        match (
            end_p.checked_sub(1).and_then(|d| file_sys.get(d)),
            &file_sys.get(end_p + 1),
        ) {
            (Some(FileSpace::Empty(s1)), Some(FileSpace::Empty(s2))) => {
                file_sys[end_p] = FileSpace::Empty(len + s1 + s2);
                file_sys.remove(end_p - 1);
                file_sys.remove(end_p);
                end_p -= 2;
            }
            (None, Some(FileSpace::Empty(s))) => {
                file_sys[end_p] = FileSpace::Empty(len + s);
                file_sys.remove(end_p + 1);
                end_p -= 1;
            }
            (Some(FileSpace::Empty(s)), None) => {
                file_sys[end_p] = FileSpace::Empty(len + s);
                file_sys.remove(end_p - 1);
                end_p -= 1;
            }
            _ => file_sys[end_p] = FileSpace::Empty(len),
        };
        start_p = 0;
    }

    file_sys
        .into_iter()
        .flat_map(|x| match x {
            FileSpace::Full { id, len } => repeat(id).take(len),
            FileSpace::Empty(len) => repeat(0).take(len),
        })
        .enumerate()
        .map(|(i, id)| i * id)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 1928);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), 2858);
    }
}
