use crate::common::TaskOfDay;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Bit {
    Zero,
    One,
    X,
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
            _ => panic!("Unexpected mask character"),
        }
    }
    res
}

fn parse_memory(line: &str, reg: &Regex) -> Option<(u64, u64)> {
    let caps = reg.captures(&line).unwrap();
    Some((
        caps[0][1..caps[0].len() - 1].parse::<u64>().unwrap(),
        line.split("=").nth(1)?.trim().parse::<u64>().unwrap(),
    ))
}

fn set_bit_at(value: u64, bit: u64, bit_position: usize) -> u64 {
    match bit {
        1 => value | (1u64 << bit_position),
        0 => value & (u64::MAX - 2u64.pow(bit_position as u32)),
        _ => panic!("Bit value must be either 1 or 0"),
    }
}

fn get_bit_at(value: u64, bit_position: usize) -> u64 {
    (value & (1u64 << bit_position)) >> bit_position
}

fn set_all_bits_at(value: u64, bits: u64, bit_positions: &Vec<usize>) -> u64 {
    let mut res = value;
    for (i, pos) in bit_positions.iter().enumerate() {
        res = set_bit_at(res, get_bit_at(bits, i), *pos);
    }
    res
}

fn filter_positions(bit: Bit, mask: &[Bit; 36]) -> Vec<usize> {
    mask.iter()
        .enumerate()
        .filter(|(_, b)| **b == bit)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>()
}

fn apply_mask(value: u64, mask: &[Bit; 36]) -> u64 {
    let one_inds = filter_positions(Bit::One, &mask);
    let zero_inds = filter_positions(Bit::Zero, &mask);
    set_all_bits_at(set_all_bits_at(value, 0, &zero_inds), u64::MAX, &one_inds)
}

fn floating_address_masking(address: u64, mask: &[Bit; 36]) -> Vec<u64> {
    let one_inds = filter_positions(Bit::One, &mask);
    let address_with_ones = set_all_bits_at(address, u64::MAX, &one_inds);
    let x_inds = filter_positions(Bit::X, &mask);
    (0..2u64.pow(x_inds.len() as u32))
        .map(|i| set_all_bits_at(address_with_ones, i, &x_inds))
        .collect::<Vec<u64>>()
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<u64> {
    let mut mask = [Bit::Zero; 36];
    let mut mem: HashMap<u64, u64> = HashMap::with_capacity(0);
    let regex = Regex::new(r"\[[0-9]+\]").unwrap();

    for line in input {
        if &line[..4] == "mask" {
            mask = convert_mask(&line);
        } else {
            let (address, value) = parse_memory(&line, &regex)?;
            match part {
                TaskOfDay::First => {
                    mem.insert(address, apply_mask(value, &mask));
                }
                TaskOfDay::Second => {
                    let addresses = floating_address_masking(address, &mask);
                    for a in addresses {
                        mem.insert(a, value);
                    }
                }
            }
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

    assert_eq!(get_bit_at(1, 0), 1);
    assert_eq!(get_bit_at(1, 1), 0);
    assert_eq!(get_bit_at(0, 1), 0);
    assert_eq!(set_bit_at(0, 1, 2), 4);
    assert_eq!(set_bit_at(7, 0, 2), 3);
    assert_eq!(set_bit_at(4, 0, 2), 0);
    assert_eq!(set_all_bits_at(0, 7, &vec![1usize, 2usize, 3usize]), 14);
    assert_eq!(set_all_bits_at(15, 65, &vec![1usize, 2usize, 3usize]), 3);

    assert_eq!(run(&input, TaskOfDay::First).unwrap(), 165u64);

    let input2 = string_to_lines(
        "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1",
    );
    assert_eq!(run(&input2, TaskOfDay::Second).unwrap(), 208u64);
}
