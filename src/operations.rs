use std::array::from_fn;

use crate::base::Matrix;

macro_rules! impl_op {
    ($trait:tt, $op:tt) => {
        use std::ops::$trait;
        impl<LT, RT, OT, const ROWS: usize, const COLS: usize> $trait<&Matrix<RT, ROWS, COLS>>
            for &Matrix<LT, ROWS, COLS>
        where
            for<'a, 'b> &'a LT: $trait<&'b RT, Output = OT>,
        {
            type Output = Matrix<OT, ROWS, COLS>;

            fn $op(self, rhs: &Matrix<RT, ROWS, COLS>) -> Self::Output {
                let matrix: [[OT; COLS]; ROWS] =
                    from_fn(|i| from_fn(|j| $trait::$op(&self[i][j], &rhs[i][j])));

                Self::Output::new(matrix)
            }
        }

        impl<LT, RT, const ROWS: usize, const COLS: usize> $trait<&Matrix<RT, ROWS, COLS>>
            for Matrix<LT, ROWS, COLS>
        where
            for<'a, 'b> &'a LT: $trait<&'b RT, Output = LT>,
        {
            type Output = Self;

            fn $op(self, rhs: &Matrix<RT, ROWS, COLS>) -> Self::Output {
                for i in 0..ROWS {
                    for j in 0..COLS {
                        self.data[i][j] = $trait::$op(&self.data[i][j], &rhs[i][j]);
                    }
                }
                self
            }
        }

        impl<LT, RT, const ROWS: usize, const COLS: usize> $trait<Matrix<RT, ROWS, COLS>>
            for Matrix<LT, ROWS, COLS>
        where
            for<'a, 'b> &'a LT: $trait<&'b RT, Output = LT>,
        {
            type Output = Self;

            fn $op(self, rhs: Matrix<RT, ROWS, COLS>) -> Self::Output {
                $trait::$op(self, &rhs)
            }
        }
    };
}

impl_op!(Add, add);
impl_op!(Sub, sub);
impl_op!(Mul, mul);
impl_op!(Div, div);

impl_op!(BitAnd, bitand);
impl_op!(BitOr, bitor);
impl_op!(BitXor, bitxor);

impl_op!(Shl, shl);
impl_op!(Shr, shr);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multipy() {
        const SIZE: usize = 1290;
        let matrix: Matrix<i8, SIZE, SIZE> = Matrix::default();
        let matrix2: Matrix<i8, SIZE, SIZE> = Matrix::default();
        let matrix3 = matrix + &matrix2;
        assert_eq!(matrix3, matrix2);
    }
}
