use num::Num;
use std::ops::{Index, IndexMut};
use std::{fmt, ops::Range};

pub trait DataType: Num + Clone + Copy + fmt::Debug {}
impl<T: Num + Clone + Copy + fmt::Debug> DataType for T {}

#[derive(Clone, PartialEq)]
pub struct Grid<T: DataType> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<'a, T: DataType> Grid<T> {
    pub fn as_view(&'a self) -> GridView<'a, T, Identity> {
        self.transformed_view::<Identity>(0..self.rows, 0..self.cols)
    }

    pub fn as_tf_view<TF: IdxTransform>(&'a self) -> GridView<'a, T, TF> {
        self.transformed_view::<TF>(0..self.rows, 0..self.cols)
    }

    pub fn view(&'a self,
        row_range: Range<usize>,
        col_range: Range<usize>,
    ) -> GridView<'a, T, Identity> {
        self.transformed_view::<Identity>(row_range, col_range)
    }

    pub fn transform<TF: IdxTransform>(&self) -> Self {
        self.transformed_view::<TF>(0..self.rows, 0..self.cols)
            .to_grid()
    }

    /// First extracts the view, then applies the transformation.
    pub fn transformed_view<TF: IdxTransform>(
        &'a self,
        row_range: Range<usize>,
        col_range: Range<usize>,
    ) -> GridView<'a, T, TF> {
        GridView {
            row_start: row_range.start,
            row_end: row_range.end,
            col_start: col_range.start,
            col_end: col_range.end,
            grid: &self,
            tf: TF::make(
                row_range.end - row_range.start,
                col_range.end - col_range.start,
            ),
        }
    }

    pub fn at(&self, row: usize, col: usize) -> &T {
        &self[row][col]
    }

    pub fn rot90(&self) -> Self {
        self.transform::<Rot90>()
    }
    pub fn rot180(&self) -> Self {
        self.transform::<Rot180>()
    }
    pub fn rot270(&self) -> Self {
        self.transform::<Rot270>()
    }
    pub fn flipud(&self) -> Self {
        self.transform::<FlipUd>()
    }
    pub fn fliplr(&self) -> Self {
        self.transform::<FlipLr>()
    }
}

pub trait IdxTransform {
    fn tf_rows(&self) -> usize;
    fn tf_cols(&self) -> usize;
    fn apply(&self, row: usize, col: usize) -> (usize, usize);
    fn make(rows: usize, cols: usize) -> Self;
}

#[derive(Clone, Copy)]
pub struct Rot90 {
    tf_rows: usize,
    tf_cols: usize,
}
impl IdxTransform for Rot90 {
    fn apply(&self, row: usize, col: usize) -> (usize, usize) {
        (col, self.tf_rows - 1 - row)
    }
    fn tf_rows(&self) -> usize {
        self.tf_rows
    }
    fn tf_cols(&self) -> usize {
        self.tf_cols
    }
    fn make(rows: usize, cols: usize) -> Self {
        Self {
            tf_rows: cols,
            tf_cols: rows,
        }
    }
}

