use super::common::split_in2_tuple;
use super::common::TaskOfDay;
use super::common::to_string_vec;
use regex::Regex;
use std::iter;

fn are_needles_in_haystack(needles: &Vec<String>, haystack: &Vec<String>, any: bool) -> bool {
    let hits = needles.iter().filter(|n| haystack.contains(&n.to_string())).count();
    (any && hits > 0) || (!any && hits == needles.len())
}

fn validator_part_1(passport: &String) -> bool {
    let available = passport
        .split(" ")
        .map(|s| s.split(":").next().clone().unwrap().to_string())
        .collect::<Vec<String>>();
    let needed: Vec<String> = to_string_vec(&vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);
    are_needles_in_haystack(&needed, &available, false)
}
fn in_between(num: &String, lb: i32, ub: i32, num_digits: i32) -> bool {
    let parsed = num.parse::<i32>().unwrap_or(lb - 1);
    lb <= parsed && parsed <= ub &&  num.len() == num_digits as usize
}

fn hgt_check(hgt_value: &String) -> bool {
    let siz = hgt_value.len();
    let num = &hgt_value[..siz - 2].to_string();
    let unit = &hgt_value[siz - 2..].to_string();
    
    match unit.as_str() {
        "cm" => in_between(num, 150, 193, 3),
        "in" => in_between(num, 59, 76, 2),
        _ => false
    }
}

fn hcl_check(hcl_value: &String) -> bool {
    let hashtag = &hcl_value[0..1].to_string();
    let hashtag_true = "#".to_string();
    let color = &hcl_value[1..].to_string();
    let re = Regex::new("[a-f0-9]{6}").unwrap();
    re.is_match(color) && *hashtag == hashtag_true
}
fn ecl_check(ecl_value: &String) -> bool {  
    let colors = to_string_vec(&vec!["amb", "blu", "gry", "brn", "grn", "hzl", "oth"]);
    colors.contains(ecl_value)
}
fn pid_check(pid_value:&String) -> bool {
    Regex::new("[0-9]{9}").unwrap().is_match(&pid_value) && pid_value.len() == 9
}
fn validator_part_2(passport: &String) -> bool {
    passport
        .split(" ")
        .filter(|s| {
            let (key, value) = split_in2_tuple(s, ":");
            match key.as_str() {
                "byr" => in_between(&value, 1920, 2002, 4),
                "iyr" => in_between(&value, 2010, 2020, 4),
                "eyr" => in_between(&value, 2020, 2030, 4),
                "hgt" => hgt_check(&value),
                "hcl" => hcl_check(&value),
                "ecl" => ecl_check(&value),
                "pid" => pid_check(&value),
                _ => false
            }
        })
        .count() == 7
}

pub fn day4(input: &Vec<String>, part: TaskOfDay) -> i32 {
    let split_positions = input
        .iter()
        .enumerate()
        .filter(|t: &(usize, &String)| t.1.len() == 0)
        .map(|t: (usize, &String)| t.0)
        .collect::<Vec<usize>>();
    let splits_shifted = &split_positions[1..];
    let passports = iter::once(input[0..split_positions[0]].join(" ")).chain(
        izip!(&split_positions, splits_shifted)
            .map(|t| input[t.0.clone() + 1..t.1.clone()].join(" ")),
    );
    match part {
        TaskOfDay::First => passports.filter(validator_part_1).count() as i32,
        TaskOfDay::Second => passports.filter(validator_part_2).count() as i32,
    }
}


#[test]
fn test_in_between(){
    let num = "5".to_string();
    assert_eq!(in_between(&num, 0, 5, 1), true);
    assert_eq!(in_between(&num, 0, 1, 1), false);
    assert_eq!(in_between(&num, 5, 5, 1), true);
    assert_eq!(in_between(&num, 6, 5, 1), false);
    assert_eq!(in_between(&num, 3, 9, 1), true);
    assert_eq!(in_between(&num, 4, 23, 1), true);
    assert_eq!(in_between(&num, 230, 234, 1), false);
    let num = "1192".to_string();
    assert_eq!(in_between(&num, 0, 5, 1), false);
    assert_eq!(in_between(&num, 0, 1, 1), false);
    assert_eq!(in_between(&num, 5, 5, 1), false);
    assert_eq!(in_between(&num, 6, 5, 1), false);
    assert_eq!(in_between(&num, 3, 9, 1), false);
    assert_eq!(in_between(&num, 4, 23, 1), false);
    assert_eq!(in_between(&num, 230, 234, 1), false);
    assert_eq!(in_between(&num, 230, 2000, 4), true);
    assert_eq!(in_between(&num, 1100, 2000, 4), true);
    assert_eq!(in_between(&num, 1100, 2000, 3), false);

}

