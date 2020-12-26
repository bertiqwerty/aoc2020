use super::common::TaskOfDay;
use super::grid::Grid;

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
    fn create(grid: &Grid<u8>, row: i32, col: i32) -> Hood {
        let mut hood_len = 0;
        let mut data = [0; 8];
        for (r, c) in iproduct!([-1, 0, 1].iter(), [-1, 0, 1].iter()) {
            let ri = row + r;
            let ci = col + c;
            if ri >= 0
                && ci >= 0
                && ri < grid.rows as i32
                && ci < grid.cols as i32
                && (ri, ci) != (row, col)
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

    fn create2(grid: &Grid<u8>, row: i32, col: i32) -> Hood {
        let (rows, cols) = (grid.rows as i32, grid.cols as i32);
        let mut hood_len = 0;
        let mut data = [0; 8];
        for (r, c) in iproduct!([-1, 0, 1].iter(), [-1, 0, 1].iter()) {
            let mut scale: i32 = 1;
            while (*r != 0 || *c != 0)
                && row + (r * scale) >= 0
                && col + (c * scale) >= 0
                && row + (r * scale) < rows
                && col + (c * scale) < cols
                && grid[(row + (r * scale)) as usize][(col + (c * scale)) as usize] == 0
            {
                scale += 1;
            }
            let ri = row + r * scale;
            let ci = col + c * scale;

            if ri >= 0
                && ci >= 0
                && ri < rows
                && ci < cols
                && (ri, ci) != (row, col)
                && grid[ri as usize][ci as usize] > 0
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

fn simulation_step(
    grid: &Grid<u8>,
    hood_creator: fn(&Grid<u8>, i32, i32) -> Hood,
    max_annoyance: usize,
) -> Grid<u8> {
    let mut new_grid = grid.clone();
    for (r, c) in iproduct!(0..grid.rows as i32, 0..grid.cols as i32) {
        let hood = hood_creator(&grid, r, c);
        let hood_iter = hood.into_iter();
        new_grid[r as usize][c as usize] = match grid[r as usize][c as usize] {
            0 => 0,
            1 => match hood_iter.clone().filter(|v| *v == 2).count() {
                0 => 2,
                _ => 1,
            },
            2 => {
                if hood_iter.clone().filter(|v| *v == 2).count() < max_annoyance {
                    2
                } else {
                    1
                }
            }
            _ => panic!("Unknown grid value {}", grid[r as usize][c as usize]),
        };
    }
    new_grid
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<usize> {
    let grid = Grid::from_lines(input)?;
    type HoodCreatorType = fn(&Grid<u8>, i32, i32) -> Hood;
    let first_create: HoodCreatorType = Hood::create;
    let second_create: HoodCreatorType = Hood::create2;
    let (hood_creator, max_annoyance) = match part {
        TaskOfDay::First => (first_create, 4),
        TaskOfDay::Second => (second_create, 5),
    };
    let mut prev_grid = grid.clone();
    let mut new_grid = simulation_step(&grid, hood_creator, max_annoyance);
    while new_grid.data != prev_grid.data {
        prev_grid = new_grid;
        new_grid = simulation_step(&prev_grid, hood_creator, max_annoyance);
    }
    Some(new_grid.data.iter().filter(|v| *v == &2u8).count())
}

#[test]
fn test() {
    use std::collections::HashSet;
    use super::common::string_to_lines;

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
    assert_eq!(run(&input, TaskOfDay::First).unwrap(), 37);

    let grid_after_1 = simulation_step(&grid, Hood::create2, 5);
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
    let grid_after_1_ref: Grid<u8> = Grid::from_lines(&after_1_str).unwrap();
    assert_eq!(grid_after_1_ref.data, grid_after_1.data);

    let after_2_str = string_to_lines(
        "#.LL.LL.L#
         #LLLLLL.LL
         L.L.L..L..
         LLLL.LL.LL
         L.LL.LL.LL
         L.LLLLL.LL
         ..L.L.....
         LLLLLLLLL#
         #.LLLLLL.L
         #.LLLLL.L#",
    );
    let grid_after_2 = simulation_step(&grid_after_1, Hood::create2, 5);
    let grid_after_2_ref: Grid<u8> = Grid::from_lines(&after_2_str).unwrap();
    assert_eq!(grid_after_2_ref.data, grid_after_2.data);

    let after_3_str = string_to_lines(
        "#.L#.##.L#
        #L#####.LL
        L.#.#..#..
        ##L#.##.##
        #.##.#L.##
        #.#####.#L
        ..#.#.....
        LLL####LL#
        #.L#####.L
        #.L####.L#",
    );
    let grid_after_3 = simulation_step(&grid_after_2, Hood::create2, 5);
    let grid_after_3_ref: Grid<u8> = Grid::from_lines(&after_3_str).unwrap();
    assert_eq!(grid_after_3_ref.data, grid_after_3.data);

    let after_4_str = string_to_lines(
        "#.L#.L#.L#
        #LLLLLL.LL
        L.L.L..#..
        ##LL.LL.L#
        L.LL.LL.L#
        #.LLLLL.LL
        ..L.L.....
        LLLLLLLLL#
        #.LLLLL#.L
        #.L#LL#.L#",
    );
    let grid_after_4 = simulation_step(&grid_after_3, Hood::create2, 5);
    let grid_after_4_ref: Grid<u8> = Grid::from_lines(&after_4_str).unwrap();
    assert_eq!(grid_after_4_ref.data, grid_after_4.data);

    let after_5_str = string_to_lines(
        "#.L#.L#.L#
        #LLLLLL.LL
        L.L.L..#..
        ##L#.#L.L#
        L.L#.#L.L#
        #.L####.LL
        ..#.#.....
        LLL###LLL#
        #.LLLLL#.L
        #.L#LL#.L#",
    );
    let grid_after_5 = simulation_step(&grid_after_4, Hood::create2, 5);
    let grid_after_5_ref: Grid<u8> = Grid::from_lines(&after_5_str).unwrap();
    assert_eq!(grid_after_5_ref.data, grid_after_5.data);
    assert_eq!(run(&input, TaskOfDay::Second).unwrap(), 26);
}
