pub fn day1(input: &Vec<String>) -> i32 {
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