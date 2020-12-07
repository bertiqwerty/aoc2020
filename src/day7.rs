use super::common::split_in2_tuple;
use super::common::TaskOfDay;
use std::collections::HashMap;

fn parse_line(line: &str) -> HashMap<&str, HashMap<&str, i32>> {
    let empty_hashmap: HashMap<&str, i32> = HashMap::with_capacity(0);

    let (container, content) = split_in2_tuple(line, " bags contain ");
    fn strip(s: &str) -> &str {
        let ws_pos = s.chars().count() - s.chars().rev().position(|c| c == ' ').unwrap();
        &s[..ws_pos].trim()
    }
    fn number_vs_rest(s: &str) -> (&str, i32) {
        let num = s.split_whitespace().next().unwrap().parse::<i32>().unwrap();
        (strip(&s[2..]), num)
    }
    if content.trim() == "no other bags." {
        return [(container.trim(), empty_hashmap)]
            .iter()
            .cloned()
            .collect();
    } else {
        if content.chars().any(|c| c == ',') {
            let sub_bags = content.split(", ").map(|s| number_vs_rest(s)).collect();
            return [(container.trim(), sub_bags)].iter().cloned().collect();
        } else {
            let (container1, num1) = number_vs_rest(content);
            return [(
                container.trim(),
                [(container1, num1)].iter().cloned().collect(),
            )]
            .iter()
            .cloned()
            .collect();
        }
    }
}

fn merge_line_maps(input: &Vec<String>) -> HashMap<&str, HashMap<&str, i32>> {
    let mut bag_map: HashMap<&str, HashMap<&str, i32>> = HashMap::with_capacity(0);
    for line in input.iter() {
        bag_map.extend(parse_line(line));
    }
    bag_map
}

fn is_in_bag<'a>(
    needle: &str,
    haystack: &str,
    bag_map: &'a HashMap<&str, HashMap<&str, i32>>,
) -> bool {
    if bag_map[haystack].len() == 0 {
        return false;
    }
    for k in bag_map[haystack].keys() {
        if k == &needle {
            return true;
        }
    }
    bag_map[haystack]
        .keys()
        .any(|s| is_in_bag(needle, s, &bag_map))
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> i32 {
    let bag_map = merge_line_maps(input);
    match part {
        TaskOfDay::First => bag_map
            .keys()
            .filter(|k| **k != "shiny gold")
            .filter(|k| is_in_bag("shiny gold", k, &bag_map))
            .count() as i32,
        TaskOfDay::Second => 0i32,
    }
}

#[test]
fn test() {
    let empty_hashmap: HashMap<&str, i32> = HashMap::with_capacity(0);
    let input_str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags, 3 dotted black bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    let input: Vec<String> = input_str.split("\n").map(|s| s.to_string()).collect();
    let bag_map = merge_line_maps(&input);

    assert_eq!(
        parse_line(&input[0]),
        [(
            "light red",
            [("bright white", 1), ("muted yellow", 2)]
                .iter()
                .cloned()
                .collect()
        )]
        .iter()
        .cloned()
        .collect()
    );
    assert_eq!(
        parse_line(&input[1]),
        [(
            "dark orange",
            [
                ("bright white", 3),
                ("muted yellow", 4),
                ("dotted black", 3)
            ]
            .iter()
            .cloned()
            .collect()
        )]
        .iter()
        .cloned()
        .collect()
    );
    assert_eq!(
        parse_line(&input[2]),
        [(
            "bright white",
            [("shiny gold", 1)].iter().cloned().collect()
        )]
        .iter()
        .cloned()
        .collect()
    );
    assert_eq!(
        parse_line(&input[3]),
        [(
            "muted yellow",
            [("shiny gold", 2), ("faded blue", 9)]
                .iter()
                .cloned()
                .collect()
        )]
        .iter()
        .cloned()
        .collect()
    );
    assert_eq!(
        parse_line(&input[4]),
        [(
            "shiny gold",
            [("dark olive", 1), ("vibrant plum", 2)]
                .iter()
                .cloned()
                .collect()
        )]
        .iter()
        .cloned()
        .collect()
    );
    assert_eq!(
        parse_line(&input[5]),
        [(
            "dark olive",
            [("faded blue", 3), ("dotted black", 4)]
                .iter()
                .cloned()
                .collect()
        )]
        .iter()
        .cloned()
        .collect()
    );
    assert_eq!(
        parse_line(&input[6]),
        [(
            "vibrant plum",
            [("faded blue", 5), ("dotted black", 6)]
                .iter()
                .cloned()
                .collect()
        )]
        .iter()
        .cloned()
        .collect()
    );
    assert_eq!(
        parse_line(&input[7]),
        [("faded blue", empty_hashmap.clone())]
            .iter()
            .cloned()
            .collect()
    );
    assert_eq!(
        parse_line(&input[8]),
        [("dotted black", empty_hashmap.clone())]
            .iter()
            .cloned()
            .collect()
    );

    assert_eq!(is_in_bag("shiny gold", "bright white", &bag_map), true);
    assert_eq!(is_in_bag("shiny gold", "muted yellow", &bag_map), true);
    assert_eq!(is_in_bag("shiny gold", "dark orange", &bag_map), true);
    assert_eq!(is_in_bag("shiny gold", "light red", &bag_map), true);
    assert_eq!(is_in_bag("shiny gold", "dark olive", &bag_map), false);
    assert_eq!(is_in_bag("shiny gold", "vibrant plum", &bag_map), false);
    assert_eq!(is_in_bag("shiny gold", "faded blue", &bag_map), false);
    assert_eq!(is_in_bag("shiny gold", "dotted black", &bag_map), false);

    assert_eq!(run(&input, TaskOfDay::First), 4);

    let input2_str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    assert_eq!(run(&input, TaskOfDay::Second), 126);
}
