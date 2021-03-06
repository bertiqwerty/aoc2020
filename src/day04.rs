use super::common::split_in2_tuple;
use super::common::TaskOfDay;
use super::common::separate_by_blanks;
use regex::Regex;

fn are_needles_in_haystack(needles: &Vec<&str>, haystack: &Vec<&str>, any: bool) -> bool {
    let hits = needles.iter().filter(|n| haystack.contains(n)).count();
    (any && hits > 0) || (!any && hits == needles.len())
}

fn validator_part_1(passport: &&String) -> bool {
    let available = passport
        .split(" ")
        .map(|s| s.split(":").next().clone().unwrap())
        .collect::<Vec<&str>>();
    let needed = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    are_needles_in_haystack(&needed, &available, false)
}

fn in_between(num: &str, lb: i32, ub: i32, num_digits: i32) -> bool {
    let parsed = num.parse::<i32>().unwrap_or(lb - 1);
    lb <= parsed && parsed <= ub &&  num.len() == num_digits as usize
}

fn hgt_check(hgt_value: &str) -> bool {
    let siz = hgt_value.len();
    let num = &hgt_value[..siz - 2];
    let unit = &hgt_value[siz - 2..];
    
    match unit {
        "cm" => in_between(num, 150, 193, 3),
        "in" => in_between(num, 59, 76, 2),
        _ => false
    }
}

fn hcl_check(hcl_value: &str) -> bool {
    let hashtag = &hcl_value[0..1];
    let hashtag_true = "#";
    let color = &hcl_value[1..];
    let re = Regex::new("[a-f0-9]{6}").unwrap();
    re.is_match(color) && hashtag == hashtag_true
}

fn ecl_check(ecl_value: &str) -> bool {  
    let colors = vec!["amb", "blu", "gry", "brn", "grn", "hzl", "oth"];
    colors.contains(&ecl_value)
}

fn pid_check(pid_value:&str) -> bool {
    Regex::new("[0-9]{9}").unwrap().is_match(&pid_value) && pid_value.len() == 9
}

fn validator_part_2(passport: &&String) -> bool {
    passport
        .split(" ")
        .filter(|s| {
            let (key, value) = split_in2_tuple(s, ":");
            match key {
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

pub fn run(input: &Vec<String>, part: TaskOfDay) -> i32 {
    let passports: Vec<String> = separate_by_blanks(input, " ");
    match part {
        TaskOfDay::First => passports.iter().filter(validator_part_1).count() as i32,
        TaskOfDay::Second => passports.iter().filter(validator_part_2).count() as i32,
    }
}


#[test]
fn test_in_between(){
    let num = "5";
    assert_eq!(in_between(&num, 0, 5, 1), true);
    assert_eq!(in_between(&num, 0, 1, 1), false);
    assert_eq!(in_between(&num, 5, 5, 1), true);
    assert_eq!(in_between(&num, 6, 5, 1), false);
    assert_eq!(in_between(&num, 3, 9, 1), true);
    assert_eq!(in_between(&num, 4, 23, 1), true);
    assert_eq!(in_between(&num, 230, 234, 1), false);
    let num = "1192";
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
    assert_eq!(hgt_check("123cm"), false);
    assert_eq!(hgt_check("149cm"), false);
    assert_eq!(hgt_check("150cm"), true);
    assert_eq!(hgt_check("193cm"), true);
    assert_eq!(hgt_check("194cm"), false);
    assert_eq!(hgt_check("149cm"), false);
    assert_eq!(hgt_check("150cmq"), false);
    assert_eq!(hgt_check("4193cm"), false);
    assert_eq!(hgt_check("s194cm"), false);
    assert_eq!(hgt_check("149in"), false);
    assert_eq!(hgt_check("150in"), false);
    assert_eq!(hgt_check("4193in"), false);
    assert_eq!(hgt_check("s194in"), false);
    assert_eq!(hgt_check("59in"), true);
    assert_eq!(hgt_check("76in"), true);
    assert_eq!(hgt_check("58in"), false);
    assert_eq!(hgt_check("77in"), false);

    assert_eq!(hgt_check("60in"), true);
    assert_eq!(hgt_check("190cm"), true);
    assert_eq!(hgt_check("190in"), false);
    assert_eq!(hgt_check("190"), false);

    assert_eq!(hcl_check("123cm"), false);
    assert_eq!(hcl_check("149cm"), false);
    assert_eq!(hcl_check("150cm"), false);
    assert_eq!(hcl_check("193cm"), false);
    assert_eq!(hcl_check("194cm"), false);
    assert_eq!(hcl_check("149cm"), false);
    assert_eq!(hcl_check("150cmq"), false);
    assert_eq!(hcl_check("4193cm"), false);
    assert_eq!(hcl_check("s194cm"), false);
    assert_eq!(hcl_check("149in"), false);
    assert_eq!(hcl_check("150in"), false);
    assert_eq!(hcl_check("4193in"), false);
    assert_eq!(hcl_check("s194in"), false);
    assert_eq!(hcl_check("#59"), false);
    assert_eq!(hcl_check("#asfd59"), false);
    assert_eq!(hcl_check("#a3fd59"), true);
    assert_eq!(hcl_check("#asfz59"), false);
    
    assert_eq!(hcl_check("#123abc"), true);
    assert_eq!(hcl_check("#123abz"), false);
    assert_eq!(hcl_check("123abc"), false);

    assert_eq!(ecl_check("4193in"), false);
    assert_eq!(ecl_check("s194in"), false);
    assert_eq!(ecl_check("#59"), false);
    assert_eq!(ecl_check("amb"), true);
    assert_eq!(ecl_check("blu"), true);
    assert_eq!(ecl_check("brn"), true);
    assert_eq!(ecl_check("gry"), true);
    assert_eq!(ecl_check("grn"), true);
    assert_eq!(ecl_check("hzl"), true);
    assert_eq!(ecl_check("oth"), true);
    assert_eq!(ecl_check("hzl oth"), false);
    assert_eq!(ecl_check("hzoth"), false);

    assert_eq!(ecl_check("brn"), true);
    assert_eq!(ecl_check("wat"), false);

    assert_eq!(pid_check("hzl"), false);
    assert_eq!(pid_check("oth"), false);
    assert_eq!(pid_check("hzl oth"), false);
    assert_eq!(pid_check("hzoth"), false);
    assert_eq!(pid_check("hzl oth"), false);
    assert_eq!(pid_check("123"), false);
    assert_eq!(pid_check("5345 oth"), false);
    assert_eq!(pid_check("023412344"), true);
    assert_eq!(pid_check("923412344"), true);
    assert_eq!(pid_check("234 oth"), false);
    assert_eq!(pid_check("hzo54th"), false);

    assert_eq!(pid_check("000000001"), true);
    assert_eq!(pid_check("0123456789"), false);
    
}