use super::common::TaskOfDay;

#[derive(Clone)]
struct State {
    pos: (i32, i32),
    facing: Cardir,
}

impl State {
    fn move_forward(&self, steps: i32) -> State {
        State {
            pos: move_direction(self.pos, self.facing.clone(), steps),
            facing: self.facing.clone(),
        }
    }

    fn turn_left(&self, angle: i32) -> State {
        State {
            pos: self.pos,
            facing: self.facing.turn_left(angle),
        }
    }
    fn turn_right(&self, angle: i32) -> State {
        State {
            pos: self.pos,
            facing: self.facing.turn_right(angle),
        }
    }
    fn from_pos(&self, pos: (i32, i32)) -> State {
        State {pos: pos, facing: self.facing}
    }
}

fn move_direction(pos: (i32, i32), dir: Cardir, steps: i32) -> (i32, i32) {
    match dir {
        Cardir::E => (pos.0 + steps, pos.1),
        Cardir::W => (pos.0 - steps, pos.1),
        Cardir::N => (pos.0, pos.1 + steps),
        Cardir::S => (pos.0, pos.1 - steps),
    }
}

#[derive(Clone, Copy)]
enum Cardir {
    N,
    S,
    E,
    W,
}
fn match_angle(angle: i32, ninety: Cardir, one_eighty: Cardir, two_seventy: Cardir) -> Cardir {
    match angle {
        90 => ninety,
        180 => one_eighty,
        270 => two_seventy,
        _ => panic!("Unknown angle {}", angle),
    }
}

impl Cardir {
    fn turn_left(self, angle: i32) -> Cardir {
        match self {
            Cardir::E => match_angle(angle, Cardir::N, Cardir::W, Cardir::S),
            Cardir::N => match_angle(angle, Cardir::W, Cardir::S, Cardir::E),
            Cardir::W => match_angle(angle, Cardir::S, Cardir::E, Cardir::N),
            Cardir::S => match_angle(angle, Cardir::E, Cardir::N, Cardir::W),
        }
    }
    fn turn_right(self, angle: i32) -> Cardir {
        match self {
            Cardir::E => match_angle(angle, Cardir::S, Cardir::W, Cardir::N),
            Cardir::N => match_angle(angle, Cardir::E, Cardir::S, Cardir::W),
            Cardir::W => match_angle(angle, Cardir::N, Cardir::E, Cardir::S),
            Cardir::S => match_angle(angle, Cardir::W, Cardir::N, Cardir::E),
        }
    }
}

fn rotate_waypoint_r(wp_pos: (i32, i32), angle: i32) -> (i32, i32) {
    return match angle {
        90 => (wp_pos.1, -wp_pos.0),
        180 => (-wp_pos.0, -wp_pos.1),
        270 => (-wp_pos.1, wp_pos.0),
        _ => panic!("Unknown angle {}", angle),
    };
}
fn rotate_waypoint_l(wp_pos: (i32, i32), angle: i32) -> (i32, i32) {
    return match angle {
        90 => (-wp_pos.1, wp_pos.0),
        180 => (-wp_pos.0, -wp_pos.1),
        270 => (wp_pos.1, -wp_pos.0),
        _ => panic!("Unknown angle {}", angle),
    };
}

fn move_ship_in_wp_dir(ship_pos: (i32, i32), wp_pos: (i32, i32), steps: i32) -> (i32, i32) {
    (ship_pos.0 + steps * wp_pos.0, ship_pos.1 + steps * wp_pos.1)
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
    let step_iter = lines
        .iter()
        .map(|line| line[1..].parse::<i32>().expect("Could not parse line."));
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
            Action::N => state.from_pos(move_direction(state.pos, Cardir::N, a.1)),
            Action::E => state.from_pos(move_direction(state.pos, Cardir::E, a.1)),
            Action::W => state.from_pos(move_direction(state.pos, Cardir::W, a.1)),
            Action::S => state.from_pos(move_direction(state.pos, Cardir::S, a.1)),
        }
    }
    state.pos
}

fn compute_final_position2(actions: &Vec<(Action, i32)>) -> (i32, i32) {
    let mut wp_pos = (10, 1);
      
    let mut ship_pos = (0, 0);
    for a in actions {
        wp_pos = match a.0 {
            Action::L => rotate_waypoint_l(wp_pos, a.1),
            Action::R => rotate_waypoint_r(wp_pos, a.1),
            Action::F => wp_pos,
            Action::N => move_direction(wp_pos, Cardir::N, a.1),
            Action::E => move_direction(wp_pos, Cardir::E, a.1),
            Action::W => move_direction(wp_pos, Cardir::W, a.1),
            Action::S => move_direction(wp_pos, Cardir::S, a.1),
        };
        ship_pos = match a.0 {
            Action::F => move_ship_in_wp_dir(ship_pos, wp_pos, a.1),
            _ => ship_pos,
        };
    }
    ship_pos
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<i32> {
    let actions = convert_lines(input);
    type PosComputer = fn(&Vec<(Action, i32)>) -> (i32, i32);
    let pos_computer: PosComputer = match part {
        TaskOfDay::First => compute_final_position,
        TaskOfDay::Second => compute_final_position2,
    };
    let pos = pos_computer(&actions);
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
    assert!(Action::F != Action::E);
    assert_eq!(run(&input, TaskOfDay::First).unwrap(), 25);
    assert_eq!(run(&input, TaskOfDay::Second).unwrap(), 286);

    assert_eq!(move_direction((0, 0), Cardir::E, 1), (1, 0));
    assert_eq!(move_direction((0, 0), Cardir::N, 1), (0, 1));
    assert_eq!(move_direction((0, 0), Cardir::W, 1), (-1, 0));
    assert_eq!(move_direction((0, 0), Cardir::S, 1), (0, -1));
    assert_eq!(rotate_waypoint_l((1, 0), 90), (0, 1));
    assert_eq!(rotate_waypoint_l((1, 0), 180), (-1, 0));
    assert_eq!(rotate_waypoint_l((1, 0), 270), (0, -1));
    assert_eq!(rotate_waypoint_r((1, 0), 90), (0, -1));
    assert_eq!(rotate_waypoint_r((1, 0), 180), (-1, 0));
    assert_eq!(rotate_waypoint_r((1, 0), 270), (0, 1));

    
}
