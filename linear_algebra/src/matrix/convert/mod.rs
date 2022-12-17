//!
//!
//!

use rustiny_number::types::Number;

use super::types::Matrix;

impl<T: Number, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T, R, C> {
    fn from(raw: [[T; C]; R]) -> Self {
        Self(raw)
    }
}

impl<T: Number, const R: usize, const C: usize> From<&[[T; C]; R]> for Matrix<T, R, C> {
    fn from(raw: &[[T; C]; R]) -> Self {
        Self(*raw)
    }
}
