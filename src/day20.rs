use super::common::separate_by_blanks;
use super::common::AxisIterator;
use super::common::Grid;

use super::common::string_to_lines;
use super::common::TaskOfDay;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref NUMERIC: Regex = Regex::new(r"[0-9]+").unwrap();
}

#[derive(Clone, Copy)]
enum Dir {
    E,
    W,
    N,
    S,
}

impl Dir {
    fn invert(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }
}

static ALL_DIRS: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];

fn resolve_orientation<T>(orientation: Dir, dir: Dir, results: (T, T, T, T)) -> T {
    match (orientation, dir) {
        (Dir::N, Dir::N) | (Dir::S, Dir::S) | (Dir::E, Dir::E) | (Dir::W, Dir::W) => results.0,
        (Dir::N, Dir::E) | (Dir::E, Dir::N) | (Dir::S, Dir::W) | (Dir::W, Dir::S) => results.1,
        (Dir::N, Dir::S) | (Dir::S, Dir::N) | (Dir::E, Dir::W) | (Dir::W, Dir::E) => results.2,
        (Dir::N, Dir::W) | (Dir::W, Dir::N) | (Dir::S, Dir::E) | (Dir::E, Dir::S) => results.3,
    }
}

#[derive(Clone)]
struct Node {
    id: i32,
    grid: Grid<u8>,
    n: Option<i32>,
    e: Option<i32>,
    s: Option<i32>,
    w: Option<i32>,
}

impl Node {
    pub fn get_neighbor(&self, my_ori: Dir, dir: Dir) -> Option<i32> {
        resolve_orientation::<Option<i32>>(my_ori, dir, (self.n, self.e, self.s, self.w))
    }
    pub fn set_neighbor(&mut self, my_ori: Dir, dir: Dir, n_id: i32){
        let raw_neighbor_dir = resolve_orientation::<Dir>(my_ori, dir, (Dir::N, Dir::E, Dir::S, Dir::W));
        match raw_neighbor_dir {
            Dir::N => self.n = Some(n_id),
            Dir::E => self.e = Some(n_id),
            Dir::S => self.s = Some(n_id),
            Dir::W => self.w = Some(n_id),
        }
    }
    pub fn from_grid(id: i32, grid: Grid<u8>) -> Node {
        Node {
            id: id,
            grid: grid,
            n: None,
            e: None,
            s: None,
            w: None,
        }
    }

    fn get_axis_iter(&self, orientation: Dir, dir: Dir) -> (AxisIterator<u8>, AxisIterator<u8>) {
        let res = (
            (
                // north
                AxisIterator::make_row_forward(0, &self.grid),
                AxisIterator::make_row_backward(0, &self.grid),
            ),
            (
                // east
                AxisIterator::make_col_forward(self.grid.cols - 1, &self.grid),
                AxisIterator::make_col_backward(self.grid.cols - 1, &self.grid),
            ),
            (
                // south
                AxisIterator::make_row_forward(self.grid.rows - 1, &self.grid),
                AxisIterator::make_row_backward(self.grid.rows - 1, &self.grid),
            ),
            (
                // west
                AxisIterator::make_col_forward(0, &self.grid),
                AxisIterator::make_col_backward(0, &self.grid),
            ),
        );
        resolve_orientation::<(AxisIterator<u8>, AxisIterator<u8>)>(orientation, dir, res)
    }
}

fn str_to_id_grid(s: &str) -> Option<(i32, Grid<u8>)> {
    let lines = string_to_lines(s);
    let id = NUMERIC.captures(&lines[0]).unwrap()[0]
        .parse::<i32>()
        .unwrap();
    let grid = Grid::from_lines(&lines[1..])?;
    Some((id, grid))
}

fn match_axis(iter1: &AxisIterator<u8>, iter2: &AxisIterator<u8>) -> bool {
    return izip!(iter1.clone(), iter2.clone()).all(|(i1, i2)| i1 == i2);
}

fn match_node(node1: &Node, node2: &Node, node1_dir: Dir, node1_ori: Dir) -> Option<Dir> {
    // returns orientation
    let iters1 = node1.get_axis_iter(node1_ori, node1_dir);

    for node_2_ori in ALL_DIRS.iter() {
        let iters2 = node2.get_axis_iter(*node_2_ori, node1_dir.invert());
        if match_axis(&iters1.0, &iters2.0) || match_axis(&iters1.0, &iters2.1) {
            return Some(*node_2_ori);
        }
    }
    None
}


pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<u64> {
    let input_grids = separate_by_blanks(&input, "\n");
    let mut nodes = input_grids
        .iter()
        .map(|s| {
            let (id, grid) = str_to_id_grid(s).unwrap();
            (id, (Node::from_grid(id, grid), Dir::N))
        })
        .collect::<HashMap<i32, (Node, Dir)>>();

    let mut floatings = nodes.keys().map(|id| *id).collect::<Vec<i32>>();
    let mut fixeds: Vec<i32> = Vec::with_capacity(0);

    fixeds.push(floatings.pop()?);
    while fixeds.len() < floatings.len() {
        let mut to_be_moved:Vec<i32> = Vec::with_capacity(0);
        for (flo, fix) in iproduct!(&floatings, &fixeds) {
            let relevant_flos = ALL_DIRS
                .iter()
                .filter(|dir| nodes[fix].0.get_neighbor(nodes[fix].1, **dir).is_none())
                .map(|n1_dir| {
                    (
                        n1_dir,
                        match_node(&nodes[fix].0, &nodes[flo].0, *n1_dir, nodes[fix].1),
                    )
                })
                .find(|(_, n2_ori)| n2_ori.is_some());

            for (n1_dir, n2_ori) in relevant_flos {
                let n1_ori = nodes[fix].1;
                let n2_dir = n1_dir.invert();
                nodes.get_mut(fix)?.0.set_neighbor(n1_ori, *n1_dir,*flo);
                nodes.get_mut(flo)?.0.set_neighbor(n2_ori?, n2_dir,*fix);
                to_be_moved.push(*flo);
                
            }
        }
        for item in to_be_moved {
            floatings.retain(|i| *i != item);
            fixeds.push(item);
        }
    }


    Some(0u64)
}

#[test]
fn test() {
    let input = string_to_lines(
        "Tile 2311:
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###
        
        Tile 1951:
        #.##...##.
        #.####...#
        .....#..##
        #...######
        .##.#....#
        .###.#####
        ###.##.##.
        .###....#.
        ..#.#..#.#
        #...##.#..
        
        Tile 1171:
        ####...##.
        #..##.#..#
        ##.#..#.#.
        .###.####.
        ..###.####
        .##....##.
        .#...####.
        #.##.####.
        ####..#...
        .....##...
        
        Tile 1427:
        ###.##.#..
        .#..#.##..
        .#.##.#..#
        #.#.#.##.#
        ....#...##
        ...##..##.
        ...#.#####
        .#.####.#.
        ..#..###.#
        ..##.#..#.
        
        Tile 1489:
        ##.#.#....
        ..##...#..
        .##..##...
        ..#...#...
        #####...#.
        #..#.#.#.#
        ...#.#.#..
        ##.#...##.
        ..##.##.##
        ###.##.#..
        
        Tile 2473:
        #....####.
        #..#.##...
        #.##..#...
        ######.#.#
        .#...#.#.#
        .#########
        .###.#..#.
        ########.#
        ##...##.#.
        ..###.#.#.
        
        Tile 2971:
        ..#.#....#
        #...###...
        #.#.###...
        ##.##..#..
        .#####..##
        .#..####.#
        #..#.#..#.
        ..####.###
        ..#.#.###.
        ...#.#.#.#
        
        Tile 2729:
        ...#.#.#.#
        ####.#....
        ..#.#.....
        ....#..#.#
        .##..##.#.
        .#.####...
        ####.#.#..
        ##.####...
        ##..#.##..
        #.##...##.
        
        Tile 3079:
        #.#.#####.
        .#..######
        ..#.......
        ######....
        ####.#..#.
        .#...#.##.
        #.#####.##
        ..#.###...
        ..#.......
        ..#.###...",
    );

    let input_grids = separate_by_blanks(&input, "\n");
    let grids = input_grids
        .iter()
        .map(|s| str_to_id_grid(s).unwrap())
        .collect::<Vec<(i32, Grid<u8>)>>();

    assert_eq!(grids[0].0, 2311);
    let grid = &grids[0usize].1;
    assert_eq!(grid[0usize][0usize], 0);
}
