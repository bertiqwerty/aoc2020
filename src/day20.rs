use crate::grid::{Grid, AxisIterator, Rot90, Rot270, FlipLr, FlipUd, Twice};

use super::common::separate_by_blanks;
use super::common::string_to_lines;
use super::common::TaskOfDay;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
lazy_static! {
    static ref NUMERIC: Regex = Regex::new(r"[0-9]+").unwrap();
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub fn get_neighbor(&self, dir: Dir) -> Option<i32> {
        match dir {
            Dir::N => self.n,
            Dir::E => self.e,
            Dir::S => self.s,
            Dir::W => self.w,
        }
    }
    pub fn set_neighbor(&mut self, dir: Dir, n_id: i32) {
        match dir {
            Dir::N => self.n = Some(n_id),
            Dir::E => self.e = Some(n_id),
            Dir::S => self.s = Some(n_id),
            Dir::W => self.w = Some(n_id),
        }
    }
    pub fn count_neighbors(&self) -> u8 {
        self.n.is_some() as u8
            + self.e.is_some() as u8
            + self.s.is_some() as u8
            + self.w.is_some() as u8
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

    fn get_axis_iter(&self, dir: Dir) -> AxisIterator<u8> {
        match dir {
            Dir::N => AxisIterator::make_row(0, &self.grid, 1),
            Dir::E => AxisIterator::make_col(self.grid.cols - 1, &self.grid, 1),
            Dir::S => AxisIterator::make_row(self.grid.rows - 1, &self.grid, 1),
            Dir::W => AxisIterator::make_col(0, &self.grid, 1),
        }
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
    if izip!(iter1.clone(), iter2.clone()).all(|(i1, i2)| i1 == i2) {
        Some(false)
    } else if izip!(iter1.clone().rev(), iter2.clone()).all(|(i1, i2)| i1 == i2) {
        Some(true)
    } else {
        None
    }
}

enum MatchResult {
    NewGrid(Grid<u8>),
    SameGrid,
}

fn transform_grid(grid: &Grid<u8>, ori: Dir, dir: Dir, flipped: bool) -> Grid<u8> {
    let aligned = match (ori, dir) {
        (Dir::N, Dir::N) | (Dir::E, Dir::E) | (Dir::S, Dir::S) | (Dir::W, Dir::W) => grid.clone(),
        (Dir::W, Dir::S) | (Dir::E, Dir::N) => grid.rot90(),
        (Dir::W, Dir::E) | (Dir::E, Dir::W) => grid.fliplr(),
        (Dir::N, Dir::S) | (Dir::S, Dir::N) => grid.flipud(),
        (Dir::N, Dir::E) | (Dir::S, Dir::W) => grid.rot270(),
        (Dir::S, Dir::E) | (Dir::N, Dir::W) => grid.transform::<Twice<Rot90, FlipUd>>(),
        (Dir::E, Dir::S) | (Dir::W, Dir::N) => grid.transform::<Twice<Rot270, FlipLr>>(),
    };
    if flipped {
        match dir {
            Dir::N | Dir::S => aligned.fliplr(),
            Dir::E | Dir::W => aligned.flipud(),
        }
    } else {
        aligned
    }
}

fn match_node(
    node1: &Node,
    node2: &Node,
    node1_dir: Dir,
    node2_grid_fixed: bool,
) -> Option<MatchResult> {
    // returns geometry of second node if it fits
    let iters1 = node1.get_axis_iter(node1_dir);
    let node2_dir = node1_dir.invert();
    if node2_grid_fixed {
        let iters2 = node2.get_axis_iter(node2_dir);
        let unflipped_match = match_axis(&iters1, &iters2);
        return match unflipped_match {
            Some(m) => {
                if m {
                    None
                } else {
                    Some(MatchResult::SameGrid)
                }
            }
            None => None,
        };
    }
    for node2_ori in ALL_DIRS.iter() {
        let iters2 = node2.get_axis_iter(*node2_ori);
        let are_axis_matching = match_axis(&iters1, &iters2);
        if are_axis_matching.is_some() {
            let flipped = are_axis_matching?;
            return Some(MatchResult::NewGrid(transform_grid(
                &node2.grid,
                *node2_ori,
                node2_dir,
                flipped,
            )));
        }
    }
    None
}

fn get_corners(nodes: &BTreeMap<i32, Node>) -> Vec<i32> {
    nodes
        .iter()
        .filter(|(id, node)| {
            let border_sides = node.n.is_none() as u8
                + node.e.is_none() as u8
                + node.s.is_none() as u8
                + node.w.is_none() as u8;
            if border_sides > 2 {
                panic!(
                    "More than 2 border sides are not possible. Found {} for id {}.",
                    border_sides, id
                );
            }
            border_sides == 2
        })
        .map(|(id, _)| *id)
        .collect::<Vec<i32>>()
}

fn print_hood(node_id: i32, nodes: &BTreeMap<i32, Node>) {
    let un = |id: Option<i32>| id.unwrap_or(-1);
    println!("\t{:04}", un(nodes[&node_id].n));
    println!(
        "{:04}\t{:04}\t{:04}",
        un(nodes[&node_id].w),
        node_id,
        un(nodes[&node_id].e)
    );
    println!("\t{:04}", un(nodes[&node_id].s));
    println!("");
}

struct NodeIterator<'a> {
    dir: Dir,
    current_id: Option<i32>,
    nodes: &'a BTreeMap<i32, Node>

}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.current_id;
        self.current_id = self.nodes[&self.current_id?].get_neighbor(self.dir);
        res
    }
}