#[derive(Clone, Copy)]
pub struct FlipLr {
    rows: usize,
    cols: usize,
}
impl IdxTransform for FlipLr {
    fn apply(&self, row: usize, col: usize) -> (usize, usize) {
        (row, self.cols - 1 - col)
    }
    fn tf_rows(&self) -> usize {
        self.rows
    }
    fn tf_cols(&self) -> usize {
        self.cols
    }
    fn make(rows: usize, cols: usize) -> Self {
        Self {
            rows: rows,
            cols: cols,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Twice<TF1: IdxTransform, TF2: IdxTransform> {
    tf1: TF1,
    tf2: TF2,
}
impl<TF1: IdxTransform, TF2: IdxTransform> IdxTransform for Twice<TF1, TF2> {
    fn apply(&self, row: usize, col: usize) -> (usize, usize) {
        let (r2, c2) = self.tf2.apply(row, col);
        self.tf1.apply(r2, c2)
    }
    fn tf_rows(&self) -> usize {
        self.tf2.tf_rows()
    }
    fn tf_cols(&self) -> usize {
        self.tf2.tf_cols()
    }
    fn make(rows: usize, cols: usize) -> Self {
        let tf1 = TF1::make(rows, cols);
        let tf2 = TF2::make(tf1.tf_rows(), tf1.tf_cols());
        Twice { tf1: tf1, tf2: tf2 }
    }
}

pub type Rot180 = Twice<Rot90, Rot90>;
pub type Rot270 = Twice<Rot90, Rot180>;
pub type FlipUd = Twice<Rot90, Twice<FlipLr, Rot270>>;
#[derive(Clone, Copy)]
pub struct Identity {
    rows: usize,
    cols: usize,
}
impl IdxTransform for Identity {
    fn apply(&self, row: usize, col: usize) -> (usize, usize) {
        (row, col)
    }
    fn tf_rows(&self) -> usize {
        self.rows
    }
    fn tf_cols(&self) -> usize {
        self.cols
    }
    fn make(rows: usize, cols: usize) -> Self {
        Self {
            rows: rows,
            cols: cols,
        }
    }
}

#[derive(Clone, Copy)]
pub struct GridView<'a, T: DataType, TF: IdxTransform> {
    pub row_start: usize,
    pub col_start: usize,
    pub row_end: usize,
    pub col_end: usize,
    pub grid: &'a Grid<T>,
    pub tf: TF,
}

impl<'a, T: DataType, TF: IdxTransform> GridView<'a, T, TF> {
    pub fn rows(&self) -> usize {
        self.tf.tf_rows()
    }
    pub fn cols(&self) -> usize {
        self.tf.tf_cols()
    }
    pub fn at(&self, row: usize, col: usize) -> &'a T {
        let (t_row, t_col) = self.tf.apply(row, col);

        let shifted_row = t_row + self.row_start;

        &self.grid.data[shifted_row * self.grid.cols..(shifted_row + 1) * self.grid.cols]
            [self.col_start..self.col_end][t_col]
    }
    pub fn to_grid(&self) -> Grid<T> {
        let data = iproduct!(0..self.rows(), 0..self.cols())
            .map(|(r, c)| *self.at(r, c))
            .collect::<Vec<T>>();
        Grid {
            rows: self.rows(),
            cols: self.cols(),
            data: data,
        }
    }
}

impl<T: DataType> Index<usize> for Grid<T> {
    type Output = [T];
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx * self.cols..(idx + 1) * self.cols]
    }
}
impl<T: DataType> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx * self.cols..(idx + 1) * self.cols]
    }
}

impl<T: DataType> Grid<T> {
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

impl<'a, T: DataType, TF: IdxTransform> fmt::Debug for GridView<'a, T, TF> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = format!("rows {}, cols {}\n", self.rows(), self.cols());
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                let val = &self.at(r, c);
                let formatted = format!("{:?} ", val);
                res.push_str(&formatted);
            }
            res.push_str("\n");
        }
        fmt::Display::fmt(&format!("{}", res), f)
    }
}

impl<T: DataType> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        GridView {
            grid: &self,
            row_start: 0,
            row_end: self.rows,
            col_start: 0,
            col_end: self.cols,
            tf: Identity::make(self.rows, self.cols),
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
#[derive(Clone, Copy)]
pub enum Axis {
    Row,
    Col,
}

#[derive(Clone, Copy)]
pub struct AxisIterator<'a, T: DataType, TF: IdxTransform> {
    view: GridView<'a, T, TF>,
    start: isize,
    end: isize,
    step: isize,
    axis: Axis,
    axis_idx: usize,
}

impl<'a, T: DataType> AxisIterator<'a, T, Identity> {
    pub fn make_row(row: usize, grid: &'a Grid<T>, step: isize) -> AxisIterator<'a, T, Identity> {
        let (start, end) = start_end(0, grid.cols as isize, step);
        return AxisIterator {
            view: grid.as_view(),
            start: start,
            end: end,
            step: step,
            axis: Axis::Row,
            axis_idx: row,
        };
    }
    pub fn make_col(col: usize, grid: &'a Grid<T>, step: isize) -> AxisIterator<'a, T, Identity> {
        let (start, end) = start_end(0, grid.rows as isize, step);
        return AxisIterator {
            view: grid.as_view(),
            start: start,
            end: end,
            step: step,
            axis: Axis::Col,
            axis_idx: col,
        };
    }
} 

impl<'a, T: DataType, TF: IdxTransform> AxisIterator<'a, T, TF> {

    pub fn make_row_view(
        row: usize,
        grid_view: GridView<'a, T, TF>,
        step: isize,
    ) -> AxisIterator<'a, T, TF> {
        let (start, end) = start_end(0isize, grid_view.cols() as isize,
            step,
        );

        return AxisIterator {
            view: grid_view,
            start: start,
            end: end,
            step: step,
            axis: Axis::Row,
            axis_idx: row,
        };
    }
    pub fn make_col_view(
        col: usize,
        grid_view: GridView<'a, T, TF>,
        step: isize,
    ) -> AxisIterator<'a, T, TF> {
        let (start, end) = start_end(0isize, grid_view.rows() as isize,
            step,
        );

        return AxisIterator {
            view: grid_view,
            start: start,
            end: end,
            step: step,
            axis: Axis::Col,
            axis_idx: col,
        };
    }
}

