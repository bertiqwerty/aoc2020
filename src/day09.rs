use super::common::TaskOfDay;

fn find_invalid(input: &Vec<usize>, preambel_len: usize) -> usize {
    let idx = (preambel_len..input.len())
        .find(|i| {
            let slice = &input[i - preambel_len..*i];
            iproduct!(slice.iter().clone(), slice.iter().clone())
                .filter(|(i, j)| i < j)
                .map(|(i, j)| i + j)
                .find(|sum| input[*i] == *sum)
                == None
        })
        .unwrap();
    input[idx]
}

fn find_contiguous_starting_at(input: &Vec<usize>, target: usize, start: usize) -> Option<usize> {
    let mut current_sum = 0usize;
    for i in start..input.len() - 1 {
        current_sum += input[i];
        if current_sum == target {
            return Some(i + 1);
        } else if current_sum > target {
            return None;
        }
    }
    None
}

fn find_contiguous(input: &Vec<usize>, target: usize) -> Option<(usize, usize)> {
    let mut end: Option<usize> = None;
    let start = (0..input.len()).find(|i| {
        end = find_contiguous_starting_at(&input, target, *i);
        end != None
    });
    Some((start?, end?))
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> usize {
    let input_usize = input
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let invalid_number = find_invalid(&input_usize, 25usize);
    match part {
        TaskOfDay::First => invalid_number,
        TaskOfDay::Second => {
            let (start, end) = find_contiguous(&input_usize, invalid_number).unwrap();
            input_usize[start..end].iter().max().unwrap() + input_usize[start..end].iter().min().unwrap()
        },
    }
}

#[test]
fn test() {
    let input = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    let invalid_number = find_invalid(&input, 5);
    assert_eq!(invalid_number, 127);
    assert_eq!(find_contiguous_starting_at(&input, 35, 0).unwrap(), 1);
    assert_eq!(find_contiguous_starting_at(&input, 120, 7).unwrap(), 9);
    assert_eq!(find_contiguous_starting_at(&input, 112, 3).unwrap(), 6);
    let (start, end) = find_contiguous(&input, 127).unwrap();
    assert_eq!(input[start], 15);
    assert_eq!(input[end - 1], 40);
    
}
