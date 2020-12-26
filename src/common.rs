use std::iter;

pub enum TaskOfDay {
    First,
    Second,
}



pub fn split_in2_tuple<'a>(to_be_split: &'a str, splitter: &str) -> (&'a str, &'a str) {
    let mut splt = to_be_split.split(splitter).map(|s| s.trim());
    (splt.next().unwrap(), splt.next().unwrap())
}

pub fn separate_by_blanks(input: &Vec<String>, joiner: &str) -> Vec<String> {
    // TODO: currently, last element of input must be a blank line/string
    let split_positions = input
        .iter()
        .enumerate()
        .filter(|t: &(usize, &String)| t.1.len() == 0)
        .map(|t: (usize, &String)| t.0)
        .collect::<Vec<usize>>();
    let splits_shifted = &split_positions[1..];
    iter::once(input[0..split_positions[0]].join(joiner))
        .chain(
            izip!(&split_positions, splits_shifted)
                .map(|t| input[t.0.clone() + 1..t.1.clone()].join(joiner)),
        )
        .collect::<Vec<String>>()
}

pub fn string_to_lines(s: &str) -> Vec<String> {
    s.split("\n").map(|s| s.trim().to_string()).collect()
}