#[test]
fn test_checks()
{
    assert_eq!(hgt_check(&"123cm".to_string()), false);
    assert_eq!(hgt_check(&"149cm".to_string()), false);
    assert_eq!(hgt_check(&"150cm".to_string()), true);
    assert_eq!(hgt_check(&"193cm".to_string()), true);
    assert_eq!(hgt_check(&"194cm".to_string()), false);
    assert_eq!(hgt_check(&"149cm".to_string()), false);
    assert_eq!(hgt_check(&"150cmq".to_string()), false);
    assert_eq!(hgt_check(&"4193cm".to_string()), false);
    assert_eq!(hgt_check(&"s194cm".to_string()), false);
    assert_eq!(hgt_check(&"149in".to_string()), false);
    assert_eq!(hgt_check(&"150in".to_string()), false);
    assert_eq!(hgt_check(&"4193in".to_string()), false);
    assert_eq!(hgt_check(&"s194in".to_string()), false);
    assert_eq!(hgt_check(&"59in".to_string()), true);
    assert_eq!(hgt_check(&"76in".to_string()), true);
    assert_eq!(hgt_check(&"58in".to_string()), false);
    assert_eq!(hgt_check(&"77in".to_string()), false);

    assert_eq!(hgt_check(&"60in".to_string()), true);
    assert_eq!(hgt_check(&"190cm".to_string()), true);
    assert_eq!(hgt_check(&"190in".to_string()), false);
    assert_eq!(hgt_check(&"190".to_string()), false);

    assert_eq!(hcl_check(&"123cm".to_string()), false);
    assert_eq!(hcl_check(&"149cm".to_string()), false);
    assert_eq!(hcl_check(&"150cm".to_string()), false);
    assert_eq!(hcl_check(&"193cm".to_string()), false);
    assert_eq!(hcl_check(&"194cm".to_string()), false);
    assert_eq!(hcl_check(&"149cm".to_string()), false);
    assert_eq!(hcl_check(&"150cmq".to_string()), false);
    assert_eq!(hcl_check(&"4193cm".to_string()), false);
    assert_eq!(hcl_check(&"s194cm".to_string()), false);
    assert_eq!(hcl_check(&"149in".to_string()), false);
    assert_eq!(hcl_check(&"150in".to_string()), false);
    assert_eq!(hcl_check(&"4193in".to_string()), false);
    assert_eq!(hcl_check(&"s194in".to_string()), false);
    assert_eq!(hcl_check(&"#59".to_string()), false);
    assert_eq!(hcl_check(&"#asfd59".to_string()), false);
    assert_eq!(hcl_check(&"#a3fd59".to_string()), true);
    assert_eq!(hcl_check(&"#asfz59".to_string()), false);
    
    assert_eq!(hcl_check(&"#123abc".to_string()), true);
    assert_eq!(hcl_check(&"#123abz".to_string()), false);
    assert_eq!(hcl_check(&"123abc".to_string()), false);

    assert_eq!(ecl_check(&"4193in".to_string()), false);
    assert_eq!(ecl_check(&"s194in".to_string()), false);
    assert_eq!(ecl_check(&"#59".to_string()), false);
    assert_eq!(ecl_check(&"amb".to_string()), true);
    assert_eq!(ecl_check(&"blu".to_string()), true);
    assert_eq!(ecl_check(&"brn".to_string()), true);
    assert_eq!(ecl_check(&"gry".to_string()), true);
    assert_eq!(ecl_check(&"grn".to_string()), true);
    assert_eq!(ecl_check(&"hzl".to_string()), true);
    assert_eq!(ecl_check(&"oth".to_string()), true);
    assert_eq!(ecl_check(&"hzl oth".to_string()), false);
    assert_eq!(ecl_check(&"hzoth".to_string()), false);

    assert_eq!(ecl_check(&"brn".to_string()), true);
    assert_eq!(ecl_check(&"wat".to_string()), false);


    assert_eq!(pid_check(&"hzl".to_string()), false);
    assert_eq!(pid_check(&"oth".to_string()), false);
    assert_eq!(pid_check(&"hzl oth".to_string()), false);
    assert_eq!(pid_check(&"hzoth".to_string()), false);
    assert_eq!(pid_check(&"hzl oth".to_string()), false);
    assert_eq!(pid_check(&"123".to_string()), false);
    assert_eq!(pid_check(&"5345 oth".to_string()), false);
    assert_eq!(pid_check(&"023412344".to_string()), true);
    assert_eq!(pid_check(&"923412344".to_string()), true);
    assert_eq!(pid_check(&"234 oth".to_string()), false);
    assert_eq!(pid_check(&"hzo54th".to_string()), false);

    assert_eq!(pid_check(&"000000001".to_string()), true);
    assert_eq!(pid_check(&"0123456789".to_string()), false);

    

    
}