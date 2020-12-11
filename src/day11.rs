use super::common::TaskOfDay;
use std::collections::HashSet;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
struct Grid {
    rows: usize,
    cols: usize,
    data: Vec<u8>,
}

impl Index<usize> for Grid {
    type Output = [u8];

    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx * self.cols..(idx + 1) * self.cols]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx * self.cols..(idx + 1) * self.cols]
    }
}

impl Grid {
    fn from_lines(lines: &Vec<String>) -> Option<Grid> {
        let lengths = lines
            .iter()
            .map(|line| line.chars().count())
            .collect::<Vec<usize>>();
        let cols = lengths.iter().min()?;
        let rows = lines.len();
        if cols != lengths.iter().max()? {
            return None;
        }
        Some(Grid {
            cols: *cols,
            rows: rows,
            data: lines
                .iter()
                .map(|line| {
                    line.chars().map(|c| match c {
                        '.' => 0u8,
                        'L' => 1u8,
                        '#' => 2u8,
                        _ => panic!("Unknown character '{}'", c),
                    })
                })
                .flatten()
                .collect::<Vec<u8>>(),
        })
    }
}
#[derive(Clone)]
struct Hood {
    data: [u8; 8],
    len: usize,
}

impl IntoIterator for Hood {
    type Item = u8;
    type IntoIter = HoodIterator;
    fn into_iter(self) -> Self::IntoIter {
        HoodIterator {
            hood: self,
            index: 0,
        }
    }
}

#[derive(Clone)]
struct HoodIterator {
    hood: Hood,
    index: usize,
}
impl Iterator for HoodIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.hood.len {
            self.index += 1;
            return Some(self.hood.data[self.index - 1]);
        } else {
            None
        }
    }
}

impl Hood {
    fn create(grid: &Grid, row: usize, col: usize) -> Hood {
        let mut hood_len = 0;
        let mut data = [0; 8];
        for (r, c) in iproduct!([-1, 0, 1].iter(), [-1, 0, 1].iter()) {
            let ri = row as i32 + r;
            let ci = col as i32 + c;
            if ri >= 0
                && ci >= 0
                && ri < grid.rows as i32
                && ci < grid.cols as i32
                && (ri, ci) != (row as i32, col as i32)
            {
                data[hood_len] = grid[ri as usize][ci as usize];
                hood_len += 1;
            }
        }
        Hood {
            data: data,
            len: hood_len,
        }
    }
}

fn simulation_step(grid: &Grid) -> Grid {
    let mut new_grid = grid.clone();
    for (r, c) in iproduct!(0..grid.rows, 0..grid.cols) {
        let hood = Hood::create(&grid, r, c);
        let hood_iter = hood.into_iter();
        new_grid[r][c] = match grid[r][c] {
            0 => 0,
            1 => match hood_iter.clone().filter(|v| *v == 2).count() {
                0 => 2,
                _ => 1,
            },
            2 => match hood_iter.clone().filter(|v| *v == 2).count() {
                0 => 2,
                1 => 2,
                2 => 2,
                3 => 2,
                _ => 1,
            },
            _ => panic! {"Unknown grid value {}", grid[r][c]},
        };
    }
    new_grid
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<usize> {
    let grid = Grid::from_lines(input)?;
    match part {
        TaskOfDay::First => {
            let mut prev_grid = grid.clone();
            let mut new_grid = simulation_step(&grid);
            while new_grid.data != prev_grid.data {
                prev_grid = new_grid;
                new_grid = simulation_step(&prev_grid);
            }
            Some(new_grid.data.iter().filter(|v| *v == &2u8).count())
        }
        TaskOfDay::Second => Some(0usize),
    }
}

#[test]
fn test() {
    fn string_to_lines(s: &str) -> Vec<String> {
        s.split("\n").map(|s| s.trim().to_string()).collect()
    }

    let input = string_to_lines(
        "L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL",
    );

    let grid = Grid::from_lines(&input).unwrap();

    let hood = Hood::create(&grid, 9, 9);
    assert_eq!(hood.len, 3);
    assert!(hood.data.contains(&0u8) && hood.data.contains(&1u8));
    assert_eq!(hood.clone().into_iter().collect::<HashSet<u8>>().len(), 2);
    assert_eq!(hood.into_iter().collect::<Vec<u8>>().len(), 3);

    let hood = Hood::create(&grid, 4, 4);
    assert_eq!(hood.len, 8);
    assert!(hood.data.contains(&0u8) && hood.data.contains(&1u8));
    assert_eq!(hood.clone().into_iter().collect::<HashSet<u8>>().len(), 2);
    assert_eq!(hood.into_iter().collect::<Vec<u8>>().len(), 8);

    assert_eq!(grid[0][0], 1);
    assert_eq!(grid[0][1], 0);
    assert_eq!(grid[9][9], 1);
    assert_eq!(grid[3][4], 0);
    let grid_after_1 = simulation_step(&grid);
    let after_1_str = string_to_lines(
        "#.##.##.##
    #######.##
    #.#.#..#..
    ####.##.##
    #.##.##.##
    #.#####.##
    ..#.#.....
    ##########
    #.######.#
    #.#####.##",
    );
    let grid_after_1_ref = Grid::from_lines(&after_1_str).unwrap();
    assert_eq!(grid_after_1_ref.data, grid_after_1.data);

    let after_2_str = string_to_lines(
        "#.LL.L#.##
    #LLLLLL.L#
    L.L.L..L..
    #LLL.LL.L#
    #.LL.LL.LL
    #.LLLL#.##
    ..L.L.....
    #LLLLLLLL#
    #.LLLLLL.L
    #.#LLLL.##",
    );
    let grid_after_2 = simulation_step(&grid_after_1);
    let grid_after_2_ref = Grid::from_lines(&after_2_str).unwrap();
    assert_eq!(grid_after_2_ref.data, grid_after_2.data);

    let after_3_str = string_to_lines(
        "#.##.L#.##
    #L###LL.L#
    L.#.#..#..
    #L##.##.L#
    #.##.LL.LL
    #.###L#.##
    ..#.#.....
    #L######L#
    #.LL###L.L
    #.#L###.##",
    );
    let grid_after_3 = simulation_step(&grid_after_2);
    let grid_after_3_ref = Grid::from_lines(&after_3_str).unwrap();
    assert_eq!(grid_after_3_ref.data, grid_after_3.data);

    let after_4_str = string_to_lines(
        "#.#L.L#.##
    #LLL#LL.L#
    L.L.L..#..
    #LLL.##.L#
    #.LL.LL.LL
    #.LL#L#.##
    ..L.L.....
    #L#LLLL#L#
    #.LLLLLL.L
    #.#L#L#.##",
    );
    let grid_after_4 = simulation_step(&grid_after_3);
    let grid_after_4_ref = Grid::from_lines(&after_4_str).unwrap();
    assert_eq!(grid_after_4_ref.data, grid_after_4.data);

    let after_5_str = string_to_lines(
        "#.#L.L#.##
    #LLL#LL.L#
    L.#.L..#..
    #L##.##.L#
    #.#L.LL.LL
    #.#L#L#.##
    ..L.L.....
    #L#L##L#L#
    #.LLLLLL.L
    #.#L#L#.##",
    );
    let grid_after_5 = simulation_step(&grid_after_4);
    let grid_after_5_ref = Grid::from_lines(&after_5_str).unwrap();
    assert_eq!(grid_after_5_ref.data, grid_after_5.data);

    assert_eq!(run(&input, TaskOfDay::First).unwrap(), 37);
}
