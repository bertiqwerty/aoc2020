//
// Task 2:
//      1.) Create a gap array, e.g.,
//          input:      (0), 1, 4, 5, 6, 7, 8, 9 (12)
//          gap array:     1, 3, 1, 1, 1, 1, 1, 3 
//      2.) Look in the gap-array for subsequent 1s and extract all chains of this kind.
//      3.) Count possible gap combinations per chain recursively, e.g., for 111 we have 111, 21, 12 and 3.
//      4.) Multiply all combinations of the different chains in the gap array.
//
use super::common::TaskOfDay;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter;

#[derive(Clone)]
struct Chain {
    // value is intepreted as number with basis 4 instead of 10 to make longer chains
    // possible with 128 bit.
    pub value: u128,
    pub len: usize,
}

impl Chain {
    fn at(&self, idx: usize) -> u128 {
        if idx >= self.len {
            panic!("Chain idx out of bounds.")
        }
        self.value / (4u128.pow(idx as u32)) - 4u128 * (self.value / (4u128.pow((idx + 1) as u32)))
    }
    fn set(&self, idx: usize, value: u128) -> Chain {
        if idx >= self.len {
            panic!("Chain idx out of bounds.")
        }
        let mut new_val: u128 = 0u128;
        for i in 0..self.len {
            new_val += (if i != idx { self.at(i) } else { value }) * 4u128.pow(i as u32);
        }
        Chain {
            value: new_val,
            len: self.len,
        }
    }

    fn sum_with_next(&self, idx: usize) -> Chain {
        // E.g., 111 -> 12 for index 0 and 21 for index 1
        let new_digit = self.at(idx) + self.at(idx + 1);
        if new_digit > 3 {
            panic!("Only digits until 3 are valid.")
        }
        if idx >= self.len - 1 {
            panic! {"sum_with_next is only available for all but the highest index. len {}, idx {}", self.len, idx}
        }
        let mut new_val = Chain {
            value: 0,
            len: self.len - 1,
        };
        for i in 0..self.len - 1 {
            let old_idx = if i < idx { i } else { i + 1 };
            let current_digit = if i != idx {
                self.at(old_idx)
            } else {
                new_digit
            };
            new_val = new_val.set(i, current_digit);
        }
        new_val
    }

    fn from_chain_len(chain_len: usize) -> Chain {
        Chain {
            value: { (0..chain_len).map(|i| 4u128.pow(i as u32)).sum() },
            len: chain_len,
        }
    }
}

fn compute_gaps(input: &Vec<usize>) -> Option<Vec<usize>> {
    let max_val = *input.iter().max()?;
    let iter_prefix = iter::once(0usize).chain(input.iter().map(|g| *g));
    let iter_suffix = input.iter().map(|g| *g).chain(iter::once(max_val + 3));
    Some(
        izip!(iter_prefix, iter_suffix)
            .map(|(i, j)| j - i)
            .collect::<Vec<usize>>(),
    )
}

fn count_gaps(gaps: &Vec<usize>) -> Option<(usize, usize)> {
    let ones = gaps.iter().filter(|elt| **elt == 1).count();
    let threes = gaps.iter().filter(|elt| **elt == 3).count();
    if ones + threes != gaps.len() {
        return None;
    }
    Some((ones, threes))
}

fn compute_chain_lens(gaps: &Vec<usize>) -> Vec<usize> {
    izip!(gaps.iter(), gaps[1..].iter())
        .enumerate()
        .filter(|(prv_idx, (prv, cur))| (**cur == 1 && **prv != 1) || (**prv == 1 && *prv_idx == 0))
        .map(|(prv_idx, _)| {
            let start_idx = if prv_idx == 0 { 0 } else { prv_idx + 1 };
            let mut chain_len = 0;
            while start_idx + chain_len < gaps.len() && gaps[start_idx + chain_len] == 1 {
                chain_len += 1;
            }
            chain_len
        })
        .filter(|clen| *clen > 1)
        .collect::<Vec<usize>>()
}

fn count_combinations_of_chain(chain: Chain, memoization: &mut HashMap<u128, usize>) -> usize {
    let mut duplicate_absorber: HashSet<u128> = HashSet::with_capacity(0);
    fn inner_compute(chain: Chain, dupla: &mut HashSet<u128>) {
        dupla.insert(chain.value);
        for i in 0..chain.len - 1 {
            if chain.at(i) + chain.at(i + 1) <= 3 {
                inner_compute(chain.sum_with_next(i), dupla)
            }
        }
    }

    if memoization.contains_key(&chain.value) {
        return memoization[&chain.value];
    } else {
        inner_compute(chain.clone(), &mut duplicate_absorber);
        memoization.insert(chain.value, duplicate_absorber.len());
        duplicate_absorber.len()
    }
}

