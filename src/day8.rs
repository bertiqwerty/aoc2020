use super::common::split_in2_tuple;
use super::common::TaskOfDay;
use std::collections::HashSet;

fn map_instructions(input: &Vec<String>) -> Vec<(&str, i32)> {
    input
        .iter()
        .map(|s| {
            let (instruction, num_str) = split_in2_tuple(s, " ");
            (instruction, num_str.parse::<i32>().unwrap())
        })
        .collect::<Vec<(&str, i32)>>()
}

fn accumulator_at_loop_or_termination(instructions: &Vec<(&str, i32)>) -> (i32, bool) {
    let mut set: HashSet<usize> = HashSet::with_capacity(0);
    let mut accumulator = 0i32;
    let mut idx = 0usize;
    while !set.contains(&idx) && idx < instructions.len() {
        set.insert(idx);
        match instructions[idx].0 {
            "acc" => {
                accumulator += instructions[idx].1;
                idx += 1usize;
            }
            "nop" => {
                idx += 1usize;
            }
            "jmp" => {
                let tmp = idx as i32 + instructions[idx].1;
                idx = tmp as usize;
            }
            _ => {
                panic!("Unknown instruction {}.", instructions[idx].0);
            }
        }
    }
    (accumulator, idx < instructions.len())
}

fn swap(ins: &str) -> &str {
    if ins == "nop" {
        "jmp"
    } else if ins == "jmp" {
        "nop"
    } else {
        panic!("AAAAAAAAAHH!");
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
                .filter(|(_, (s, _))| s == &"nop" || s == &"jmp")
                .map(|(i, _)| i).collect::<Vec<usize>>();

            for i in swap_candidates {
                instructions[i].0 = swap(instructions[i].0);

                let (acc, looping) = accumulator_at_loop_or_termination(&instructions);
                if looping {
                    // undo swap and try next
                    instructions[i].0 = swap(instructions[i].0);   
                }
                else {
                    return acc;
                }
            }
            panic!("Could not repair instructions.");
        }
    }
}

#[test]
fn test() {
    let ref_instructions = vec![
        ("nop", 0i32),
        ("acc", 1i32),
        ("jmp", 4i32),
        ("acc", 3i32),
        ("jmp", -3i32),
        ("acc", -99i32),
        ("acc", 1i32),
        ("jmp", -4i32),
        ("acc", 6i32),
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
        assert_eq!(ref_ins.0, ins.0);
        assert_eq!(ref_ins.1, ins.1);
    }
    assert_eq!(run(&input, TaskOfDay::First), 5);
    assert_eq!(run(&input, TaskOfDay::Second), 8);
}
