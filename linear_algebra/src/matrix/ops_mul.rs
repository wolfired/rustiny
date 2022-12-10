//!
//!
//!

use std::ops::AddAssign;
use std::ops::Mul;

use rustiny_number::Number;
use rustiny_number::Zero;

use crate::Matrix;

impl<T: Number, const R: usize, const M: usize, const C: usize> Mul<Matrix<T, M, C>>
    for Matrix<T, R, M>
where
    T: AddAssign<T>,
    T: Mul<T, Output = T>,
    T: Zero,
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: Matrix<T, M, C>) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];

        for r in 0..R {
            for m in 0..M {
                for c in 0..C {
                    raw[r][c] += self.0[r][m] * rhs.0[m][c];
                }
            }
        }

        Matrix::<T, R, C>(raw)
    }
}

impl<T: Number, const R: usize, const M: usize, const C: usize> Mul<&Matrix<T, M, C>>
    for Matrix<T, R, M>
where
    T: AddAssign<T>,
    for<'a> T: Mul<&'a T, Output = T>,
    T: Zero,
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: &Matrix<T, M, C>) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];

        for r in 0..R {
            for m in 0..M {
                for c in 0..C {
                    raw[r][c] += self.0[r][m] * &rhs.0[m][c];
                }
            }
        }

        Matrix::<T, R, C>(raw)
    }
}

impl<T: Number, const R: usize, const M: usize, const C: usize> Mul<&Matrix<T, M, C>>
    for &Matrix<T, R, M>
where
    for<'a> T: AddAssign<&'a T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
    T: Zero,
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: &Matrix<T, M, C>) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];

        for r in 0..R {
            for m in 0..M {
                for c in 0..C {
                    raw[r][c] += &(&self.0[r][m] * &rhs.0[m][c]);
                }
            }
        }

        Matrix::<T, R, C>(raw)
    }
}

impl<T: Number, const R: usize, const M: usize, const C: usize> Mul<Matrix<T, M, C>>
    for &Matrix<T, R, M>
where
    T: AddAssign<T>,
    for<'a> &'a T: Mul<T, Output = T>,
    T: Zero,
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: Matrix<T, M, C>) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];

        for r in 0..R {
            for m in 0..M {
                for c in 0..C {
                    raw[r][c] += &self.0[r][m] * rhs.0[m][c];
                }
            }
        }

        Matrix::<T, R, C>(raw)
    }
}
