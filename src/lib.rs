#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod base;
mod general_operations;
mod index;
mod matrix_op;
mod square_matrix;
mod vector;

// Operations to do (and by ref):
// Add
// Sub
// Mult by scalar
// divide by scalar
// Determinant
// Transpose
// Inverse

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overall_testing() {
        todo!("Implement overall testing");
    }

    #[test]
    fn memory_test() {
        todo!("Implement memory testing");
        // What is the size limit the matrix can have before an overflow ?
    }
}
