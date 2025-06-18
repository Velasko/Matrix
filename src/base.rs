use std::{any::type_name, cmp::PartialEq, fmt::Debug};

pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    pub(crate) data: Box<[[T; COLS]; ROWS]>,
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

impl<T, const ROWS: usize, const COLS: usize> From<[[T; COLS]; ROWS]> for Matrix<T, ROWS, COLS> {
    fn from(value: [[T; COLS]; ROWS]) -> Self {
        Self {
            data: Box::new(value),
        }
    }
}

impl<T, const ROWS: usize, const COLS: usize> AsRef<Matrix<T, ROWS, COLS>>
    for Matrix<T, ROWS, COLS>
{
    fn as_ref(&self) -> &Matrix<T, ROWS, COLS> {
        &self
    }
}

impl<T, const ROWS: usize, const COLS: usize> AsRef<[[T; COLS]; ROWS]> for Matrix<T, ROWS, COLS> {
    fn as_ref(&self) -> &[[T; COLS]; ROWS] {
        &self.data
    }
}
