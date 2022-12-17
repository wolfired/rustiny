//!
//!
//!

use rustiny_number::Number;

use crate::Vector;

impl<T: Number, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(raw: [T; N]) -> Self {
        Self([raw; 1])
    }
}

impl<T: Number, const N: usize> From<&[T; N]> for Vector<T, N> {
    fn from(raw: &[T; N]) -> Self {
        Self([*raw; 1])
    }
}
