use super::common::TaskOfDay;
use super::common::split_in2_tuple;

pub fn run(input: &Vec<String>, part: TaskOfDay) -> i32 {
    input
        .iter()
        .filter(|s| {
            
            let (before_colon, haystack) = split_in2_tuple(s, ": ");
            let (range, needle) = split_in2_tuple(&before_colon, " ");
            let (minval_s, maxval_s) = split_in2_tuple(&range, "-");
            let minval = minval_s
                .parse::<usize>()
                .expect("could not parse string to int");
            let maxval = maxval_s
                .parse::<usize>()
                .expect("could not parse string to int");

            match part {
                TaskOfDay::First => (|| {
                    let num_matches = haystack.matches(&needle).count();
                    minval <= num_matches && num_matches <= maxval
                })(),
                TaskOfDay::Second => {
                    (haystack.chars().nth(minval - 1).unwrap() == needle.chars().next().unwrap())
                        != (haystack.chars().nth(maxval - 1).unwrap()
                            == needle.chars().next().unwrap())
                }
            }
        })
        .count() as i32
}

