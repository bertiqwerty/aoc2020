use num::Num;
use std::ops::{Index, IndexMut};
use std::{fmt, ops::Range};

#[derive(Clone, PartialEq)]
pub struct Grid<T: num::Num> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<'a, T: Num + Clone + Copy + fmt::Debug> Grid<T> {
    pub fn view(&'a self, row_range: Range<usize>, col_range: Range<usize>) -> GridView<'a, T> {
        GridView {
            row_start: row_range.start,
            row_end: row_range.end,
            col_start: col_range.start,
            col_end: col_range.end,
            grid: &self,
        }
    }
    pub fn rot90(&self) -> Self {
        let mut res: Self = Grid {
            rows: self.cols,
            cols: self.rows,
            data: vec![T::zero(); self.rows * self.cols],
        };
        for r in 0..self.rows {
            for c in 0..self.cols {
                res[self.cols - 1 - c][r] = self[r][c];
            }
        }
        res
    }
    pub fn rot180(&self) -> Self {
        let mut res: Self = Grid {
            rows: self.rows,
            cols: self.cols,
            data: vec![T::zero(); self.rows * self.cols],
        };
        for r in 0..self.rows {
            for c in 0..self.cols {
                res[self.rows - 1 - r][self.cols - 1 - c] = self[r][c];
            }
        }
        res
    }

    pub fn rot270(&self) -> Self {
        let mut res: Self = Grid {
            rows: self.cols,
            cols: self.rows,
            data: vec![T::zero(); self.rows * self.cols],
        };
        for r in 0..self.rows {
            for c in 0..self.cols {
                res[c][self.rows - 1 - r] = self[r][c];
            }
        }
        res
    }
    pub fn flipud(&self) -> Self {
        let mut res: Self = Grid {
            rows: self.rows,
            cols: self.cols,
            data: vec![T::zero(); self.rows * self.cols],
        };
        for r in 0..self.rows {
            for c in 0..self.cols {
                res[self.rows - 1 - r][c] = self[r][c];
            }
        }
        res
    }
    pub fn fliplr(&self) -> Self {
        let mut res: Self = Grid {
            rows: self.rows,
            cols: self.cols,
            data: vec![T::zero(); self.rows * self.cols],
        };
        for r in 0..self.rows {
            for c in 0..self.cols {
                res[r][self.cols - 1 - c] = self[r][c];
            }
        }
        res
    }
}

pub struct GridView<'a, T: Num + Clone + fmt::Debug> {
    pub row_start: usize,
    pub col_start: usize,
    pub row_end: usize,
    pub col_end: usize,
    pub grid: &'a Grid<T>,
}

impl<'a, T: Num + Clone + fmt::Debug> Index<usize> for GridView<'a, T> {
    type Output = [T];
    fn index(&self, idx: usize) -> &Self::Output {
        let shifted_idx = idx + self.row_start;
        &self.grid.data[shifted_idx * self.grid.cols..(shifted_idx + 1) * self.grid.cols]
            [self.col_start..self.col_end]
    }
}



impl<'a, T: Num + Clone + fmt::Debug> GridView<'a, T> {
    pub fn rows(&self) -> usize {
        self.row_end - self.row_start
    }
    pub fn cols(&self) -> usize {
        self.col_end - self.col_start
    }

}

