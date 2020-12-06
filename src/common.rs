use std::iter;

pub enum TaskOfDay {
    First,
    Second,
}

pub fn split_in2_tuple(s_: &str, ssplit: &str) -> (String, String) {
    let mut splt = s_.split(ssplit).map(|s| s.trim().to_string());
    (splt.next().unwrap(), splt.next().unwrap())
}

pub fn to_string_vec(v: &Vec<&str>) -> Vec<String>
{
    v.iter().map(|elt| elt.to_string()).collect::<Vec<String>>()
}

pub fn separate_by_blanks(input: &Vec<String>, joiner: &str) -> Vec<String>
{
    // TODO: currently, last element of input must be a blank line/string
    let split_positions = input
        .iter()
        .enumerate()
        .filter(|t: &(usize, &String)| t.1.len() == 0)
        .map(|t: (usize, &String)| t.0)
        .collect::<Vec<usize>>();
    let splits_shifted = &split_positions[1..];
    iter::once(input[0..split_positions[0]].join(joiner)).chain(
        izip!(&split_positions, splits_shifted)
            .map(|t| input[t.0.clone() + 1..t.1.clone()].join(joiner)),
    ).collect::<Vec<String>>()
}