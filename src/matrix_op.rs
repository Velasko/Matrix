use std::{
    array::from_fn,
    iter::Sum,
    mem::{swap, transmute_copy},
    ops::Mul,
    ptr,
};

use crate::base::Matrix;

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn transpose(self) -> Matrix<T, COLS, ROWS> {
        let mut data = *self.data;
        let transposed: [[T; ROWS]; COLS] = unsafe {
            for i in 0..ROWS {
                for j in i..COLS {
                    if i != j {
                        let pa: *mut T = &mut data[i][j];
                        let pb: *mut T = &mut data[j][i];
                        ptr::swap(pa, pb);
                    }
                }
            }
            std::mem::transmute_copy(&data)
        };
        return Matrix::<T, COLS, ROWS>::from(transposed);
    }

    pub fn dot<RHS, RT, IT, OT, const RCOLS: usize>(&self, rhs: RHS) -> Matrix<OT, ROWS, RCOLS>
    where
        RHS: AsRef<[[RT; COLS]; RCOLS]>,
        for<'a, 'b> &'a T: Mul<&'b RT, Output = IT>,
        OT: Sum<IT>,
    {
        let other = rhs.as_ref();
        let matrix: [[OT; RCOLS]; ROWS] =
            from_fn(|i| from_fn(|j| (0..COLS).map(|a| &self.data[i][a] * &other[a][j]).sum()));

        Matrix::<OT, ROWS, RCOLS>::from(matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::alloc::{Layout, alloc};

    #[test]
    fn transpose_test() {
        let mut a: [[u8; 2]; 3] = [[0; 2]; 3];
        a[1][0] = 1;
        let a = Matrix::from(a);
        println!("Initial: {:?}", a);
        let a = a.transpose();
        println!("Result: {:?}", a);
    }

    // #[test]
    fn casting() {
        let mut a: Box<[[u8; 2]; 3]> = Box::new([[0; 2]; 3]);
        a[1][0] = 1;
        println!("{:?}", a);
    }

    // #[test]
    fn hardcasting() {
        let mut a: [[u8; 2]; 3] = [[0; 2]; 3];
        a[1][0] = 1;
        println!("a: {:?}", a);
        unsafe {
            let b: [[u8; 3]; 2] = std::mem::transmute_copy(&a);
            println!("b: {:?}", b);
        }
    }
}
