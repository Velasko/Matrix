#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod base;
mod index;
mod operations;
mod square_matrix;
mod vector;

#[cfg(test)]
mod tests {
    use crate::base::Matrix;

    use super::*;

    #[test]
    fn matrix() {
        let matrix: Matrix<i8, 3, 3> = Matrix::new([[1; 3]; 3]);
        // let matrix2: Matrix<i8, 3, 3> = Matrix::new([[1; 3]; 3]);
        // let matrix3 = &matrix + &matrix2;
        // let a = &matrix[[0, 0]];
        // println!("{:?}", a);
    }
}
