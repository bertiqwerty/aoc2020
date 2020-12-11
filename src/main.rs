#[macro_use]
extern crate itertools;
use std::fs;
use std::time::Instant;
mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
use common::TaskOfDay;

fn read_file_with_blank_lines(path: &str) -> Vec<String> {
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

fn unwrap_print_res<T: std::fmt::Display>(res: (Option<T>, Option<T>)) {
    println!("{}, {}\n", res.0.unwrap(), res.1.unwrap());
}

fn main() {
    print_res(run(1, day01::run));
    print_res(run(2, day02::run));
    print_res(run(3, day03::run));
    print_res(run_with_blank_lines(4, day04::run));
    print_res(run(5, day05::run));
    print_res(run_with_blank_lines(6, day06::run));
    print_res(run(7, day07::run));
    print_res(run(8, day08::run));
    print_res(run(9, day09::run));
    unwrap_print_res(run(10, day10::run));
    unwrap_print_res(run(11, day11::run));

}

#[test]
fn test() {
    assert_eq!(run(1, day01::run), (1007331, 48914340));
    assert_eq!(run(2, day02::run), (434, 509));
    assert_eq!(run(3, day03::run), (247, 2983070376i64));
    assert_eq!(run_with_blank_lines(4, day04::run), (247, 145));
    assert_eq!(run(5, day05::run), (938, 696));
    assert_eq!(run_with_blank_lines(6, day06::run), (6542, 3299));
    assert_eq!(run(7, day07::run), (177, 34988));
    assert_eq!(run(8, day08::run), (1782, 797));
    assert_eq!(run(9, day09::run), (1930745883, 268878261)); 
    assert_eq!(run(10, day10::run), (Some(2738), Some(74049191673856))); 
    assert_eq!(run(11, day11::run), (Some(2476), Some(0))); 
      
}
