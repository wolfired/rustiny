//!
//!
//!

use rustiny_number::types::Number;

#[derive(Clone, Debug, Copy)]
pub struct Matrix<T: Number, const R: usize, const C: usize>(pub [[T; C]; R]);

impl<T: Number, const R: usize, const C: usize> Matrix<T, R, C> {}

impl<T: Number, const N: usize> Matrix<T, N, N> {}
