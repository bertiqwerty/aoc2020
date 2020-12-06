use super::common::separate_by_blanks;
use super::common::TaskOfDay;
use std::collections::HashSet;

fn num_chars_in_all_splits(s: &str) -> i32 {
    let subtrings = s.split_whitespace().collect::<Vec<&str>>();
    let is_char_in_all_substrings = |c: &char| {
        for sub in &subtrings[1..] {
            if !sub.chars().any(|c_sub| c_sub == *c) {
                return false;
            }
        }
        true
    };
    let first = subtrings[0];
    first
        .chars()
        .filter(|c_first| is_char_in_all_substrings(c_first))
        .count() as i32
}

pub fn day6(input: &Vec<String>, part: TaskOfDay) -> i32 {
    match part {
        TaskOfDay::First => separate_by_blanks(input, "")
            .iter()
            .map(|s| s.chars().collect::<HashSet<char>>().len())
            .sum::<usize>() as i32,
        TaskOfDay::Second => separate_by_blanks(input, " ")
            .iter()
            .map(|s| num_chars_in_all_splits(s))
            .sum(),
    }
}

#[test]
fn test() {
    assert_eq!(num_chars_in_all_splits("a a a"), 1);
    assert_eq!(num_chars_in_all_splits("a ab a"), 1);
    assert_eq!(num_chars_in_all_splits("ac ac ac"), 2);
    assert_eq!(num_chars_in_all_splits("a ac ab"), 1);
    assert_eq!(num_chars_in_all_splits("ab bac ab"), 2);
    assert_eq!(num_chars_in_all_splits("acb bac abc"), 3);
    assert_eq!(num_chars_in_all_splits("acb bac cab"), 3);
    assert_eq!(num_chars_in_all_splits("acb bac d abc"), 0);
    assert_eq!(num_chars_in_all_splits("acb bac a cab"), 1);

    let tmp = vec![
        "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b", "",
    ];
    let input = tmp.iter().map(|elt| elt.to_string()).collect();
    assert_eq!(day6(&input, TaskOfDay::First), 11);
    assert_eq!(day6(&input, TaskOfDay::Second), 6);
}
