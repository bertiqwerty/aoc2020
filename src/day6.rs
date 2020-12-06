use super::common::separate_by_blanks;
use super::common::to_string_vec;
use super::common::TaskOfDay;
use std::collections::HashSet;

fn string_intersection(s1: String, s2: String) -> String
{
    let code_for_all = "###";
    if s1 == code_for_all {
        return s2.to_string();
    }
    let mut res = "".to_string();
    let char_set1 = s1.chars().collect::<HashSet<char>>();
    for c2 in s2.chars(){
        if char_set1.contains(&c2) {
            res.push(c2.clone())
        }
    }
    res
}

pub fn day6(input: &Vec<String>, part: TaskOfDay) -> i32 {
    match part {
        TaskOfDay::First => {
            let answers_per_group = separate_by_blanks(input, "");
            answers_per_group
                .iter()
                .map(|s| s.chars().collect::<HashSet<char>>().len())
                .sum::<usize>() as i32
        }
        TaskOfDay::Second => {
            let answers_per_group = separate_by_blanks(input, " ");
            answers_per_group
                .iter()
                .map(|s| s.split(" ").fold("###".to_string(), |s1, s2| string_intersection(s1.to_string(), s2.to_string())).len())
                .sum::<usize>() as i32
        }
    }
}

#[test]
fn test() {

    let s1 = "derg";
    let s2 = "asrdbets";
    assert_eq!(string_intersection(s1.to_string(), s2.to_string()), "rde".to_string());
    let tmp = vec![
        "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b", "",
    ];
    let input = to_string_vec(&tmp);
    assert_eq!(day6(&input, TaskOfDay::First), 11);
    assert_eq!(day6(&input, TaskOfDay::Second), 6);
}
