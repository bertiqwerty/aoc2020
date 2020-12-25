use num::Num;
use std::fmt;
use std::iter;
use std::ops::{Index, IndexMut};
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
    start: usize,
    end: usize,
    step: usize,
    axis: Axis,
    axis_idx: usize,
}

impl<'a, T> AxisIterator<'a, T> {
    pub fn make_row(row: usize, grid: &'a Grid<T>) -> AxisIterator<'a, T> {
        return AxisIterator {
            grid: grid,
            start: 0,
            end: grid.cols,
            step: 1,
            axis: Axis::Row,
            axis_idx: row,
        };
    }
    pub fn make_col(col: usize, grid: &'a Grid<T>) -> AxisIterator<'a, T> {
        return AxisIterator {
            grid: grid,
            start: 0,
            end: grid.rows,
            step: 1,
            axis: Axis::Col,
            axis_idx: col,
        };
    }
}

impl<'a, T> Iterator for AxisIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        match self.axis {
            Axis::Col => {
                self.start += self.step;
                Some(&self.grid[self.start - self.step][self.axis_idx])
            }
            Axis::Row => {
                self.start += self.step;
                Some(&self.grid[self.axis_idx][self.start - self.step])
            }
        }
    }
}
impl<'a, T> DoubleEndedIterator for AxisIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        match self.axis {
            Axis::Col => {
                self.end -= self.step;
                Some(&self.grid[self.end][self.axis_idx])
            }
            Axis::Row => {
                self.end -= self.step;
                Some(&self.grid[self.axis_idx][self.end])
            }
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

impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = format!("rows {}, cols {}\n", self.rows, self.cols);
        for r in 0usize..self.rows {
            for c in 0usize..self.cols {
                let val = &self[r][c];
                let formatted = format!("{:?} ", val);
                res.push_str(&formatted);
            }
            res.push_str("\n");
        }
        fmt::Display::fmt(&format!("{}", res), f)
    }
}

pub fn split_in2_tuple<'a>(to_be_split: &'a str, splitter: &str) -> (&'a str, &'a str) {
    let mut splt = to_be_split.split(splitter).map(|s| s.trim());
    (splt.next().unwrap(), splt.next().unwrap())
}

pub fn separate_by_blanks(input: &Vec<String>, joiner: &str) -> Vec<String> {
    // TODO: currently, last element of input must be a blank line/string
    let split_positions = input
        .iter()
        .enumerate()
        .filter(|t: &(usize, &String)| t.1.len() == 0)
        .map(|t: (usize, &String)| t.0)
        .collect::<Vec<usize>>();
    let splits_shifted = &split_positions[1..];
    iter::once(input[0..split_positions[0]].join(joiner))
        .chain(
            izip!(&split_positions, splits_shifted)
                .map(|t| input[t.0.clone() + 1..t.1.clone()].join(joiner)),
        )
        .collect::<Vec<String>>()
}

pub fn string_to_lines(s: &str) -> Vec<String> {
    s.split("\n").map(|s| s.trim().to_string()).collect()
}
