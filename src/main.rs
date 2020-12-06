#[macro_use]
extern crate itertools;
use std::fs;
use std::time::Instant;
mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
use common::TaskOfDay;

fn read_file_with_blank_lines(path: &String) -> Vec<String> {
    let res: Vec<String> = fs::read_to_string(path)
        .expect("Could not read file.")
        .split("\n")
        .map(|s| s.trim())
        .map(|s| s.to_string())
        .collect();
    res
}

fn read_file(path: &String) -> Vec<String> {
    let lines = read_file_with_blank_lines(&path);
    let res: Vec<String> = lines
        .iter()
        .filter(|s| s.len() > 0)
        .map(|s| s.clone())
        .collect();
    res
}

fn run_on_content<T>(f: fn(&Vec<String>, TaskOfDay) -> T, contents: &Vec<String>) -> (T, T) {
    let now = Instant::now();
    let res_first = f(contents, TaskOfDay::First);
    let elapsed = now.elapsed();
    println!("Elapsed first:\t{:.2} millis", elapsed.as_millis());
    let now = Instant::now();
    let res_second = f(contents, TaskOfDay::Second);
    let elapsed = now.elapsed();
    println!("Elapsed second:\t{:.2} millis", elapsed.as_millis());
    (res_first, res_second)
}

fn get_path(day: i32) -> String {
    println!("Running day {}", day);
    format!("res/input_{:02}.txt", day)    
}

fn run<T>(day: i32, f: fn(&Vec<String>, TaskOfDay) -> T) -> (T, T) {
    let path = get_path(day);
    let contents: Vec<String> = read_file(&path);
    run_on_content(f, &contents)
}

fn run_with_blank_lines<T>(day: i32, f: fn(&Vec<String>, TaskOfDay) -> T) -> (T, T) {
    let path = get_path(day);
    let contents: Vec<String> = read_file_with_blank_lines(&path);
    run_on_content(f, &contents)
}

fn print_res<T: std::fmt::Display>(res: (T, T)) {
    println!("{}, {}\n", res.0, res.1);
}

fn main() {
    print_res(run(1, day1::day1));
    print_res(run(2, day2::day2));
    print_res(run(3, day3::day3));
    print_res(run_with_blank_lines(4, day4::day4));
    print_res(run(5, day5::day5));
    print_res(run_with_blank_lines(6, day6::day6));
}

#[test]
fn test() {
    assert_eq!(run(1, day1::day1), (1007331, 48914340));
    assert_eq!(run(2, day2::day2), (434, 509));
    assert_eq!(run(3, day3::day3), (247, 2983070376i64));
    assert_eq!(run_with_blank_lines(4, day4::day4), (247, 145));
    assert_eq!(run(5, day5::day5), (938, 696));
    assert_eq!(run_with_blank_lines(6, day6::day6), (6542, 3299));
}
