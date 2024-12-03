use advent_of_code_2024::str_reader::StrReader;

const PUZZLE_INPUT: &'static str = include_str!("input/day3.txt");

fn main() {
    println!("PART1: {}", part1(PUZZLE_INPUT));
    println!("PART2: {}", part2(PUZZLE_INPUT));
}

fn parse_mul(reader_og: &mut StrReader) -> Option<i32> {
    let mut reader = reader_og.clone();

    if !reader.act_on_slice(|sl| sl.starts_with("ul(")) {
        return None;
    }
    reader.consume(3);

    let num_str_1: String =
        reader.act_on_slice(|sl| sl.chars().take_while(|x| ('0'..='9').contains(x)).collect());

    if num_str_1.len() > 3 || num_str_1.len() <= 0 {
        return None;
    }

    let first_num: i32 = num_str_1.parse().ok()?;

    reader.consume(num_str_1.len());

    if let Some(',') = reader.peek() {
        reader.consume(1);
    } else {
        return None;
    };

    let num_str_2: String =
        reader.act_on_slice(|sl| sl.chars().take_while(|x| ('0'..='9').contains(x)).collect());

    if num_str_2.len() > 3 || num_str_2.len() <= 0 {
        return None;
    }

    let second_num: i32 = num_str_2.parse().ok()?;

    reader.consume(num_str_2.len());

    if let Some(')') = reader.peek() {
        reader.consume(1);
    } else {
        return None;
    }

    *reader_og = reader;

    return Some(first_num * second_num);
}

fn parse_enabler(reader: &mut StrReader) -> Option<bool> {
    let disable = reader.act_on_slice(|sl| sl.starts_with("on't()"));
    let enable = reader.act_on_slice(|sl| sl.starts_with("o()"));
    assert!(!enable || !disable);

    if enable {
        reader.consume(3);
        return Some(true);
    } else if disable {
        reader.consume(6);
        return Some(false);
    }

    return None;
}

fn part1(input: &str) -> i32 {
    StrReader::new(input.trim())
        .gen_iter(|val, reader| match val {
            'm' => parse_mul(reader),
            _ => None,
        })
        .sum()
}

enum Op {
    Muled(i32),
    Do,
    Dont,
}
fn part2(input: &str) -> i32 {
    let mut enabled = true;

    StrReader::new(input.trim())
        .gen_iter(|val, reader| match val {
            'm' => parse_mul(reader).map(|muled| Op::Muled(muled)),
            'd' => parse_enabler(reader).map(|val| if val { Op::Do } else { Op::Dont }),
            _ => None,
        })
        .filter_map(|v| match (v, enabled) {
            (Op::Muled(v), true) => Some(v),
            (Op::Muled(_), false) => None,
            (Op::Do, _) => {
                enabled = true;
                None
            }
            (Op::Dont, _) => {
                enabled = false;
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