impl<'a, T: DataType, TF: IdxTransform> Iterator for AxisIterator<'a, T, TF> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        let nxt = self.start + self.step;
        match self.axis {
            Axis::Col => {
                let res: Option<&'a T> = Some(self.view.at(self.start as usize, self.axis_idx));
                self.start = nxt;
                res
            }
            Axis::Row => {
                let res = Some(self.view.at(self.axis_idx, self.start as usize));
                self.start = nxt;
                res
            }
        }
    }
}
impl<'a, T: DataType, TF: IdxTransform> DoubleEndedIterator for AxisIterator<'a, T, TF> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        let nxt = self.end - self.step;
        match self.axis {
            Axis::Col => {
                self.end = nxt;
                Some(self.view.at(self.end as usize, self.axis_idx))
            }
            Axis::Row => {
                self.end = nxt;
                Some(self.view.at(self.axis_idx, self.end as usize))
            }
        }
    }
}

#[test]
fn test_grid() {
    let grid_axis_iter_test = Grid {
        rows: 4,
        cols: 3,
        data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    };
    let grid_14_13 = grid_axis_iter_test.view(1..4, 1..3);
    println!(
        "===GRID\n{:?}\n===GRIDVIEW\n{:?}",
        grid_axis_iter_test, grid_14_13
    );
    let row_iter = AxisIterator::make_row_view(1, grid_14_13, 1);
    for (result, reference) in izip!(row_iter, vec![7, 8].iter()) {
        assert_eq!(result, reference);
    }
    let row_iter = AxisIterator::make_row_view(1, grid_14_13, -1);
    for (result, reference) in izip!(row_iter, vec![8, 7].iter()) {
        assert_eq!(result, reference);
    }
    let row_iter = AxisIterator::make_row_view(1, grid_14_13, -1).rev();
    for (result, reference) in izip!(row_iter, vec![7, 8].iter()) {
        assert_eq!(result, reference);
    }
    let col_iter = AxisIterator::make_col_view(1, grid_14_13, 1);
    for (result, reference) in izip!(col_iter, vec![5, 8, 11].iter()) {
        assert_eq!(result, reference);
    }
    let col_iter = AxisIterator::make_col_view(1, grid_14_13, -1);
    for (result, reference) in izip!(col_iter, vec![11, 8, 5].iter()) {
        assert_eq!(result, reference);
    }
    let col_iter = AxisIterator::make_col_view(1, grid_14_13, 1).rev();
    for (result, reference) in izip!(col_iter, vec![11, 8, 5].iter()) {
        assert_eq!(result, reference);
    }
    let grid = Grid {
        rows: 2,
        cols: 3,
        data: vec![1, 2, 3, 4, 5, 6],
    };
    let rot_90_test = grid.rot90();
    let rot_180_test = grid.rot180();
    let rot_270_test = grid.rot270();
    println!("rot_test\n{:#?}", grid);
    println!("rot_90\n{:#?}", rot_90_test);
    println!("rot_180\n{:#?}", rot_180_test);
    println!("rot_270\n{:#?}", rot_270_test);
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
    let lr_flip = grid.fliplr();
    let ud_flip = grid.flipud();
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

    let grid = grid_axis_iter_test.view(1..4, 1..3).to_grid();
    let rot90_view = grid_axis_iter_test.transformed_view::<Rot90>(1..4, 1..3);
    assert_eq!(grid.rot90(), rot90_view.to_grid());
    let col_iter = AxisIterator::make_col_view(2, rot90_view, 1);
    assert_eq!(col_iter.collect::<Vec<&i32>>(), vec![&11, &10]);
}
