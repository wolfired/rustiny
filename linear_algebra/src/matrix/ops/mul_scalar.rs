//!
//!
//!

use rustiny_number::types::Number;

use crate::matrix::types::Matrix;

pub trait MulScalar<Rhs> {
    type Output;

    fn mul_scalar(self, rhs: Rhs) -> Self::Output;
}

impl<T: Number, const R: usize, const C: usize> MulScalar<T> for Matrix<T, R, C> {
    type Output = Self;

    fn mul_scalar(self, rhs: T) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];

        for r in 0..R {
            for c in 0..C {
                raw[r][c] = self.0[r][c] * rhs;
            }
        }

        Self(raw)
    }
}

impl<T: Number, const R: usize, const C: usize> MulScalar<&T> for Matrix<T, R, C> {
    type Output = Self;

    fn mul_scalar(self, rhs: &T) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];

        for r in 0..R {
            for c in 0..C {
                raw[r][c] = self.0[r][c] * *rhs;
            }
        }

        Self(raw)
    }
}

impl<T: Number, const R: usize, const C: usize> MulScalar<&T> for &Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn mul_scalar(self, rhs: &T) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];

        for r in 0..R {
            for c in 0..C {
                raw[r][c] = self.0[r][c] * *rhs;
            }
        }

        Matrix::<T, R, C>(raw)
    }
}

impl<T: Number, const R: usize, const C: usize> MulScalar<T> for &Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn mul_scalar(self, rhs: T) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];

        for r in 0..R {
            for c in 0..C {
                raw[r][c] = self.0[r][c] * rhs;
            }
        }

        Matrix::<T, R, C>(raw)
    }
}

pub trait MulScalarAssign<Rhs> {
    fn mul_scalar_assign(&mut self, rhs: Rhs);
}

impl<T: Number, const R: usize, const C: usize> MulScalarAssign<T> for Matrix<T, R, C> {
    fn mul_scalar_assign(&mut self, rhs: T) {
        for r in 0..R {
            for c in 0..C {
                self.0[r][c] *= rhs;
            }
        }
    }
}

impl<T: Number, const R: usize, const C: usize> MulScalarAssign<&T> for Matrix<T, R, C> {
    fn mul_scalar_assign(&mut self, rhs: &T) {
        for r in 0..R {
            for c in 0..C {
                self.0[r][c] *= *rhs;
            }
        }
    }
}
