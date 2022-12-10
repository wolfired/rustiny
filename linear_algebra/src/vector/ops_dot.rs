//!
//!
//!

use std::ops::AddAssign;
use std::ops::Mul;

use rustiny_number::Number;
use rustiny_number::Zero;

use crate::Vector;

pub trait Dot<Rhs = Self> {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}

impl<T: Number, const N: usize> Dot<Self> for Vector<T, N>
where
    T: AddAssign<T>,
    T: Mul<T, Output = T>,
    T: Zero,
{
    type Output = T;

    fn dot(self, rhs: Self) -> Self::Output {
        let mut result = T::zero();

        for c in 0..N {
            result += self.0[0][c] * rhs.0[0][c];
        }

        result
    }
}

impl<T: Number, const N: usize> Dot<&Self> for Vector<T, N>
where
    T: AddAssign<T>,
    for<'a> T: Mul<&'a T, Output = T>,
    T: Zero,
{
    type Output = T;

    fn dot(self, rhs: &Self) -> Self::Output {
        let mut result = T::zero();

        for c in 0..N {
            result += self.0[0][c] * &rhs.0[0][c];
        }

        result
    }
}

impl<T: Number, const N: usize> Dot<Self> for &Vector<T, N>
where
    T: AddAssign<T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
    T: Zero,
{
    type Output = T;

    fn dot(self, rhs: Self) -> Self::Output {
        let mut result = T::zero();

        for c in 0..N {
            result += &self.0[0][c] * &rhs.0[0][c];
        }

        result
    }
}

impl<T: Number, const N: usize> Dot<Vector<T, N>> for &Vector<T, N>
where
    T: AddAssign<T>,
    for<'a> &'a T: Mul<T, Output = T>,
    T: Zero,
{
    type Output = T;

    fn dot(self, rhs: Vector<T, N>) -> Self::Output {
        let mut result = T::zero();

        for c in 0..N {
            result += &self.0[0][c] * rhs.0[0][c];
        }

        result
    }
}
