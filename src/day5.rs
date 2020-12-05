use super::common::TaskOfDay;

fn parse_binary(binary_string: &str, one: char) -> i32 {
    binary_string
        .chars()
        .rev()
        .enumerate()
        .map(|t| ((t.1 == one) as i32) * 2i32.pow(t.0 as u32))
        .sum()
}

fn get_id(binary_string: &str) -> i32 {
    let row = &binary_string[..7];
    let col = &binary_string[7..];
    parse_binary(row, 'B') * 8 + parse_binary(col, 'R')
}

pub fn day5(input: &Vec<String>, part: TaskOfDay) -> i32 {
    match part {
        TaskOfDay::First => input.iter().map(|s| get_id(s)).max().unwrap(),
        TaskOfDay::Second => {
            let mut ids = input.iter().map(|s| get_id(s)).collect::<Vec<i32>>();
            ids.sort();
            izip!(ids.iter(), ids[1..].iter())
                .find(|t| t.1 - t.0 == 2)
                .unwrap()
                .0
                + 1
        }
    }
}

#[test]
fn test() {
    assert_eq!(parse_binary("BFFFBBF", 'B'), 70);
    assert_eq!(parse_binary("FFFBBBF", 'B'), 14);
    assert_eq!(parse_binary("BBFFBBF", 'B'), 102);
    assert_eq!(parse_binary("RRR", 'R'), 7);
    assert_eq!(parse_binary("RLL", 'R'), 4);
    assert_eq!(get_id("BFFFBBFRRR"), 567);
    assert_eq!(get_id("FFFBBBFRRR"), 119);
    assert_eq!(get_id("BBFFBBFRLL"), 820);
}
