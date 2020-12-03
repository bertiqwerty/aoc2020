#[macro_use]
extern crate itertools;
use std::fs;
use std::time::Instant;

fn read_file(path: &String) -> Vec<String> {
    let res: Vec<String> = fs::read_to_string(path)
        .expect("oink")
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| s.to_string())
        .collect();
    res
}

enum TaskOfDay {
    First,
    Second,
}

fn run<T>(day: i32, f: fn(&Vec<String>) -> T) -> T {
    let path = format!("res/input_{:02}.txt", day);
    let contents: Vec<String> = read_file(&path);
    let now = Instant::now();
    let res = f(&contents);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2} millis", elapsed.as_millis());
    res
}

fn day1(input: &Vec<String>) -> i32 {
    let converted = input
        .iter()
        .map(|s| s.parse::<i32>().expect("could not parse string to int {}"));
    let t = iproduct!(
        iproduct!(converted.clone(), converted.clone()).filter(|&(i, j)| i + j <= 2020),
        converted
    )
    .find(|&(t, k)| t.0 + t.1 + k == 2020)
    .unwrap();
    t.0 .0 * t.0 .1 * t.1
}

fn day2(input: &Vec<String>, part: TaskOfDay) -> i32 {
    input
        .iter()
        .filter(|s| {
            fn split_in2_tuple(s_: &str, ssplit: &str) -> (String, String) {
                let mut splt = s_.split(ssplit).map(|s| s.trim().to_string());
                (splt.next().unwrap(), splt.next().unwrap())
            };

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

fn sum_of_steps(tree_matrix: &Vec<Vec<i32>>, row_step: usize, col_step: usize) -> i32 {
    let mut sum = 0i32;
    for (row, col) in izip!(
        (row_step..tree_matrix.len()).step_by(row_step),
        (col_step..tree_matrix.len() * col_step).step_by(col_step)
    ) {
        sum += tree_matrix[row].iter().cycle().nth(col).unwrap();
    }
    sum
}

fn day3(input: &Vec<String>, part: TaskOfDay) -> i64 {
    let tree_matrix = input
        .iter()
        .map(|s| s.chars().map(|c| (c == '#') as i32).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();
    match part {
        TaskOfDay::First => sum_of_steps(&tree_matrix, 1, 3) as i64,
        TaskOfDay::Second => izip!(
            vec!(1usize, 1usize, 1usize, 1usize, 2usize).iter(),
            vec!(1usize, 3usize, 5usize, 7usize, 1usize).iter()
        )
        .map(|(r, c)| sum_of_steps(&tree_matrix, r.clone(), c.clone()) as i64)
        .product(),
    }
}

fn main() {
    println!("day1 {}", run(1, day1));
    println!(
        "day2, first {}",
        run(2, |input| day2(input, TaskOfDay::First))
    );
    println!(
        "day2, second {}",
        run(2, |input| day2(input, TaskOfDay::Second))
    );
    println!(
        "day3, first {}",
        run(3, |input| day3(input, TaskOfDay::First))
    );
    println!(
        "day3, second {}",
        run(3, |input| day3(input, TaskOfDay::Second))
    );
}
