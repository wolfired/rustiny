//!
//!
//!

use std::ops::Add;
use std::ops::AddAssign;

use rustiny_number::Number;
use rustiny_number::Zero;

use crate::Matrix;

impl<T: Number, const R: usize, const C: usize> Add<Self> for Matrix<T, R, C>
where
    T: Add<T, Output = T>,
    T: Zero,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];
        for r in 0..R {
            for c in 0..C {
                raw[r][c] = self.0[r][c] + rhs.0[r][c];
            }
        }
        Self(raw)
    }
}

impl<T: Number, const R: usize, const C: usize> Add<&Self> for Matrix<T, R, C>
where
    for<'a> T: Add<&'a T, Output = T>,
    T: Zero,
{
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];
        for r in 0..R {
            for c in 0..C {
                raw[r][c] = self.0[r][c] + &rhs.0[r][c];
            }
        }
        Self(raw)
    }
}

impl<T: Number, const R: usize, const C: usize> Add<Self> for &Matrix<T, R, C>
where
    for<'a> &'a T: Add<&'a T, Output = T>,
    T: Zero,
{
    type Output = Matrix<T, R, C>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];
        for r in 0..R {
            for c in 0..C {
                raw[r][c] = &self.0[r][c] + &rhs.0[r][c];
            }
        }
        Matrix::<T, R, C>(raw)
    }
}

impl<T: Number, const R: usize, const C: usize> Add<Matrix<T, R, C>> for &Matrix<T, R, C>
where
    for<'a> &'a T: Add<T, Output = T>,
    T: Zero,
{
    type Output = Matrix<T, R, C>;

    fn add(self, rhs: Matrix<T, R, C>) -> Self::Output {
        let mut raw = [[T::zero(); C]; R];
        for r in 0..R {
            for c in 0..C {
                raw[r][c] = &self.0[r][c] + rhs.0[r][c];
            }
        }
        Matrix::<T, R, C>(raw)
    }
}

impl<T: Number, const R: usize, const C: usize> AddAssign<Self> for Matrix<T, R, C>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        for r in 0..R {
            for c in 0..C {
                self.0[r][c] += rhs.0[r][c];
            }
        }
    }
}

impl<T: Number, const R: usize, const C: usize> AddAssign<&Self> for Matrix<T, R, C>
where
    for<'a> T: AddAssign<&'a T>,
{
    fn add_assign(&mut self, rhs: &Self) {
        for r in 0..R {
            for c in 0..C {
                self.0[r][c] += &rhs.0[r][c];
            }
        }
    }
}