fn count_combinations(gaps: &Vec<usize>) -> usize {
    let mut memoization: HashMap<u128, usize> = HashMap::with_capacity(0);

    compute_chain_lens(&gaps)
        .iter()
        .map(|cl| {
            count_combinations_of_chain(Chain::from_chain_len(*cl), &mut memoization)
        })
        .product()
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<usize> {
    let sort_input = || {
        let mut sor_in: Vec<usize> = input.iter().map(|s| s.parse::<usize>().unwrap()).collect();
        sor_in.sort();
        sor_in
    };
    let sorted_input = sort_input();
    let gaps = compute_gaps(&sorted_input)?;
    match part {
        TaskOfDay::First => {
            let res = count_gaps(&gaps);
            Some(res?.0 * res?.1)
        }
        TaskOfDay::Second => Some(count_combinations(&gaps)),
    }
}

#[test]
fn test() {
    let input: Vec<String> = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
    .split("\n")
    .map(|s| s.to_string())
    .collect();
    assert_eq!(run(&input, TaskOfDay::First).unwrap(), 220);

    assert_eq!(
        compute_chain_lens(&vec![1, 1, 3, 3, 1, 1, 1, 1]),
        vec![2, 4]
    );
    assert_eq!(compute_chain_lens(&vec![1, 1, 3, 3, 1, 1]), vec![2, 2]);
    assert_eq!(
        compute_chain_lens(&vec![1, 3, 3, 1, 1, 1, 1, 1, 3, 1]),
        vec![5]
    );
    assert_eq!(compute_chain_lens(&vec![1, 1, 1, 1, 1, 1, 1, 1]), vec![8]);
    assert_eq!(Chain { value: 57, len: 3 }.at(0), 1);
    assert_eq!(Chain { value: 57, len: 3 }.at(1), 2);
    assert_eq!(Chain { value: 57, len: 3 }.at(2), 3);
    assert_eq!(Chain { value: 57, len: 4 }.at(3), 0);
    assert_eq!(Chain { value: 57, len: 3 }.set(0, 3).at(0), 3);
    assert_eq!(Chain { value: 57, len: 3 }.set(1, 2).at(1), 2);
    assert_eq!(Chain { value: 57, len: 4 }.set(3, 1).at(3), 1);
    assert_eq!(Chain { value: 57, len: 5 }.set(4, 1).at(4), 1);
    assert_eq!(Chain { value: 56, len: 3 }.set(0, 1).value, 57);

    let swn = Chain { value: 21, len: 3 }.sum_with_next(0);
    assert_eq!(swn.len, 2);
    assert_eq!(swn.at(0), 2);
    assert_eq!(swn.at(1), 1);

    let swn = Chain { value: 21, len: 3 }.sum_with_next(1);
    assert_eq!(swn.len, 2);
    assert_eq!(swn.at(0), 1);
    assert_eq!(swn.at(1), 2);

    let swn = Chain { value: 57, len: 3 }.sum_with_next(0);
    assert_eq!(swn.len, 2);
    assert_eq!(swn.at(0), 3);
    assert_eq!(swn.at(1), 3);

    let swn = Chain { value: 341, len: 5 }.sum_with_next(0);
    assert_eq!(swn.len, 4);
    assert_eq!(swn.at(0), 2);
    assert_eq!(swn.at(1), 1);
    assert_eq!(swn.at(2), 1);
    assert_eq!(swn.at(3), 1);

    let swn = Chain { value: 341, len: 5 }.sum_with_next(1);
    assert_eq!(swn.len, 4);
    assert_eq!(swn.at(0), 1);
    assert_eq!(swn.at(1), 2);
    assert_eq!(swn.at(2), 1);
    assert_eq!(swn.at(3), 1);

    let swn = Chain { value: 341, len: 5 }.sum_with_next(2);
    assert_eq!(swn.len, 4);
    assert_eq!(swn.at(0), 1);
    assert_eq!(swn.at(1), 1);
    assert_eq!(swn.at(2), 2);
    assert_eq!(swn.at(3), 1);

    let swn = Chain { value: 341, len: 5 }.sum_with_next(3);
    assert_eq!(swn.len, 4);
    assert_eq!(swn.at(0), 1);
    assert_eq!(swn.at(1), 1);
    assert_eq!(swn.at(2), 1);
    assert_eq!(swn.at(3), 2);

    assert_eq!(Chain::from_chain_len(1).value, 1);
    assert_eq!(Chain::from_chain_len(2).value, 5);
    assert_eq!(Chain::from_chain_len(3).value, 21);
    assert_eq!(Chain::from_chain_len(4).value, 85);
    assert_eq!(Chain::from_chain_len(5).value, 341);

    let mut mem: HashMap<u128, usize> = HashMap::with_capacity(0);
    assert_eq!(count_combinations_of_chain(Chain::from_chain_len(3), &mut mem), 4);
    assert_eq!(count_combinations_of_chain(Chain::from_chain_len(4), &mut mem), 7);


    let input_2: Vec<String> = "16
10
15
5
1
11
7
19
6
12
4"
    .split("\n")
    .map(|s| s.to_string())
    .collect();
    assert_eq!(run(&input_2, TaskOfDay::Second).unwrap(), 8);
    assert_eq!(run(&input, TaskOfDay::Second).unwrap(), 19208);
}
