use super::common::TaskOfDay;

fn sum_of_steps(tree_matrix: &Vec<Vec<i32>>, row_step: usize, col_step: usize) -> i32 {
    let mut sum = 0i32;
    for (row, col) in izip!(
        (row_step..tree_matrix.len()).step_by(row_step),
        (col_step..tree_matrix.len() * col_step).step_by(col_step)
    ) {
        sum += tree_matrix[row].iter().cycle().nth(col).unwrap();
    }
    sum
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> i64 {
    let tree_matrix = input
        .iter()
        .map(|s| s.chars().map(|c| (c == '#') as i32).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();
    match part {
        TaskOfDay::First => sum_of_steps(&tree_matrix, 1, 3) as i64,
        TaskOfDay::Second => izip!(
            vec!(1usize, 1usize, 1usize, 1usize, 2usize).iter(),
            vec!(1usize, 3usize, 5usize, 7usize, 1usize).iter()
        )
        .map(|(r, c)| sum_of_steps(&tree_matrix, *r, *c) as i64)
        .product(),
    }
}