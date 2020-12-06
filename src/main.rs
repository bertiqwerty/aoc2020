#[macro_use]
extern crate itertools;
use std::fs;
use std::time::Instant;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod common;
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
    let res: Vec<String> = lines.iter()
        .filter(|s| s.len() > 0)
        .map(|s| s.clone())
        .collect();
    res
}

fn run_on_content<T>(f: fn(&Vec<String>) -> T, contents: &Vec<String>) -> T {
    let now = Instant::now();
    let res = f(contents);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2} millis", elapsed.as_millis());
    res
}

fn run<T>(day: i32, f: fn(&Vec<String>) -> T) -> T {
    let path = format!("res/input_{:02}.txt", day);
    let contents: Vec<String> = read_file(&path);
    run_on_content(f, &contents)
}

fn run_with_blank_lines<T>(day: i32, f: fn(&Vec<String>) -> T) -> T {
    let path = format!("res/input_{:02}.txt", day);
    let contents: Vec<String> = read_file_with_blank_lines(&path);
    run_on_content(f, &contents)
}

fn main() {
    println!("day1 {}", run(1, day1::day1));
    println!(
        "day2, first {}",
        run(2, |input| day2::day2(input, TaskOfDay::First))
    );
    println!(
        "day2, second {}",
        run(2, |input| day2::day2(input, TaskOfDay::Second))
    );
    println!(
        "day3, first {}",
        run(3, |input| day3::day3(input, TaskOfDay::First))
    );
    println!(
        "day3, second {}",
        run(3, |input| day3::day3(input, TaskOfDay::Second))
    );
    println!(
        "day4, first {}",
        run_with_blank_lines(4, |input| day4::day4(input, TaskOfDay::First))
    );
    println!(
        "day4, second {}",
        run_with_blank_lines(4, |input| day4::day4(input, TaskOfDay::Second))
    );
    println!(
        "day5, first {}",
        run(5, |input| day5::day5(input, TaskOfDay::First))
    );
    println!(
        "day5, second {}",
        run(5, |input| day5::day5(input, TaskOfDay::Second))
    );
    println!(
        "day6, first {}",
        run_with_blank_lines(6, |input| day6::day6(input, TaskOfDay::First))
    );
    println!(
        "day6, second {}",
        run_with_blank_lines(6, |input| day6::day6(input, TaskOfDay::Second))
    );

}

#[test]
fn test(){
    assert_eq!(run(1, day1::day1), 48914340);
    assert_eq!(run(2, |input| day2::day2(input, TaskOfDay::First)), 434);
    assert_eq!(run(2, |input| day2::day2(input, TaskOfDay::Second)), 509);
    assert_eq!(run(3, |input| day3::day3(input, TaskOfDay::First)), 247);
    assert_eq!(run(3, |input| day3::day3(input, TaskOfDay::Second)), 2983070376i64);
    assert_eq!(run_with_blank_lines(4, |input| day4::day4(input, TaskOfDay::First)), 247);
    assert_eq!(run_with_blank_lines(4, |input| day4::day4(input, TaskOfDay::Second)), 145);
    assert_eq!(run(5, |input| day5::day5(input, TaskOfDay::First)), 938);
    assert_eq!(run(5, |input| day5::day5(input, TaskOfDay::Second)), 696);
}