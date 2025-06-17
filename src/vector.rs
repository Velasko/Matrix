use crate::base::Matrix;

pub trait Vector {}
impl<T, const SIZE: usize> Vector for Matrix<T, 1, SIZE> {}
