#[macro_use] extern crate itertools;
use std::fs;
use std::time::Instant;
fn read_file(path: &String) -> Vec<String> {
    let res: Vec<String> = fs::read_to_string(path).expect("oink")
        .split("\n").map(|s| s.trim()).filter(|s| s.len() > 0).map(|s| s.to_string()).collect();
    res
}


enum TaskOfDay {
    First,
    Second,
}


fn run(day: i32, f: &dyn Fn(&Vec<String>) -> i32) -> i32
{
    let path =  format!("res/input_{:02}.txt", day);
    let contents: Vec<String> = read_file(&path);
    let now = Instant::now();
    let res = f(&contents);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2} millis", elapsed.as_millis());
    res
}


fn day1(input: &Vec<String>) -> i32 {
    let converted = input.iter().map(|s| s.parse::<i32>().expect("could not parse string to int {}"));
    let t = iproduct!(iproduct!(converted.clone(), converted.clone())
            .filter(|&(i, j)| i + j <= 2020), converted)
        .find(|&(t, k)| t.0 + t.1 + k == 2020).unwrap();    
    t.0.0 * t.0.1 * t.1
}   


fn day2(input: &Vec<String>, part: TaskOfDay) -> i32 {
    input.iter().filter(|s|{
        fn split_in2_tuple(s_: &str, ssplit: &str) -> (String, String){ 
            let mut splt =  s_.split(ssplit).map(|s| s.trim().to_string());
            (splt.next().unwrap(), splt.next().unwrap())
        };
        
        let (before_colon, haystack) = split_in2_tuple(s, ": ");
        let (range, needle) = split_in2_tuple(&before_colon, " ");
        let (minval_s, maxval_s) = split_in2_tuple(&range, "-");
        let minval = minval_s.parse::<usize>().expect("could not parse string to int");
        let maxval = maxval_s.parse::<usize>().expect("could not parse string to int");

        match part {
            TaskOfDay::First => (|| {
                let num_matches = haystack.matches(&needle).count();
                minval <= num_matches && num_matches <= maxval
            })(),        
            TaskOfDay::Second => (haystack.chars().nth(minval-1).unwrap() == needle.chars().next().unwrap()) !=
                (haystack.chars().nth(maxval-1).unwrap() == needle.chars().next().unwrap())
        }
        
    }).count() as i32

}

fn main() {
    println!("day1 {}", run(1, &day1));
    println!("day2, first {}", run(2, &|input| day2(input, TaskOfDay::First)));
    println!("day2, second {}", run(2, &|input| day2(input, TaskOfDay::Second)));
}