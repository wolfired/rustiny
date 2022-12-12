//!
//!
//!

use std::ops::AddAssign;
use std::ops::Mul;

use rustiny_number::CheckedSqrt;
use rustiny_number::Number;
use rustiny_number::Zero;

use crate::Matrix;

pub type Vector<T, const N: usize> = Matrix<T, 1, N>;
pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;

impl<T: Number, const N: usize> Vector<T, N>
where
    T: AddAssign<T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
    T: CheckedSqrt,
    T: Zero,
{
    pub fn magnitude(&self) -> T {
        let mut result = T::zero();

        for c in 0..N {
            result += &self.0[0][c] * &self.0[0][c];
        }

        result.checked_sqrt().unwrap()
    }
}