impl<T: num::Num> Index<usize> for Grid<T> {
    type Output = [T];
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx * self.cols..(idx + 1) * self.cols]
    }
}
impl<T: Num> IndexMut<usize> for Grid<T> {
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
pub struct AxisIterator<'a, T: Num> {
    grid: &'a Grid<T>,
    start: isize,
    end: isize,
    step: isize,
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

impl<'a, T: fmt::Debug + Num + Clone> fmt::Debug for GridView<'a, T> {
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

impl<T: Clone + fmt::Debug + Num> fmt::Debug for Grid<T> {
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

fn start_end(start: isize, end: isize, step: isize) -> (isize, isize) {
    (
        if step > 0 { start } else { end - 1 },
        if step > 0 { end } else { start - 1 },
    )
}

impl<'a, T: Num + Clone + fmt::Debug> AxisIterator<'a, T> {
    pub fn make_row(row: usize, grid: &'a Grid<T>, step: isize) -> AxisIterator<'a, T> {
        let (start, end) = start_end(0, grid.cols as isize, step);
        return AxisIterator {
            grid: grid,
            start: start,
            end: end,
            step: step,
            axis: Axis::Row,
            axis_idx: row,
        };
    }
    pub fn make_col(col: usize, grid: &'a Grid<T>, step: isize) -> AxisIterator<'a, T> {
        let (start, end) = start_end(0, grid.rows as isize, step);
        return AxisIterator {
            grid: grid,
            start: start,
            end: end,
            step: step,
            axis: Axis::Col,
            axis_idx: col,
        };
    }

    pub fn make_row_view(
        row: usize,
        grid_view: &'a GridView<T>,
        step: isize,
    ) -> AxisIterator<'a, T> {
        let (start, end) = start_end(
            grid_view.col_start as isize,
            grid_view.col_end as isize,
            step,
        );

        return AxisIterator {
            grid: grid_view.grid,
            start: start,
            end: end,
            step: step,
            axis: Axis::Row,
            axis_idx: row + grid_view.row_start,
        };
    }
    pub fn make_col_view(
        col: usize,
        grid_view: &'a GridView<T>,
        step: isize,
    ) -> AxisIterator<'a, T> {
        let (start, end) = start_end(
            grid_view.row_start as isize,
            grid_view.row_end as isize,
            step,
        );

        return AxisIterator {
            grid: grid_view.grid,
            start: start,
            end: end,
            step: step,
            axis: Axis::Col,
            axis_idx: col + grid_view.col_start,
        };
    }
}

impl<'a, T: Num> Iterator for AxisIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        let nxt = self.start + self.step;
        match self.axis {
            Axis::Col => {
                let res = Some(&self.grid[self.start as usize][self.axis_idx]);
                self.start = nxt;
                res
            }
            Axis::Row => {
                let res = Some(&self.grid[self.axis_idx][self.start as usize]);
                self.start = nxt;
                res
            }
        }
    }
}
impl<'a, T: Num> DoubleEndedIterator for AxisIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        let nxt = self.end - self.step;
        match self.axis {
            Axis::Col => {
                self.end = nxt;
                Some(&self.grid[self.end as usize][self.axis_idx])
            }
            Axis::Row => {
                self.end = nxt;
                Some(&self.grid[self.axis_idx][self.end as usize])
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
    let row_iter = AxisIterator::make_row_view(1, &grid_14_13, 1);
    for (result, reference) in izip!(row_iter, vec![7, 8].iter()) {
        assert_eq!(result, reference);
    }
    let row_iter = AxisIterator::make_row_view(1, &grid_14_13, -1);
    for (result, reference) in izip!(row_iter, vec![8, 7].iter()) {
        assert_eq!(result, reference);
    }
    let row_iter = AxisIterator::make_row_view(1, &grid_14_13, -1).rev();
    for (result, reference) in izip!(row_iter, vec![7, 8].iter()) {
        assert_eq!(result, reference);
    }
    let col_iter = AxisIterator::make_col_view(1, &grid_14_13, 1);
    for (result, reference) in izip!(col_iter, vec![5, 8, 11].iter()) {
        assert_eq!(result, reference);
    }
    let col_iter = AxisIterator::make_col_view(1, &grid_14_13, -1);
    for (result, reference) in izip!(col_iter, vec![11, 8, 5].iter()) {
        assert_eq!(result, reference);
    }
    let col_iter = AxisIterator::make_col_view(1, &grid_14_13, 1).rev();
    for (result, reference) in izip!(col_iter, vec![11, 8, 5].iter()) {
        assert_eq!(result, reference);
    }
    let rot_test = Grid {
        rows: 2,
        cols: 3,
        data: vec![1, 2, 3, 4, 5, 6],
    };
    let rot_90_test = rot_test.rot90();
    let rot_180_test = rot_test.rot180();
    let rot_270_test = rot_test.rot270();
    println!("{:#?}", rot_test);
    println!("{:#?}", rot_90_test);
    println!("{:#?}", rot_180_test);
    println!("{:#?}", rot_270_test);
    assert_eq!(rot_90_test.cols, 2);
    assert_eq!(rot_90_test.rows, 3);
    assert_eq!(rot_90_test[0][0], 3);
    assert_eq!(rot_90_test[0][1], 6);
    assert_eq!(rot_90_test[1][0], 2);
    assert_eq!(rot_90_test[1][1], 5);
    assert_eq!(rot_90_test[2][0], 1);
    assert_eq!(rot_90_test[2][1], 4);
    assert_eq!(rot_180_test.cols, 3);
    assert_eq!(rot_180_test.rows, 2);
    assert_eq!(rot_180_test[0][0], 6);
    assert_eq!(rot_180_test[0][1], 5);
    assert_eq!(rot_180_test[0][2], 4);
    assert_eq!(rot_180_test[1][0], 3);
    assert_eq!(rot_180_test[1][1], 2);
    assert_eq!(rot_180_test[1][2], 1);
    assert_eq!(rot_270_test.cols, 2);
    assert_eq!(rot_270_test.rows, 3);
    assert_eq!(rot_270_test[0][0], 4);
    assert_eq!(rot_270_test[0][1], 1);
    assert_eq!(rot_270_test[1][0], 5);
    assert_eq!(rot_270_test[1][1], 2);
    assert_eq!(rot_270_test[2][0], 6);
    assert_eq!(rot_270_test[2][1], 3);
    let lr_flip = rot_test.fliplr();
    let ud_flip = rot_test.flipud();
    assert_eq!(lr_flip[0][0], 3);
    assert_eq!(lr_flip[0][1], 2);
    assert_eq!(lr_flip[0][2], 1);
    assert_eq!(lr_flip[1][0], 6);
    assert_eq!(lr_flip[1][1], 5);
    assert_eq!(lr_flip[1][2], 4);
    assert_eq!(ud_flip[0][0], 4);
    assert_eq!(ud_flip[0][1], 5);
    assert_eq!(ud_flip[0][2], 6);
    assert_eq!(ud_flip[1][0], 1);
    assert_eq!(ud_flip[1][1], 2);
    assert_eq!(ud_flip[1][2], 3);
}