fn node_row_iter<'a>(row_anchor: i32, nodes: &'a BTreeMap<i32, Node>) -> NodeIterator<'a> {
    NodeIterator{dir: Dir::E, current_id:Some(row_anchor), nodes:nodes}
}
fn node_col_iter<'a>(col_anchor: i32, nodes: &'a BTreeMap<i32, Node>) -> NodeIterator<'a> {
    NodeIterator{dir: Dir::S, current_id:Some(col_anchor), nodes:nodes}
}

fn merge_grids(nodes: &BTreeMap<i32, Node>) -> Grid<u8> {
    let row_anchor0 = *nodes
        .iter()
        .find(|(_, n)| n.get_neighbor(Dir::N).is_none() && n.get_neighbor(Dir::W).is_none())
        .unwrap().0;
    let grids_in_a_row = node_row_iter(row_anchor0, &nodes).count();
    let grids_in_a_col = nodes.len() / grids_in_a_row; 
    let rows_per_view = nodes[&row_anchor0].grid.rows - 2;
    let cols_per_view = nodes[&row_anchor0].grid.cols - 2;
    let res_rows = grids_in_a_row * (rows_per_view); 
    let res_cols = grids_in_a_col * (cols_per_view);
    let mut res: Grid<u8> = Grid {
        rows: res_rows,
        cols: res_cols,
        data: vec![0; res_rows*res_cols]
    };

    for (cur_row, row_anchor) in node_col_iter(row_anchor0, &nodes).enumerate() {
        let row_offset = cur_row * rows_per_view;
        let row_iter = node_row_iter(row_anchor, &nodes);
        for (grids_col, current_id) in row_iter.enumerate() {
            let view = &nodes[&current_id].grid.view(1..rows_per_view+1, 1..cols_per_view+1); 
            let col_offset = grids_col * cols_per_view;
            for (r, c) in iproduct!(0..view.rows(), 0..view.cols()) {
                res[row_offset + r][col_offset + c] = *view.at(r, c);
            }
        }
    }
    res
}

fn collect_nodes(grids: &Vec<String>) -> BTreeMap<i32, Node>{
    grids
    .iter()
    .map(|s| {
        let (id, grid) = str_to_id_grid(s).unwrap();
        (id, Node::from_grid(id, grid))
    })
    .collect::<BTreeMap<i32, Node>>()
}

