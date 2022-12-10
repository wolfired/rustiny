//!
//!
//!

use rustiny_number::Number;
use rustiny_number::Zero;

use crate::Matrix;

pub trait Transpose {
    type Output;

    fn transpose(self) -> Self::Output;
}

impl<T: Number, const R: usize, const C: usize> Transpose for Matrix<T, R, C>
where
    T: Zero,
{
    type Output = Matrix<T, C, R>;

    fn transpose(self) -> Self::Output {
        let mut raw = [[T::zero(); R]; C];

        for r in 0..R {
            for c in 0..C {
                raw[c][r] = self.0[r][c];
            }
        }

        Matrix::<T, C, R>(raw)
    }
}

impl<T: Number, const R: usize, const C: usize> Transpose for &Matrix<T, R, C>
where
    T: Zero,
{
    type Output = Matrix<T, C, R>;

    fn transpose(self) -> Self::Output {
        let mut raw = [[T::zero(); R]; C];

        for r in 0..R {
            for c in 0..C {
                raw[c][r] = self.0[r][c];
            }
        }

        Matrix::<T, C, R>(raw)
    }
}
