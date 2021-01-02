use super::common::find_split_positions;
use super::common::TaskOfDay;
use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    And(Vec<usize>),
    Or(Vec<Vec<usize>>),
    Char(char),
}

/// Rule or Index
enum Rori<'a> {
    Idx(usize),
    OrRule(&'a Rule)
}

fn check_message<'a>(msg: &'a str, r_or_i: Rori, rules: &Vec<Rule>) -> (bool, &'a str) {
    if msg.len() == 0 {
        return (false, msg);
    }
    let mut submsg = msg;

    let rule = match r_or_i {
        Rori::Idx(i) => &rules[i],
        Rori::OrRule(r) => &r
    };
    match rule {
        Rule::And(v) => {
            for vi in v {
                let tmp = check_message(submsg,Rori::Idx(*vi), rules);
                let res = tmp.0;
                submsg = tmp.1;
                if !res {
                    return (false, submsg);
                }                
            }
            (true, submsg)
        }
        Rule::Char(c) => if submsg.chars().next().unwrap() == *c {(true, &submsg[1..])} else {(false, submsg)}
        Rule::Or(vv) => {
            for v in vv {
                let (res, submsg_candidate) = check_message(submsg, Rori::OrRule(&Rule::And(v.clone())), rules);
                if res {
                    return (true, submsg_candidate);
                }
            }
            (false, submsg)
        }
    }

}

fn parse_rules(rules: &[String]) -> Vec<Rule> {
    let mut rules_map = rules
    .iter()
    .map(|rule| {
        let mut splitted = rule.split(" ");
        let key_str = splitted.next().unwrap();
        let key = key_str[0..key_str.len() - 1].parse::<usize>().unwrap();
        let rule_str = rule[key_str.len() + 1..].trim();
        if "\"a\"" == rule_str || "\"b\"" == rule_str {
            (key, Rule::Char(rule_str.chars().find(|c| c != &'"').unwrap()))
        } else if rule_str.contains("|") {
            (key, Rule::Or(
                rule_str
                    .split("|")
                    .map(|and_part|
                        and_part
                            .split(" ").filter(|s| s.len() > 0)
                            .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
                    )
                    .collect::<Vec<Vec<usize>>>(),
            ))
        } else {
            (key, Rule::And(rule_str
                .split(" ")
                .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()))
        }
    })
    .collect::<HashMap<usize, Rule>>();
    let mut res: Vec<Rule> = Vec::with_capacity(rules_map.len());
    for _ in 0..rules_map.len() {
        res.push(Rule::Char('0'));
    }
    for (i, r) in rules_map.drain() {
        res[i] = r;
    }
    res
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<usize> {
    let split_pos = find_split_positions(input);
    let rules = &input[0..split_pos[0]];
    let messages = &input[split_pos[0] + 1..];
    let parsed_rules = parse_rules(rules);
    Some(messages.iter().filter(|msg| {
        let (str_match, rest_str) = check_message(msg, Rori::Idx(0), &parsed_rules);
        str_match && rest_str.len() == 0 && msg.len() > 0
    }).count())
}

#[test]
fn test_day_19() {
    use super::common::string_to_lines;
    let input = string_to_lines(
        "0: 4 1 5
    1: 2 3 | 3 2
    2: 4 4 | 5 5
    3: 4 5 | 5 4
    4: \"a\"
    5: \"b\"
    
    a
    ababbb
    bababa
    abbbab
    aaabbb
    aaaabbb",
    );
    assert_eq!(run(&input, TaskOfDay::First), Some(2usize));
}
