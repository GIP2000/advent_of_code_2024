#![feature(iter_next_chunk)]
use std::str::FromStr;
use std::usize;

use anyhow::anyhow;
use anyhow::Result;

const PUZZLE_INPUT: &'static str = include_str!("input/day13.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

trait Vec2D<T> {
    fn y(self) -> T;
    fn x(self) -> T;
}

impl<T: Copy + Clone> Vec2D<T> for (T, T) {
    fn y(self) -> T {
        self.1
    }

    fn x(self) -> T {
        self.0
    }
}

struct ClawMachine<T> {
    a: (T, T),
    b: (T, T),
    loc: (T, T),
}

impl<T: FromStr> FromStr for ClawMachine<T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [a, b, loc]: [Result<(T, T)>; 3] = s
            .lines()
            .map(|x| {
                let x_pos = x
                    .bytes()
                    .position(|x| x == b'X')
                    .ok_or(anyhow!("Couldn't find X"))?
                    + 2;
                let nums = &x[x_pos..];
                let (a, b) = nums.split_once(", Y").ok_or(anyhow!("can't find Y"))?;
                Ok((
                    a.parse().map_err(|_| anyhow!("can't get a"))?,
                    b[1..].parse().map_err(|_| anyhow!("can't get b"))?,
                ))
            })
            .next_chunk()
            .map_err(|_| anyhow!("Can't find 3"))?;

        Ok(Self {
            a: a?,
            b: b?,
            loc: loc?,
        })
    }
}

// k=\frac{\left(\frac{A_{y}}{A_{x}}X_{x}-X_{y}\right)}{\left(\frac{A_{y}}{A_{x}}B_{x}-B_{y}\right)}
// n=\frac{\left(X_{y}-kB_{y}\right)}{A_{y}}\

fn is_whole_number(value: f64) -> bool {
    if value.is_sign_negative() {
        return false;
    }
    let rounded = value.round();
    (value - rounded).abs() < 1e-10
}

fn part1(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|x| x.parse::<ClawMachine<f64>>().unwrap())
        .filter_map(|x| {
            let ayax = x.a.y() / x.a.x();
            let k = (ayax * x.loc.x() - x.loc.y()) / (ayax * x.b.x() - x.b.y());
            let n = (x.loc.y() - k * x.b.y()) / x.a.y();

            if is_whole_number(k) && is_whole_number(n) {
                return Some(k.round() as usize + ((n.round() as usize) * 3));
            } else {
                return None;
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|x| {
            let mut claw = x.parse::<ClawMachine<isize>>().unwrap();
            claw.loc.0 += 10000000000000;
            claw.loc.1 += 10000000000000;
            claw
        })
        .filter_map(|claw| {
            let den = claw.a.x() * claw.b.y() - claw.a.y() * claw.b.x();
            let n_num = claw.loc.x() * claw.b.y() - claw.loc.y() * claw.b.x();
            let k_num = claw.loc.y() * claw.a.x() - claw.loc.x() * claw.a.y();

            if k_num % den == 0
                && n_num % den == 0
                && den != 0
                && (den.signum() == n_num.signum() || n_num == 0)
                && (den.signum() == k_num.signum() || k_num == 0)
            {
                let k = (k_num / den) as usize;
                let n = (n_num / den) as usize;
                return Some(k + n * 3);
            }

            return None;
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 480);
    }
}
