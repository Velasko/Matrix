use std::{
    array::from_fn,
    iter::Sum,
    ops::{MulAssign, Sub},
};

use crate::base::Matrix;

pub trait SquareMatrix<T> {
    fn determinant(&self) -> T;
    // fn inverse(&self) -> Self;
}
impl<T, const SIZE: usize> SquareMatrix<T> for Matrix<T, SIZE, SIZE>
where
    T: Clone + for<'a> MulAssign<&'a T> + for<'a> Sum<&'a T> + Sub<Output = T>,
{
    fn determinant(&self) -> T {
        let mut add: [T; SIZE] = self.data[0].clone();
        let mut sub: [T; SIZE] = from_fn(|i| self.data[0][SIZE - i - 1].clone());
        for i in 1..SIZE {
            for j in 0..SIZE {
                let val = &self.data[i][j];
                add[(SIZE + i - j).rem_euclid(SIZE)] *= val;
                sub[(i + j).rem_euclid(SIZE)] *= val;
            }
        }
        add.iter().sum::<T>() - sub.iter().sum::<T>()
    }
}
