use super::common::find_split_positions;
use super::common::TaskOfDay;

use exmex::{
    literal_matcher_from_pattern, ops_factory, BinOp, Express, ExResult, FlatEx, MakeOperators,
    MatchLiteral, Operator,
};
use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum RuleOp {
    Idx(usize),
    Char(char),
    Concatenate(Vec<RuleOp>),
    Union(Vec<RuleOp>),
}

impl RuleOp {
    pub fn eval<'a>(&self, msgs: &Vec<&'a str>, rules: &Vec<RuleOp>) -> Vec<&'a str> {
        if msgs.len() == 0 {
            return msgs.clone();
        }
        match self {
            RuleOp::Idx(idx) => rules[*idx].eval(msgs, rules),
            RuleOp::Char(c) => msgs
                .iter()
                .filter(|msg| msg.chars().nth(0) == Some(*c))
                .map(|msg| &msg[1..])
                .collect::<Vec<_>>(),
            RuleOp::Concatenate(ops) => {
                let mut res = msgs.clone();
                for op in ops {
                    res = op.eval(&res, rules);
                }
                res
            }
            RuleOp::Union(ops) => ops
                .iter()
                .flat_map(|op| op.eval(msgs, rules))
                .collect::<Vec<_>>(),
        }
    }
}

impl FromStr for RuleOp {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.chars().nth(1);
        Ok(match first {
            Some(letter) if letter == 'a' || letter == 'b' => RuleOp::Char(letter),
            _ => RuleOp::Idx(s.parse::<usize>()?),
        })
    }
}

ops_factory!(
    OpsOpsFactory,
    RuleOp,
    Operator::make_bin(
        "|",
        BinOp {
            apply: |op1, op2| { RuleOp::Union(vec![op1, op2]) },
            prio: 0,
            is_commutative: true
        }
    ),
    Operator::make_bin(
        "o",
        BinOp {
            apply: |op1, op2| { RuleOp::Concatenate(vec![op1, op2]) },
            prio: 1,
            is_commutative: false
        }
    )
);

pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<usize> {
    // basic idea is that numbers in rules are operators, use exmex with operator literals
    let split_pos = find_split_positions(input);
    let rules_raw = &input[0..split_pos[0]];
    let mut rules_strs = vec!["".to_string(); rules_raw.len()];
    for rule_raw in rules_raw.iter() {
        let rule_raw = match part {
            TaskOfDay::Second if rule_raw[0..2].eq("8:") => {
                rule_raw.replace("8: 42", "8: 42 | 42 8")
            }
            TaskOfDay::Second if rule_raw[0..3].eq("11:") => {
                rule_raw.replace("11: 42 31", "11: 42 31 | 42 11 31")
            }
            _ => rule_raw.clone(),
        };
        let mut split = rule_raw.split(":");
        let i = split.next()?.parse::<usize>().ok()?;
        rules_strs[i] = split
            .next()?
            .trim()
            .replace(" ", "o")
            .replace("o|", "|")
            .replace("|o", "|");
    }
    const LITERAL_PATTERN: &str = "^([0-9]+|\"a\"|\"b\")";
    literal_matcher_from_pattern!(OpsMatcher, LITERAL_PATTERN);
    type FlatExOps<'a> = FlatEx::<'a, RuleOp, OpsOpsFactory, OpsMatcher>;
    let rules = rules_strs
        .iter()
        .map(|s| -> ExResult<_> {
            let flatex = FlatExOps::from_str(s)?;
            flatex.eval(&[])
        })
        .collect::<ExResult<Vec<_>>>()
        .ok()?;
    let messages = &input[split_pos[0] + 1..];
    Some(
        messages
            .iter()
            .filter(|msg| {
                // we consider an evaluation a match if all letters have been consumed
                // and precisely the empty string is left in the result vector.
                let res = rules[0].eval(&vec![msg.as_str()], &rules);
                res.iter().filter(|s| s.len() == 0).count() == 1
            })
            .count(),
    )
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

    // a ((a a | b b) (a b | b a)) | (a b | b a) (a a| b b)) b
    // 0(ababbb) = 5(1(4(ababbb)))
    // 4(ababbb) =  a[babbb] == a => [(a, babbb)]
    // 1(babbb) = 3(2(babbb)) | 2(3(babbb) => [(a, bbb)]) => [(bb, b)]
    // 2(babbb) = 4(4(babbb) => []) => [] | 5(5(babbb) => [(a, abbb)]) => []
    // 3(babbb) = 5(4(babbb) => []) => [] | 4(5(babbb) => [(b, abbb)]) => [(a, bbb)]
    // 5(bbb) => [(b, bb)]
}
