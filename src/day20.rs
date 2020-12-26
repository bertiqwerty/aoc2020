use super::common::separate_by_blanks;
use super::grid::AxisIterator;
use super::grid::Grid;

use super::common::string_to_lines;
use super::common::TaskOfDay;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref NUMERIC: Regex = Regex::new(r"[0-9]+").unwrap();
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Debug)]
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
    pub fn set_neighbor(&mut self, my_ori: Dir, dir: Dir, n_id: i32) {
        let raw_neighbor_dir =
            resolve_orientation::<Dir>(my_ori, dir, (Dir::N, Dir::E, Dir::S, Dir::W));
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

    fn get_axis_iter(&self, orientation: Dir, dir: Dir) -> AxisIterator<u8> {
        let res = (
            // north
            AxisIterator::make_row(0, &self.grid),
            // east
            AxisIterator::make_col(self.grid.cols - 1, &self.grid),
            // south
            AxisIterator::make_row(self.grid.rows - 1, &self.grid),
            // west
            AxisIterator::make_col(0, &self.grid),
        );
        resolve_orientation::<AxisIterator<u8>>(orientation, dir, res)
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

fn match_axis(iter1: &AxisIterator<u8>, iter2: &AxisIterator<u8>) -> Option<bool> {
    if izip!(iter1.clone(), iter2.clone()).all(|(i1, i2)| i1 == i2){
        Some(false)
    }
    else if izip!(iter1.clone().rev(), iter2.clone()).all(|(i1, i2)| i1 == i2) {
        Some(true)
    }
    else {
        None
    }
}

fn match_node(node1: &Node, node2: &Node, node1_dir: Dir, node1_ori: Dir) -> Option<Geometry> {
    // returns orientation
    let iters1 = node1.get_axis_iter(node1_ori, node1_dir);

    for node_2_ori in ALL_DIRS.iter() {
        let iters2 = node2.get_axis_iter(*node_2_ori, node1_dir.invert());
        let flipped = match_axis(&iters1, &iters2);
        if flipped.is_some() {
            return Some(Geometry{ori: *node_2_ori, flipped: flipped?});
        }
    }
    None
}

fn get_corners(nodes: &HashMap<i32, (Node, Geometry)>) -> Vec<i32> {
    nodes.iter().filter(|(id, (node, _))| {
        let border_sides = node.n.is_none() as u8
            + node.e.is_none() as u8
            + node.s.is_none() as u8
            + node.w.is_none() as u8;
        if border_sides > 2 {
            panic!("More than 2 border sides are not possible. Found {} for id {}.", border_sides, id);
        }
        border_sides == 2      
    }).map(|(id, _)| *id).collect::<Vec<i32>>()
}
#[derive(Clone, Copy)]
struct Geometry {
    flipped: bool,
    ori: Dir,
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<u64> {
    let input_grids = separate_by_blanks(&input, "\n");
    let mut nodes = input_grids
        .iter()
        .map(|s| {
            let (id, grid) = str_to_id_grid(s).unwrap();
            (id, (Node::from_grid(id, grid),  Geometry {flipped: false, ori: Dir::N}))
        })
        .collect::<HashMap<i32, (Node, Geometry)>>();

    let mut floatings = nodes.keys().map(|id| *id).collect::<Vec<i32>>();
    let mut fixeds: Vec<i32> = Vec::with_capacity(0);

    fixeds.push(floatings.pop()?);
    while floatings.len() > 0 {
        let mut to_be_moved: Vec<i32> = Vec::with_capacity(0);
        for (flo, fix) in iproduct!(&floatings, &fixeds) {
            let relevant_ori = ALL_DIRS
                .iter()
                .filter(|dir| nodes[fix].0.get_neighbor(nodes[fix].1.ori, **dir).is_none())
                .map(|n1_dir| {
                    (
                        n1_dir,
                        match_node(&nodes[fix].0, &nodes[flo].0, *n1_dir, nodes[fix].1.ori),
                    )
                })
                .find(|(_, n2_ori)| n2_ori.is_some());

            for (n1_dir, n2_ori) in relevant_ori {
                let n1_ori = nodes[fix].1.ori;
                let n2_dir = n1_dir.invert();
                nodes.get_mut(fix)?.0.set_neighbor(n1_ori, *n1_dir, *flo);
                nodes.get_mut(flo)?.0.set_neighbor(n2_ori?.ori, n2_dir, *fix);
                nodes.get_mut(flo)?.1 = n2_ori?;
                to_be_moved.push(*flo);
            }
        }
        for item in to_be_moved {
            floatings.retain(|i| *i != item);
            fixeds.push(item);
        }
    }
    let corners = get_corners(&nodes);
    if corners.clone().len() != 4 {
        for corner in &corners {
            println!("orientation {:?}", nodes[corner].1.ori);
            println!("{:?}", &nodes[corner].0);
        }
        panic!("We have exactly 4 corners not {}.", corners.clone().len())
    }

    Some(corners.iter().map(|id| *id as u64).product())
}


#[test]
fn test_day_20() {
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
        ..#.###...
        ",
    );

    let input_grids = separate_by_blanks(&input, "\n");
    let grids = input_grids
        .iter()
        .map(|s| str_to_id_grid(s).unwrap())
        .collect::<Vec<(i32, Grid<u8>)>>();

    assert_eq!(grids[0].0, 2311);
    let grid = &grids[0usize].1;
    assert_eq!(grid[0usize][0usize], 0);

    // 4 south matches 3 north
    let it4north: AxisIterator<u8> = AxisIterator::make_row(grids[4].1.rows - 1, &grids[4].1);
    let it3south: AxisIterator<u8> = AxisIterator::make_row(0, &grids[3].1);
    assert!(match_axis(&it4north, &it3south).is_some());
    let it31: AxisIterator<u8> = AxisIterator::make_row(1, &grids[3].1);
    assert!(!match_axis(&it4north, &it31).is_some());

    let grid_1489 = grids.iter().find(|(i, _)| *i == 1489).unwrap();
    let grid_1951 = grids.iter().find(|(i, _)| *i == 1951).unwrap();
    let node_1489 = Node::from_grid(grid_1489.0, grid_1489.1.clone());
    println!("{:#?}", node_1489);
    let node_1951 = Node::from_grid(grid_1951.0, grid_1951.1.clone());
    let ori_n_dir_w_1489= node_1489.get_axis_iter(Dir::W, Dir::N);
    let ori_w_dir_e_1951= node_1951.get_axis_iter(Dir::W, Dir::W.invert());
    assert!(!match_axis(&ori_n_dir_w_1489, &ori_w_dir_e_1951).is_some());
    assert!(match_node(&node_1489, &node_1951, Dir::N, Dir::N).is_none());
    assert!(match_node(&node_1489, &node_1951, Dir::E, Dir::N).is_none());
    assert!(match_node(&node_1489, &node_1951, Dir::S, Dir::N).is_none());
    assert!(match_node(&node_1489, &node_1951, Dir::W, Dir::N).is_none());

    let grid_2729 = grids.iter().find(|(i, _)| *i == 2729).unwrap();
    let node_2729 = Node::from_grid(grid_2729.0, grid_2729.1.clone());
    assert!(match_node(&node_1951, &node_2729, Dir::N, Dir::N).is_some());
    assert!(match_node(&node_1951, &node_2729, Dir::E, Dir::N).is_none());
    assert!(match_node(&node_1951, &node_2729, Dir::S, Dir::N).is_none());
    assert!(match_node(&node_1951, &node_2729, Dir::W, Dir::N).is_none());

    let grid_2311 = grids.iter().find(|(i, _)| *i == 2311).unwrap();
    let node_2311 = Node::from_grid(grid_2311.0, grid_2311.1.clone());
    let grid_3079 = grids.iter().find(|(i, _)| *i == 3079).unwrap();
    let node_3079 = Node::from_grid(grid_3079.0, grid_3079.1.clone());
    assert!(match_node(&node_2311, &node_3079, Dir::N, Dir::N).is_none());
    assert!(match_node(&node_2311, &node_3079, Dir::E, Dir::N).is_some());
    assert!(match_node(&node_2311, &node_3079, Dir::S, Dir::N).is_none());
    assert!(match_node(&node_2311, &node_3079, Dir::W, Dir::N).is_none());

    assert_eq!(run(&input, TaskOfDay::First), Some(20899048083289));

    // let tst = grid_2311.1[0..5][0..5];

}
