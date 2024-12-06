#![feature(iter_next_chunk)]
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
};

const PUZZLE_INPUT: &'static str = include_str!("input/day5.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

struct MapSet<K, V>(HashMap<K, HashSet<V>>)
where
    V: Hash + Eq,
    K: Hash + Eq;

impl<K, V> FromIterator<(K, V)> for MapSet<K, V>
where
    V: Hash + Eq,
    K: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = HashMap::<K, HashSet<V>>::new();
        for (k, v) in iter {
            match map.get_mut(&k) {
                Some(vec) => {
                    vec.insert(v);
                }
                None => {
                    let mut set = HashSet::new();
                    set.insert(v);
                    map.insert(k, set);
                }
            };
        }
        Self(map)
    }
}

fn part1(input: &str) -> i32 {
    let (rules, seq) = input.trim().split_once("\n\n").unwrap();

    let MapSet(rules_map) = rules
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").unwrap();
            (b.parse::<i32>().unwrap(), a.parse::<i32>().unwrap())
        })
        .collect();

    seq.lines()
        .filter_map(|line| {
            let els: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
            let mut not_allowed: HashSet<i32> = HashSet::new();

            if !els.iter().all(|x| {
                if not_allowed.contains(x) {
                    return false;
                }

                if let Some(inner_not_allowed) = rules_map.get(x) {
                    not_allowed.extend(inner_not_allowed);
                }

                true
            }) {
                return None;
            }

            Some(els[els.len() / 2])
        })
        .sum::<i32>()
}
fn part2(input: &str) -> i32 {
    let (rules, seq) = input.trim().split_once("\n\n").unwrap();

    let MapSet(rules_map) = rules
        .lines()
        .map(|line| {
            let [a, b] = line
                .split("|")
                .map(|v| v.parse::<i32>().unwrap())
                .next_chunk()
                .unwrap();

            (b, a)
        })
        .collect();

    seq.lines()
        .filter_map(|line| {
            let mut els: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
            let mut not_allowed: HashSet<i32> = HashSet::new();

            if els.iter().all(|x| {
                if not_allowed.contains(x) {
                    return false;
                }

                if let Some(inner_not_allowed) = rules_map.get(x) {
                    not_allowed.extend(inner_not_allowed);
                }

                return true;
            }) {
                return None;
            }

            els.sort_by(|a, b| {
                if let Some(a_not_allowed) = rules_map.get(a) {
                    if a_not_allowed.contains(b) {
                        return Ordering::Greater;
                    }
                }

                if let Some(b_not_allowed) = rules_map.get(b) {
                    if b_not_allowed.contains(a) {
                        return Ordering::Less;
                    }
                }

                return Ordering::Equal;
            });

            Some(els[els.len() / 2])
        })
        .sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 143);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), 123);
    }
}
