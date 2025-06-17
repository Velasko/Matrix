use std::ops::{Index, IndexMut};

use crate::base::Matrix;

impl<T, const ROWS: usize, const COLS: usize> Index<[usize; 2]> for Matrix<T, ROWS, COLS> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [row, col] = index;
        &self.data[row][col]
    }
}

impl<T, const ROWS: usize, const COLS: usize> Index<usize> for Matrix<T, ROWS, COLS> {
    type Output = [T; COLS];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const ROWS: usize, const COLS: usize> IndexMut<[usize; 2]> for Matrix<T, ROWS, COLS> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let [row, col] = index;
        &mut self.data[row][col]
    }
}

impl<T, const ROWS: usize, const COLS: usize> IndexMut<usize> for Matrix<T, ROWS, COLS> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
