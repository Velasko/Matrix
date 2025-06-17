use std::{
    any::type_name,
    array::from_fn,
    fmt::{Debug, Display},
    iter::Sum,
    marker::Copy,
    ops::{Add, AddAssign, BitAnd, Index, IndexMut, Mul, MulAssign, Range, Sub},
};

// Operations to do (and by ref):
// Add
// Sub
// Mult by scalar
// divide by scalar
// Determinant
// Transpose
// Inverse

// Is optmized if transposed beforehand
// This is the wrong impl. This is internal product with transpose...
impl<LT, RT, OT, const LROWS: usize, const CSIZE: usize, const RCOLS: usize>
    Mul<&Matrix<RT, CSIZE, RCOLS>> for &Matrix<LT, LROWS, CSIZE>
where
    LT: Copy + for<'a> Mul<&'a RT, Output = OT>,
    OT: Sum,
{
    type Output = Matrix<OT, LROWS, RCOLS>;

    fn mul(self, rhs: &Matrix<RT, CSIZE, RCOLS>) -> Self::Output {
        let matrix: [[OT; RCOLS]; LROWS] =
            // from_fn(|i| from_fn(|j| self.data[i][j] * &rhs.data[j][i]));
            from_fn(|i| from_fn(|j| (0..CSIZE).map(|a| self.data[i][a] * &rhs.data[a][j]).sum() )) ;

        Self::Output::new(matrix)
    }
}

impl<LT, RT, OT, const ROWS: usize, const COLS: usize> Add<&Matrix<RT, ROWS, COLS>>
    for &Matrix<LT, ROWS, COLS>
where
    for<'a, 'b> &'a LT: Add<&'b RT, Output = OT>,
{
    type Output = Matrix<OT, ROWS, COLS>;

    fn add(self, rhs: &Matrix<RT, ROWS, COLS>) -> Self::Output {
        let matrix: [[OT; COLS]; ROWS] =
            from_fn(|i| from_fn(|j| &self.data[i][j] + &rhs.data[i][j]));

        Matrix::new(matrix)
    }
}

impl<LT, RT, const ROWS: usize, const COLS: usize> AddAssign<&Matrix<RT, ROWS, COLS>>
    for Matrix<LT, ROWS, COLS>
where
    for<'a> LT: AddAssign<&'a RT>,
{
    fn add_assign(&mut self, rhs: &Matrix<RT, ROWS, COLS>) {
        for i in 0..ROWS {
            for j in 0..COLS {
                self.data[i][j] += &rhs.data[i][j];
            }
        }
    }
}

impl<LT, RT, OT, const ROWS: usize, const COLS: usize> Sub<&Matrix<RT, ROWS, COLS>>
    for &Matrix<LT, ROWS, COLS>
where
    for<'a, 'b> &'a LT: Sub<&'b RT, Output = OT>,
{
    type Output = Matrix<OT, ROWS, COLS>;

    fn sub(self, rhs: &Matrix<RT, ROWS, COLS>) -> Self::Output {
        let matrix: [[OT; COLS]; ROWS] =
            from_fn(|i| from_fn(|j| &self.data[i][j] - &rhs.data[i][j]));

        Matrix::new(matrix)
    }
}

impl<T, Rhs, const ROWS: usize, const COLS: usize> Mul<Rhs> for Matrix<T, ROWS, COLS>
where
    Rhs: Copy,
    T: MulAssign<Rhs>,
{
    type Output = Self;

    fn mul(mut self, rhs: Rhs) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T, Rhs, const ROWS: usize, const COLS: usize> MulAssign<Rhs> for Matrix<T, ROWS, COLS>
where
    Rhs: Copy,
    T: MulAssign<Rhs>,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        for row in self.data.iter_mut() {
            for cell in row.iter_mut() {
                *cell *= rhs;
            }
        }
    }
}

impl<LT, RT, OT, const ROWS: usize, const COLS: usize> BitAnd<&Matrix<RT, ROWS, COLS>>
    for &Matrix<LT, ROWS, COLS>
where
    for<'a, 'b> &'a LT: BitAnd<&'b RT, Output = OT>,
{
    type Output = Matrix<OT, ROWS, COLS>;
    fn bitand(self, rhs: &Matrix<RT, ROWS, COLS>) -> Self::Output {
        let matrix: [[OT; COLS]; ROWS] =
            from_fn(|i| from_fn(|j| &self.data[i][j] & &rhs.data[i][j]));

        Matrix::new(matrix)
    }
}
