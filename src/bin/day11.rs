use std::collections::BinaryHeap;

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

struct VecSizeOrd<T>(Vec<T>);

impl<T> Ord for VecSizeOrd<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.len().cmp(&other.0.len())
    }
}

impl<T> PartialOrd for VecSizeOrd<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.len().partial_cmp(&other.0.len())
    }
}

impl<T> PartialEq for VecSizeOrd<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
    }
}

impl<T> Eq for VecSizeOrd<T> {}

fn puzzle(input: &str, size: usize) -> usize {
    let mut lst: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut dp: BinaryHeap<VecSizeOrd<usize>> = BinaryHeap::new();
    for generation in 0..size {
        println!("got to {}/{} len = {}", generation, size - 1, lst.len());

        let mut new_lst: Vec<usize> = vec![];

        'outer: for i in 0..lst.len() {
            for VecSizeOrd(c) in dp.iter() {
                // 3 >= 5 - 1
                if c.len() >= lst.len() - i {
                    // make sure the sublist is long enough
                    continue;
                }

                let sublst = &lst[i..(i + c.len())];

                assert_eq!(
                    sublst.len(),
                    c.len(),
                    "sublist len: {}, c len: {}, lst len: {}, i: {}",
                    sublst.len(),
                    c.len(),
                    lst.len(),
                    i
                );

                if c.iter().zip(sublst.iter()).all(|(a, b)| a == b) {
                    new_lst.extend(c.iter());
                    println!("found: skipping {} els", c.len());
                    break 'outer;
                }
            }

            let x = lst[i];
            if x == 0 {
                new_lst.push(1);
                // return vec![1].into_iter();
            } else if count_digits(x) % 2 == 0 {
                let xstr = format!("{x}");
                let (a, b) = xstr.split_at(count_digits(x) / 2);
                new_lst.push(a.parse().unwrap());
                new_lst.push(b.parse().unwrap());
            } else {
                new_lst.push(x * 2024);
            }
        }

        lst = new_lst;
        dp.push(VecSizeOrd(lst.clone()));
    }
    lst.len()
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