pub fn run(input: &Vec<String>, part: TaskOfDay) -> Option<u64> {
    let input_grids = separate_by_blanks(&input, "\n");
    let mut nodes = collect_nodes(&input_grids);

    let mut floatings = nodes.keys().map(|id| *id).collect::<Vec<i32>>();
    let mut fixeds: Vec<i32> = Vec::with_capacity(0);

    fixeds.push(floatings.pop()?);
    while floatings.len() > 0 {
        let mut to_be_moved: Vec<i32> = Vec::with_capacity(0);
        for (flo, fix) in iproduct!(&floatings, &fixeds) {
            let flo_match = ALL_DIRS
                .iter()
                .filter(|dir| nodes[fix].get_neighbor(**dir).is_none())
                .map(|fix_dir| {
                    (
                        fix_dir,
                        match_node(
                            &nodes[fix],
                            &nodes[flo],
                            *fix_dir,
                            nodes[flo].count_neighbors() > 0,
                        ),
                    )
                })
                .find(|(_, node_match)| node_match.is_some());

            if flo_match.is_some() {
                let (fix_dir, node_match) = flo_match?;
                let flo_dir = fix_dir.invert();
                nodes.get_mut(fix)?.set_neighbor(*fix_dir, *flo);
                nodes.get_mut(flo)?.set_neighbor(flo_dir, *fix);
                let new_grid = match node_match? {
                    MatchResult::SameGrid => nodes[flo].grid.clone(),
                    MatchResult::NewGrid(g) => g,
                };
                nodes.get_mut(flo)?.grid = new_grid.clone();

                to_be_moved.push(*flo);
            }
        }
        for item in to_be_moved {
            floatings.retain(|i| *i != item);
            if !fixeds.contains(&item) {
                fixeds.push(item);
            }
        }
    }
    let corners = get_corners(&nodes);
    if corners.clone().len() != 4 {
        for corner in &corners {
            println!("===== NODE =====");
            println!("{:#?}", &nodes[corner]);
        }
        panic!("We have exactly 4 corners not {}.", corners.clone().len())
    }

    match part {
        TaskOfDay::First => Some(corners.iter().map(|id| *id as u64).product()),
        TaskOfDay::Second => {
                let merged_grid = merge_grids(&nodes);
                let monster: Grid<u8> = Grid::from_lines(&vec![
                    "..................#.".to_string(),
                    "#....##....##....###".to_string(),
                    ".#..#..#..#..#..#...".to_string()
                ]).unwrap();
                Some(0u64)}
            ,
    }
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
    let it4north: AxisIterator<u8> = AxisIterator::make_row(grids[4].1.rows - 1, &grids[4].1, 1);
    let it3south: AxisIterator<u8> = AxisIterator::make_row(0, &grids[3].1, 1);
    assert!(match_axis(&it4north, &it3south).is_some());
    let it31: AxisIterator<u8> = AxisIterator::make_row(1, &grids[3].1, 1);
    assert!(!match_axis(&it4north, &it31).is_some());

    let grid_1489 = grids.iter().find(|(i, _)| *i == 1489).unwrap();
    let grid_1951 = grids.iter().find(|(i, _)| *i == 1951).unwrap();
    let node_1489 = Node::from_grid(grid_1489.0, grid_1489.1.clone());
    let node_1951 = Node::from_grid(grid_1951.0, grid_1951.1.clone());
    assert!(match_node(&node_1489, &node_1951, Dir::N, false).is_none());
    assert!(match_node(&node_1489, &node_1951, Dir::E, false).is_none());
    assert!(match_node(&node_1489, &node_1951, Dir::S, false).is_none());
    assert!(match_node(&node_1489, &node_1951, Dir::W, false).is_none());

    let grid_2971 = grids.iter().find(|(i, _)| *i == 2971).unwrap();
    let node_2971 = Node::from_grid(grid_2971.0, grid_2971.1.clone());

    let iters1 = node_1489.get_axis_iter(Dir::W);
    let iters2 = node_2971.get_axis_iter(Dir::E);
    assert_eq!(
        iters1.map(|x| *x).collect::<Vec<u8>>(),
        iters2.map(|x| *x).collect::<Vec<u8>>()
    );
    assert!(match_node(&node_1489, &node_2971, Dir::W, false).is_some());

    assert!(match_node(&node_1951, &node_1489, Dir::N, false).is_none());
    assert!(match_node(&node_1951, &node_1489, Dir::E, false).is_none());
    assert!(match_node(&node_1951, &node_1489, Dir::S, false).is_none());
    assert!(match_node(&node_1951, &node_1489, Dir::W, false).is_none());

    let grid_2473 = grids.iter().find(|(i, _)| *i == 2473).unwrap();
    let node_2473 = Node::from_grid(grid_2473.0, grid_2473.1.clone());
    let grid_1171 = grids.iter().find(|(i, _)| *i == 1171).unwrap();
    let node_1171 = Node::from_grid(grid_1171.0, grid_1171.1.clone());

    assert!(match_node(&node_2473, &node_1171, Dir::N, false).is_none());
    assert!(match_node(&node_2473, &node_1171, Dir::E, false).is_none());
    assert!(match_node(&node_2473, &node_1171, Dir::S, false).is_none());
    assert!(match_node(&node_2473, &node_1171, Dir::W, false).is_some());
    match match_node(&node_2473, &node_1171, Dir::W, false).unwrap() {
        MatchResult::SameGrid => assert!(false),
        MatchResult::NewGrid(_) => assert!(true),
    }

    assert_eq!(run(&input, TaskOfDay::First), Some(20899048083289));
    assert_eq!(run(&input, TaskOfDay::Second), Some(0));


}
