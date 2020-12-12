use super::common::TaskOfDay;

#[derive(Clone)]
struct State {
    pos: (i32, i32),
    facing: Cardir,
}




impl State {
    fn move_forward(&self, steps: i32) -> State {
        self.move_direction(self.facing.clone(), steps)
    }
    
    fn move_direction(&self, dir: Cardir, steps: i32) -> State {
        match dir {
            Cardir::E => State {
                pos: (self.pos.0 + steps, self.pos.1),
                facing: self.facing.clone(),
            },
            Cardir::W => State {
                pos: (self.pos.0 - steps, self.pos.1),
                facing: self.facing.clone(),
            },
            Cardir::N => State {
                pos: (self.pos.0, self.pos.1 + steps),
                facing: self.facing.clone(),
            },
            Cardir::S => State {
                pos: (self.pos.0, self.pos.1 - steps),
                facing: self.facing.clone(),
            },
        }
    }

    fn turn_left(&self, angle: i32) -> State {
        State {
                pos: self.pos,
                facing: self.facing.turn_left(angle)
            }

    }
    fn turn_right(&self, angle: i32) -> State {
        State {
                pos: self.pos,
                facing: self.facing.turn_right(angle)
            }

    }
    
}

#[derive(Clone, Copy)]
enum Cardir {
    N,
    S,
    E,
    W,
}
fn match_angle(angle: i32, ninety: Cardir, one_eighty:Cardir, two_seventy: Cardir) -> Cardir{
    match angle {
        90 => ninety,
        180 => one_eighty,
        270 => two_seventy,
        _ => panic!("Unknown angle {}", angle),
    }
}

static DIRECTIONS: [Cardir;4] = [Cardir::E, Cardir::N, Cardir::W, Cardir::S]; 

impl Cardir {
    fn turn_left(self, angle: i32) -> Cardir {
        match self {
            Cardir::E => {
                match_angle(angle, Cardir::N, Cardir::W, Cardir::S)
            }
            Cardir::N => {
                match_angle(angle, Cardir::W, Cardir::S, Cardir::E)
            }
            Cardir::W => {
                match_angle(angle, Cardir::S, Cardir::E, Cardir::N)
            }
            Cardir::S => {
                match_angle(angle, Cardir::E, Cardir::N, Cardir::W)
            }
        }
    }
    fn turn_right(self, angle: i32) -> Cardir {
        match self {
            Cardir::E => {
                match_angle(angle, Cardir::S, Cardir::W, Cardir::N)
            }
            Cardir::N => {
                match_angle(angle, Cardir::E, Cardir::S, Cardir::W)
            }
            Cardir::W => {
                match_angle(angle, Cardir::N, Cardir::E, Cardir::S)
            }
            Cardir::S => {
                match_angle(angle, Cardir::W, Cardir::N, Cardir::E)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Action {
    F,
    N,
    S,
    E,
    W,
    L,
    R,
}


fn convert_lines(lines: &Vec<String>) -> Vec<(Action, i32)> {
    let action_iter = lines.iter().map(|line| match line.chars().next().unwrap() {
        'F' => Action::F,
        'N' => Action::N,
        'S' => Action::S,
        'E' => Action::E,
        'W' => Action::W,
        'L' => Action::L,
        'R' => Action::R,
        _ => panic!("Unknown action string '{}'", line),
    });
    let step_iter = lines.iter().map(|line| {
        line[1..]
            .parse::<i32>().expect("Could not parse line.")
    });
    izip!(action_iter, step_iter).collect::<Vec<(Action, i32)>>()
}

fn compute_final_position(actions: &Vec<(Action, i32)>) -> (i32, i32) {
    let mut state = State {
        pos: (0, 0),
        facing: Cardir::E,
    };
    for a in actions {
        state = match a.0 {
            Action::L => state.turn_left(a.1),
            Action::R => state.turn_right(a.1),
            Action::F => state.move_forward(a.1),
            Action::N => state.move_direction(Cardir::N, a.1),
            Action::E => state.move_direction(Cardir::E, a.1),
            Action::W => state.move_direction(Cardir::W, a.1),
            Action::S => state.move_direction(Cardir::S, a.1),
        }
    }
    state.pos
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<i32> {
    let actions = convert_lines(input);
    let pos = compute_final_position(&actions);
    Some(pos.0.abs() + pos.1.abs())
}

#[test]
fn test() {
    use super::common::string_to_lines;

    let input = string_to_lines(
        "F10
N3
F7
R90
F11",
    );
    assert_eq!(
        convert_lines(&input),
        vec![
            (Action::F, 10),
            (Action::N, 3),
            (Action::F, 7),
            (Action::R, 90),
            (Action::F, 11)
        ]
    );
    assert!(Action::F!=Action::E);
    assert_eq!(run(&input, TaskOfDay::First).unwrap(), 25);
}
