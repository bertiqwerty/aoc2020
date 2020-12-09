use super::common::split_in2_tuple;
use super::common::TaskOfDay;
use std::collections::HashSet;

enum Op {
    Jmp,
    Nop,
    Acc,
}

fn str_2_op(s: &str) -> Op {
    match s {
        "jmp" => Op::Jmp,
        "nop" => Op::Nop,
        "acc" => Op::Acc,
        _ => panic!("Unknown op {}", s),
    }
}

fn map_instructions(input: &Vec<String>) -> Vec<(Op, i32)> {
    input
        .iter()
        .map(|s| {
            let (operation, num_str) = split_in2_tuple(s, " ");
            (str_2_op(operation), num_str.parse::<i32>().unwrap())
        })
        .collect::<Vec<(Op, i32)>>()
}

fn accumulator_at_loop_or_termination(operations: &Vec<(Op, i32)>) -> (i32, bool) {
    let mut set: HashSet<usize> = HashSet::with_capacity(0);
    let mut accumulator = 0i32;
    let mut idx = 0usize;
    while !set.contains(&idx) && idx < operations.len() {
        set.insert(idx);
        match operations[idx].0 {
            Op::Acc => {
                accumulator += operations[idx].1;
                idx += 1usize;
            }
            Op::Nop => {
                idx += 1usize;
            }
            Op::Jmp => {
                let tmp = idx as i32 + operations[idx].1;
                idx = tmp as usize;
            }
        }
    }
    (accumulator, idx < operations.len())
}

fn swap(op: &Op) -> Op {
    match op {
        Op::Jmp => Op::Nop,
        Op::Nop => Op::Jmp,
        Op::Acc => panic!("Acc is not swapable."),
    }
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> i32 {
    let mut instructions = map_instructions(input);

    match part {
        TaskOfDay::First => accumulator_at_loop_or_termination(&instructions).0,
        TaskOfDay::Second => {
            let swap_candidates = instructions
                .iter()
                .enumerate()
                .filter(|(_, (op, _))| match op {
                    Op::Jmp => true,
                    Op::Nop => true,
                    Op::Acc => false,
                })
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();

            for i in swap_candidates {
                instructions[i].0 = swap(&instructions[i].0);

                let (acc, looping) = accumulator_at_loop_or_termination(&instructions);
                if looping {
                    // undo swap and try next
                    instructions[i].0 = swap(&instructions[i].0);
                } else {
                    return acc;
                }
            }
            panic!("Could not repair operations.");
        }
    }
}

#[test]
fn test() {
    fn op_to_num(op: &Op) -> i32 {
        match op {
            Op::Acc => 0i32,
            Op::Nop => 1i32,
            Op::Jmp => 2i32,
        }
    }
    let ref_instructions = vec![
        (Op::Nop, 0i32),
        (Op::Acc, 1i32),
        (Op::Jmp, 4i32),
        (Op::Acc, 3i32),
        (Op::Jmp, -3i32),
        (Op::Acc, -99i32),
        (Op::Acc, 1i32),
        (Op::Jmp, -4i32),
        (Op::Acc, 6i32),
    ];

    let input_str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let input: Vec<String> = input_str.split("\n").map(|s| s.to_string()).collect();
    for (ref_ins, ins) in izip!(ref_instructions.iter(), map_instructions(&input).iter()) {
        assert_eq!(op_to_num(&ref_ins.0), op_to_num(&ins.0));
        assert_eq!(ref_ins.1, ins.1);
    }
    assert_eq!(run(&input, TaskOfDay::First), 5);
    assert_eq!(run(&input, TaskOfDay::Second), 8);
}
