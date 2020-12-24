use std::iter;
use std::ops::{Index, IndexMut};
use num::Num;
pub enum TaskOfDay {
    First,
    Second,
}

#[derive(Clone)]
pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx * self.cols as usize..(idx + 1) * self.cols as usize]
    }
}
impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx * self.cols as usize..(idx + 1) * self.cols as usize]
    }
}
impl<T> Index<i32> for Grid<T> {
    type Output = [T];
    fn index(&self, idx: i32) -> &Self::Output {
        &self[idx as usize]
    }
}
impl<T> IndexMut<i32> for Grid<T> {
    fn index_mut(&mut self, idx: i32) -> &mut Self::Output {
        &mut self[idx as usize]
    }
}

#[derive(Clone, Copy)]
pub enum Axis {
    Row,
    Col,
}

#[derive(Clone, Copy)]
pub struct AxisIterator<'a, T> {
    grid: &'a Grid<T>,
    row_idx: usize,
    col_idx: usize,
    row_step: isize,
    col_step: isize,
    axis: Axis,
}

impl<'a, T> AxisIterator<'a, T> {
    pub fn make_row_forward(row: usize, grid: &'a Grid<T>) -> AxisIterator<'a, T>{
        return AxisIterator {
            grid: grid,
            row_idx: row,
            col_idx: 0,
            row_step: 1,
            col_step: 0,
            axis: Axis::Row,
        }
    }
    pub fn make_col_forward(col: usize, grid: &'a Grid<T>) -> AxisIterator<'a, T>{
        return AxisIterator {
            grid: grid,
            row_idx: 0,
            col_idx: col,
            row_step: 0,
            col_step: 1,
            axis: Axis::Col,
        }
    }
    pub fn make_row_backward(row: usize, grid: &'a Grid<T>) -> AxisIterator<'a, T>{
        return AxisIterator {
            grid: grid,
            row_idx: row,
            col_idx: grid.cols - 1,
            row_step: 0,
            col_step: -1,
            axis: Axis::Row,
        }
    }
    pub fn make_col_backward(col: usize, grid: &'a Grid<T>) -> AxisIterator<'a, T>{
        return AxisIterator {
            grid: grid,
            row_idx: grid.rows - 1,
            col_idx: col,
            row_step: -1,
            col_step: 0,
            axis: Axis::Col,
        }
    }
}

impl<'a, T> Iterator for AxisIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.axis {
            Axis::Col => {
                if self.row_idx < self.grid.rows - 1 && self.row_idx > 0 {
                    self.row_idx = (self.row_idx as isize + self.row_step) as usize;
                    return Some(&self.grid[self.row_idx][self.col_idx]);
                } else {
                    None
                }
            },
            Axis::Row => {
                if self.col_idx < self.grid.cols - 1 && self.col_idx > 0 {
                    self.col_idx = (self.col_idx as isize + self.col_step) as usize;
                    return Some(&self.grid[self.row_idx][self.col_idx]);
                } else {
                    None
                }
            },          
        }
    }
}


impl<T: Num> Grid<T> {
    pub fn from_lines(lines: &[String]) -> Option<Grid<T>> {
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
                        '.' => T::zero(),
                        'L' => T::one(),
                        '#' => T::one() + T::one(),
                        _ => panic!("Unknown character '{}'", c),
                    })
                })
                .flatten()
                .collect::<Vec<T>>(),
        })
    }
}

pub fn split_in2_tuple<'a>(to_be_split: &'a str, splitter: &str) -> (&'a str, &'a str) {
    let mut splt = to_be_split.split(splitter).map(|s| s.trim());
    (splt.next().unwrap(), splt.next().unwrap())
}

pub fn separate_by_blanks(input: &Vec<String>, joiner: &str) -> Vec<String>
{
    // TODO: currently, last element of input must be a blank line/string
    let split_positions = input
        .iter()
        .enumerate()
        .filter(|t: &(usize, &String)| t.1.len() == 0)
        .map(|t: (usize, &String)| t.0)
        .collect::<Vec<usize>>();
    let splits_shifted = &split_positions[1..];
    iter::once(input[0..split_positions[0]].join(joiner)).chain(
        izip!(&split_positions, splits_shifted)
            .map(|t| input[t.0.clone() + 1..t.1.clone()].join(joiner)),
    ).collect::<Vec<String>>()
}

pub fn string_to_lines(s: &str) -> Vec<String> {
    s.split("\n").map(|s| s.trim().to_string()).collect()
}