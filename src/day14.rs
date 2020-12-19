use crate::common::TaskOfDay;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Bit {
    Zero,
    One,
    X
}

fn convert_mask(line: &str) -> [Bit; 36] {
    if line[7..].len() != 36 {
        panic!("Wrong line length!");
    }
    let mut res = [Bit::Zero; 36];
    for (i, c) in line[7..].chars().rev().enumerate() {
        res[i] = match c {
            '0' => Bit::Zero,
            '1' => Bit::One,
            'X' => Bit::X,
            _ => panic!("Unexpected mask character")
        }
    }
    res
}

fn parse_memory(line: &str, reg: &Regex) -> Option<(usize, u64)> {
    let caps = reg.captures(&line).unwrap();
    Some((
        caps[0][1..caps[0].len() - 1].parse::<usize>().unwrap(),
        line.split("=").nth(1)?.trim().parse::<u64>().unwrap(),
    ))
}

fn apply_mask(value: u64, mask: &[Bit; 36]) -> u64 {
    let mut res = value;
    for (i, b) in mask.iter().enumerate() {
        res = match b {
            Bit::One => res | (1u64 << i),
            Bit::Zero => res & (u64::MAX - 2u64.pow(i as u32)),
            Bit::X => res,
        }
    }
    res
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<u64> {
    let mut mask = [Bit::Zero; 36];
    let mut mem: HashMap<usize, u64> = HashMap::with_capacity(0);
    let regex = Regex::new(r"\[[0-9]+\]").unwrap();
    for line in input {
        if &line[..4] == "mask" {
            mask = convert_mask(&line);
        } else {
            let (address, value) = parse_memory(&line, &regex)?;
            mem.insert(address, apply_mask(value, &mask));
        }
    }

    return Some(mem.values().sum());
}

#[test]
fn test() {
    use super::common::string_to_lines;

    let input = string_to_lines(
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[18] = 11
mem[7] = 101
mem[18] = 0",
    );
    
    assert_eq!(run(&input, TaskOfDay::First).unwrap(), 165u64);
}
