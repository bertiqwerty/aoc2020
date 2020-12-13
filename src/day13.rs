use super::common::TaskOfDay;
use num::PrimInt;

fn ts_id_diff<I: PrimInt>(bus_id: I, time_stamp: I) -> I {
    bus_id * (time_stamp / bus_id + I::one()) - time_stamp
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<u128> {
    match part {
        TaskOfDay::First => {
            let estimated_arrival = input[0].parse::<i32>().unwrap();
            let min_tuple = input[1]
                .split(',')
                .filter(|s| s != &"x")
                .map(|s| s.parse::<i32>().unwrap())
                .map(|i| (i, ts_id_diff::<i32>(i, estimated_arrival)))
                .min_by_key(|elt| elt.1)?;
            Some((min_tuple.0 * min_tuple.1) as u128)
        }
        TaskOfDay::Second => {
            let offset_id_pairs = input[1]
                .split(',')
                .enumerate()
                .filter(|(_, s)| s != &"x")
                .map(|(i, s)| (i as u128, s.parse::<u128>().unwrap()))
                .collect::<Vec<(u128, u128)>>();

            let mut time_stamp = offset_id_pairs[0].1;
            let mut product = offset_id_pairs[0].1;
            for (offset, bus_id) in &offset_id_pairs[1..] {
                while (time_stamp + offset) % bus_id != 0 {
                    time_stamp += product;
                }
                product *= bus_id;
            }
            Some(time_stamp)
        }
    }
}

#[test]
fn test() {
    use super::common::string_to_lines;

    let input = string_to_lines(
        "939
    7,13,x,x,59,x,31,19",
    );

    assert_eq!(ts_id_diff(7, 10), 4);
    assert_eq!(ts_id_diff(7, 8), 6);
    assert_eq!(ts_id_diff(8, 57), 7);
    assert_eq!(ts_id_diff(57, 241), 44);

    assert_eq!(run(&input, TaskOfDay::First).unwrap(), 295);
    assert_eq!(run(&input, TaskOfDay::Second).unwrap(), 1068781);

    let input = string_to_lines(
        "939
        17,x,13,19",
    );
    assert_eq!(run(&input, TaskOfDay::Second).unwrap(), 3417);

    let input = string_to_lines(
        "939
        67,7,59,61",
    );
    assert_eq!(run(&input, TaskOfDay::Second).unwrap(), 754018);

    let input = string_to_lines(
        "939
        67,x,7,59,61",
    );
    assert_eq!(run(&input, TaskOfDay::Second).unwrap(), 779210);

    let input = string_to_lines(
        "939
        67,7,x,59,61",
    );
    assert_eq!(run(&input, TaskOfDay::Second).unwrap(), 1261476);
}
