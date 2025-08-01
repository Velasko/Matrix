use std::array::from_fn;

use crate::base::Matrix;

// General trait implementation for common operations for values and their references
// the "for &Matrix<...>" allocate new memory while the others do an inplace
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

                Self::Output::from(matrix)
            }
        }

        impl<LT, RT, OT, const ROWS: usize, const COLS: usize> $trait<Matrix<RT, ROWS, COLS>>
            for &Matrix<LT, ROWS, COLS>
        where
            for<'a, 'b> &'a LT: $trait<&'b RT, Output = OT>,
        {
            type Output = Matrix<OT, ROWS, COLS>;

            fn $op(self, rhs: Matrix<RT, ROWS, COLS>) -> Self::Output {
                $trait::$op(self, &rhs)
            }
        }

        impl<LT, RT, const ROWS: usize, const COLS: usize> $trait<&Matrix<RT, ROWS, COLS>>
            for Matrix<LT, ROWS, COLS>
        where
            for<'a, 'b> &'a LT: $trait<&'b RT, Output = LT>,
        {
            type Output = Self;

            fn $op(mut self, rhs: &Matrix<RT, ROWS, COLS>) -> Self::Output {
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

            #[allow(unused_mut)]
            fn $op(mut self, rhs: Matrix<RT, ROWS, COLS>) -> Self::Output {
                $trait::$op(self, &rhs)
            }
        }
    };
}

// Standard default operations
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
    fn general_operations() {
        // Test every 4 ref/ownership for functions T + R -> T and T + R -> O
        todo!("Implement general operations testing");
    }

    #[test]
    fn multipy() {
        const SIZE: usize = 1290;
        let matrix: Matrix<i8, SIZE, SIZE> = Matrix::default();
        let matrix2: Matrix<i8, SIZE, SIZE> = Matrix::default();
        let matrix3 = matrix + &matrix2;
        assert_eq!(matrix3, matrix2);
    }
}
