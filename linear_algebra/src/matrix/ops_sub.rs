//!
//!
//!

use std::ops::Sub;
use std::ops::SubAssign;

use rustiny_number::Number;

use crate::Matrix;

impl<T: Number, const R: usize, const C: usize> Sub<Self> for Matrix<T, R, C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];
        for r in 0..R {
            for c in 0..C {
                raw[r][c] = self.0[r][c] - rhs.0[r][c];
            }
        }
        Self(raw)
    }
}

impl<T: Number, const R: usize, const C: usize> Sub<&Self> for Matrix<T, R, C> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];
        for r in 0..R {
            for c in 0..C {
                raw[r][c] = self.0[r][c] - rhs.0[r][c];
            }
        }
        Self(raw)
    }
}

impl<T: Number, const R: usize, const C: usize> Sub<Self> for &Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];
        for r in 0..R {
            for c in 0..C {
                raw[r][c] = self.0[r][c] - rhs.0[r][c];
            }
        }
        Matrix::<T, R, C>(raw)
    }
}

impl<T: Number, const R: usize, const C: usize> Sub<Matrix<T, R, C>> for &Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn sub(self, rhs: Matrix<T, R, C>) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];
        for r in 0..R {
            for c in 0..C {
                raw[r][c] = self.0[r][c] - rhs.0[r][c];
            }
        }
        Matrix::<T, R, C>(raw)
    }
}

impl<T: Number, const R: usize, const C: usize> SubAssign<Self> for Matrix<T, R, C> {
    fn sub_assign(&mut self, rhs: Self) {
        for r in 0..R {
            for c in 0..C {
                self.0[r][c] -= rhs.0[r][c];
            }
        }
    }
}

impl<T: Number, const R: usize, const C: usize> SubAssign<&Self> for Matrix<T, R, C> {
    fn sub_assign(&mut self, rhs: &Self) {
        for r in 0..R {
            for c in 0..C {
                self.0[r][c] -= rhs.0[r][c];
            }
        }
    }
}
