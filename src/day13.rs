use super::common::TaskOfDay;

pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<i32> {
    let estimated_arrival = input[0].parse::<i32>().unwrap();
    let min_tuple = input[1]
        .split(',')
        .filter(|s| s != &"x")
        .map(|s| s.parse::<i32>().unwrap())
        .map(|i| 
            (i, 
            i * (estimated_arrival / i + 1) - estimated_arrival))
        
        .min_by_key(|elt| elt.1)?;
    Some(min_tuple.0 * min_tuple.1)
}

#[test]
fn test() {
    use super::common::string_to_lines;

    let input = string_to_lines(
        "939
    7,13,x,x,59,x,31,19",
    );
    assert_eq!(run(&input, TaskOfDay::First).unwrap(), 295);
}
