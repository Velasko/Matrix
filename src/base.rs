use std::{any::type_name, cmp::PartialEq, fmt::Debug};

pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    pub(crate) data: Box<[[T; COLS]; ROWS]>,
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn new(data: [[T; COLS]; ROWS]) -> Self {
        Self {
            data: Box::new(data),
        }
    }

    pub fn prod(&self, other: u8) {
        todo!("Matrix multiplication");
    }
}

impl<T: Debug, const ROWS: usize, const COLS: usize> Debug for Matrix<T, ROWS, COLS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(format!("Matrix<{:?}> {}x{}:", type_name::<T>(), ROWS, COLS).as_str())
            .field("data", &self.data)
            .finish()
    }
}

impl<T: Clone, const ROWS: usize, const COLS: usize> Clone for Matrix<T, ROWS, COLS> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS> {
    fn default() -> Self {
        Self {
            data: Box::new([[T::default(); COLS]; ROWS]),
        }
    }
}

impl<T: PartialEq, const ROWS: usize, const COLS: usize> PartialEq for Matrix<T, ROWS, COLS> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
