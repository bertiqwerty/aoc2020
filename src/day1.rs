use super::common::TaskOfDay;

pub fn run(input: &Vec<String>, part: TaskOfDay) -> i32 {
    let converted = input
        .iter()
        .map(|s| s.parse::<i32>().expect("could not parse string to int {}"));
    match part {
        TaskOfDay::First => {
            let t = iproduct!(converted.clone(), converted.clone())
                .find(|(i, j)| i + j == 2020)
                .unwrap();
            t.0 * t.1
        }
        TaskOfDay::Second => {
            let t = iproduct!(
                iproduct!(converted.clone(), converted.clone()).filter(|&(i, j)| i + j <= 2020),
                converted
            )
            .find(|&(t, k)| t.0 + t.1 + k == 2020)
            .unwrap();
            t.0 .0 * t.0 .1 * t.1
        }
    }
}
