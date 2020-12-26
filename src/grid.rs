use num::Num;
use std::{fmt, ops::Range};
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<'a, T> Grid<T> {
    fn view(&'a self, row_range: Range<usize>, col_range: Range<usize>) -> GridView<'a, T> {
        GridView {
            row_start : row_range.start,
            row_end : row_range.end,
            col_start : col_range.start,
            col_end : col_range.end,
            grid : &self,
        }
    }
}

pub struct GridView<'a, T> {
    pub row_start: usize,
    pub col_start: usize,
    pub row_end: usize,
    pub col_end: usize,
    pub grid: &'a Grid<T>,
}
impl<'a, T> Index<usize> for GridView<'a, T> {
    type Output = [T];
    fn index(&self, idx: usize) -> &Self::Output {
        let shifted_idx = idx + self.row_start;
        &self.grid.data[shifted_idx * self.grid.cols..(shifted_idx + 1) * self.grid.cols]
            [self.col_start..self.col_end]
    }
}
impl<'a, T> GridView<'a, T> {
    fn rows(&self) -> usize {
        self.row_end - self.row_start
    }
    fn cols(&self) -> usize {
        self.col_end - self.col_start
    }
}


impl<T> Index<usize> for Grid<T> {
    type Output = [T];
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx * self.cols..(idx + 1) * self.cols]
    }
}
impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx * self.cols..(idx + 1) * self.cols]
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

impl<'a, T: fmt::Debug> fmt::Debug for GridView<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = format!("rows {}, cols {}\n", self.rows(), self.cols());
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                let val = &self[r][c];
                let formatted = format!("{:?} ", val);
                res.push_str(&formatted);
            }
            res.push_str("\n");
        }
        fmt::Display::fmt(&format!("{}", res), f)
    }
}

impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        GridView {
            grid: &self,
            row_start: 0,
            row_end: self.rows,
            col_start: 0,
            col_end: self.cols,
        }
        .fmt(f)
    }
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
    pub fn make_row_view(row: usize, grid_view: &'a GridView<T>) -> AxisIterator<'a, T> {
        return AxisIterator {
            grid: grid_view.grid,
            start: grid_view.col_start,
            end: grid_view.col_end,
            step: 1,
            axis: Axis::Row,
            axis_idx: row + grid_view.row_start,
        };
    }
    pub fn make_col_view(col: usize, grid_view: &'a GridView<T>) -> AxisIterator<'a, T> {
        return AxisIterator {
            grid: grid_view.grid,
            start: grid_view.row_start,
            end: grid_view.row_end,
            step: 1,
            axis: Axis::Col,
            axis_idx: col + grid_view.col_start,
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

#[test]
fn test_grid() {
    let grid = Grid {
        rows: 4,
        cols: 3,
        data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    };
    let grid_14_13 = grid.view(1..4, 1..3);
    println!("===GRID\n{:?}\n===GRIDVIEW\n{:?}", grid, grid_14_13);
    let row_iter: AxisIterator<i32> = AxisIterator::make_row_view(1, &grid_14_13);
    for (result, reference) in izip!(row_iter, vec![7, 8].iter()) {
        assert_eq!(result, reference);
    }
    let col_iter: AxisIterator<i32> = AxisIterator::make_col_view(1, &grid_14_13);
    for (result, reference) in izip!(col_iter, vec![5, 8, 11].iter()) {
        assert_eq!(result, reference);
    }
    
}
