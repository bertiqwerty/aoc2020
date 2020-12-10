use super::common::TaskOfDay;
use std::iter;

fn compute_gaps(input: &Vec<usize>) -> Option<Vec<usize>> {
    let max_val = *input.iter().max()?;
    let iter_prefix = iter::once(0usize)
        .chain(input.iter().map(|g| *g));
    let iter_suffix = 
        input.iter().map(|g| *g)
        .chain(iter::once(max_val + 3));
    Some(izip!(iter_prefix, iter_suffix).map(|(i, j)| j - i).collect::<Vec<usize>>())
}
fn count_gaps(gaps: &Vec<usize>) -> Option<(usize, usize)> {
    let ones = gaps.iter().filter(|elt| **elt == 1).count();
    let threes = gaps.iter().filter(|elt| **elt == 3).count();
    if ones + threes != gaps.len() {
        return None;
    }
    Some((ones, threes))
}

fn count_combinations(gaps: &Vec<usize>) {
    let mut combinations = 0usize;
    for i in (0..gaps.len()) {}
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<usize> {
    let sort_input = || {
        let mut sor_in: Vec<usize> = input.iter().map(|s| s.parse::<usize>().unwrap()).collect();
        sor_in.sort();
        sor_in
    };
    let sorted_input = sort_input();
    let gaps = compute_gaps(&sorted_input)?;
    match part {
        TaskOfDay::First => {
            let res = count_gaps(&gaps);
            Some(res?.0 * res?.1)
        }
        TaskOfDay::Second => Some(0),
    }
}

#[test]
fn test() {
    let input: Vec<String> = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
    .split("\n")
    .map(|s| s.to_string())
    .collect();
    assert_eq!(run(&input, TaskOfDay::First).unwrap(), 220);
}
